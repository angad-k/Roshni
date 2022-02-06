use crate::ray;
use crate::sphere;
use crate::vector3;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: vector3::Point,
    pub normal: vector3::Vec3,
    pub t: f64,
    pub front_face: bool,
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
pub struct HittableList<HitType> {
    objects: Vec<HitType>,
}

impl HittableList<sphere::Sphere> {
    pub fn new() -> HittableList<sphere::Sphere> {
        return HittableList {
            objects: Vec::new(),
        };
    }
    pub fn clear(mut self) {
        self.objects.clear();
    }
    pub fn add(mut self, object: sphere::Sphere) {
        self.objects.push(object);
    }
}

impl Hittable for Vec<sphere::Sphere> {
    fn hit(self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(hit) = object.hit(r, t_min, closest_so_far) {
                hit_record = Some(hit);
                closest_so_far = hit.t;
            }
        }
        hit_record
    }
}
