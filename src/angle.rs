#![allow(dead_code)]
use std::f64::consts::PI;

pub struct Deg(pub f64);
pub struct Rad(pub f64);

impl Deg{
    pub fn to_rad(&self) -> Rad{
        Rad(self.0 * PI / 180.)
    }
}