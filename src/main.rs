use rand::prelude::*;
use std::rc::Rc;

use hittable::Hittable;
use ray::Ray;

use crate::{
    camera::Camera,
    hittable::{HittableList, Sphere},
    vec3::{write_color, Color, Point3, Vec3},
};

mod camera;
mod hittable;
mod ray;
mod util;
mod vec3;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Vec3(0., 0., 0.);
    }

    if let Some(hit) = world.hit(r, 0.001, f64::INFINITY) {
        let target = hit.p + Vec3::random_in_hemisphere(&hit.normal);
        return 0.5 * ray_color(&Ray::new(hit.p, target - hit.p), world, depth - 1);
    }
    let unit_direction = Vec3::unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere {
        center: Vec3(0., 0., -1.),
        radius: 0.5,
    }));
    world.add(Rc::new(Sphere {
        center: Vec3(0., -100.5, -1.),
        radius: 100.,
    }));

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color: Color = Vec3(0., 0., 0.);
            for _sample in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + random::<f64>()) / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone.");
}
