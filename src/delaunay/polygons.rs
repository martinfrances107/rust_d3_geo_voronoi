use num_traits::cast::FromPrimitive;
use num_traits::Float;

use rust_d3_geo::cartesian::cartesian;
use rust_d3_geo::cartesian::cartesian_add;
use rust_d3_geo::cartesian::cartesian_cross;
use rust_d3_geo::cartesian::cartesian_normalize;
use rust_d3_geo::cartesian::spherical;

use super::o_midpoint::o_midpoint;

pub struct PolygonReturn<F>
where F: Float + FromPrimitive {
  pub polygons: Vec<Vec<usize>>,
  pub centers: Vec<[F; 2]>,
}

pub fn polygons<F>(
  circumcenter: Vec<[F; 2]>,
  triangles: Vec<[usize; 3]>,
  points: Vec<[F; 2]>,
) -> PolygonReturn<F>
where
  F: Float + FromPrimitive
{
  let mut polygons: Vec<usize> = Vec::new();
  let centers = circumcenter;

  let supplement = |point: [F; 2]| {
    let f: usize;

    let centers_slice = &centers[triangles.len()..centers.len()];
    centers_slice.iter().enumerate().map(|(i, p)| {
      if p[0] == point[0] && p[1] == point[1] {
        f = i + triangles.len();
      };
    });
    if f < 0 {
      f = centers.len();
      centers.push(point);
    }
    return f;
  };

  if triangles.len() == 0 {
    if points.len() < 2 {
      return PolygonReturn { polygons, centers };
    }
    let mut a;
    let mut b;
    let mut c;
    let m;
    if points.len() == 2usize {
      // two hemispheres.
      a = cartesian(&points[0]);
      b = cartesian(&points[0]);
      m = cartesian_normalize(&cartesian_add(a, b));

      let d = cartesian_normalize(&cartesian_cross(&a, &b));
      let c = cartesian_cross(&m, &d);
      let poly = [
        m,
        cartesian_cross(&m, &c),
        cartesian_cross(&cartesian_cross(&m, &c), &c),
        cartesian_cross(&cartesian_cross(&cartesian_cross(&m, &c), &c), &c),
      ]
      .iter()
      .map(|p| spherical(p))
      .collect()
      .map(|p| supplement(p))
      .collect();
      polygons.push(poly);
      polygons.push(poly[..].reverse());
      return PolygonReturn { polygons, centers };
    }
  }

  let polygons_tuple: Vec<Vec<(usize, usize, usize, (usize, usize, usize))>> = Vec::new();
  for (t, tri) in triangles.iter().enumerate() {
    for j in 0..3 {
      let a = tri[j];
      let b = tri[(j + 1) % 3];
      let c = tri[(j + 2) % 3];
      if polygons[a].is_none() {
        polygons[a] = None;
      }
      polygons_tuple[a].push((b, c, t, (a, b, c)));
    }
  }

  // reorder each polygon.
  let reordered = polygons_tuple
    .iter()
    .map(|poly| {
      let p = vec![poly[0].2]; // t
      let k = poly[0].1; // k = c

      for i in 0..poly.len() {
        // look for b = k
        for j in 0..poly.len() {
          if poly[j].0 == k {
            k = poly[j].1;
            p.push(poly[j].2);
            break;
          }
        }
      }

      if p.len() > 2usize {
        return p;
      } else if p.len() == 2usize {
        let R0 = o_midpoint(
          &points[(poly[0].3).0],
          &points[(poly[0].3).1],
          &centers[p[0]],
        );
        let R1 = o_midpoint(
          &points[(poly[0].3).2],
          &points[(poly[0].3).0],
          &centers[p[0]],
        );
        let i0 = supplement(R0);
        let i1 = supplement(R1);
        return vec![p[0], i1, p[1], i0];
      }
    })
    .collect();

  return PolygonReturn {
    polygons: reordered,
    centers,
  };
}

// function geo_polygons(circumcenters, triangles, points) {
//   const polygons = [];

//   const centers = circumcenters.slice();

//   // supplementary centers for degenerate cases like n = 1,2,3
//   const supplements = [];

//   if (triangles.length === 0) {
//     if (points.length < 2) return { polygons, centers };
//     if (points.length === 2) {
//       // two hemispheres
//       const a = cartesian(points[0]),
//         b = cartesian(points[1]),
//         m = normalize(cartesianAdd(a, b)),
//         d = normalize(cross(a, b)),
//         c = cross(m, d);
//       const poly = [
//         m,
//         cross(m, c),
//         cross(cross(m, c), c),
//         cross(cross(cross(m, c), c), c)
//       ]
//         .map(spherical)
//         .map(supplement);
//       return (
//         polygons.push(poly),
//         polygons.push(poly.slice().reverse()),
//         { polygons, centers }
//       );
//     }
//   }

//   triangles.forEach((tri, t) => {
//     for (let j = 0; j < 3; j++) {
//       const a = tri[j],
//         b = tri[(j + 1) % 3],
//         c = tri[(j + 2) % 3];
//       polygons[a] = polygons[a] || [];
//       polygons[a].push([b, c, t, [a, b, c]]);
//     }
//   });

//   // reorder each polygon
//   const reordered = polygons.map(poly => {
//     const p = [poly[0][2]]; // t
//     let k = poly[0][1]; // k = c
//     for (let i = 1; i < poly.length; i++) {
//       // look for b = k
//       for (let j = 0; j < poly.length; j++) {
//         if (poly[j][0] == k) {
//           k = poly[j][1];
//           p.push(poly[j][2]);
//           break;
//         }
//       }
//     }

//     if (p.length > 2) {
//       return p;
//     } else if (p.length == 2) {
//       const R0 = o_midpoint(
//           points[poly[0][3][0]],
//           points[poly[0][3][1]],
//           centers[p[0]]
//         ),
//         R1 = o_midpoint(
//           points[poly[0][3][2]],
//           points[poly[0][3][0]],
//           centers[p[0]]
//         );
//       const i0 = supplement(R0),
//         i1 = supplement(R1);
//       return [p[0], i1, p[1], i0];
//     }
//   });

//   function supplement(point) {
//     let f = -1;
//     centers.slice(triangles.length, Infinity).forEach((p, i) => {
//       if (p[0] === point[0] && p[1] === point[1]) f = i + triangles.length;
//     });
//     if (f < 0) (f = centers.length), centers.push(point);
//     return f;
//   }

//   return { polygons: reordered, centers };
// }
