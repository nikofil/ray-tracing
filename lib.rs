#![feature(associated_type_bounds)]
use std::ops::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3(f64, f64, f64);
pub type Point = Vec3;
pub type Color = Vec3;

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self(self.0*rhs.0, self.1*rhs.1, self.2*rhs.2)
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = *self * rhs;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0*rhs, self.1*rhs, self.2*rhs)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0/rhs, self.1/rhs, self.2/rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Vec3 {
    pub fn len_sq(&self) -> f64 {
        self.0*self.0 + self.1*self.1 + self.2*self.2
    }

    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
    }
}

#[test]
fn name() {
    let mut x = Vec3(1.0, 2.0, 3.0);
    let y = Vec3(4.0, 5.0, 6.0);
    assert_eq!(x + y, Vec3(5.0, 7.0, 9.0));
    assert_eq!(x * y, Vec3(4.0, 10.0, 18.0));
    assert_eq!(x / 2.0, Vec3(0.5, 1.0, 1.5));
    assert_eq!(x * 2.0, Vec3(2.0, 4.0, 6.0));

    x *= y;
    x += y;
    x /= 2.0;
    x *= 3.0;
    assert_eq!(x, Vec3(12.0, 22.5, 36.0));
}