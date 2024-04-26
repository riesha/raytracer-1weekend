use glam::DVec3;

use crate::ray::Ray;
#[derive(Default, Copy, Clone)]
pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &DVec3) {
        self.front_face = ray.direction().dot(*outward_normal) < 0.0;
        self.normal = if self.front_face == true {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}
pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, hit_record: &mut HitRecord) -> bool;
}

impl<T> Hittable for Vec<T>
where
    T: Hittable + Sync,
{
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();

        let (_closest, hit_anything) = self.iter().fold((ray_tmax, false), |acc, item| {
            if item.hit(ray, ray_tmin, acc.0, &mut temp_rec) {
                (temp_rec.t, true)
            } else {
                acc
            }
        });
        *hit_record = temp_rec;
        return hit_anything;
    }
}
