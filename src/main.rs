use std::{sync::Arc, time::Instant};

use ray_tracing::{
    camera::Camera,
    colour::Colour,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, MaterialEnum, Metal},
    ray::Point3,
    sphere::Sphere,
    utils::{random_double, random_double_in_range},
    vec3::Vec3,
};

fn main() {
    let camera = Camera::new(
        16.0 / 9.0,
        1200,
        500,
        50,
        20,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
    );

    let mut world = HittableList::new();

    let ground = Lambertian::new(Colour::new(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        MaterialEnum::Lambertian(ground),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Colour::random() * Colour::random();
                    let sphere_material = Lambertian::new(albedo);
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        MaterialEnum::Lambertian(sphere_material),
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Colour::random_in_range(0.5, 1.0);
                    let fuzz = random_double_in_range(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        MaterialEnum::Metal(sphere_material),
                    )));
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        MaterialEnum::Dielectric(sphere_material),
                    )));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        MaterialEnum::Dielectric(material1),
    )));

    let material2 = Lambertian::new(Colour::new(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        MaterialEnum::Lambertian(material2),
    )));

    let material3 = Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        MaterialEnum::Metal(material3),
    )));

    let start_time = Instant::now();
    camera.render(Arc::new(world));
    let duration = start_time.elapsed();

    eprintln!("Done in: {} seconds", duration.as_secs())
}
