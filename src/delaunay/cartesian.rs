use geo::{Coordinate, Point};
use num_traits::Float;

/// Converts spherical coordinates (degrees) to 3D Cartesian.
/// Note there is a similar but different function  rust_d3_geo
/// This only difference this one convert to radians first.
pub fn cartesian<T: Float>(spherical: &Coordinate<T>) -> [T; 3] {
    let lambda = spherical.x.to_radians();
    let phi = spherical.y.to_radians();
    let cos_phi = phi.cos();
    return [cos_phi * lambda.cos(), cos_phi * lambda.sin(), phi.sin()];
}
