use std::ops::{Add, Div, Mul};

#[derive(Copy, Clone)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

pub struct AvgColor(Color, f64);
impl AvgColor {
    pub fn new(color: Color) -> AvgColor {
        AvgColor(color, 1.)
    }

    pub fn empty() -> AvgColor {
        const BLACK: Color = Color {
            red: 0.,
            green: 0.,
            blue: 0.,
        };
        AvgColor(BLACK, 0.)
    }

    pub fn avg(self) -> Color {
        self.0 / self.1
    }
}
impl Add<Color> for AvgColor {
    type Output = Self;

    fn add(mut self, rhs: Color) -> Self::Output {
        self.0 = self.0 + rhs;
        self.1 += 1.;

        self
    }
}

impl Color {
    pub const fn new(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }
    pub fn scale(&self, range: f64) -> (u32, u32, u32) {
        let c = range * *self;
        (c.red as u32, c.green as u32, c.blue as u32)
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Color {
            red: self.red / rhs,
            green: self.green / rhs,
            blue: self.blue / rhs,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
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
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}
