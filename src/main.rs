use ray_tracing::{
    camera::Camera,
    colour::Colour,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, MaterialEnum, Metal},
    ray::Point3,
    sphere::Sphere,
};

fn main() {
    let camera = Camera::new(16.0 / 9.0, 400, 100, 50);

    let mut world = HittableList::new();

    let ground = Lambertian::new(Colour::new(0.8, 0.8, 0.0));
    let center = Lambertian::new(Colour::new(0.1, 0.2, 0.5));

    let left = Dielectric::new(1.00 / 1.33);
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
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        MaterialEnum::Metal(right),
    )));

    camera.render(&world);
}
