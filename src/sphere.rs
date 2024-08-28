use std::ops::Mul;

use crate::material::{self, Material, Scatter};
use crate::vec3::{div, LengthSquared, mul, Dot, Vec3};
use crate::point3::Point3;
use crate::ray::{At, Direction, Origin, Ray};
use crate::interval::{Interval, Surrounds};

pub trait Hit {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

pub trait Clear {
    fn clear(&mut self);
}

#[derive(Clone)]
pub enum HittableObject {
    Sphere(Sphere),
}
impl Hit for HittableObject {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        match self {
            HittableObject::Sphere(sphere) => sphere.hit(r, ray_t, rec),
            // Add other cases here as needed
        }
    }
}

pub trait New {
    fn new() -> Self;
}
pub trait SetFaceNormal {
    fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3);
}

pub trait Add {
    fn add(&mut self, rhs: HittableObject);
}

pub struct HitRecord {
    pub(crate) p: Point3,
    pub(crate) normal: Vec3,
    pub(crate) t: f32,
    pub(crate) front_face: bool,
    pub(crate) mat: Material,
}
pub struct Sphere {
    pub(crate) center: Point3,
    pub(crate) radius: f32,
    pub(crate) mat: Material,
}

pub struct HittableList {
    objects: Vec<HittableObject>,
}


impl Hit for Sphere {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = h*h - a*c;

        if discriminant < 0.0 {
            return false;
        } else {
            let root = discriminant.sqrt();
            let mut temp = (h - root) / a;
            if !ray_t.surrounds(temp) {
                temp = (h + root) / a;
                if !ray_t.surrounds(temp) {
                    return false;
                }
            }
            rec.t = temp;
            rec.p = r.at(rec.t);
            
            let outward_normal = (rec.p - self.center) / self.radius;

            rec.set_face_normal(r, outward_normal);

            rec.mat = self.mat;

            return true;
        }
    }
}

impl SetFaceNormal for HitRecord {
    fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else {-outward_normal};
    }
}

impl Copy for HitRecord {}

impl Clone for HitRecord {
    fn clone(&self) -> Self {
        *self
    }
}

impl Origin for Sphere {
    fn origin(&self) -> Point3 {
        self.center
    }
}

impl Clear for HittableList {
    fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hit for HittableList {
    fn hit(&self, r: Ray, ray_t: Interval,  rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            p: Point3 { x: 0.0, y: 0.0, z: 0.0 },
            normal: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            t: 0.0,
            mat: material::Material::Lambertian(material::Lambertian::new(Vec3::new(0.0, 0.0, 0.0))),
            front_face: false,
        };
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(r,  Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.t = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
                rec.front_face = temp_rec.front_face;
                rec.mat = temp_rec.mat;
            }
        }

        hit_anything
    }
}
impl New for HittableList {
    fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
}

impl Clone for HittableList {
    fn clone(&self) -> Self {
        HittableList {
            objects: self.objects.clone(),
        }
    }
}

impl Clone for Sphere {
    fn clone(&self) -> Self {
        Sphere {
            center: self.center,
            radius: self.radius,
            mat: self.mat.clone(),
        }
    }
}

impl Add for HittableList {
    fn add(&mut self, rhs: HittableObject) {
        self.objects.push(rhs);
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        
        Self { center, radius, mat: material::Material::Lambertian(material::Lambertian::new(Vec3::new(0.0, 0.0, 0.0))) }
    }
}

impl New for HitRecord {
    fn new() -> Self {
        HitRecord {
            p: Point3 { x: 0.0, y: 0.0, z: 0.0 },
            normal: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            t: 0.0,
            front_face: false,
            mat: material::Material::Lambertian(material::Lambertian::new(Vec3::new(0.0, 0.0, 0.0))),
        }
    }
}

