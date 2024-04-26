use glam::{dvec3, DVec3};

use crate::{
    hittable::{HitRecord, Hittable},
    utils::random_on_hemisphere,
};

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
    pub fn color(&self, world: &impl Hittable, depth: u32) -> DVec3 {
        if depth <= 0 {
            return DVec3::ZERO;
        }
        let mut hit_record = HitRecord::default();
        if world.hit(&self, 0.001, f64::INFINITY, &mut hit_record) {
            let direction = random_on_hemisphere(&hit_record.normal);
            let ray = Ray::new(hit_record.point, direction);
            return 0.5 * ray.color(world, depth - 1);
        }

        let unit_dir = *self.direction() / self.direction().length();
        let a = 0.5 * (unit_dir.y + 1.0);

        return (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0);
    }
}
