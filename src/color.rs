use crate::vector3;
use cast::u8;
impl vector3::Color {
    pub fn get_color(self) -> image::Rgb<u8> {
        let ir: u8 = u8(255.0 * self.x).unwrap();
        let ig: u8 = u8(255.0 * self.y).unwrap();
        let ib: u8 = u8(255.0 * self.z).unwrap();
        image::Rgb([ir, ig, ib])
    }
}
