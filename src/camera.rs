use indicatif::ProgressBar;

use crate::{color::{self, Color}, interval::Interval, material::Scatter, point3::Point3, ray::{self, Direction, Ray}, rtweekend::{degrees_to_radians, random_double, INF}, sphere::{Hit, HitRecord, HittableList, New}, vec3::{cross, element_wise_mul, random_in_unit_disk, random_on_hemisphere, random_unit_vector, Length, Unit, Vec3}};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: i32,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f32,
    pub focus_dist: f32,
    pixel_samples_scale: f32,
    image_height: i32,
    center: Point3,
    pixel00_loc : Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,   
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

pub trait Initialize {
    fn initialize(&mut self);
}

pub trait Render {
    fn render(&mut self, world: &HittableList);
}

trait GetRay {
    fn get_ray(&self, i: i32, j: i32) -> Ray;
}

trait RayColor {
    fn ray_color(&self, r: Ray, depth: i32, world: &HittableList) -> Vec3;
}

trait DefocusDiskSample {
    fn defocus_disk_sample(&self) -> Vec3;
}

impl RayColor for Camera {
    fn ray_color(&self, r: Ray, depth: i32, world: &HittableList) -> Vec3 {
        if (depth <= 0) {
            return Vec3::new(0.0,0.0,0.0) as Color
        }
        let mut rec = HitRecord::new();
        if world.hit(r, Interval::new(0.001, INF), &mut rec) {
            let mut scattered = Ray::new(Vec3::new(0.0,0.0,0.0), Vec3::new(0.0,0.0,0.0));
            let mut attenuation = Vec3::new(0.0,0.0,0.0);
            if rec.mat.clone().scatter(&r, &mut rec, &mut attenuation, &mut scattered) {
                // eprintln!("RayColor: attenuation = {:?}, scattered = {:?}", attenuation, scattered);
                return element_wise_mul(attenuation, self.ray_color(scattered, depth-1, world))
            }
            return Color::new(0.0,0.0,0.0)
        }
        let unit_direction = r.direction().unit();
        let a = 0.5 * (unit_direction.y + 1.0);
        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a;
    }
}

impl Initialize for Camera {
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {1} else {self.image_height};

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f32;

        self.center = self.lookfrom;

        let theta = degrees_to_radians(self.vfov as f32);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = (self.image_width as f32 / self.image_height as f32)* viewport_height;

        self.w = (self.lookfrom - self.lookat).unit();
        self.u = cross(self.vup, self.w).unit();
        self.v = cross(self.w, self.u);

        let viewport_u = self.u * viewport_width;
        let viewport_v = -self.v * viewport_height;

        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        let viewport_upper_left = self.center - (self.w * self.focus_dist) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + self.pixel_delta_u * 0.5 + self.pixel_delta_v * 0.5;

        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }
}

impl Render for Camera {
    fn render(&mut self, world: &HittableList) {
        self.initialize();

        // Progress

        let bar = ProgressBar::new(self.image_height as u64);

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        // Main Render Loop

        for j in 0..self.image_height {
            bar.inc(1);
            for i in 0..self.image_width {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i,j);
                    pixel_color = pixel_color + self.ray_color(r, self.max_depth, &world.clone());
                }
                color::write_color(pixel_color * self.pixel_samples_scale);
            }
        }

        eprintln!("\nDone!", );
    }
}

impl New for Camera {
    fn new() -> Self {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            pixel_samples_scale: 1.0 / (10.0 * 10.0),
            image_height: 0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            vfov: 90,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
            defocus_angle: 0.0,
            focus_dist  : 10.0,
        }
    }
}

impl GetRay for Camera {
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc + (self.pixel_delta_u * (i as f32 + offset.x)) + (self.pixel_delta_v * (j as f32 + offset.y) );
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray {
            orig: ray_origin,
            dir: ray_direction,
        }
    }
}

impl DefocusDiskSample for Camera {
    fn defocus_disk_sample(&self) -> Vec3 {
        let p = random_in_unit_disk();
        self.center + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
    }
}
fn sample_square() -> Vec3 {
    Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
}
