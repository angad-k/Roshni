use crate::aabb;
use crate::hittable;
use crate::material;
use crate::ray;
use crate::vector3;
use std::sync::Arc;

// --------------------------- XY ---------------------------------------

#[derive(Clone)]
pub struct XYRect {
  material: Arc<material::Material>,
  x0: f64,
  x1: f64,
  y0: f64,
  y1: f64,
  k: f64,
}

impl XYRect {
  pub fn new(
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: Arc<material::Material>,
  ) -> XYRect {
    XYRect {
      x0,
      x1,
      y0,
      y1,
      k,
      material,
    }
  }
}

impl hittable::Hittable for XYRect {
  fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
    let t = (self.k - r.origin.z()) / r.dir.z();
    if t < t_min || t > t_max {
      return None;
    }

    let x = r.origin.x() + t * r.dir.x();
    let y = r.origin.y() + t * r.dir.y();

    if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
      return None;
    }

    let mut hit_record = hittable::HitRecord {
      p: r.at(t),
      t: t,
      normal: vector3::Vec3::new(0.0, 0.0, 0.0),
      front_face: false,
      material: self.material.clone(),
      u: (x - self.x0) / (self.x1 - self.x0),
      v: (y - self.y0) / (self.y1 - self.y0),
    };

    hit_record.set_face_normal(r, &vector3::Vec3::new(0.0, 0.0, 1.0));
    Some(hit_record)
  }
  fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<aabb::AABB> {
    Some(aabb::AABB::new(
      vector3::Point::new(self.x0, self.y0, self.k - 0.0001),
      vector3::Point::new(self.x1, self.y1, self.k + 0.0001),
    ))
  }
}

// --------------------------- XZ ---------------------------------------

#[derive(Clone)]
pub struct XZRect {
  material: Arc<material::Material>,
  x0: f64,
  x1: f64,
  z0: f64,
  z1: f64,
  k: f64,
}

impl XZRect {
  pub fn new(
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Arc<material::Material>,
  ) -> XZRect {
    XZRect {
      x0,
      x1,
      z0,
      z1,
      k,
      material,
    }
  }
}

impl hittable::Hittable for XZRect {
  fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
    let t = (self.k - r.origin.y()) / r.dir.y();
    if t < t_min || t > t_max {
      return None;
    }

    let x = r.origin.x() + t * r.dir.x();
    let z = r.origin.z() + t * r.dir.z();

    if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
      return None;
    }

    let mut hit_record = hittable::HitRecord {
      p: r.at(t),
      t: t,
      normal: vector3::Vec3::new(0.0, 0.0, 0.0),
      front_face: false,
      material: self.material.clone(),
      u: (x - self.x0) / (self.x1 - self.x0),
      v: (z - self.z0) / (self.z1 - self.z0),
    };

    hit_record.set_face_normal(r, &vector3::Vec3::new(0.0, 0.0, 1.0));
    Some(hit_record)
  }
  fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<aabb::AABB> {
    Some(aabb::AABB::new(
      vector3::Point::new(self.x0, self.k - 0.0001, self.z0),
      vector3::Point::new(self.x1, self.k + 0.0001, self.z1),
    ))
  }
}

// --------------------------- YZ ---------------------------------------

#[derive(Clone)]
pub struct YZRect {
  material: Arc<material::Material>,
  y0: f64,
  y1: f64,
  z0: f64,
  z1: f64,
  k: f64,
}

impl YZRect {
  pub fn new(
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Arc<material::Material>,
  ) -> YZRect {
    YZRect {
      y0,
      y1,
      z0,
      z1,
      k,
      material,
    }
  }
}

impl hittable::Hittable for YZRect {
  fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
    let t = (self.k - r.origin.x()) / r.dir.x();
    if t < t_min || t > t_max {
      return None;
    }

    let y = r.origin.y() + t * r.dir.y();
    let z = r.origin.z() + t * r.dir.z();

    if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
      return None;
    }

    let mut hit_record = hittable::HitRecord {
      p: r.at(t),
      t: t,
      normal: vector3::Vec3::new(0.0, 0.0, 0.0),
      front_face: false,
      material: self.material.clone(),
      u: (y - self.y0) / (self.y1 - self.y0),
      v: (z - self.z0) / (self.z1 - self.z0),
    };

    hit_record.set_face_normal(r, &vector3::Vec3::new(0.0, 0.0, 1.0));
    Some(hit_record)
  }
  fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<aabb::AABB> {
    Some(aabb::AABB::new(
      vector3::Point::new(self.k - 0.0001, self.y0, self.z0),
      vector3::Point::new(self.k + 0.0001, self.y1, self.z1),
    ))
  }
}
