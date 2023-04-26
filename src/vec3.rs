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

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn near_zero(v: Vec3) -> bool {
    let s = 1e-8;
    f32::abs(v.x) < s && f32::abs(v.y) < s && f32::abs(v.z) < s
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2. * Vec3::dot(*v, *n) * *n
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = (-*uv).dot(*n).min(1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *n;
    r_out_perp + r_out_parallel
}

pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) -> (u8, u8, u8) {
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

        ((256. * r.clamp(0., 0.999)) as u8,
        (256. * g.clamp(0., 0.999)) as u8,
        (256. * b.clamp(0., 0.999)) as u8)
}
