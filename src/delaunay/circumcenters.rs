use geo::{CoordFloat, Coordinate};
use rust_d3_geo::cartesian::cartesian_add;
use rust_d3_geo::cartesian::cartesian_cross;
use rust_d3_geo::cartesian::cartesian_normalize;
use rust_d3_geo::cartesian::spherical;

use super::cartesian::cartesian;

pub fn circumcenters<T: CoordFloat>(
    triangles: &[Vec<usize>],
    points: &[Coordinate<T>],
) -> Vec<Coordinate<T>> {
    return triangles
        .iter()
        .map(|tri| {
            let c: Vec<[T; 3]> = tri
                .iter()
                .map(|i| points[*i])
                .map(|i| cartesian(&i))
                .collect();

            let v: [T; 3] = cartesian_add(
                cartesian_add(cartesian_cross(&c[1], &c[0]), cartesian_cross(&c[2], &c[1])),
                cartesian_cross(&c[0], &c[2]),
            );
            return spherical(&cartesian_normalize(&v));
        })
        .collect();
}
