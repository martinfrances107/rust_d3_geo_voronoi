use geo::{CoordFloat, Coordinate};

/// Converts spherical coordinates (degrees) to 3D Cartesian.
/// Note there is a similar but different function  rust_d3_geo
/// This only difference this one convert to radians first.
pub(super) fn cartesian<T: CoordFloat>(coordinates: &Coordinate<T>) -> [T; 3] {
    let lambda = coordinates.x.to_radians();
    let phi = coordinates.y.to_radians();
    let cos_phi = phi.cos();
    [cos_phi * lambda.cos(), cos_phi * lambda.sin(), phi.sin()]
}
