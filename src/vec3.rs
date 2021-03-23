use core::ops::Neg;
use rand::prelude::*;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct RBG {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub trait Vec3Attributes {
    fn length(self) -> f32;
    fn length_squared(self) -> f32;
    fn near_zero(self) -> bool;
    fn reflect(self, other: Vec3) -> Vec3;
}

impl Vec3 {
    pub fn new(t: f32) -> Vec3 {
        Vec3 { x: t, y: t, z: t }
    }
    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen_range(0. ..1.),
            y: rng.gen_range(0. ..1.),
            z: rng.gen_range(0. ..1.),
        }
    }
    pub fn random_rng(min: f32, max: f32) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }
}

impl Vec3Attributes for Vec3 {
    fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    fn near_zero(self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
    fn reflect(self, other: Vec3) -> Vec3 {
        return self - Vec3::new(2.) * Vec3::new(self.dot(other)) * other;
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, t: Vec3) {
        self.x *= 1. / t.x;
        self.y *= 1. / t.y;
        self.z *= 1. / t.z;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    fn add(self, other: Vec3) -> <Self as std::ops::Add<Vec3>>::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
    type Output = Self;
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Vec3) -> <Self as std::ops::Sub<Vec3>>::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Vec3) -> <Self as std::ops::Mul<Vec3>>::Output {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Vec3) -> <Self as std::ops::Div<Vec3>>::Output {
        Vec3::new(1. / other.x) * self
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

pub trait Math {
    fn dot(self, other: Vec3) -> f32;
    fn cross(self, other: Vec3) -> Vec3;
    fn unit(self) -> Vec3;
}

pub trait ToColor {
    fn to_color(self, scale: Vec<f32>) -> RBG;
}

impl ToColor for Color {
    fn to_color(self, scale: Vec<f32>) -> RBG {
        RBG {
            r: (256. * clamp((self.x * scale[0]).sqrt(), 0.0, 0.999)) as u8,
            g: (256. * clamp((self.y * scale[1]).sqrt(), 0.0, 0.999)) as u8,
            b: (256. * clamp((self.z * scale[2]).sqrt(), 0.0, 0.999)) as u8,
        }
    }
}

impl Math for Vec3 {
    fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    fn unit(self) -> Vec3 {
        self / Vec3::new(self.length())
    }
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
