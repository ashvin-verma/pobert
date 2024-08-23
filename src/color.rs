use crate::vec3::vec3;

type color = vec3;

pub fn write_color(color: color) {
    let ir = (255.999 * color.x) as i32;
    let ig = (255.999 * color.y) as i32;
    let ib = (255.999 * color.z) as i32;

    println!("{} {} {}", ir, ig, ib);
}
