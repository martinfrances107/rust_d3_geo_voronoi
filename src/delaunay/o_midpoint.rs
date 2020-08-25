use num_traits::cast::FromPrimitive;
use num_traits::Float;

use rust_d3_geo::cartesian::cartesian;
use rust_d3_geo::cartesian::cartesian_add;
use rust_d3_geo::cartesian::cartesian_cross;
use rust_d3_geo::cartesian::cartesian_dot;
use rust_d3_geo::cartesian::cartesian_normalize;
use rust_d3_geo::cartesian::cartesian_scale;
use rust_d3_geo::cartesian::spherical;

pub fn o_midpoint<F>(a: &[F; 2], b: &[F; 2], c: &[F; 2]) -> [F; 2]
where
  F: Float + FromPrimitive,
{
  let a = &cartesian(a);
  let b = &cartesian(b);
  let c = &cartesian(c);
  let s = (cartesian_dot(&cartesian_cross(b, a), c)).signum();

  let norm = cartesian_normalize(&mut cartesian_add(*a, *b));
  let signed_norm = cartesian_scale(&norm, s);
  return spherical(&signed_norm);
}
