// use num_traits::cast::FromPrimitive;
use num_traits::Float;

use rust_d3_geo::cartesian::cartesian;
use rust_d3_geo::cartesian::cartesian_add;
use rust_d3_geo::cartesian::cartesian_cross;
use rust_d3_geo::cartesian::cartesian_normalize;
use rust_d3_geo::cartesian::spherical;

pub fn circumcenters<F>(triangles: Vec<[usize; 3]>, points: Vec<[F; 2]>) -> Vec<[F; 2]>
where
  F: Float,
{
  return triangles
    .iter()
    .map(|tri| {
      let c: Vec<[F; 3]> = tri
        .iter()
        .map(|i| points[*i])
        .map(|i| cartesian(&i))
        .collect();

      let v: [F; 3] = cartesian_add(
        cartesian_add(cartesian_cross(&c[1], &c[0]), cartesian_cross(&c[2], &c[1])),
        cartesian_cross(&c[0], &c[2]),
      );
      return spherical(&cartesian_normalize(&v));
    })
    .collect();
}

// function geo_circumcenters(triangles, points) {
//   // if (!use_centroids) {
//   return triangles.map(tri => {
//     const c = tri.map(i => points[i]).map(cartesian),
//       V = cartesianAdd(
//         cartesianAdd(cross(c[1], c[0]), cross(c[2], c[1])),
//         cross(c[0], c[2])
//       );
//     return spherical(normalize(V));
//   });
//   /*} else {
//     return triangles.map(tri => {
//       return d3.geoCentroid({
//         type: "MultiPoint",
//         coordinates: tri.map(i => points[i])
//       });
//     });
//   }*/
// }
