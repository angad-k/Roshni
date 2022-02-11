use crate::utils;
use std::ops;
#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(p_x: f64, p_y: f64, p_z: f64) -> Vec3 {
        Vec3 {
            x: p_x,
            y: p_y,
            z: p_z,
        }
    }
    pub fn x(self) -> f64 {
        self.x
    }

    pub fn y(self) -> f64 {
        self.y
    }

    pub fn z(self) -> f64 {
        self.z
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(self) -> Vec3 {
        let length = self.length();
        self / length
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        return Vec3::new(
            utils::random_double(min, max),
            utils::random_double(min, max),
            utils::random_double(min, max),
        );
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn near_zero(self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if dot(in_unit_sphere, normal) > 0.0
        // In the same hemisphere as the normal
        {
            return in_unit_sphere;
        } else {
            return in_unit_sphere * (-1.0);
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                utils::random_double(-1.0, 1.0),
                utils::random_double(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x + _rhs,
            y: self.y + _rhs,
            z: self.z + _rhs,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x - _rhs,
            y: self.y - _rhs,
            z: self.z - _rhs,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
            z: self.z / _rhs.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * 2.0 * dot(v, n)
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let mut cos_theta = dot(uv * (-1.0), n);
    if cos_theta > 1.0 {
        cos_theta = 1.0
    }
    let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
    let r_out_parallel = n * -((1.0 - r_out_perp.length_squared()).abs()).sqrt();
    return r_out_perp + r_out_parallel;
}

pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

pub type Color = Vec3;
pub type Point = Vec3;
