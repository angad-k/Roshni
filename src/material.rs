use crate::ray;
use crate::vector3;
pub trait Material {
    fn scatter(r: &ray::Ray, attenuation: vector3::Color, scattered: ray::Ray) -> bool;
}
