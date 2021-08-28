// use delaunator::Point;

use geo::{CoordFloat, Coordinate, Point};
use num_traits::Float;
use rust_d3_geo::cartesian::cross;
use rust_d3_geo::cartesian::dot;

use super::cartesian::cartesian;

/// Spherical excess of a triangle (in spherical coordinates)
pub fn excess<T: CoordFloat>(triangle_p: &[Coordinate<T>]) -> T {
    let triangle: Vec<[T; 3]> = triangle_p.iter().map(|p| cartesian(p)).collect();
    return dot(&triangle[0], &cross(&triangle[2], &triangle[1]));
}
