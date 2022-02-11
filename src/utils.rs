use rand::Rng;
pub fn clamp(p: f64) -> f64 {
    if p < 0.0 {
        return 0.0;
    } else if p > 0.999 {
        return 0.999;
    } else {
        return p;
    }
}

pub fn random_double(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
