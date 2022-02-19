use crate::aabb;
use crate::hittable;
use crate::material;
use crate::ray;
use crate::vector3;
use std::sync::Arc;
#[derive(Clone)]
pub struct Sphere {
    center: vector3::Point,
    radius: f64,
    material: Arc<material::Material>,
}
impl Sphere {
    pub fn new(cen: vector3::Point, r: f64, mat: Arc<material::Material>) -> Sphere {
        Sphere {
            material: mat,
            center: cen,
            radius: r,
        }
    }
}

pub fn get_sphere_uv(p: vector3::Point) -> (f64, f64) {
    let theta = (-p.y).acos();
    let phi = (-p.z).atan2(p.x);
    let u = phi / (2.0 * std::f64::consts::PI);
    let v = theta / std::f64::consts::PI;
    (u, v)
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
            u: 0.0,
            v: 0.0,
        };
        let outward_normal: vector3::Vec3 = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(r, &outward_normal);
        let (u, v) = get_sphere_uv(outward_normal);
        hit_record.u = u;
        hit_record.v = v;
        Some(hit_record)
    }
    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<aabb::AABB> {
        Some(aabb::AABB::new(
            self.center - vector3::Vec3::new(self.radius, self.radius, self.radius),
            self.center + vector3::Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}
