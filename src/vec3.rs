use glam::Vec3;
use rand::prelude::*;

#[inline]
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

#[inline]
pub fn random() -> Vec3 {
    thread_rng().gen()
}

#[inline]
pub fn random_in_range(min: f32, max: f32) -> Vec3 {
    let mut rng = thread_rng();
    let x = rng.gen_range(min..max);
    let y = rng.gen_range(min..max);
    let z = rng.gen_range(min..max);
    Vec3::new(x, y, z)
}

#[inline]
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_in_range(-1., 1.);
        if p.length_squared() >= 1. {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0. {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn near_zero(v: Vec3) -> bool {
    let s = 1e-8;
    f32::abs(v.x) < s && f32::abs(v.y) < s && f32::abs(v.z) < s
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2. * Vec3::dot(*v, *n) * *n
}

pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let Vec3 {
        x: mut r,
        y: mut g,
        z: mut b,
    } = pixel_color;

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f32;
    r = f32::sqrt(scale * r);
    g = f32::sqrt(scale * g);
    b = f32::sqrt(scale * b);

    println!(
        "{} {} {}",
        (256. * r.clamp(0., 0.999)) as i32,
        (256. * g.clamp(0., 0.999)) as i32,
        (256. * b.clamp(0., 0.999)) as i32
    );
}
