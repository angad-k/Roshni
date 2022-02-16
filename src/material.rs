use crate::hittable;
use crate::ray;
use crate::texture;
use crate::texture::TextureTrait;
use crate::utils;
use crate::vector3;
use std::sync::Arc;
use std::sync::Mutex;

pub trait MaterialTrait {
    fn scatter(
        &self,
        r: &ray::Ray,
        rec: &hittable::HitRecord,
    ) -> Option<(vector3::Color, ray::Ray)>;
    fn emit(&self, u: f64, v: f64, p: vector3::Point) -> vector3::Color {
        vector3::Color::new(0.0, 0.0, 0.0)
    }
}

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
}

impl MaterialTrait for Material {
    fn scatter(
        &self,
        _r: &ray::Ray,
        rec: &hittable::HitRecord,
    ) -> Option<(vector3::Color, ray::Ray)> {
        match self {
            Material::Lambertian(x) => x.scatter(_r, rec),
            Material::Metal(x) => x.scatter(_r, rec),
            Material::Dielectric(x) => x.scatter(_r, rec),
            Material::DiffuseLight(x) => x.scatter(_r, rec),
        }
    }

    fn emit(&self, u: f64, v: f64, p: vector3::Point) -> vector3::Color {
        match self {
            Material::Lambertian(x) => x.emit(u, v, p),
            Material::Metal(x) => x.emit(u, v, p),
            Material::Dielectric(x) => x.emit(u, v, p),
            Material::DiffuseLight(x) => x.emit(u, v, p),
        }
    }
}

//------------------------ LAMBERTIAN -------------------------------------------------------

pub struct Lambertian {
    albedo: Arc<Mutex<texture::Texture>>,
}

impl Lambertian {
    pub fn new(p_albedo: vector3::Color) -> Lambertian {
        Lambertian {
            albedo: Arc::new(Mutex::new(texture::Texture::SolidColor(
                texture::SolidColor::new(p_albedo.x, p_albedo.y, p_albedo.z),
            ))),
        }
    }
    pub fn new_from_texture(p_albedo: Arc<Mutex<texture::Texture>>) -> Lambertian {
        Lambertian { albedo: p_albedo }
    }
}

impl MaterialTrait for Lambertian {
    fn scatter(
        &self,
        r: &ray::Ray,
        rec: &hittable::HitRecord,
    ) -> Option<(vector3::Color, ray::Ray)> {
        let mut scatter_direction = rec.normal + vector3::Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }
        // yeh sab jo change krke bhej rhe usse bhi hit record mei dalna mangtau
        let scattered = ray::Ray::new(rec.p, scatter_direction, Some(r.time));
        let attenuation = self.albedo.lock().unwrap().value(rec.u, rec.v, rec.p);
        Some((attenuation, scattered))
    }
}

//------------------------ METAL -------------------------------------------------------

pub struct Metal {
    albedo: vector3::Color,
}

impl Metal {
    pub fn new(p_albedo: vector3::Color) -> Metal {
        Metal { albedo: p_albedo }
    }
}

impl MaterialTrait for Metal {
    fn scatter(
        &self,
        r: &ray::Ray,
        rec: &hittable::HitRecord,
    ) -> Option<(vector3::Color, ray::Ray)> {
        let reflected = vector3::reflect(r.dir.unit_vector(), rec.normal);
        let scattered = ray::Ray::new(rec.p, reflected, Some(r.time));
        let attenuation = self.albedo.clone();
        Some((attenuation, scattered))
    }
}

//------------------------ DIELECTRIC -------------------------------------------------------

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

impl MaterialTrait for Dielectric {
    fn scatter(
        &self,
        r: &ray::Ray,
        rec: &hittable::HitRecord,
    ) -> Option<(vector3::Color, ray::Ray)> {
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

        if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > utils::random_double(0.0, 1.0)
        {
            direction = vector3::reflect(unit_direction, rec.normal);
        } else {
            direction = vector3::refract(unit_direction, rec.normal, refraction_ratio);
        }

        let scattered = ray::Ray::new(rec.p, direction, Some(r.time));
        Some((attenuation, scattered))
    }
}

//------------------------ DIFFUSE LIGHT -------------------------------------------------------

pub struct DiffuseLight {
    emit_tex: Arc<Mutex<texture::Texture>>,
}

impl DiffuseLight {
    pub fn new(tex: Arc<Mutex<texture::Texture>>) -> DiffuseLight {
        DiffuseLight { emit_tex: tex }
    }
}

impl MaterialTrait for DiffuseLight {
    fn scatter(
        &self,
        r: &ray::Ray,
        rec: &hittable::HitRecord,
    ) -> Option<(vector3::Color, ray::Ray)> {
        None
    }

    fn emit(&self, u: f64, v: f64, p: vector3::Point) -> vector3::Color {
        self.emit_tex.lock().unwrap().value(u, v, p)
    }
}
