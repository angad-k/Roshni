use crate::hittable;
use crate::ray;
use crate::utils;
use crate::vector3;
use rand::Rng;
use std::cmp;

pub trait materialtrait {
    fn scatter(self, r: &ray::Ray, rec: &hittable::HitRecord) -> (bool, vector3::Color, ray::Ray);
}
#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl materialtrait for Material {
    fn scatter(self, _r: &ray::Ray, rec: &hittable::HitRecord) -> (bool, vector3::Color, ray::Ray) {
        match self {
            Material::Lambertian(x) => x.scatter(_r, rec),
            Material::Metal(x) => x.scatter(_r, rec),
            Material::Dielectric(x) => x.scatter(_r, rec),
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

#[derive(Copy, Clone)]
pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(p_ir: f64) -> Dielectric {
        Dielectric { ir: p_ir }
    }
    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
    }
}

impl materialtrait for Dielectric {
    fn scatter(self, r: &ray::Ray, rec: &hittable::HitRecord) -> (bool, vector3::Color, ray::Ray) {
        let attenuation = vector3::Color::new(1.0, 1.0, 1.0);
        let mut refraction_ratio = 1.0 / self.ir;
        if !rec.front_face {
            refraction_ratio = self.ir;
        }

        let unit_direction = vector3::Vec3::unit_vector(r.dir);

        let cos_theta: f64 = vector3::dot(unit_direction * (-1.0), rec.normal).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
        let direction;

        let mut rng = rand::thread_rng();

        if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0)
        {
            direction = vector3::reflect(unit_direction, rec.normal);
        } else {
            direction = vector3::refract(unit_direction, rec.normal, refraction_ratio);
        }

        let scattered = ray::Ray::new(rec.p, direction);
        (true, attenuation, scattered)
    }
}
