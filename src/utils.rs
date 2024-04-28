use std::ops::Range;

use glam::DVec3;
use rand::Rng;

pub const PI: f64 = 3.1415926535897932385;

pub fn deg2rad(deg: f64) -> f64 {
    deg * PI / 100.0
}
#[inline(always)]
pub fn randf64(range: Option<Range<f64>>) -> f64 {
    let mut rng = rand::thread_rng();
    if let Some(range) = range {
        rng.gen_range(range)
    } else {
        rng.gen_range(0.0..=1.0)
    }
}

pub fn random_in_unit_sphere() -> DVec3 {
    let mut rng = rand::thread_rng();
    loop {
        let vec = DVec3::new(
            rng.gen_range(-1.0..1.),
            rng.gen_range(-1.0..1.),
            rng.gen_range(-1.0..1.),
        );

        if vec.length_squared() < 1. {
            break vec;
        }
    }
}
pub fn random_unit_vector() -> DVec3 {
    return random_in_unit_sphere().normalize();
}
pub fn random_on_hemisphere(normal: &DVec3) -> DVec3 {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(*normal) > 0.0
    // In the same hemisphere as the normal
    {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}
pub fn random_in_unit_disk() -> DVec3 {
    let mut rng = rand::thread_rng();
    loop {
        let v = DVec3::new(rng.gen_range(-1.0..1.), rng.gen_range(-1.0..1.), 0.);

        if v.length_squared() < 1. {
            break v;
        }
    }
}
