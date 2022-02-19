// For reading and opening files
use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;

pub fn write_image(path: &str, width: u32, height: u32, data: &[u8]) {
    let output = File::create(path).unwrap();
    let encoder = PNGEncoder::new(output);
    encoder
        .encode(data, width, height, ColorType::RGB(8))
        .unwrap();
}

pub fn read_image(path: &str) -> image::DynamicImage {
    let img = image::open(path).unwrap();
    img
}
