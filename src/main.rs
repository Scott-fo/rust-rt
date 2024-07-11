use ray_tracing::{
    camera::Camera, hittable_list::HittableList, material::MaterialEnum, ray::Point3,
    sphere::Sphere,
};

fn main() {
    let camera = Camera::new(16.0 / 9.0, 400, 100, 50);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        MaterialEnum::default(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        MaterialEnum::default(),
    )));

    camera.render(&world);
}
