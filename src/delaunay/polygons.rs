use std::rc::Rc;

use num_traits::cast::FromPrimitive;
use num_traits::Float;

use rust_d3_geo::cartesian::cartesian;
use rust_d3_geo::cartesian::cartesian_add;
use rust_d3_geo::cartesian::cartesian_cross;
use rust_d3_geo::cartesian::cartesian_normalize;
use rust_d3_geo::cartesian::spherical;

use super::o_midpoint::o_midpoint;

pub fn polygons<'a, F>(
  circumcenter: Vec<[F; 2]>,
  triangles: &Vec<Vec<usize>>,
  points: &'a Vec<[F; 2]>,
) -> (Vec<Vec<usize>>, Vec<[F; 2]>)
where
  F: Float + FromPrimitive,
{
  let mut polygons: Vec<Vec<usize>> = Vec::new();
  let mut centers = circumcenter;

  let supplement = Rc::new(|point: &[F; 2]| -> usize {
    let mut f: Option<usize> = None;

    let centers_slice = &centers[triangles.len()..centers.len()];
    centers_slice.iter().enumerate().map(|(i, p)| {
      if p[0] == point[0] && p[1] == point[1] {
        f = Some(i + triangles.len());
      };
    });

    if f.is_none() {
      f = Some(centers.len());
      centers.push(*point);
    }
    match f {
      Some(f) => {
        return f;
      }
      None => {
        panic!("Suppliment did not find a value to return");
      }
    }
  });

  if triangles.len() == 0 {
    match points.len() {
      0 | 1 => {
        return (polygons, centers);
      }
      2 => {
        if points.len() == 2 {
          // two hemispheres.
          let a = cartesian(&points[0]);
          let b = cartesian(&points[0]);
          let m = cartesian_normalize(&cartesian_add(a, b));

          let d = cartesian_normalize(&cartesian_cross(&a, &b));
          let c = cartesian_cross(&m, &d);

          let supplement_copy = supplement.clone();
          let poly: Vec<usize> = [
            m,
            cartesian_cross(&m, &c),
            cartesian_cross(&cartesian_cross(&m, &c), &c),
            cartesian_cross(&cartesian_cross(&cartesian_cross(&m, &c), &c), &c),
          ]
          .iter()
          .map(|p| spherical(p))
          .map(|p| {
            // let out: usize = supplement_copy(&p);
            let out = 0; // TODO must resolve suppliment issues.
            return out;
          })
          .collect();
          polygons.push(poly);
          // let rev: Vec<usize> = poly.iter().rev().map(|x| *x).collect();
          // polygons.push(rev);
          return (polygons, centers);
        }
      }
      _ => { // further processing needed.}
      }
    }
  };

  let mut polygons_tuple: Vec<Vec<(usize, usize, usize, (usize, usize, usize))>> = Vec::new();
  for (t, tri) in triangles.iter().enumerate() {
    for j in 0..3 {
      let a = tri[j];
      let b = tri[(j + 1) % 3];
      let c = tri[(j + 2) % 3];
      // if polygons[a].is_none() {
      //   polygons[a] = Vec::new();
      // }
      polygons_tuple[a].push((b, c, t, (a, b, c)));
    }
  }

  // reorder each polygon.
  let reordered: Vec<Vec<usize>> = polygons_tuple
    .iter()
    .map(|poly| {
      let mut p = vec![poly[0].2]; // t
      let mut k = poly[0].1; // k = c

      for _i in 0..poly.len() {
        // look for b = k
        for j in 0..poly.len() {
          if poly[j].0 == k {
            k = poly[j].1;
            p.push(poly[j].2);
            break;
          }
        }
      }

      match p.len() {
        0 | 1 => {
          return Vec::new();
        }
        2 => {
          let i0;
          let i1;
          // borrow and release centers.
          {
            let r0 = o_midpoint(
              &points[(poly[0].3).0],
              &points[(poly[0].3).1],
              &centers[p[0]],
            );
            let r1 = o_midpoint(
              &points[(poly[0].3).2],
              &points[(poly[0].3).0],
              &centers[p[0]],
            );
            // i0 = supplement(&R0);
            // i1 = supplement(&R1);
            // TODO must reolsve suppliement issues
            i0 = 0;
            i1 = 0;
          }
          return vec![p[0], i1, p[1], i0];
        }
        _ => {
          return p;
        }
      }
    })
    .collect();

  return (reordered, centers);
}
