use std::ops::Range;

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
