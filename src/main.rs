use glam::dvec3;

use raytracer_1weekend::{camera::Camera, shapes::Sphere};

fn main() -> std::io::Result<()> {
    let mut hittable_list: Vec<Sphere> = Vec::new();

    hittable_list.push(Sphere::new(dvec3(0.0, 0.0, -1.0), 0.5));
    hittable_list.push(Sphere::new(dvec3(0.0, -100.5, -1.0), 100.0));

    let camera = Camera::new(16.0 / 9.0, 400, 50);

    camera.render(&hittable_list)?;
    Ok(())
}
