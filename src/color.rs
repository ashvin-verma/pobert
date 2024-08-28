use crate::{interval::{self, Clamp}, vec3::Vec3};

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f32) -> f32{
    if linear_component > 0.0 {
        return linear_component.sqrt()
    }
    return 0.0;
}

pub fn write_color(color: Color) {

    let intensity: interval::Interval = interval::Interval::new(0.0, 0.999);

    let r = linear_to_gamma(color.x);
    let g = linear_to_gamma(color.y);
    let b = linear_to_gamma(color.z);

    let ir = (255.999 * intensity.clamp(r)) as i32;
    let ig = (255.999 * intensity.clamp(g)) as i32;
    let ib = (255.999 * intensity.clamp(b)) as i32;



    println!("{} {} {}", ir, ig, ib);
}

