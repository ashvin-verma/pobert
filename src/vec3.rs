use std::ops;
use std::fmt;

pub trait length {
    fn length(&self) -> f32;
}

pub trait length_squared {
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

pub struct vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ops::Add for vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Multiplication for vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Division for vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl length_squared for vec3 {
    fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl length for vec3 {
    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
}

impl fmt::Display for vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Dot for vec3 {
    fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
impl Cross for vec3 {
    fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Unit for vec3 {
    fn unit(&self) -> Self {
        self.div(self.length())
    }
}

impl Copy for vec3 {}

impl Clone for vec3 {
    fn clone(&self) -> Self {
        *self
    }
}

pub fn dot (v1: vec3, v2: vec3) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn cross (v1: vec3, v2: vec3) -> vec3 {
    vec3 {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    }
}

pub fn mul (v: vec3, t: f32) -> vec3 {
    vec3 {
        x: v.x * t,
        y: v.y * t,
        z: v.z * t,
    }
}

pub fn div (v: vec3, t: f32) -> vec3 {
    vec3 {
        x: v.x / t,
        y: v.y / t,
        z: v.z / t,
    }
}
