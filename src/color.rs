use crate::vector3;
use cast::u8;
use std::cmp::max;
use std::cmp::min;
impl vector3::Color {
    pub fn get_color(self, samples_per_pixel: i32) -> image::Rgb<u8> {
        let ir: u8 = u8(255.0 * clamp(self.x / samples_per_pixel as f64)).unwrap();
        let ig: u8 = u8(255.0 * clamp(self.y / samples_per_pixel as f64)).unwrap();
        let ib: u8 = u8(255.0 * clamp(self.z / samples_per_pixel as f64)).unwrap();

        image::Rgb([ir, ig, ib])
    }
}
pub fn clamp(p: f64) -> f64 {
    if p < 0.0 {
        return 0.0;
    } else if p > 0.999 {
        return 0.999;
    } else {
        return p;
    }
}
