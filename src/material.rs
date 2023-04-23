use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{near_zero, random_in_unit_sphere, random_unit_vector, reflect, unit_vector, Color},
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
