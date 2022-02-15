pub mod camera;
pub mod color;
pub mod hittable;
pub mod image_encoder;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod utils;
pub mod vector3;
pub mod moving_sphere;
pub mod aabb;
pub mod bvh;
use crate::hittable::Hittable;
use crate::material::MaterialTrait;
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
    let image_width: u32 = 400;
    let image_height: u32 = u32(image_width as f64 / aspect_ratio).unwrap();
    let samples_per_pixel = 150;
    let max_depth: i32 = 50;

    // World
    let mut world = random_scene();
    let bvh_root = bvh::BVHNode::new(world.clone(), 0, world.objects.len() as i32, 0.0, 1.0);
    world = hittable::HittableList::new();
    world.add(hittable::HittableObj::BVHNode(bvh_root));

    // Camera
    let lookfrom = vector3::Point::new(13.0, 2.0, 3.0);
    let lookat = vector3::Point::new(0.0, 0.0, 0.0);
    let vup = vector3::Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = camera::Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0
    );

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
    println!(" Image rendered in {:.2?}", elapsed);
}

pub fn random_scene() -> hittable::HittableList {
    let mut world = hittable::HittableList::new();

    let ground_material = Arc::new(Mutex::new(material::Material::Lambertian(
        material::Lambertian::new(vector3::Color::new(0.5, 0.5, 0.5)),
    )));
    
    world.add(hittable::HittableObj::Sphere(sphere::Sphere::new(
        vector3::Point::new(0.0, -1000.0, -0.0),
        1000.0,
        ground_material.clone(),
    )));
    /*
    //let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utils::random_double(0.0, 1.0);
            let center = vector3::Point::new(
                a as f64 + 0.9 * utils::random_double(0.0, 1.0),
                0.2,
                b as f64 + 0.9 * utils::random_double(0.0, 1.0),
            );

            if (center - vector3::Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo =
                        vector3::Color::random(0.0, 1.0) * vector3::Color::random(0.0, 1.0);
                    let sphere_material = Arc::new(Mutex::new(material::Material::Lambertian(
                        material::Lambertian::new(albedo),
                    )));
                    world.add(hittable::HittableObj::Sphere(sphere::Sphere::new(
                        center,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = vector3::Color::random(0.5, 1.0);
                    let _fuzz = utils::random_double(0.0, 0.5);
                    let sphere_material = Arc::new(Mutex::new(material::Material::Metal(
                        material::Metal::new(albedo),
                    )));
                    world.add(hittable::HittableObj::Sphere(sphere::Sphere::new(
                        center,
                        0.2,
                        sphere_material,
                    )));
                } else {
                    // glass
                    let sphere_material = Arc::new(Mutex::new(material::Material::Dielectric(
                        material::Dielectric::new(1.5),
                    )));
                    world.add(hittable::HittableObj::Sphere(sphere::Sphere::new(
                        center,
                        0.2,
                        sphere_material,
                    )));
                }
            }
        }
    }*/

    let material1 = Arc::new(Mutex::new(material::Material::Dielectric(
        material::Dielectric::new(1.5),
    )));
    world.add(hittable::HittableObj::Sphere(sphere::Sphere::new(
        vector3::Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Mutex::new(material::Material::Lambertian(
        material::Lambertian::new(vector3::Color::new(0.4, 0.2, 0.1)),
    )));
    
    world.add(hittable::HittableObj::MovingSphere(moving_sphere::MovingSphere::new(
        vector3::Point::new(-4.0, 1.0, -1.0),
        vector3::Point::new(-4.0, 1.0, 1.0),
        0.0,
        1.0,
        1.0,
        material2,
    )));

    let material3 = Arc::new(Mutex::new(material::Material::Metal(material::Metal::new(
        vector3::Color::new(0.4, 0.2, 0.1),
    ))));
    world.add(hittable::HittableObj::Sphere(sphere::Sphere::new(
        vector3::Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    return world;
}
