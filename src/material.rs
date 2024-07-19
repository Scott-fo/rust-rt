use crate::{
    colour::Colour,
    hittable::HitRecord,
    ray::Ray,
    vec3::{random_unit_vector, Vec3},
};

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
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Colour,
        _scattered: &mut Ray,
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
        _r_in: &Ray,
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

#[derive(Default, Clone, Copy)]
pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f64) -> Self {
        match fuzz < 1.0 {
            true => Self { albedo, fuzz },
            false => Self { albedo, fuzz: 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = Vec3::reflect(r_in.direction(), rec.normal);
        reflected = reflected.unit_vector() + (self.fuzz * random_unit_vector());
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;

        Vec3::dot(scattered.direction(), rec.normal) > 0.0
    }
}

#[derive(Default, Clone, Copy)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Colour::new(1.0, 1.0, 1.0);
        let ri = match rec.front_face {
            true => 1.0 / self.refraction_index,
            false => self.refraction_index,
        };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = Vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = match cannot_refract {
            true => Vec3::reflect(unit_direction, rec.normal),
            false => Vec3::refract(unit_direction, rec.normal, ri),
        };

        *scattered = Ray::new(rec.p, direction);
        true
    }
}

#[derive(Clone, Copy)]
pub enum MaterialEnum {
    Default(DefaultMaterial),
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
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
            MaterialEnum::Metal(m) => m.scatter(r_in, rec, attenuation, scattered),
            MaterialEnum::Dielectric(m) => m.scatter(r_in, rec, attenuation, scattered),
        }
    }
}

impl Default for MaterialEnum {
    fn default() -> Self {
        MaterialEnum::Default(DefaultMaterial)
    }
}
