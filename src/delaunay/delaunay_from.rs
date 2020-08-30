use std::cmp;
use std::rc::Rc;

use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use rust_d3_geo::projection::projection::Projection;
use rust_d3_geo::projection::stereographic::StereographicRaw;
use rust_d3_geo::rotation::rotation::Rotation;
use rust_d3_geo::Transform;

use super::Delaunay;

pub fn delaunay_from<F>(points: Rc<Vec<[F; 2]>>) -> Option<Delaunay<F>>
where
  F: Float + FloatConst + FromPrimitive + 'static,
{
  if points.len() < 2 {
    return None;
  };

  // Find a valid PIvot point.
  // The index of the first acceptable point in
  // which the x or y component is not inifinty.
  let pivot: usize = points
    .iter()
    .position(|p| (p[0] + p[1]).is_finite())
    .unwrap();

  // TODO must fix this
  // let r = Rotation::new(points[pivot][0], points[pivot][1], points[pivot][2]);
  let r = Rotation::new(points[pivot][0], points[pivot][1],F::zero());

  let mut projection = StereographicRaw::gen_projection_mutator();
  projection.translate(Some(&[F::zero(), F::zero()]));
  projection.scale(Some(&F::one()));
  let angles2: [F; 2] = r.invert(&[F::from(180f64).unwrap(), F::from(0f64).unwrap()]);
  let angles: [F; 3] = [angles2[0], angles2[1], F::zero()];
  projection.rotate(Some(angles));

  let mut points: Vec<[F; 2]> = points.iter().map(|p| projection.transform(&p)).collect();

  let mut zeros = Vec::new();
  let mut max2 = F::one();
  // for (i, elem) in points.iter().enumerate() {
  for i in 0..points.len() {
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

  // Add infinite horizon points
  points.push([F::zero(), far]);
  points.push([-far, F::zero()]);
  points.push([F::zero(), -far]);

  let points = Rc::new(points);

  // const delaunay = Delaunay.from(points);
  let delaunay: Option<Delaunay<F>> = delaunay_from(points.clone());

  match delaunay {
    Some(mut delaunay) => {
      delaunay.projection = Some(projection);

      // clean up the triangulation
      // let  {triangles, halfedges, inedges} = delaunay;
      // let triangles: &mut Vec<usize> = &mut delaunay.triangles;
      // let halfedges: &mut Vec<i32> = &mut delaunay.halfedges;
      // let mut inedges = delaunay.inedges;

      // const degenerate = [];
      let mut degenerate: Vec<usize> = Vec::new();
      // for (let i = 0, l = halfedges.length; i < l; i++) {
      for i in 0..delaunay.halfedges.len() {
        if delaunay.halfedges[i] < 0 {
          let j = match i % 3 == 2 {
            true => i - 2,
            false => i + 1,
          };
          let k = match i % 3 == 0 {
            true => i + 2,
            false => i - 1,
          };
          let a = delaunay.halfedges[j] as usize;
          let b = delaunay.halfedges[k] as usize;
          delaunay.halfedges[a] = b as i32;
          delaunay.halfedges[b] = a as i32;
          delaunay.halfedges[j] = -1;
          delaunay.halfedges[k] = -1;
          delaunay.triangles[i] = pivot;
          delaunay.triangles[j] = pivot;
          delaunay.triangles[k] = pivot;
          match a % 3 == 0 {
            true => {
              delaunay.inedges[delaunay.triangles[a]] = a as i32 + 2;
              delaunay.inedges[delaunay.triangles[b]] = b as i32 + 2;
            }
            false => {
              delaunay.inedges[delaunay.triangles[a]] = a as i32 - 1;
              delaunay.inedges[delaunay.triangles[b]] = b as i32 - 1;
            }
          };
          let m = cmp::min(i, j);
          let m = cmp::min(m, k);
          degenerate.push(m);

        // TODO must rework loop
        // i += 2 - i % 3;
        } else if delaunay.triangles[i] > points.len() - 3 - 1 {
          delaunay.triangles[i] = pivot;
        }
      }

      // // there should always be 4 degenerate triangles
      // // console.warn(degenerate);
      return Some(delaunay);
    }
    None => {
      return None;
    }
  }
}
