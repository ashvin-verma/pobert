use crate::rtweekend::INF;

pub trait Size {
    fn size(&self) -> f32;
}

pub trait Contains {
    fn contains(&self, x: f32) -> bool;
}

pub trait Surrounds {
    fn surrounds(&self, x: f32) -> bool;
}

pub trait Clamp {
    fn clamp(&self, x: f32) -> f32;
}

pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Size for Interval {
    fn size(&self) -> f32 {
        self.max - self.min
    }
}

impl Contains for Interval {
    fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }
}

impl Surrounds for Interval {
    fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }
}
 

impl Interval {
    const EMPTY: Interval = Interval { min: INF, max: -INF };
    const UNIVERSE: Interval = Interval { min: -INF, max: INF };
}

impl Clamp for Interval {
    fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}