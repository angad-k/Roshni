use crate::material;
use crate::ray;
use crate::sphere;
use crate::vector3;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct HitRecord {
    pub p: vector3::Point,
    pub normal: vector3::Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<Mutex<material::Material>>,
}

impl HitRecord {
    pub fn is_front_face(r: &ray::Ray, outward_normal: &vector3::Vec3) -> bool {
        vector3::dot(r.dir, *outward_normal) < 0.0
    }

    pub fn set_face_normal(mut self, r: &ray::Ray, outward_normal: &vector3::Vec3) {
        self.front_face = vector3::dot(r.dir, *outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = (*outward_normal) * -1.0;
        }
    }
}

pub trait Hittable {
    fn hit(self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
#[derive(Clone)]
pub struct HittableList {
    objects: Vec<HittableObj>,
}

#[derive(Clone)]
pub enum HittableObj {
    Sphere(sphere::Sphere),
}

impl Hittable for HittableObj {
    fn hit(self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            HittableObj::Sphere(x) => x.hit(r, t_min, t_max),
        }
    }
}

impl HittableList {
    pub fn new() -> HittableList {
        return HittableList {
            objects: Vec::new(),
        };
    }
    pub fn clear(mut self) -> HittableList {
        self.objects.clear();
        self
    }
    pub fn add(mut self, object: HittableObj) -> HittableList {
        self.objects.push(object);
        self
    }
}

impl Hittable for HittableList {
    fn hit(self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;

        for object in self.objects {
            if let Some(hit) = object.hit(r, t_min, closest_so_far) {
                hit_record = Some(hit.clone());
                closest_so_far = hit.t;
            }
        }
        hit_record
    }
}
