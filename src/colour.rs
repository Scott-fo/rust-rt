use crate::{interval::Interval, vec3};

pub type Colour = vec3::Vec3;

impl Colour {
    pub fn display(&self) {
        let intensity = Interval::new(0.000, 0.999);
        let rbyte = (255.999 * intensity.clamp(self.x())) as i64;
        let gbyte = (255.999 * intensity.clamp(self.y())) as i64;
        let bbyte = (255.999 * intensity.clamp(self.z())) as i64;

        print!("{} {} {}\n", rbyte, gbyte, bbyte)
    }
}
