use glam::{dvec3, DVec3};

use crate::hittable::{HitRecord, Hittable};

pub struct Ray {
    origin: DVec3,
    direction: DVec3,
}
impl Ray {
    pub fn new(origin: DVec3, direction: DVec3) -> Self {
        Self { origin, direction }
    }
    #[inline(always)]
    pub fn origin(&self) -> &DVec3 {
        return &self.origin;
    }
    #[inline(always)]
    pub fn direction(&self) -> &DVec3 {
        return &self.direction;
    }
    #[inline(always)]
    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
    pub fn color(&self, world: &impl Hittable) -> DVec3 {
        let mut hit_record = HitRecord::default();
        if world.hit(&self, 0.001, f64::INFINITY, &mut hit_record) {
            return 0.5 * (hit_record.normal + dvec3(1.0, 1.0, 1.0));
        }

        let unit_dir = *self.direction() / self.direction().length();
        let a = 0.5 * (unit_dir.y + 1.0);

        return (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0);
    }
}
