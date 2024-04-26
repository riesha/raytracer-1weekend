use glam::DVec3;

pub fn write_color(buf: &mut Vec<u8>, color: DVec3) {
    let intensity_range = 0.0..=0.999;
    let rbyte = (255.999
        * color
            .x
            .clamp(*intensity_range.start(), *intensity_range.end())) as i32;
    let gbyte = (255.999
        * color
            .y
            .clamp(*intensity_range.start(), *intensity_range.end())) as i32;
    let bbyte = (255.999
        * color
            .z
            .clamp(*intensity_range.start(), *intensity_range.end())) as i32;

    buf.extend_from_slice(format!("{rbyte} {gbyte} {bbyte} \n").as_bytes());
}
