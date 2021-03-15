use std::ops::{Add, Div, Mul};

#[derive(Copy, Clone)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub const EMPTY: Color = Color::new(0., 0., 0.);

    pub const fn new(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }

    pub fn scale(&self, range: f64) -> (u32, u32, u32) {
        let c = range * *self;
        (c.red as u32, c.green as u32, c.blue as u32)
    }

    pub fn map<F>(self, f: F) -> Self
        where F: Fn(f64,f64,f64)->(f64,f64,f64)
    {
        let (red, green, blue) = f(self.red, self.green, self.blue);
        Color::new(red, green, blue)
    }

    pub fn map_each<F>(self, f: F) -> Self
        where F: Fn(f64)->(f64)
    {
        Color::new(f(self.red), f(self.green), f(self.blue))
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

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
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
