use crate::vector3;
use std::sync::Arc;
use std::sync::Mutex;
pub trait texture_trait {
  fn value(&self, u: f64, v: f64, p: vector3::Point) -> vector3::Color;
}

pub enum Texture {
  SolidColor(SolidColor),
  Checker(Checker),
}

impl texture_trait for Texture {
  fn value(&self, u: f64, v: f64, p: vector3::Point) -> vector3::Color {
    match self {
      Texture::SolidColor(x) => x.value(u, v, p),
      Texture::Checker(x) => x.value(u, v, p),
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

impl texture_trait for SolidColor {
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

impl texture_trait for Checker {
  fn value(&self, u: f64, v: f64, p: vector3::Point) -> vector3::Color {
    let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
    if sines < 0.0 {
      return self.odd.lock().unwrap().value(u, v, p);
    } else {
      return self.even.lock().unwrap().value(u, v, p);
    }
  }
}
