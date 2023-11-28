use geo::CoordFloat;
use geo_types::Coord;

use d3_geo_rs::cartesian::cross;
use d3_geo_rs::cartesian::dot;

use super::cartesian::cartesian;

/// Spherical excess of a triangle (in spherical coordinates).
pub fn excess<T: CoordFloat>(triangle_p: &[Coord<T>; 3]) -> T {
    let triangle = [
        cartesian(&triangle_p[0]),
        cartesian(&triangle_p[1]),
        cartesian(&triangle_p[2]),
    ];
    dot(&triangle[0], &cross(&triangle[2], &triangle[1]))
}
