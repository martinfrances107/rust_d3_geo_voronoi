/// Determins the great circle distance between two points on a sphere.
pub fn haversin(x: f64) -> f64 {
    let sinxdiv2: f64 = (x / 2f64).sin();
    sinxdiv2 * sinxdiv2
}
