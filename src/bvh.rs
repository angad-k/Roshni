use crate::hittable;
use crate::hittable::Hittable;
use crate::aabb;
use crate::ray;
use crate::utils;
#[derive(Clone)]
pub struct BVHNode{
  left : Box<hittable::HittableObj>,
  right : Box<hittable::HittableObj>,
  aabb_box : aabb::AABB,
}

impl hittable::Hittable for BVHNode {
  fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
    // Adding comments here since this is too much of spagetti.
    // if ray doesnt hit the bounding box for this node - sori tata bye bye
    if !self.aabb_box.hit(r, t_min, t_max)
    {
      return None;
    }

    // check for hit on the left node. if not return the result for right node
    let hit_left = self.left.hit(r, t_min, t_max);
    if hit_left.is_none()
    {
      return self.right.hit(r, t_min, t_max);
    }
    // check for hit on right with shorter t range. If found - right's t will be better
    // so, if right is not hit return left
    // else return right
    let hit_left = hit_left.unwrap();
    let hit_right = self.right.hit(r, t_min, hit_left.t);
    if hit_right.is_none()
    {
      return Some(hit_left);
    }
    return hit_right;
  }
  fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<aabb::AABB> {
    Some(self.aabb_box)
  }
}

impl BVHNode {
  pub fn new(mut objects :  hittable::HittableList, start : i32, end : i32, time_0 : f64, time_1 : f64) -> BVHNode {
    let axis_d = utils::random_double(0.0, 3.0);
    let axis : i32;
    if axis_d <= 1.0
    {
      axis = 0;
    }
    else if axis_d <= 2.0
    {
      axis = 1;
    }
    else
    {
      axis = 2;
    }

    let object_span = end - start;
    let left : hittable::HittableObj;
    let right : hittable::HittableObj;
    if object_span == 1
    {
      left = objects.objects[start as usize].clone();
      right = objects.objects[start as usize].clone();
    }
    else if object_span == 2
    {
      if aabb::box_compare(objects.objects[start as usize].clone(), objects.objects[(start+1) as usize].clone(), axis) == std::cmp::Ordering::Less
      {
        left = objects.objects[start as usize].clone();
        right = objects.objects[(start+1) as usize].clone();
      }
      else
      {
        left = objects.objects[(start+1) as usize].clone();
        right = objects.objects[start as usize].clone();
      }
    }
    else
    {
      objects.objects.sort_by(|a, b| {aabb::box_compare(a.clone(), b.clone(), axis)});
      let mid = start + object_span/2;
      left = hittable::HittableObj::BVHNode(BVHNode::new(objects.clone(), start, mid, time_0, time_1));
      right = hittable::HittableObj::BVHNode(BVHNode::new(objects.clone(), mid, end, time_0, time_1));
    }

    let box_left = left.bounding_box(time_0, time_1);
    let box_right = right.bounding_box(time_0, time_1);

    let aabb_box = aabb::surrounding_box(box_left.unwrap(), box_right.unwrap());

    BVHNode{
      left : Box::new(left),
      right : Box::new(right),
      aabb_box
    }
  }
}