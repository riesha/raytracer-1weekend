use std::ops::Range;

use glam::{dvec3, BVec3, DVec3};

use crate::hittable::Hittable;
#[derive(Default)]
pub struct Sphere {
    center: DVec3,
    radius: f64,
}
impl Sphere {
    pub fn new(center: DVec3, radius: f64) -> Self {
        Self { center, radius }
    }
}
impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_tmin: f64,
        ray_tmax: f64,
        hit_record: &mut crate::hittable::HitRecord,
    ) -> bool {
        let oc = *ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let h = oc.dot(*ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discrim = h * h - a * c;
        let range = ray_tmin..ray_tmax;
        if discrim < 0.0 {
            return false;
        }

        let sqrt_discrim = discrim.sqrt();

        let mut root = (-h - sqrt_discrim) / a;

        if !range.contains(&root) {
            root = (-h + sqrt_discrim) / a;

            if !range.contains(&root) {
                return false;
            }
        }
        hit_record.t = root;
        hit_record.point = ray.at(root);

        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(&ray, &outward_normal);

        return true;
    }
}
