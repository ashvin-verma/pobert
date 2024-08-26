use std::ops;
use std::fmt;
use std::ops::Neg;

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
        self.div(self.length())
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