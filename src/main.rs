pub mod image_encoder;

use cast::u8;
fn main() {
    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 256;
    let mut img: image::RgbImage = image::ImageBuffer::new(WIDTH, HEIGHT);
    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let r = i as f64 / (WIDTH - 1) as f64;
            let g = j as f64 / (HEIGHT - 1) as f64;
            let b = 0.25 as f64;
            let ir: u8 = u8(255.0 * r).unwrap();
            let ig: u8 = u8(255.0 * g).unwrap();
            let ib: u8 = u8(255.0 * b).unwrap();

            let p = image::Rgb([ir, ig, ib]);

            img.put_pixel(i, j, p);
        }
    }
    img.save("image.png").unwrap();
}
