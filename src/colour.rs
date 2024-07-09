use crate::vec3;

pub type Colour = vec3::Vec3;

impl Colour {
    pub fn display(&self) {
        let rbyte = (255.999 * self.x()) as i64;
        let gbyte = (255.999 * self.y()) as i64;
        let bbyte = (255.999 * self.z()) as i64;

        print!("{} {} {}\n", rbyte, gbyte, bbyte)
    }
}
