use crate::material;
use crate::ray;
use crate::sphere;
use crate::vector3;
use crate::moving_sphere;
use crate::aabb;
use crate::bvh;
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
    pub fn set_face_normal(&mut self, r: &ray::Ray, outward_normal: &vector3::Vec3) {
        self.front_face = vector3::dot(r.dir, *outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = (*outward_normal) * -1.0;
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time_0 : f64, time_1 : f64) -> Option<aabb::AABB>;
}
#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<HittableObj>,
}

#[derive(Clone)]
pub enum HittableObj {
    Sphere(sphere::Sphere),
    MovingSphere(moving_sphere::MovingSphere),
    BVHNode(bvh::BVHNode),
}

impl Hittable for HittableObj {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            HittableObj::Sphere(x) => x.hit(r, t_min, t_max),
            HittableObj::MovingSphere(x) => x.hit(r, t_min, t_max),
            HittableObj::BVHNode(x) => x.hit(r, t_min, t_max),
        }
    }
    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<aabb::AABB> {
        match self {
            HittableObj::Sphere(x) => x.bounding_box(time_0, time_1),
            HittableObj::MovingSphere(x) => x.bounding_box(time_0, time_1),
            HittableObj::BVHNode(x) => x.bounding_box(time_0, time_1),
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
    pub fn add(&mut self, object: HittableObj) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;

        for object in self.objects.clone() {
            if let Some(hit) = object.hit(r, t_min, closest_so_far) {
                hit_record = Some(hit.clone());
                closest_so_far = hit.t;
            }
        }
        hit_record
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<aabb::AABB> {
        if self.objects.is_empty()
        {
            return None;
        }
        let mut output_box = None;
        for object in self.objects.clone() {
            let temp_box = object.bounding_box(time_0, time_1);
            if temp_box.is_none()
            {
                return None;
            }
            if output_box.is_none()
            {
                output_box = Some(temp_box.unwrap());
            }
            else
            {
                output_box = Some(aabb::surrounding_box(temp_box.unwrap(), output_box.unwrap()));
            }
        }
        output_box
    } 
}
