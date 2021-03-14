use std::ops::{Add, Mul};

#[derive(Copy, Clone)]
pub struct Color(pub f64, pub f64, pub f64);

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl Add<Color> for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Self::Output {
        Color(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
//
// impl Add<f64> for Color {
//     type Output = Self;
//
//     fn add(self, rhs: f64) -> Self::Output {
//         Color(self.0 + rhs, self.1 + rhs, self.2 + rhs)
//     }
// }
