// use delaunator::Point;

use geo::{Coordinate, CoordinateType, Point};
use num_traits::Float;
use rust_d3_geo::cartesian::cartesian_cross;
use rust_d3_geo::cartesian::cartesian_dot;

use super::cartesian::cartesian;

/// Spherical excess of a triangle (in spherical coordinates)
pub fn excess<T: CoordinateType + Float>(triangle_p: &[Coordinate<T>]) -> T {
    let triangle: Vec<[T; 3]> = triangle_p.iter().map(|p| cartesian(p)).collect();
    return cartesian_dot(&triangle[0], &cartesian_cross(&triangle[2], &triangle[1]));
}
