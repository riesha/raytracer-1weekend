use std::{fs::File, io::Write, time::Instant};

use glam::{dvec3, DVec3};
use indicatif::ProgressIterator;
use itertools::Itertools;

use crate::{color::write_color, hittable::Hittable, ray::Ray, utils::randf64};
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
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32) -> Self {
        let image_height =
            ((image_width as f64 / aspect_ratio) as i32).clamp(1, image_width as i32);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let center = dvec3(0.0, 0.0, 0.0);

        // U = horizontal +X // V = vertical -Y
        let viewport_u = dvec3(viewport_width, 0.0, 0.0);
        let viewport_v = dvec3(0.0, -viewport_height, 0.0);

        // Space between pixels for uniform distribution
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - dvec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

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
        let ray_origin = self.center;
        let ray_dir = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_dir)
    }
    #[inline(always)]
    fn sample_square() -> DVec3 {
        dvec3(randf64(None) - 0.5, randf64(None) - 0.5, 0.)
    }
}
