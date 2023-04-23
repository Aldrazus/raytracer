use material::ScatterModel;
use rand::prelude::*;
use std::rc::Rc;

use hittable::Hittable;
use ray::Ray;

use crate::{
    camera::Camera,
    hittable::{HittableList, Sphere},
    material::{Lambertian, Metal},
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

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Vec3::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

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
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone.");
}
