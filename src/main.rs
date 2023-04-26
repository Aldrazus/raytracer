use material::{Material, ScatterModel};
use rand::prelude::*;
use std::{rc::Rc, fs::File, path::Path, io::{Write, stdout}};
use vec3::random_in_range;

use hittable::Hittable;
use ray::Ray;

use crate::{
    camera::Camera,
    hittable::{HittableList, Sphere},
    material::{Dielectric, Lambertian, Metal},
    vec3::{unit_vector, write_color, Color, Point3},
};

use glam::Vec3;

mod camera;
mod hittable;
mod material;
mod ray;
mod util;
mod vec3;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Vec3::new(0., 0., 0.);
    }

    if let Some(hit) = world.hit(r, 0.001, f32::INFINITY) {
        if let Some(ScatterModel {
            attenuation,
            scattered,
        }) = hit.material.scatter(r, &hit)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
    }
    let unit_direction = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = random();
            let center: Point3 = Vec3::new(
                a as f32 + 0.9 * random::<f32>(),
                0.2,
                b as f32 + 0.9 * random::<f32>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material> = match choose_mat {
                    m if m < 0.8 => {
                        // diffuse
                        let albedo = random::<Vec3>() + random::<Vec3>();
                        Rc::new(Lambertian::new(albedo))
                    }
                    m if m < 0.95 => {
                        // metal
                        let albedo = random_in_range(0.5, 1.0);
                        let fuzz = thread_rng().gen_range(0.0..0.5);
                        Rc::new(Metal::new(albedo, fuzz))
                    }
                    _ => {
                        // glass
                        Rc::new(Dielectric::new(1.5))
                    }
                };
                world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn write_ppm(buf: Vec<u8>, image_width: i32, image_height: i32) {
    let mut out = stdout().lock();
    writeln!(out, "P3\n{} {}\n255", image_width, image_height).unwrap();
    buf.chunks_exact(3).for_each(|chunk| {
        if let [r, g, b] = chunk {
            writeln!(out, "{} {} {}", r, g, b).unwrap();
        }
    });
}

fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    const SAMPLES_PER_PIXEL: i32 = 50;
    const MAX_DEPTH: i32 = 10;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Render

    let mut buf = Vec::<u8>::with_capacity((image_height * image_width * 3) as usize);

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color: Color = Vec3::new(0., 0., 0.);
            for _sample in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + random::<f32>()) / (image_width - 1) as f32;
                let v = (j as f32 + random::<f32>()) / (image_height - 1) as f32;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            let (r, g, b) = write_color(pixel_color, SAMPLES_PER_PIXEL);
            buf.push(r);
            buf.push(g);
            buf.push(b);
        }
    }

    write_ppm(buf, image_width, image_height);

    eprintln!("\nDone.");
}
