use ray::Ray;

use crate::vec3::{write_color, Color, Point3, Vec3};

mod ray;
mod vec3;

fn ray_color(r: &Ray) -> Color {
    if let Some(t) = hit_sphere(&Vec3(0., 0., -1.), 0.5, r) {
        let normal = Vec3::unit_vector(r.at(t) - Vec3(0., 0., -1.));
        return 0.5 * Vec3(normal.x() + 1., normal.y() + 1., normal.z() + 1.);
    }
    let unit_direction = Vec3::unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> Option<f64> {
    let oc = r.origin - *center;
    let a = r.direction.length_squared();
    let half_b = Vec3::dot(oc, r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    
    if discriminant < 0. {
        None
    } else {
        Some((-half_b - f64::sqrt(discriminant)) / a)
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin: Point3 = Vec3(0., 0., 0.);
    let horizontal = Vec3(viewport_width, 0., 0.);
    let vertical = Vec3(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal / 2. - vertical / 2. - Vec3(0., 0., focal_length);

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r);
            write_color(pixel_color);
        }
    }

    eprintln!("\nDone.");
}
