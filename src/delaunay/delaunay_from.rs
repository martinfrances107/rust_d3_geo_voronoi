use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use rust_d3_geo::projection::projection::Projection;
use rust_d3_geo::projection::stereographic::StereographicRaw;
use rust_d3_geo::rotation::rotation::Rotation;
use rust_d3_geo::Transform;

use super::delaunay::Delaunay;

pub fn delaunay_from<F>(points: Vec<[F; 2]>) -> Option<Delaunay<F>>
where
  F: Float + FloatConst + FromPrimitive,
{
  if points.len() < 2 {
    return None;
  };

  // Find a valid PIvot point.
  // The index of the first acceptable point in
  // which the x or y component is not inifinty.
  let Pivot = points
    .iter()
    .position(|p| (p[0] + p[1]).is_finite())
    .unwrap();

  let r = Rotation::new(points[Pivot][0], points[Pivot][1], points[Pivot][2]);
  let mut projection = StereographicRaw::gen_projection_mutator();
  projection.translate(Some(&[F::zero(), F::zero()]));
  projection.scale(Some(&F::one()));
  let angles2: [F; 2] = r.invert(&[F::from(180f64).unwrap(), F::from(0f64).unwrap()]);
  let angles: [F; 3] = [angles2[0], angles2[1], F::zero()];
  projection.rotate(Some(angles));
  let points: Vec<[F; 2]> = points
    .iter()
    .map(|p: [F; 2]| projection.transform(&p))
    .collect();

  let mut zeros = Vec::new();
  let max2 = F::one();
  for (i, elem) in points.iter().enumerate() {
    let m = points[i][0] * points[i][0] + points[i][1] * points[i][1];
    if !m.is_finite() || m > F::from(1e32f64).unwrap() {
      zeros.push(i);
    } else {
      if m > max2 {
        max2 = m;
      }
    }
  }
  let far = F::from(1e6).unwrap() * (max2).sqrt();

  zeros.iter().for_each(|i| points[*i] = [far, F::zero()]);

  // // Add infinite horizon points
  points.push([F::zero(), far]);
  points.push([-far, F::zero()]);
  points.push([F::zero(), -far]);
}
