use crate::rtweekend::rtweekend::clamp;
use crate::SAMPLES_PER_PIXEL;
use std::fmt;
use std::ops;

use super::point::Point3d;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

// operations
impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl ops::Sub<Color> for Color {
    type Output = Self;

    fn sub(self, rhs: Color) -> Color {
        Color::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        *self = *self + rhs;
    }
}

impl ops::MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl ops::Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Color {
        Color::new(self.r / rhs, self.g / rhs, self.b / rhs)
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color::new(rhs.r * self, rhs.g * self, rhs.b * self)
    }
}

impl ops::SubAssign<Color> for Color {
    fn sub_assign(&mut self, rhs: Color) {
        *self = *self - rhs;
    }
}

impl ops::DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl From<Point3d> for Color {
    fn from(item: Point3d) -> Self {
        Color {
            r: (item.x),
            g: (item.y),
            b: (item.z),
        }
    }
}

impl Color {
    fn length(&self) -> f32 {
        (self.r * self.r + self.g * self.g + self.b * self.b).sqrt()
    }

    fn unit_vector(&self) -> Color {
        *self / self.length()
    }

    fn cross(&mut self, rhs: Color) -> Color {
        let r = self.g * rhs.b - self.b * rhs.g;
        let g = self.b * rhs.r - self.r * rhs.b;
        let b = self.r * rhs.g - self.g * rhs.r;
        return Color { r, g, b };
    }

    fn dot(&mut self, rhs: Color) -> f32 {
        let r = self.r * rhs.r;
        let g = self.g * rhs.g;
        let b = self.b * rhs.b;
        r + g + b
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scale = 1.0 / SAMPLES_PER_PIXEL as f32;

        // divide the color by the number of samples
        // and gamma-correct for gamma=2.0
        let mut r = (self.r * scale).sqrt();
        let mut g = (self.g * scale).sqrt();
        let mut b = (self.b * scale).sqrt();

        // translate the value to the [0, 255]
        let min = 0.0;
        let max = 0.999;
        r = 256.0 * clamp(r, min, max);
        g = 256.0 * clamp(g, min, max);
        b = 256.0 * clamp(b, min, max);

        write!(f, "{} {} {}", r, g, b)
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }
}
