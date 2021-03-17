use crate::point::Point3;
use rand::Rng;
use std::ops::{Add, Div, Mul, Neg, Sub};

//TODO regarder les crates existantes pour le calcul vectoriel
#[derive(Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Add<Self> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub<Self> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Vec3 {
    //on créé un vec entre deux points: origine vers destination
    pub fn points(from: Point3, to: Point3) -> Vec3 {
        Vec3(to.0 - from.0, to.1 - from.1, to.2 - from.2)
    }

    pub fn scalar_product(&self, other: Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross_product(&self, other: Vec3) -> Vec3 {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn sqr_len(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn len(&self) -> f64 {
        self.sqr_len().sqrt()
    }

    pub fn random_unit_sphere() -> Vec3 {
        loop {
            //vec origin->point random
            //pour qu'un vecteur soit dans la sphere il faut |vec| < r
            // pour une sphere unitaire r = r2 = 1, donc |vec|^2 < 1
            let dir = Vec3(
                rand::thread_rng().gen_range(-1.0..=1.),
                rand::thread_rng().gen_range(-1.0..=1.),
                rand::thread_rng().gen_range(-1.0..=1.),
            );
            if dir.sqr_len() < 1. {
                return dir;
            }
        }
    }

    pub fn random_unit_disk() -> Vec3 {
        loop {
            let dir = Vec3(
                rand::thread_rng().gen_range(-1.0..=1.),
                rand::thread_rng().gen_range(-1.0..=1.),
                0.,
            );
            if dir.sqr_len() < 1. {
                return dir;
            }
        }
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.len()
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }
}
