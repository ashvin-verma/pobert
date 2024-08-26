use crate::vec3::{mul, Vec3};
use crate::point3::Point3;

pub trait At {
    fn at(&self, t: f32) -> Vec3;
}

pub trait Origin {
    fn origin(&self) -> Point3;
}

pub trait Direction {
    fn direction(&self) -> Vec3;
}

pub struct Ray {
    pub(crate) orig: Point3,
    pub(crate) dir: Vec3,
}

impl Origin for Ray {
    fn origin(&self) -> Point3 {
        self.orig
    }
}

impl Direction for Ray {
    fn direction(&self) -> Vec3 {
        self.dir
    }
}

impl At for Ray {
    fn at(&self, t: f32) -> Vec3 {
        self.orig + mul(self.dir, t)
    }
}

impl Copy for Ray {}

impl Clone for Ray {
    fn clone(&self) -> Self {
        *self
    }
}