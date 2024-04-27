use glam::{dvec3, DVec3};

use crate::{
    hittable::HitRecord,
    ray::Ray,
    utils::{randf64, random_unit_vector},
};
use std::ops::Neg;
#[non_exhaustive]
#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: DVec3 },
    Metal { albedo: DVec3, fuzz: f64 },
    Dielectric { index_of_refraction: f64 },
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
            Material::Dielectric {
                index_of_refraction,
            } => {
                *attenuation = dvec3(1.0, 1.0, 1.0);
                let ri = if hit_record.front_face {
                    1.0 / index_of_refraction
                } else {
                    *index_of_refraction
                };
                let unit_dir = ray.direction().normalize();
                // let refracted = refract(&unit_dir, hit_record.normal, ri);
                let cos_theta = unit_dir.dot(hit_record.normal).neg().min(1.0);
                let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();
                let cannot_refract = ri * sin_theta > 1.0;

                let direction = if cannot_refract || reflectance(cos_theta, ri) > randf64(None) {
                    reflect(unit_dir, hit_record.normal)
                } else {
                    refract(&unit_dir, hit_record.normal, ri)
                };

                *scattered = Ray::new(hit_record.point, direction);
                return true;
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
pub fn refract(uv: &DVec3, n: DVec3, etai: f64) -> DVec3 {
    let cos_theta = uv.neg().dot(n).min(1.0);
    let out_perp = etai * (*uv + cos_theta * n);
    let out_parallel = (1.0 - out_perp.length_squared()).abs().sqrt().neg() * n;
    out_perp + out_parallel
}
pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;
    return r0 + (1. - r0) * (1. - cosine).powf(5.);
}
