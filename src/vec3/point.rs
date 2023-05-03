use crate::{
    rtweekend::rtweekend::{random_f32, random_f32_from_range},
    vec3::color::Color,
};
use std::ops;

#[derive(Clone, Copy)]
pub struct Point3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ops::Add<Point3d> for Point3d {
    type Output = Point3d;

    fn add(self, rhs: Point3d) -> Point3d {
        Point3d::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<Point3d> for Point3d {
    type Output = Point3d;

    fn sub(self, rhs: Point3d) -> Point3d {
        Point3d::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::AddAssign<Point3d> for Point3d {
    fn add_assign(&mut self, rhs: Point3d) {
        *self = *self + rhs;
    }
}

impl ops::SubAssign<Point3d> for Point3d {
    fn sub_assign(&mut self, rhs: Point3d) {
        *self = *self - rhs;
    }
}

impl ops::Div<f32> for Point3d {
    type Output = Point3d;

    fn div(self, rhs: f32) -> Point3d {
        Point3d::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::Mul<f32> for Point3d {
    type Output = Point3d;

    fn mul(self, rhs: f32) -> Point3d {
        Point3d::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<Point3d> for f32 {
    type Output = Point3d;

    fn mul(self, rhs: Point3d) -> Point3d {
        Point3d::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

impl ops::MulAssign<f32> for Point3d {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl ops::DivAssign<f32> for Point3d {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl From<Color> for Point3d {
    fn from(item: Color) -> Self {
        Point3d {
            x: (item.r),
            y: (item.g),
            z: (item.b),
        }
    }
}

impl Point3d {
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vector(&self) -> Point3d {
        *self / self.length()
    }

    pub fn cross(&mut self, rhs: Point3d) -> Point3d {
        let x = self.y * rhs.z - self.z * rhs.y;
        let y = self.z * rhs.x - self.x * rhs.z;
        let z = self.x * rhs.y - self.y * rhs.x;
        return Point3d { x, y, z };
    }

    pub fn dot(&self, rhs: Point3d) -> f32 {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        x + y + z
    }
}

impl Point3d {
    pub fn new(x: f32, y: f32, z: f32) -> Point3d {
        Point3d { x, y, z }
    }

    pub fn random() -> Point3d {
        Point3d::new(random_f32(), random_f32(), random_f32())
    }

    pub fn random_from_range(min: f32, max: f32) -> Point3d {
        Point3d::new(
            random_f32_from_range(min, max),
            random_f32_from_range(min, max),
            random_f32_from_range(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Point3d {
        loop {
            let p = Point3d::random_from_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    pub fn random_unit_vector() -> Point3d {
        Point3d::random_in_unit_sphere().unit_vector()
    }
}
