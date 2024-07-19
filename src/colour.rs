use crate::{interval::Interval, vec3};

pub type Colour = vec3::Vec3;

impl Colour {
    pub fn write(&self) -> String {
        let intensity = Interval::new(0.000, 0.999);

        let r = linear_to_gamma(self.x());
        let g = linear_to_gamma(self.y());
        let b = linear_to_gamma(self.z());

        let rbyte = (255.999 * intensity.clamp(r)) as i64;
        let gbyte = (255.999 * intensity.clamp(g)) as i64;
        let bbyte = (255.999 * intensity.clamp(b)) as i64;

        format!("{} {} {}\n", rbyte, gbyte, bbyte)
    }

    pub fn display(&self) {
        let intensity = Interval::new(0.000, 0.999);

        let r = linear_to_gamma(self.x());
        let g = linear_to_gamma(self.y());
        let b = linear_to_gamma(self.z());

        let rbyte = (255.999 * intensity.clamp(r)) as i64;
        let gbyte = (255.999 * intensity.clamp(g)) as i64;
        let bbyte = (255.999 * intensity.clamp(b)) as i64;

        print!("{} {} {}\n", rbyte, gbyte, bbyte)
    }
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }

    0.0
}
