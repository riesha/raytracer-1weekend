use glam::DVec3;

use crate::{hittable::HitRecord, ray::Ray, utils::random_unit_vector};
#[non_exhaustive]
#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: DVec3 },
    Metal { albedo: DVec3, fuzz: f64 },
}

pub struct Scatter {
    pub attenuation: DVec3,
    pub scattered: Ray,
}

impl Material {
    pub fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut DVec3,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_dir = hit_record.normal + random_unit_vector();
                if scatter_dir.abs_diff_eq(DVec3::ZERO, 1e-8) {
                    scatter_dir = hit_record.normal;
                }
                *scattered = Ray::new(hit_record.point, scatter_dir);
                *attenuation = *albedo;
                return true;
            }
            Material::Metal { albedo, fuzz } => {
                let mut reflected = reflect(ray.direction().normalize(), hit_record.normal);
                reflected = reflected.normalize() + (*fuzz * random_unit_vector());

                *scattered = Ray::new(hit_record.point, reflected);
                *attenuation = *albedo;

                return scattered.direction().dot(hit_record.normal) > 0.;
            }
        }
    }
}
impl Default for Material {
    fn default() -> Self {
        Self::Lambertian {
            albedo: DVec3::ZERO,
        }
    }
}

pub fn reflect(v: DVec3, n: DVec3) -> DVec3 {
    return v - 2. * v.dot(n) * n;
}
