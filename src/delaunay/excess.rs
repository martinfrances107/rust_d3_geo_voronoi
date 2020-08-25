use num_traits::Float;

use rust_d3_geo::cartesian::cartesian;
use rust_d3_geo::cartesian::cartesian_cross;
use rust_d3_geo::cartesian::cartesian_dot;

/// Spherical excess of a triangle (in spherical coordinates)
pub fn excess<F>(triangle_p: [[F; 2]; 3]) -> F
where
  F: Float,
{
  let triangle: Vec<[F; 3]> = triangle_p.iter().map(|p| cartesian(p)).collect();
  return cartesian_dot(&triangle[0], &cartesian_cross(&triangle[2], &triangle[1]));
}
