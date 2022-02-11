use crate::hittable;
use crate::material;
use crate::ray;
use crate::vector3;
use std::sync::{Arc, Mutex};
#[derive(Clone)]
pub struct Sphere {
    center: vector3::Point,
    radius: f64,
    material: Arc<Mutex<material::Material>>,
}
impl Sphere {
    pub fn new(cen: vector3::Point, r: f64, mat: Arc<Mutex<material::Material>>) -> Sphere {
        Sphere {
            material: mat,
            center: cen,
            radius: r,
        }
    }
}

impl hittable::Hittable for Sphere {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
        let oc = r.origin - self.center;
        let a = r.dir.length_squared();
        let half_b = vector3::dot(oc, r.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let mut hit_record = hittable::HitRecord {
            p: r.at(root),
            t: root,
            normal: vector3::Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
            material: self.material.clone(),
        };
        let outward_normal: vector3::Vec3 = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(r, &outward_normal);
        Some(hit_record)
    }
}
