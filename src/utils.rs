use rand::{distributions::Distribution, distributions::Uniform, thread_rng};

use crate::vec3::Vec3;

pub fn sample_square() -> Vec3 {
    Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
}

pub fn random_double() -> f64 {
    let mut rng = thread_rng();
    let uniform = Uniform::from(0.0..1.0);
    uniform.sample(&mut rng)
}

pub fn random_double_in_range(min: f64, max: f64) -> f64 {
    let mut rng = thread_rng();
    let uniform = Uniform::from(min..max);
    uniform.sample(&mut rng)
}
