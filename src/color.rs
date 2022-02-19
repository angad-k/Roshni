use crate::utils;
use crate::vector3;
use cast::u8;
use std::u8;
impl vector3::Color {
    pub fn get_color(self, samples_per_pixel: i32) -> image::Rgb<u8> {
        let ir: u8 =
            u8(255.0 * (self.x / samples_per_pixel as f64).sqrt().clamp(0.0, 0.999)).unwrap();
        let ig: u8 =
            u8(255.0 * (self.y / samples_per_pixel as f64).sqrt().clamp(0.0, 0.999)).unwrap();
        let ib: u8 =
            u8(255.0 * (self.z / samples_per_pixel as f64).sqrt().clamp(0.0, 0.999)).unwrap();

        image::Rgb([ir, ig, ib])
    }
}
