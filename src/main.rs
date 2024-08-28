use camera::{Camera, Initialize, Render};
use indicatif::ProgressBar;
use interval::Interval;
use ray::At;
use rtweekend::INF;
use sphere::{Add, Hit, HitRecord, HittableList, New, Sphere};
use vec3::{dot, LengthSquared, Unit, Vec3};
mod vec3;
pub mod color;
mod ray;
mod point3;
mod sphere;
mod rtweekend;
mod interval;
mod camera;
mod material;

fn main() {
    // World

    let mut world = HittableList::new(); 

    let material_ground = material::Material::Lambertian(material::Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = material::Material::Lambertian(material::Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = material::Material::Dialectric(material::Dialectric::new(1.50));
    let material_bubble = material::Material::Dialectric(material::Dialectric::new(1.0/1.50));
    let material_right = material::Material::Metal(material::Metal::new(Vec3::new(0.8, 0.6, 0.2),1.0));

    world.add(sphere::HittableObject::Sphere(Sphere {
        center: point3::Point3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
        mat: material_ground,
    }));


    world.add(sphere::HittableObject::Sphere(Sphere {
        center: point3::Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.2,
        },
        radius: 0.5,
        mat: material_center,
    }));

    world.add(sphere::HittableObject::Sphere(Sphere {
        center: point3::Point3 {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        mat: material_left,
    }));

    world.add(sphere::HittableObject::Sphere(Sphere {
        center: point3::Point3 {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.4,
        mat: material_bubble,
    }));

    world.add(sphere::HittableObject::Sphere(Sphere {
        center: point3::Point3 {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        mat: material_right,
    }));

    // Camera

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20;
    
    cam.lookfrom = point3::Point3 {
        x: -2.0,
        y: 2.0,
        z: 1.0,
    };
    cam.lookat = point3::Point3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    cam.vup = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    cam.defocus_angle = 10.0;
    cam.focus_dist = 3.4;
    

    cam.render(&world)

}
