use crate::vec3::{mul, vec3};

pub trait At {
    fn at(&self, t: f32) -> vec3;
}

pub trait Origin {
    fn origin(&self) -> vec3;
}

pub trait Direction {
    fn direction(&self) -> vec3;
}

pub struct ray {
    orig: vec3,
    dir: vec3,
}

impl Origin for ray {
    fn origin(&self) -> vec3 {
        self.orig
    }
}

impl Direction for ray {
    fn direction(&self) -> vec3 {
        self.dir
    }
}

impl At for ray {
    fn at(&self, t: f32) -> vec3 {
        self.orig + mul(self.dir, t)
    }
}
