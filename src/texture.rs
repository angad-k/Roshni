use crate::perlin::Perlin;
use crate::vector3;
use image::DynamicImage;
use image::GenericImage;
use std::sync::Arc;
use std::sync::Mutex;
pub trait TextureTrait {
  fn value(&self, u: f64, v: f64, p: vector3::Point) -> vector3::Color;
}

pub enum Texture {
  SolidColor(SolidColor),
  Checker(Checker),
  NoiseTexture(NoiseTexture),
  ImageTexture(ImageTexture),
}

impl TextureTrait for Texture {
  fn value(&self, u: f64, v: f64, p: vector3::Point) -> vector3::Color {
    match self {
      Texture::SolidColor(x) => x.value(u, v, p),
      Texture::Checker(x) => x.value(u, v, p),
      Texture::NoiseTexture(x) => x.value(u, v, p),
      Texture::ImageTexture(x) => x.value(u, v, p),
    }
  }
}

// ---------------- SOLID COLOR ------------------------------------------------------

pub struct SolidColor {
  color: vector3::Color,
}

impl SolidColor {
  pub fn new(r: f64, g: f64, b: f64) -> SolidColor {
    SolidColor {
      color: vector3::Color::new(r, g, b),
    }
  }
}

impl TextureTrait for SolidColor {
  fn value(&self, _u: f64, _v: f64, _p: vector3::Point) -> vector3::Color {
    self.color
  }
}

// -------------------- CHECKER -------------------------------------------------------

pub struct Checker {
  odd: Arc<Mutex<Texture>>,
  even: Arc<Mutex<Texture>>,
}

impl Checker {
  pub fn new() -> Checker {
    Checker {
      odd: Arc::new(Mutex::new(Texture::SolidColor(SolidColor::new(
        0.0, 0.0, 0.0,
      )))),
      even: Arc::new(Mutex::new(Texture::SolidColor(SolidColor::new(
        1.0, 1.0, 1.0,
      )))),
    }
  }
}

impl TextureTrait for Checker {
  fn value(&self, u: f64, v: f64, p: vector3::Point) -> vector3::Color {
    let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
    if sines < 0.0 {
      return self.odd.lock().unwrap().value(u, v, p);
    } else {
      return self.even.lock().unwrap().value(u, v, p);
    }
  }
}

// -------------------- PERLIN -------------------------------------------------------

pub struct NoiseTexture {
  noise: Perlin,
  scale: f64,
}

impl NoiseTexture {
  pub fn new(scale: f64) -> NoiseTexture {
    NoiseTexture {
      noise: Perlin::new(256),
      scale,
    }
  }
}

impl TextureTrait for NoiseTexture {
  fn value(&self, _u: f64, _v: f64, p: vector3::Point) -> vector3::Color {
    //vector3::Color::new(1.0, 1.0, 1.0) * self.noise.turb(p * self.scale, 7)
    vector3::Color::new(1.0, 1.0, 1.0)
      * 0.5
      * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
  }
}

// -------------------- IMAGE -------------------------------------------------------

pub struct ImageTexture {
  img: DynamicImage,
  height: u32,
  width: u32,
}

impl ImageTexture {
  pub fn new(path: &str) -> ImageTexture {
    let img = crate::image_encoder::read_image(path);
    let height = img.height();
    let width = img.width();
    ImageTexture {
      img,
      height: height,
      width: width,
    }
  }
}

impl TextureTrait for ImageTexture {
  fn value(&self, mut u: f64, mut v: f64, _p: vector3::Point) -> vector3::Color {
    u = u.clamp(0.0, 1.0);
    v = 1.0 - v.clamp(0.0, 1.0);
    let mut i = (u * self.width as f64) as u32;
    let mut j = (v * self.height as f64) as u32;
    if i >= self.width {
      i = self.width - 1;
    }
    if j >= self.height {
      j = self.height - 1;
    }

    let color_scale = 1.0 / 255.0;

    let pixel = self.img.get_pixel(i, j);
    vector3::Color::new(
      pixel[0] as f64 * color_scale,
      pixel[1] as f64 * color_scale,
      pixel[2] as f64 * color_scale,
    )
  }
}
