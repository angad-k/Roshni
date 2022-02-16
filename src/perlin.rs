use crate::utils;
use crate::vector3;
pub struct Perlin {
  point_count: i32,
  random_floats: Vec<vector3::Vec3>,
  perm_x: Vec<i32>,
  perm_y: Vec<i32>,
  perm_z: Vec<i32>,
}

impl Perlin {
  pub fn new(point_count: i32) -> Perlin {
    let mut random_floats = Vec::<vector3::Vec3>::new();
    for _i in 0..point_count {
      random_floats.push(vector3::Vec3::random(-1.0, 1.0));
    }
    let perm_x = perlin_generate_perm(point_count);
    let perm_y = perlin_generate_perm(point_count);
    let perm_z = perlin_generate_perm(point_count);
    Perlin {
      point_count,
      random_floats,
      perm_x,
      perm_y,
      perm_z,
    }
  }

  pub fn turb(&self, p: vector3::Point, depth: i32) -> f64 {
    let mut accum = 0.0;
    let mut temp_p = p.clone();
    let mut weight = 1.0;
    for _i in 0..depth {
      accum += self.noise(p) * weight;
      weight *= 0.5;
      temp_p = temp_p * 2.0;
    }

    accum.abs()
  }

  pub fn noise(&self, p: vector3::Point) -> f64 {
    let mut u = p.x - p.x.floor();
    let mut v = p.y - p.y.floor();
    let mut w = p.z - p.z.floor();

    u = u * u * (3.0 - 2.0 * u);
    v = v * v * (3.0 - 2.0 * v);
    w = w * w * (3.0 - 2.0 * w);

    let i = p.x.floor() as i32;
    let j = p.y.floor() as i32;
    let k = p.z.floor() as i32;

    let mut accum = 0.0;

    for di in 0..2 {
      for dj in 0..2 {
        for dk in 0..2 {
          let dfi = di as f64;
          let dfj = dj as f64;
          let dfk = dk as f64;
          let weight_v = vector3::Vec3::new(u - dfi, v - dfj, w - dfk);
          accum = accum
            + (dfi * u + (1.0 - dfi) * (1.0 - u))
              * (dfj * v + (1.0 - dfj) * (1.0 - v))
              * (dfk * w + (1.0 - dfk) * (1.0 - w))
              * vector3::dot(
                self.random_floats[(self.perm_x[((i + di) & (self.point_count - 1)) as usize]
                  ^ self.perm_y[((j + dj) & (self.point_count - 1)) as usize]
                  ^ self.perm_z[((k + dk) & (self.point_count - 1)) as usize])
                  as usize],
                weight_v,
              )
        }
      }
    }
    accum
  }
}

fn perlin_generate_perm(point_count: i32) -> Vec<i32> {
  let mut p = Vec::<i32>::new();
  for i in 0..point_count {
    p.push(i);
  }
  permute(p, point_count)
}

fn permute(mut p: Vec<i32>, point_count: i32) -> Vec<i32> {
  // We start from 1 since 0 won't be moved anywhere
  for i in (1..point_count).rev() {
    let target = utils::random_double(0.0, i as f64) as i32;
    let tmp = p[i as usize];
    p[i as usize] = p[target as usize];
    p[target as usize] = tmp;
  }
  p
}
