use crate::vec::Vec3;
use std::ops::{Add, Sub};

#[derive(Copy, Clone)]
pub struct Point3(pub f64, pub f64, pub f64);

impl Add<Vec3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Point3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self + -rhs
    }
}
