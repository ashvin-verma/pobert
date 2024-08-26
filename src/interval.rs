use crate::rtweekend::INF;

pub trait Size {
    fn size(&self) -> f64;
}

pub trait Contains {
    fn contains(&self, x: f64) -> bool;
}

pub trait Surrounds {
    fn surrounds(&self, x: f64) -> bool;
}

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Size for Interval {
    fn size(&self) -> f64 {
        self.max - self.min
    }
}

impl Contains for Interval {
    fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
}

impl Surrounds for Interval {
    fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
}
 

impl Interval {
    const EMPTY: Interval = Interval { min: INF, max: -INF };
    const UNIVERSE: Interval = Interval { min: -INF, max: INF };
}
