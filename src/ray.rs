use crate::vec::Vec3;
use crate::point::Point3;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, fact: f64) -> Point3 {
        self.origin + self.direction * fact
    }
}
