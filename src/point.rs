use crate::vec::Vec3;
use std::ops::{Add, Sub};

#[derive(Copy, Clone)]
pub struct Point3(pub f64, pub f64, pub f64);

impl Add<Vec3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Vec3) -> Self::Output {
        translate(self, rhs)
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        translate(self, -rhs)
    }
}

pub fn translate(origin: Point3, direction: Vec3) -> Point3 {
    Point3(
        origin.0 + direction.0,
        origin.1 + direction.1,
        origin.2 + direction.2,
    )
}
