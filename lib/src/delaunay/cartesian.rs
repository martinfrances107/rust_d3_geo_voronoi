use geo::CoordFloat;
use geo_types::Coord;

/// Converts spherical coordinates (degrees) to 3D Cartesian.
/// Note there is a similar but different function `rust_d3_geo`.
/// This only difference this one convert to radians first.
pub(super) fn cartesian<T: CoordFloat>(coordinates: &Coord<T>) -> [T; 3] {
    let lambda = coordinates.x.to_radians();
    let phi = coordinates.y.to_radians();
    let (sin_phi, cos_phi) = phi.sin_cos();
    let (sin_lambda, cos_lambda) = lambda.sin_cos();
    [cos_phi * cos_lambda, cos_phi * sin_lambda, sin_phi]
}
