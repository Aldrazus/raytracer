use crate::{ray::Ray, hittable::HitRecord, vec3::{Color, Vec3}};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterModel>;
}

pub struct ScatterModel {
    pub attenuation: Color,
    pub scattered: Ray
}

pub struct Lambertian {
    pub albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterModel> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();   

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        Some(ScatterModel { attenuation: self.albedo, scattered: Ray::new(rec.p, scatter_direction)})
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzziness: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f64) -> Self {
        Self { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterModel> {
        let reflected = Vec3::reflect(&Vec3::unit_vector(r_in.direction), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzziness * Vec3::random_in_unit_sphere());
        let attenuation = self.albedo;
        if Vec3::dot(scattered.direction, rec.normal) > 0. {
            return Some(ScatterModel { attenuation, scattered });
        }
        None
    }
}
