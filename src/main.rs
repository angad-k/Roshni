pub mod camera;
pub mod color;
pub mod hittable;
pub mod image_encoder;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod utils;
pub mod vector3;
use crate::hittable::Hittable;
use crate::material::materialtrait;
use cast::u32;
use pbr::ProgressBar;
use rand::Rng;
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;
pub fn ray_color(r: &ray::Ray, world: hittable::HittableList, depth: i32) -> vector3::Color {
    if depth <= 0 {
        return vector3::Color::new(0.0, 0.0, 0.0);
    }
    if let Some(hit) = world.clone().hit(r, 0.001, 10000000000.0) {
        //print!("{} {} {}", hit.normal.x, hit.normal.y, hit.normal.z);
        //let target = hit.p + vector3::Vec3::random_in_hemisphere(hit.normal);
        //return ray_color(&ray::Ray::new(hit.p, target - hit.p), world, depth - 1) * 0.5;
        let (did_scatter, attenuation, scattered) = &hit.material.lock().unwrap().scatter(&r, &hit);
        if *did_scatter {
            return *attenuation * ray_color(scattered, world, depth - 1);
        } else {
            return vector3::Color::new(0.0, 0.0, 0.0);
        }
    }
    let unit_direction: vector3::Vec3 = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return vector3::Color::new(1.0, 1.0, 1.0) * (1.0 - t) + vector3::Color::new(0.5, 0.7, 1.0) * t;
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 800;
    let image_height: u32 = u32(image_width as f64 / aspect_ratio).unwrap();
    let samples_per_pixel = 600;
    let max_depth: i32 = 50;

    // World
    let mut world = hittable::HittableList::new();
    let material_ground = Arc::new(Mutex::new(material::Material::Lambertian(
        material::Lambertian::new(vector3::Color::new(0.8, 0.8, 0.0)),
    )));
    let material_center = Arc::new(Mutex::new(material::Material::Lambertian(
        material::Lambertian::new(vector3::Color::new(0.7, 0.3, 0.3)),
    )));
    let material_left = Arc::new(Mutex::new(material::Material::Metal(material::Metal::new(
        vector3::Color::new(0.8, 0.8, 0.8),
    ))));
    let material_right = Arc::new(Mutex::new(material::Material::Metal(material::Metal::new(
        vector3::Color::new(0.8, 0.6, 0.2),
    ))));
    world = world.add(hittable::HittableObj::Sphere(sphere::Sphere::new(
        vector3::Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));
    world = world.add(hittable::HittableObj::Sphere(sphere::Sphere::new(
        vector3::Point::new(0.0, 0.0, -1.0),
        0.5,
        material_center.clone(),
    )));
    world = world.add(hittable::HittableObj::Sphere(sphere::Sphere::new(
        vector3::Point::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world = world.add(hittable::HittableObj::Sphere(sphere::Sphere::new(
        vector3::Point::new(1.0, 0.0, -1.0),
        0.5,
        material_right.clone(),
    )));

    // Camera

    let cam = camera::Camera::new();

    // Progress bar
    let pb = Mutex::new(ProgressBar::new((image_height * image_width) as u64));
    pb.lock().unwrap().format("╢▌▌░╟");
    // Render
    let mut img: image::RgbImage = image::ImageBuffer::new(image_width, image_height);
    let mut img_vec: Vec<vector3::Color> =
        vec![vector3::Color::new(0.0, 0.0, 0.0); (image_height * image_width) as usize];
    //Paralellization, yay
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
        pb.lock().unwrap().inc();
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
    let elapsed = now.elapsed();
    pb.lock().unwrap().finish_print("Image Rendered :)");
    println!("Image rendered in {:.2?}", elapsed);
}
