use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use super::Delaunay;

pub fn triangles<F>(delaunay: Delaunay<F>) -> Vec<[usize; 3]>
where
  F: Float + FloatConst + FromPrimitive,
{
  let Delaunay { triangles, .. } = delaunay;
  if triangles.len() == 0 {
    return Vec::new();
  }

  let mut geo_triangles: Vec<[usize; 3]> = Vec::new();
  let n: usize = triangles.len() / 3usize;

  for i in 0..n {
    let a = triangles[3 * i];
    let b = triangles[3 * i + 1];
    let c = triangles[3 * i + 2];
    if a != b && b != c {
      geo_triangles.push([a, c, b]);
    }
  }
  return geo_triangles;
}
