pub const EPSILON: f64 = 1e-6;
pub const EPSILON2: f64 = 1e-12;

pub fn haversin(x: f64) -> f64 {
    let sinxdiv2: f64 = (x / 2f64).sin();
    return sinxdiv2 * sinxdiv2;
}
