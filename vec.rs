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

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Vec3(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl<T> AddAssign<T> for Vec3 where Vec3: Add<T, Output=Self> {
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Vec3(self.0 - rhs, self.1 - rhs, self.2 - rhs)
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

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(rhs.0*self, rhs.1*self, rhs.2*self)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = rhs * *self;
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

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.0*v.0 + u.1*v.1 + u.2*v.2
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::new(
            u.1*v.2 - u.2*v.1,
            u.2*v.0 - u.0*v.2,
            u.0*v.1 - u.1*v.0,
        )
    }

    pub fn to_s(&self) -> String {
        format!("{} {} {}", (self.0 * 255.999) as u8, (self.1 * 255.999) as u8, (self.2 * 255.999) as u8)
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3(x, y, z)
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.len()
    }

    pub fn get_x(&self) -> f64 {self.0}
    pub fn get_y(&self) -> f64 {self.1}
    pub fn get_z(&self) -> f64 {self.2}
}


#[test]
fn test_ops() {
    let mut u = Vec3(1.0, 2.0, 3.0);
    let v = Vec3(4.0, 5.0, 6.0);
    assert_eq!(u + v, Vec3(5.0, 7.0, 9.0));
    assert_eq!(u * v, Vec3(4.0, 10.0, 18.0));
    assert_eq!(u / 2.0, Vec3(0.5, 1.0, 1.5));
    assert_eq!(2.0 * u, Vec3(2.0, 4.0, 6.0));

    assert_eq!(Vec3::dot(&u, &v), 32.0);
    assert_eq!(Vec3::cross(&u, &v), Vec3(-3.0, 6.0, -3.0));

    u *= v;
    u += v;
    u /= 2.0;
    u *= 3.0;
    assert_eq!(u, Vec3(12.0, 22.5, 36.0));
}
