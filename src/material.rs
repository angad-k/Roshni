use crate::hittable;
use crate::ray;
use crate::utils;
use crate::vector3;

pub trait materialtrait {
    fn scatter(self, r: &ray::Ray, rec: &hittable::HitRecord) -> (bool, vector3::Color, ray::Ray);
}
#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl materialtrait for Material {
    fn scatter(self, _r: &ray::Ray, rec: &hittable::HitRecord) -> (bool, vector3::Color, ray::Ray) {
        match self {
            Material::Lambertian(x) => x.scatter(_r, rec),
            Material::Metal(x) => x.scatter(_r, rec),
        }
    }
}
#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: vector3::Color,
}

impl Lambertian {
    pub fn new(p_albedo: vector3::Color) -> Lambertian {
        Lambertian { albedo: p_albedo }
    }
}

impl materialtrait for Lambertian {
    fn scatter(self, _r: &ray::Ray, rec: &hittable::HitRecord) -> (bool, vector3::Color, ray::Ray) {
        let mut scatter_direction = rec.normal + vector3::Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }
        // yeh sab jo change krke bhej rhe usse bhi hit record mei dalna mangtau
        let scattered = ray::Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo.clone();
        (true, attenuation, scattered)
    }
}
#[derive(Copy, Clone)]
pub struct Metal {
    albedo: vector3::Color,
}

impl Metal {
    pub fn new(p_albedo: vector3::Color) -> Metal {
        Metal { albedo: p_albedo }
    }
}

impl materialtrait for Metal {
    fn scatter(self, r: &ray::Ray, rec: &hittable::HitRecord) -> (bool, vector3::Color, ray::Ray) {
        let reflected = vector3::reflect(r.dir.unit_vector(), rec.normal);
        let scattered = ray::Ray::new(rec.p, reflected);
        let attenuation = self.albedo.clone();
        (
            (vector3::dot(scattered.dir, rec.normal) > 0.0),
            attenuation,
            scattered,
        )
    }
}
