use glam::{dvec3, DVec3};

use itertools::Itertools;
use rand::Rng;
use raytracer_1weekend::{camera::Camera, materials::Material, shapes::Sphere};

fn main() -> std::io::Result<()> {
    let mut world: Vec<Sphere> = Vec::new();

    world.push(Sphere::new(
        DVec3::new(0., -1000., 0.),
        1000.,
        Material::Lambertian {
            albedo: DVec3::new(0.5, 0.5, 0.5).into(),
        },
    ));
    let mut rng = rand::thread_rng();
    for (a, b) in (-11..11).cartesian_product(-11..11).into_iter() {
        let choose_mat = rng.gen::<f64>();
        let center = DVec3::new(
            a as f64 + 0.9 * rng.gen::<f64>(),
            0.2,
            b as f64 + 0.9 * rng.gen::<f64>(),
        );

        if (center - DVec3::new(4., 0.2, 0.)).length() > 0.9 {
            let material = if choose_mat < 0.8 {
                // diffuse
                let albedo = DVec3::new(
                    rng.gen_range(0f64..1.),
                    rng.gen_range(0f64..1.),
                    rng.gen_range(0f64..1.),
                ) * DVec3::new(
                    rng.gen_range(0f64..1.),
                    rng.gen_range(0f64..1.),
                    rng.gen_range(0f64..1.),
                );
                Material::Lambertian {
                    albedo: albedo.into(),
                }
            } else if choose_mat < 0.95 {
                // metal
                let albedo = DVec3::new(
                    rng.gen_range(0.5..1.),
                    rng.gen_range(0.5..1.),
                    rng.gen_range(0.5..1.),
                );
                let fuzz = rng.gen_range(0f64..0.5);

                Material::Metal { albedo, fuzz }
            } else {
                // glass
                Material::Dielectric {
                    index_of_refraction: 1.5,
                }
            };

            world.push(Sphere::new(center, 0.2, material));
        }
    }

    world.push(Sphere::new(
        DVec3::new(0., 1., 0.),
        1.0,
        Material::Dielectric {
            index_of_refraction: 1.5,
        },
    ));

    world.push(Sphere::new(
        DVec3::new(-4., 1., 0.),
        1.0,
        Material::Lambertian {
            albedo: DVec3::new(0.4, 0.2, 0.1).into(),
        },
    ));

    world.push(Sphere::new(
        DVec3::new(4., 1., 0.),
        1.0,
        Material::Metal {
            albedo: DVec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    ));

    let camera = Camera::new(
        16.0 / 9.0,
        1200,
        20.0,
        100,
        dvec3(13.0, 2.0, 3.0),
        dvec3(0.0, 0.0, 0.0),
        dvec3(0.0, 1.0, 0.0),
        10.0,
        0.6,
    );

    camera.render(&world)?;
    Ok(())
}
