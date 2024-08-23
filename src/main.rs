use indicatif::ProgressBar;
mod vec3;
mod color;
mod ray;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let bar = ProgressBar::new(image_height);

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        bar.inc(1);
        for i in 0..image_width {
            let pixel_color = vec3::vec3 {
                x: i as f32 / (image_width - 1) as f32,
                y: j as f32 / (image_height - 1) as f32,
                z: 0.0,
            };
            color::write_color(pixel_color);
        }
    }

}
