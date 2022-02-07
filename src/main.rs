pub mod camera;
pub mod color;
pub mod hittable;
pub mod image_encoder;
pub mod ray;
pub mod sphere;
pub mod utils;
pub mod vector3;
use crate::hittable::Hittable;
use cast::u32;
use pbr::ProgressBar;
use rand::Rng;
use rayon::prelude::*;
use std::sync::Mutex;
pub fn ray_color(r: &ray::Ray, world: Vec<sphere::Sphere>, depth: i32) -> vector3::Color {
    if depth <= 0 {
        return vector3::Color::new(0.0, 0.0, 0.0);
    }
    if let Some(hit) = world.clone().hit(r, 0.001, 10000000000.0) {
        //print!("{} {} {}", hit.normal.x, hit.normal.y, hit.normal.z);
        let target = hit.p + hit.normal + vector3::Vec3::random_in_unit_sphere();
        return ray_color(&ray::Ray::new(hit.p, target - hit.p), world, depth - 1) * 0.5;
    }
    let unit_direction: vector3::Vec3 = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return vector3::Color::new(1.0, 1.0, 1.0) * (1.0 - t) + vector3::Color::new(0.5, 0.7, 1.0) * t;
}

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = u32(image_width as f64 / aspect_ratio).unwrap();
    let samples_per_pixel = 100;
    let max_depth: i32 = 50;

    // World
    let mut world: Vec<sphere::Sphere> = Vec::new();
    world.push(sphere::Sphere::new(
        vector3::Point::new(0.0, 0.0, -1.0),
        0.5,
    ));
    world.push(sphere::Sphere::new(
        vector3::Point::new(0.0, -100.5, -1.0),
        100.0,
    ));

    // Camera

    let cam = camera::Camera::new();

    // Progress bar
    let mut pb = ProgressBar::new((image_height * image_width) as u64);
    pb.format("╢▌▌░╟");

    // Render
    let mut img: image::RgbImage = image::ImageBuffer::new(image_width, image_height);
    let mut img_vec: Vec<vector3::Color> =
        vec![vector3::Color::new(0.0, 0.0, 0.0); (image_height * image_width) as usize];
    for j in 0..image_height {
        for i in 0..image_width {
            img_vec[(i + image_width * j) as usize] = vector3::Color::new(i as f64, j as f64, 0.0);
        }
    }
    img_vec.par_iter_mut().enumerate().for_each(|(index, val)| {
        let mut rng = rand::thread_rng();
        let i = index % (image_width as usize);
        let j = index / (image_width as usize);
        let mut pixel_color = vector3::Color::new(0.0, 0.0, 0.0);
        for _s in 0..samples_per_pixel {
            let u = (i as f64 + rng.gen_range(0.0..1.0)) / (image_width - 1) as f64;
            let v = (j as f64 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f64;
            //println!("{} {} ", u, v);
            let r = cam.get_ray(u, v);
            pixel_color = pixel_color + ray_color(&r, world.clone(), max_depth);
        }
        *val = pixel_color;
    });

    for j in 0..image_height {
        for i in 0..image_width {
            img.put_pixel(
                i,
                image_height - j - 1,
                img_vec[(i + image_width * j) as usize].get_color(samples_per_pixel),
            );
        }
    }
    img.save("image.png").unwrap();
    //pb.finish_print("Image Rendered :)");
}
