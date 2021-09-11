use geo::{CoordFloat, Coordinate};

use rust_d3_geo::cartesian::cross;
use rust_d3_geo::cartesian::dot;

use super::cartesian::cartesian;

/// Spherical excess of a triangle (in spherical coordinates)
pub fn excess<T: CoordFloat>(triangle_p: &[Coordinate<T>]) -> T {
    let iter = triangle_p.iter();
    let triangle: [[T; 3]; 3] = [
        cartesian(&triangle_p[0]),
        cartesian(&triangle_p[1]),
        cartesian(&triangle_p[2]),
    ];
    dot(&triangle[0], &cross(&triangle[2], &triangle[1]))
}
