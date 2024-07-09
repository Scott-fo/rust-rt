use std::f64::INFINITY;

use crate::{
    colour::Colour,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    vec3::Vec3,
};

pub type Point3 = Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn colour(&self, world: &HittableList) -> Colour {
        let mut rec = HitRecord::default();
        if world.hit(self, 0.0, INFINITY, &mut rec) {
            return 0.5 * (rec.normal + Colour::new(1.0, 1.0, 1.0));
        }

        let unit_direction = Vec3::unit_vector(self.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
    }

    pub fn hit_sphere(&self, center: Point3, radius: f64) -> f64 {
        let oc = center - self.origin;
        let a = self.direction.length_squared();
        let h = Vec3::dot(self.direction, oc);
        let c = oc.length_squared() - radius * radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return -1.0;
        }

        (h - discriminant.sqrt()) / a
    }
}
