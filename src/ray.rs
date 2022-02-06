use crate::vector3;
#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: vector3::Point,
    pub dir: vector3::Vec3,
}
impl Ray {
    pub fn new(p_orig: vector3::Point, p_dir: vector3::Vec3) -> Ray {
        Ray {
            origin: p_orig,
            dir: p_dir,
        }
    }

    pub fn at(self, t: f64) -> vector3::Point {
        self.origin + self.dir * t
    }
}
