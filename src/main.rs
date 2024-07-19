use std::{sync::Arc, time::Instant};

use ray_tracing::{
    camera::Camera,
    colour::Colour,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, MaterialEnum, Metal},
    ray::Point3,
    sphere::Sphere,
    vec3::Vec3,
};

fn main() {
    let camera = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        90,
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut world = HittableList::new();

    let ground = Lambertian::new(Colour::new(0.8, 0.8, 0.0));
    let center = Lambertian::new(Colour::new(0.1, 0.2, 0.5));

    let left = Dielectric::new(1.50);
    let bubble = Dielectric::new(1.00 / 1.50);
    let right = Metal::new(Colour::new(0.8, 0.6, 0.2), 0.2);

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        MaterialEnum::Lambertian(ground),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        MaterialEnum::Lambertian(center),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        MaterialEnum::Dielectric(left),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        MaterialEnum::Dielectric(bubble),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        MaterialEnum::Metal(right),
    )));

    let start_time = Instant::now();
    camera.render(Arc::new(world));
    let duration = start_time.elapsed();

    eprintln!("Done in: {} seconds", duration.as_secs())
}
