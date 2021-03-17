#![allow(dead_code)]

use std::f64::consts::PI;

pub enum Angle {
    Deg(f64),
    Rad(f64),
}

impl Angle {
    pub fn rad(&self) -> f64 {
        match self {
            Angle::Rad(rad) => *rad,
            Angle::Deg(deg) => (deg * PI) / 180.,
        }
    }
    pub fn deg(&self) -> f64 {
        match self {
            Angle::Rad(rad) => (rad * 180.) / PI,
            Angle::Deg(deg) => *deg,
        }
    }
}
