use crate::vector3;
#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: vector3::Point,
    pub dir: vector3::Vec3,
    pub time : f64,
}
impl Ray {
    pub fn new(p_orig: vector3::Point, p_dir: vector3::Vec3, p_time : Option<f64>) -> Ray {
        Ray {
            origin: p_orig,
            dir: p_dir,
            time : p_time.unwrap_or(0.0),
        }
    }

    pub fn at(self, t: f64) -> vector3::Point {
        self.origin + self.dir * t
    }
}
