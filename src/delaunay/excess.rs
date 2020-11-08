use delaunator::Point;

use rust_d3_geo::cartesian::cartesian;
use rust_d3_geo::cartesian::cartesian_cross;
use rust_d3_geo::cartesian::cartesian_dot;

/// Spherical excess of a triangle (in spherical coordinates)
pub fn excess(triangle_p: &Vec<Point>) -> f64
{
  let triangle: Vec<[f64; 3]> = triangle_p.iter().map(|p| cartesian(p)).collect();
  return cartesian_dot(&triangle[0], &cartesian_cross(&triangle[2], &triangle[1]));
}
