use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable + Send + Sync>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        interval: Interval,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = interval.max();
        let mut temp_rec = HitRecord::default();

        for object in &self.objects {
            if object.hit(
                r,
                Interval::new(interval.min(), closest_so_far),
                &mut temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}
