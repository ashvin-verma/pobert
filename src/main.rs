use indicatif::ProgressBar;
use interval::Interval;
use ray::At;
use rtweekend::INF;
use sphere::{Add, Hit, HitRecord, HittableList, New, Sphere};
use vec3::{div, dot, LengthSquared, mul, Unit};
mod vec3;
pub mod color;
mod ray;
mod point3;
mod sphere;
mod rtweekend;
mod interval;

// fn hit_sphere (center: point3::Point3, radius: f32, r: ray::Ray) -> f32 {
//     let oc = center - r.orig;
//     let a = dot(r.dir, r.dir);
//     let half_b = -2.0 * vec3::dot(oc, r.dir);
//     let c = oc.length_squared() - radius*radius;
//     let discriminant = half_b*half_b - 4.0*a*c;

//     if discriminant < 0.0 {
//         return -1.0;
//     } else {
//         return (-half_b - discriminant.sqrt()) / (2.0*a);
//     }

// }

fn ray_color (r: ray::Ray, world: HittableList) -> color::Color {
    let mut rec = HitRecord {
        p: point3::Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: vec3::Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        front_face: false,
    };

    if world.hit(r, Interval::new(0.0, INF), &mut rec) {
        return mul(color::Color {
            x: rec.normal.x + 1.0,
            y: rec.normal.y + 1.0,
            z: rec.normal.z + 1.0,
        }, 0.5);
    }



    let unit_direction = r.dir.unit();
    let a = 0.5 * (unit_direction.y + 1.0);
    mul(color::Color {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    }, 1.0-a) + mul(color::Color {
        x: 0.5,
        y: 0.7,
        z: 1.0,
    }, a)
}

fn main() {

    // Img

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // World

    let mut world = HittableList::new(); 
    world.add(sphere::HittableObject::Sphere(Sphere {
        center: point3::Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    }));

    world.add(sphere::HittableObject::Sphere(Sphere {
        center: point3::Point3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
    }));

    // Camera

    let focal_length = 1.0;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let viewport_height = 2.0;
    let viewport_width = (image_width as f32 / image_height as f32) * viewport_height;
    let camera_center = point3::Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let viewport_u = vec3::Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };

    let viewport_v = vec3::Vec3 {
        x: 0.0,
        y: -viewport_height,
        z: 0.0,
    };

    let pixel_delta_u = div(viewport_u, image_width as f32);
    let pixel_delta_v = div(viewport_v, image_height as f32);

    let viewport_upper_left = camera_center - div(viewport_u, 2.0) - div(viewport_v, 2.0) - vec3::Vec3 {
        x: 0.0,
        y: 0.0,
        z: focal_length,
    };

    let pixel00_loc = viewport_upper_left + mul(pixel_delta_u, 0.5) + mul(pixel_delta_v, 0.5);



    // Progress

    let bar = ProgressBar::new(image_height as u64);

    println!("P3\n{} {}\n255", image_width, image_height);

    // Main render loop

    for j in 0..image_height {
        bar.inc(1);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + mul(pixel_delta_u, i as f32) + mul(pixel_delta_v, j as f32);
            let ray_dir = pixel_center - camera_center;
            let r = ray::Ray {
                orig: camera_center,
                dir: ray_dir,
            };
    
            let pixel_color = ray_color(r, world.clone());
            color::write_color(pixel_color);
        }
    }

}
