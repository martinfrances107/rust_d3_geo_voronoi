use geo::{CoordFloat, Coordinate};
use rust_d3_geo::cartesian::add;
use rust_d3_geo::cartesian::cross;
use rust_d3_geo::cartesian::normalize;
use rust_d3_geo::cartesian::spherical;

use super::cartesian::cartesian;

pub fn geo_circumcenters<T: CoordFloat>(
    triangles: &[[usize; 3]],
    points: &[Coordinate<T>],
) -> Vec<Coordinate<T>> {
    return triangles
        .iter()
        .map(|tri| {
            let c = [
                cartesian(&points[tri[0]]),
                cartesian(&points[tri[1]]),
                cartesian(&points[tri[2]]),
            ];

            let v: [T; 3] = add(
                add(cross(&c[1], &c[0]), cross(&c[2], &c[1])),
                cross(&c[0], &c[2]),
            );
            spherical(&normalize(&v))
        })
        .collect();
}
