use ray_tracing::{camera::Camera, hittable_list::HittableList, ray::Point3, sphere::Sphere};

fn main() {
    let camera = Camera::new(16.0 / 9.0, 400, 100);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    camera.render(&world);
}
