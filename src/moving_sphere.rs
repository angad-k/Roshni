use crate::hittable;
use crate::material;
use crate::ray;
use crate::vector3;
use crate::aabb;
use std::sync::{Arc, Mutex};
#[derive(Clone)]
pub struct MovingSphere {
    center0: vector3::Point,
    center1: vector3::Point,
    radius: f64,
    material: Arc<Mutex<material::Material>>,
    time_0 : f64,
    time_1 : f64,
}
impl MovingSphere {
    pub fn new(cen0: vector3::Point, cen1: vector3::Point, t_0 : f64, t_1 : f64, r: f64, mat: Arc<Mutex<material::Material>>) -> MovingSphere {
        MovingSphere {
            material: mat,
            center0: cen0,
            center1: cen1,
            time_0: t_0,
            time_1: t_1,
            radius: r,
        }
    }

    pub fn center(&self, time : f64) -> vector3::Point {
        return self.center0 + (self.center1 - self.center0) * ((time - self.time_0) / (self.time_1 - self.time_0));
    }
}

impl hittable::Hittable for MovingSphere {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
        let oc = r.origin - self.center(r.time);
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
            u : 0.0,
            v : 0.0
        };
        let outward_normal: vector3::Vec3 = (hit_record.p - self.center(r.time)) / self.radius;
        hit_record.set_face_normal(r, &outward_normal);
        let (u, v) = crate::sphere::get_sphere_uv(outward_normal);
        hit_record.u = u;
        hit_record.v = v;
        Some(hit_record)
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<aabb::AABB> {
        let box0 = aabb::AABB::new(
            self.center(time_0) - vector3::Vec3::new(self.radius, self.radius, self.radius),
            self.center(time_0) + vector3::Vec3::new(self.radius, self.radius, self.radius));
        let box1 = aabb::AABB::new(
            self.center(time_1) - vector3::Vec3::new(self.radius, self.radius, self.radius),
            self.center(time_1) + vector3::Vec3::new(self.radius, self.radius, self.radius));

        Some(aabb::surrounding_box(box0, box1))
    }
}
