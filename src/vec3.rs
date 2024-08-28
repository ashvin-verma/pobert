use std::cmp::min;
use std::ops;
use std::fmt;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;

use crate::rtweekend::random_double;
use crate::rtweekend::random_double_range;

pub trait NearZero {
    fn near_zero(&self) -> bool;
}

pub trait Length {
    fn length(&self) -> f32;
}

pub trait LengthSquared {
    fn length_squared(&self) -> f32;
}

pub trait Dot {
    fn dot(&self, rhs: Self) -> f32;
}

pub trait Cross {
    fn cross(&self, rhs: Self) -> Self;
} 

pub trait Unit {
    fn unit(&self) -> Self;
}

pub trait Division {
    type Output;
    fn div(self, rhs: f32) -> Self::Output;
}

pub trait Multiplication {
    type Output;
    fn mul(self, rhs: f32) -> Self::Output;
}
#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Multiplication for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Division for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl LengthSquared for Vec3 {
    fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Length for Vec3 {
    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Dot for Vec3 {
    fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
impl Cross for Vec3 {
    fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Unit for Vec3 {
    fn unit(&self) -> Self {
        *self / (self.length())
    }
}

impl Copy for Vec3 {}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        *self
    }
}

pub fn dot (v1: Vec3, v2: Vec3) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn cross (v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    }
}

pub fn mul (v: Vec3, t: f32) -> Vec3 {
    Vec3 {
        x: v.x * t,
        y: v.y * t,
        z: v.z * t,
    }
}

pub fn div (v: Vec3, t: f32) -> Vec3 {
    Vec3 {
        x: v.x / t,
        y: v.y / t,
        z: v.z / t,
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

pub fn random() -> Vec3 {
    Vec3::new(random_double(), random_double(), random_double())
}

pub fn random_range(min: f32, max: f32) -> Vec3 {
    Vec3::new(random_double_range(min,max), random_double_range(min,max), random_double_range(min,max))
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_range(-1.0,1.0);
        if p.length_squared() < 1.0 {
            return p
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit()
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(on_unit_sphere, *normal) > 0.0 {
        return on_unit_sphere
    }
    else {
        return - on_unit_sphere
    }
}

impl NearZero for Vec3 {
    fn near_zero(&self) -> bool {
        let s = 1e-8;
        return self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * 2.0 * dot(v,n)
}

pub fn element_wise_mul(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 {
        x: v1.x * v2.x,
        y: v1.y * v2.y,
        z: v1.z * v2.z,
    }
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = dot(-uv, n).min(1.0);
    let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
    let r_out_parallel = n * -((1.0 - r_out_perp.length_squared()).abs().sqrt());
    r_out_perp + r_out_parallel
}

