use std::{fs::File, io::Write, time::Instant};

use glam::{dvec3, DVec3};
use indicatif::ProgressIterator;
use itertools::Itertools;

use crate::{
    color::write_color,
    hittable::Hittable,
    ray::Ray,
    utils::{deg2rad, randf64, random_in_unit_disk},
};
#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    image_height: i32,
    center: DVec3,
    pixel00_loc: DVec3,
    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3,
    samples_per_pixel: i32,
    pixel_samples_scale: f64,
    max_depth: u32,
    fov: f64,
    lookfrom: DVec3,
    lookat: DVec3,
    vup: DVec3,
    u: DVec3,
    v: DVec3,
    w: DVec3,
    defocus_angle: f64,
    focus_dist: f64,
    defocus_disk_u: DVec3,
    defocus_disk_v: DVec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        fov: f64,
        samples_per_pixel: i32,
        lookfrom: DVec3,
        lookat: DVec3,
        vup: DVec3,
        focus_dist: f64,
        defocus_angle: f64,
    ) -> Self {
        let image_height =
            ((image_width as f64 / aspect_ratio) as i32).clamp(1, image_width as i32);

        let theta = fov.to_radians();
        let h = (theta / 2.0).tan();

        let center = lookfrom;

        // let focal_length = (lookfrom - lookat).length();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        // U = horizontal +X // V = vertical -Y
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * (-v);

        // Space between pixels for uniform distribution
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;
        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth: 50,
            fov,
            lookfrom,
            lookat,
            vup,
            u,
            v,
            w,
            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
    pub fn render(&self, world: &impl Hittable) -> std::io::Result<()> {
        let mut file = File::create("image.ppm")?;
        let mut image_buffer: Vec<u8> = Vec::new();

        image_buffer.extend_from_slice(
            format!("P3\n{} {} \n255\n", self.image_width, self.image_height).as_bytes(),
        );

        let time = Instant::now();

        (0..self.image_height)
            .cartesian_product(0..self.image_width)
            .progress_count((self.image_height * self.image_width) as u64)
            .for_each(|(j, i)| {
                let mut pixel_color = dvec3(0., 0., 0.);

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += ray.color(world, self.max_depth);
                }

                write_color(&mut image_buffer, self.pixel_samples_scale * pixel_color);
            });

        file.write_all(&image_buffer)?;
        println!("Render done! Took {}ms", time.elapsed().as_millis());
        Ok(())
    }
    fn get_ray(&self, width: i32, height: i32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((width as f64 + offset.x) * self.pixel_delta_u)
            + ((height as f64 + offset.y) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_dir = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_dir)
    }
    #[inline(always)]
    fn sample_square() -> DVec3 {
        dvec3(randf64(None) - 0.5, randf64(None) - 0.5, 0.)
    }
    fn defocus_disk_sample(&self) -> DVec3 {
        let p = random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}
