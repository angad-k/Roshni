use crate::vector3;
pub trait Texture {
  fn value(&self, u : f64, v : f64, p : vector3::Point) -> vector3::Color;
}

// ---------------- SOLID COLOR ------------------------------------------------------

pub struct SolidColor {
  color : vector3::Color,
}

impl SolidColor{
  pub fn new(r : f64, g : f64, b : f64) -> SolidColor{
    SolidColor{
      color : vector3::Color::new(r, g, b)
    }
  }
}

impl Texture for SolidColor{
  fn value(&self, _u: f64, _v: f64, _p: vector3::Point) -> vector3::Color {
    self.color
  }
}

// -------------------------------------------------------------------------------------