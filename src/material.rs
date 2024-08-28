use crate::{color::Color, ray::{Direction, Ray}, rtweekend::random_double, sphere::HitRecord, vec3::{random_unit_vector, reflect, refract, Dot, NearZero, Unit, Vec3}};

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dialectric(Dialectric),
    // Add other material types here as needed
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

pub struct Lambertian {
    pub albedo: Color,
}

pub struct Dialectric {
    pub ior: f32,
}

pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

impl Scatter for Material {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian(lambertian) => Lambertian::scatter(lambertian, r_in, rec, attenuation, scattered),
            Material::Metal(metal) => Metal::scatter(metal, r_in, rec, attenuation, scattered),
            Material::Dialectric(dialectric) => Dialectric::scatter(dialectric, r_in, rec, attenuation, scattered),
            // Add other material types here as needed
        }
    }
}

impl Copy for Material {}

impl Clone for Material {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Lambertian {}

impl Clone for Lambertian {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Metal {}

impl Clone for Metal {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Dialectric {}

impl Clone for Dialectric {
    fn clone(&self) -> Self {
        *self
    }
}

impl Lambertian {
    pub fn new(a: Color) -> Self {
        Self { albedo: a }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, mut attenuation: &mut Vec3, mut scattered: &mut Ray) -> bool {
        let scatter_direction: Vec3;
        if (rec.normal + random_unit_vector()).near_zero() {
            scatter_direction = rec.normal;
        }
        else {
            scatter_direction = rec.normal + random_unit_vector();
        }
        let new_ray = Ray::new(rec.p, scatter_direction);
        *scattered = new_ray.clone();
        *attenuation = self.albedo.clone();
        // eprintln!("Lambertian scatter: attenuation = {:?}, scattered = {:?}", attenuation, scattered);
        true
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, mut attenuation: &mut Color, mut scattered: &mut Ray) -> bool {
        let reflected = reflect(r_in.direction().unit(),rec.normal);
        let new_ray = Ray::new(rec.p, reflected + random_unit_vector() * self.fuzz);
        *scattered = new_ray.clone();
        *attenuation = self.albedo.clone();
        // eprintln!("Metal scatter: attenuation = {:?}, scattered = {:?}", attenuation, scattered);
        scattered.direction().dot(rec.normal) > 0.0
    }
}

impl Metal {
    pub fn new(a: Color, f: f32) -> Self {
        Self { albedo: a, fuzz: f }
    }
}

impl Dialectric {
    pub fn new(i: f32) -> Self {
        Self { ior: i }
    }
}

impl Scatter for Dialectric {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, mut attenuation: &mut Color, mut scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {1.0 / self.ior} else {self.ior};
        let unit_direction = r_in.direction().unit();

        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double() {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, refraction_ratio)
        };

        *scattered = Ray::new(rec.p, direction);
        true
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1.0-ref_idx) / (1.0+ref_idx)).powi(2);
    r0 + (1.0-r0)*(1.0-cosine).powi(5)
}