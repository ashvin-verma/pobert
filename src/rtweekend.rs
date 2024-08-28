use std::cell::RefCell;

use rand::Rng;


pub const INF: f32 = f32::INFINITY;
pub const PI: f32 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

thread_local! {
    static THREAD_RNG: RefCell<rand::rngs::ThreadRng> = RefCell::new(rand::thread_rng());
}


pub fn random_double() -> f32 {
    THREAD_RNG.with(|rng| rng.borrow_mut().gen::<f32>())
}

pub fn random_double_range(min: f32, max: f32) -> f32 {
    THREAD_RNG.with(|rng| rng.borrow_mut().gen_range(min..max))
}
