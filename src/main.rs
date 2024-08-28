use camera::{Camera, Initialize, Render};
use indicatif::ProgressBar;
use interval::Interval;
use point3::Point3;
use ray::At;
use rtweekend::{random_double_range, INF};
use sphere::{Add, Hit, HitRecord, HittableList, New, Sphere};
use vec3::{dot, element_wise_mul, random, random_range, Length, LengthSquared, Unit, Vec3};
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

    let material_ground = material::Material::Lambertian(material::Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(sphere::HittableObject::Sphere(Sphere {
        center: point3::Point3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        mat: material_ground,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rtweekend::random_double();
            let center = point3::Point3 {
                x: a as f32 + 0.9 * rtweekend::random_double(),
                y: 0.2,
                z: b as f32 + 0.9 * rtweekend::random_double(),
            };

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: material::Material;

                if choose_mat < 0.8 {
                    let albedo = element_wise_mul(random(), random());
                    sphere_material = material::Material::Lambertian(material::Lambertian::new(albedo));
                    world.add(sphere::HittableObject::Sphere(Sphere {
                        center,
                        radius: 0.2,
                        mat: sphere_material,
                    }));
                }
                else if choose_mat < 0.95 {
                    let albedo = random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    sphere_material = material::Material::Metal(material::Metal::new(albedo, fuzz));
                    world.add(sphere::HittableObject::Sphere(Sphere {
                        center,
                        radius: 0.2,
                        mat: sphere_material,
                    }));
                }
                else {
                    sphere_material = material::Material::Dialectric(material::Dialectric::new(1.5));
                    world.add(sphere::HittableObject::Sphere(Sphere {
                        center,
                        radius: 0.2,
                        mat: sphere_material,
                    }));
                }
            }
        }
    }

    let material_1 = material::Material::Dialectric(material::Dialectric::new(1.50));
    world.add(sphere::HittableObject::Sphere(Sphere {
        center: point3::Point3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat: material_1,
    }));

    let material_2 = material::Material::Lambertian(material::Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(sphere::HittableObject::Sphere(Sphere {
        center: point3::Point3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat: material_2,
    }));

    let material_3 = material::Material::Metal(material::Metal::new(Vec3::new(0.7, 0.6, 0.5),0.0));
    world.add(sphere::HittableObject::Sphere(Sphere {
        center: point3::Point3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat: material_3,
    }));

    // Camera

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;

    cam.vfov = 20;

    cam.lookfrom = point3::Point3 {
        x: 3.0,
        y: 4.0,
        z: 3.0,
    };
    cam.lookat = point3::Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    cam.vup = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.5,
    };

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    

    cam.render(&world)

}
