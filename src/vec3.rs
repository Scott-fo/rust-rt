use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

use crate::utils::{random_double, random_double_in_range};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn random() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }

    pub fn random_in_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_double_in_range(min, max),
            random_double_in_range(min, max),
            random_double_in_range(min, max),
        )
    }

    pub fn dot(u: Vec3, v: Vec3) -> f64 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3::new(
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        )
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_in_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_on_hemisphere(self) -> Vec3 {
        let on_unit_sphere = random_unit_vector();
        if Vec3::dot(on_unit_sphere, self) > 0.0 {
            return on_unit_sphere;
        }

        -on_unit_sphere
    }
}

pub fn random_unit_vector() -> Vec3 {
    Vec3::random_in_unit_sphere().unit_vector()
}

impl Default for Vec3 {
    fn default() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.e[0], self * rhs.e[1], self * rhs.e[2])
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        1.0 / rhs * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn test_negation() {
        let v = Vec3::new(1.0, -2.0, 3.0);
        let neg_v = -v;
        assert_eq!(neg_v.x(), -1.0);
        assert_eq!(neg_v.y(), 2.0);
        assert_eq!(neg_v.z(), -3.0);
    }

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v3 = v1 + v2;
        assert_eq!(v3.x(), 5.0);
        assert_eq!(v3.y(), 7.0);
        assert_eq!(v3.z(), 9.0);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v3 = v1 - v2;
        assert_eq!(v3.x(), -3.0);
        assert_eq!(v3.y(), -3.0);
        assert_eq!(v3.z(), -3.0);
    }

    #[test]
    fn test_mul_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v3 = v1 * v2;
        assert_eq!(v3.x(), 4.0);
        assert_eq!(v3.y(), 10.0);
        assert_eq!(v3.z(), 18.0);
    }

    #[test]
    fn test_mul_f64() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let v2 = v * 2.0;
        assert_eq!(v2.x(), 2.0);
        assert_eq!(v2.y(), 4.0);
        assert_eq!(v2.z(), 6.0);
    }

    #[test]
    fn test_f64_mul_vec3() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let v2 = 2.0 * v;
        assert_eq!(v2.x(), 2.0);
        assert_eq!(v2.y(), 4.0);
        assert_eq!(v2.z(), 6.0);
    }

    #[test]
    fn test_div() {
        let v = Vec3::new(2.0, 4.0, 6.0);
        let v2 = v / 2.0;
        assert_eq!(v2.x(), 1.0);
        assert_eq!(v2.y(), 2.0);
        assert_eq!(v2.z(), 3.0);
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        v1 += v2;
        assert_eq!(v1.x(), 5.0);
        assert_eq!(v1.y(), 7.0);
        assert_eq!(v1.z(), 9.0);
    }

    #[test]
    fn test_mul_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v *= 2.0;
        assert_eq!(v.x(), 2.0);
        assert_eq!(v.y(), 4.0);
        assert_eq!(v.z(), 6.0);
    }

    #[test]
    fn test_div_assign() {
        let mut v = Vec3::new(2.0, 4.0, 6.0);
        v /= 2.0;
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn test_length() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn test_length_squared() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(v.length_squared(), 25.0);
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(Vec3::dot(v1, v2), 32.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v3 = Vec3::cross(v1, v2);
        assert_eq!(v3.x(), -3.0);
        assert_eq!(v3.y(), 6.0);
        assert_eq!(v3.z(), -3.0);
    }

    #[test]
    fn test_unit_vector() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        let unit_v = v.unit_vector();
        assert!((unit_v.x() - 0.6).abs() < 1e-10);
        assert!((unit_v.y() - 0.8).abs() < 1e-10);
        assert!((unit_v.z() - 0.0).abs() < 1e-10);
    }
}
