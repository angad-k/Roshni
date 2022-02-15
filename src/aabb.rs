use crate::vector3;
use crate::ray;
use crate::hittable::HittableObj;
use crate::hittable::Hittable;
use std::cmp;
#[derive(Copy, Clone)]
pub struct AABB
{
  pub minimum : vector3::Point,
  pub maximum : vector3::Point,
}

impl AABB{
  pub fn new(minimum : vector3::Point, maximum : vector3::Point) -> AABB {
    //println!("AABB created with ({},{},{}) and ({},{},{})", minimum.x, minimum.y, minimum.z, maximum.x, maximum.y, maximum.z);
    AABB {
      minimum,
      maximum,
    }
  }
  pub fn hit(&self, r: &ray::Ray, mut t_min: f64, mut t_max: f64) -> bool {
    let tx_0 = ((self.minimum.x - r.origin.x) / r.dir.x).min((self.maximum.x - r.origin.x) / r.dir.x);
    let tx_1 = ((self.minimum.x - r.origin.x) / r.dir.x).max((self.maximum.x - r.origin.x) / r.dir.x);
    t_min = t_min.max(tx_0);
    t_max = t_max.min(tx_1);
    if t_max <= t_min {
      return false;
    }
    let ty_0 = ((self.minimum.y - r.origin.y) / r.dir.y).min((self.maximum.y - r.origin.y) / r.dir.y);
    let ty_1 = ((self.minimum.y - r.origin.y) / r.dir.y).max((self.maximum.y - r.origin.y) / r.dir.y);
    t_min = t_min.max(ty_0);
    t_max = t_max.min(ty_1); 
    if t_max <= t_min {
      return false;
    }
    let tz_0 = ((self.minimum.z - r.origin.z) / r.dir.z).min((self.maximum.z - r.origin.z) / r.dir.z);
    let tz_1 = ((self.minimum.z - r.origin.z) / r.dir.z).max((self.maximum.z - r.origin.z) / r.dir.z);
    t_min = t_min.max(tz_0);
    t_max = t_max.min(tz_1); 
    if t_max <= t_min {
      return false;
    }
    true
  }
}

pub fn surrounding_box(box0 : AABB, box1 : AABB) -> AABB {
  let smol_point = vector3::Point::new(
    box0.minimum.x.min(box1.minimum.x), 
    box0.minimum.y.min(box1.minimum.y), 
    box0.minimum.z.min(box1.minimum.z)
  );
  let big_point = vector3::Point::new(
    box0.maximum.x.max(box1.maximum.x), 
    box0.maximum.y.max(box1.maximum.y), 
    box0.maximum.z.max(box1.maximum.z)
  );
  AABB::new(smol_point, big_point)
}

pub fn box_compare(a : HittableObj, b : HittableObj, axis : i32) -> cmp::Ordering{
  let box_a = a.bounding_box(0.0, 0.0);
  let box_b = b.bounding_box(0.0, 0.0);
  let x:f64;
  let y:f64;
  if axis == 0
  {
    x = box_a.unwrap().minimum.x;
    y = box_b.unwrap().minimum.x;
  }
  else if axis == 1
  {
    x = box_a.unwrap().minimum.y;
    y = box_b.unwrap().minimum.y;
  }
  else
  {
    x = box_a.unwrap().minimum.z;
    y = box_b.unwrap().minimum.z;
  }
  if x < y
  {
    return cmp::Ordering::Less;
  }
  if x == y
  {
    return cmp::Ordering::Equal;
  }
  else
  {
    return cmp::Ordering::Greater;
  }
}