use glam::Vec3;
use rand::random;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{
        near_zero, random_in_unit_sphere, random_unit_vector, reflect, refract, unit_vector, Color,
    },
};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterModel>;
}

pub struct ScatterModel {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterModel> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }
        Some(ScatterModel {
            attenuation: self.albedo,
            scattered: Ray::new(rec.p, scatter_direction),
        })
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzziness: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f32) -> Self {
        Self { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterModel> {
        let reflected = reflect(&unit_vector(r_in.direction), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzziness * random_in_unit_sphere());
        let attenuation = self.albedo;
        if scattered.direction.dot(rec.normal) > 0. {
            return Some(ScatterModel {
                attenuation,
                scattered,
            });
        }
        None
    }
}

pub struct Dielectric {
    pub ir: f32,
}

impl Dielectric {
    pub fn new(ir: f32) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1. - ref_idx) / (1. + ref_idx);
        let r0 = r0 * r0;
        r0 + (1. - r0) * f32::powi(1.0 - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterModel> {
        let attenuation = Vec3::new(1., 1., 1.);
        let refraction_ratio = if rec.front_face {
            1. / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction.normalize();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random() {
                reflect(&unit_direction, &rec.normal)
            } else {
                refract(&unit_direction, &rec.normal, refraction_ratio)
            };

        let scattered = Ray::new(rec.p, direction);
        Some(ScatterModel {
            attenuation,
            scattered,
        })
    }
}
