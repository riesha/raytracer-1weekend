use glam::DVec3;
#[inline(always)]
fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0. {
        return linear.sqrt();
    }
    return 0.;
}
pub fn write_color(buf: &mut Vec<u8>, color: DVec3) {
    let intensity_range = 0.0..=0.999;
    let r = linear_to_gamma(color.x);
    let g = linear_to_gamma(color.y);
    let b = linear_to_gamma(color.z);
    let rbyte = (255.999 * r.clamp(*intensity_range.start(), *intensity_range.end())) as i32;
    let gbyte = (255.999 * g.clamp(*intensity_range.start(), *intensity_range.end())) as i32;
    let bbyte = (255.999 * b.clamp(*intensity_range.start(), *intensity_range.end())) as i32;

    buf.extend_from_slice(format!("{rbyte} {gbyte} {bbyte} \n").as_bytes());
}
