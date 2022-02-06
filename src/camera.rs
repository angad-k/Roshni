use crate::ray;
use crate::vector3;
#[derive(Copy, Clone)]
pub struct Camera {
    origin: vector3::Point,
    lower_left_corner: vector3::Point,
    horizontal: vector3::Vec3,
    vertical: vector3::Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let horizontal = vector3::Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = vector3::Vec3::new(0.0, viewport_height, 0.0);
        let origin = vector3::Point::new(0.0, 0.0, 0.0);
        Camera {
            horizontal: horizontal,
            vertical: vertical,
            origin: origin,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - vector3::Vec3::new(0.0, 0.0, focal_length),
        }
    }

    pub fn get_ray(self, u: f64, v: f64) -> ray::Ray {
        ray::Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
