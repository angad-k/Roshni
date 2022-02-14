use crate::ray;
use crate::vector3;
use crate::utils;
pub struct Camera {
    origin: vector3::Point,
    lower_left_corner: vector3::Point,
    horizontal: vector3::Vec3,
    vertical: vector3::Vec3,
    u: vector3::Vec3,
    v: vector3::Vec3,
    w: vector3::Vec3,
    lens_radius: f64,
    time_0 : f64,
    time_1 : f64,
}

impl Camera {
    pub fn new(
        lookfrom: vector3::Point,
        lookat: vector3::Point,
        vup: vector3::Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time_0 : f64,
        time_1 : f64,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = vector3::Vec3::unit_vector(lookfrom - lookat);
        let u = vector3::Vec3::unit_vector(vector3::cross(vup, w));
        let v = vector3::cross(w, u);

        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        Camera {
            horizontal: horizontal,
            vertical: vertical,
            origin: lookfrom,
            lower_left_corner: lookfrom - horizontal / 2.0 - vertical / 2.0 - w * focus_dist,
            u: u,
            v: v,
            w: w,
            lens_radius: aperture / 2.0,
            time_0 : time_0,
            time_1 : time_1,
        }
    }

    pub fn get_ray(&self, x: f64, t: f64) -> ray::Ray {
        let rd = vector3::Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        return ray::Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * x + self.vertical * t - self.origin - offset,
            Some(utils::random_double(self.time_0, self.time_1))
        );
    }
}
