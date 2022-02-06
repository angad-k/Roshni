pub mod color;
pub mod hittable;
pub mod image_encoder;
pub mod ray;
pub mod sphere;
pub mod vector3;
use crate::hittable::Hittable;
use cast::u32;
pub fn ray_color(r: &ray::Ray, world: Vec<sphere::Sphere>) -> vector3::Color {
    if let Some(hit) = world.clone().hit(r, 0.0, 10000000000.0) {
        //print!("{} {} {}", hit.normal.x, hit.normal.y, hit.normal.z);
        return (hit.normal + vector3::Color::new(1.0, 1.0, 1.0)) * 0.5;
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

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = vector3::Point::new(0.0, 0.0, 0.0);
    let horizontal = vector3::Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = vector3::Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - vector3::Vec3::new(0.0, 0.0, focal_length);

    // Render

    let mut img: image::RgbImage = image::ImageBuffer::new(image_width, image_height);
    for j in 0..image_height {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            //let b = 0.25 as f64;
            //let color = vector3::Color { x: r, y: g, z: b };

            let r = ray::Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let color = ray_color(&r, world.clone());
            img.put_pixel(i, image_height - j - 1, color.get_color());
        }
    }
    img.save("image.png").unwrap();
}
