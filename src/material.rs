use crate::{colour::Colour, hittable::HitRecord, ray::Ray, vec3::random_unit_vector};

pub trait Material: Copy + Clone {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Default, Clone, Copy)]
pub struct DefaultMaterial;

impl Material for DefaultMaterial {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        false
    }
}

#[derive(Default, Clone, Copy)]
pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;

        true
    }
}

#[derive(Clone, Copy)]
pub enum MaterialEnum {
    Default(DefaultMaterial),
    Lambertian(Lambertian),
}

impl Material for MaterialEnum {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            MaterialEnum::Default(m) => m.scatter(r_in, rec, attenuation, scattered),
            MaterialEnum::Lambertian(m) => m.scatter(r_in, rec, attenuation, scattered),
        }
    }
}

impl Default for MaterialEnum {
    fn default() -> Self {
        MaterialEnum::Default(DefaultMaterial)
    }
}
