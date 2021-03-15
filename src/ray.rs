use crate::vec::Vec3;
use crate::point::Point3;

//Un rayon lancé est caractérisé par son origine et un vecteur définissant sa direction
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, fact: f64) -> Point3 {
        self.origin + self.direction * fact
    }
}
