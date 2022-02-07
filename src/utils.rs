pub fn clamp(p: f64) -> f64 {
    if p < 0.0 {
        return 0.0;
    } else if p > 0.999 {
        return 0.999;
    } else {
        return p;
    }
}
