use delaunator::Point;

use rust_d3_geo::cartesian::cartesian_add;
use rust_d3_geo::cartesian::cartesian_cross;
use rust_d3_geo::cartesian::cartesian_normalize;
use rust_d3_geo::cartesian::spherical;

use super::cartesian::cartesian;

pub fn circumcenters(triangles: &Vec<Vec<usize>>, points: &Vec<Point>) -> Vec<Point> {
    return triangles
        .iter()
        .map(|tri| {
            let c: Vec<[f64; 3]> = tri
                .iter()
                .map(|i| points[*i].clone())
                .map(|i| cartesian(&i))
                .collect();

            let v: [f64; 3] = cartesian_add(
                cartesian_add(cartesian_cross(&c[1], &c[0]), cartesian_cross(&c[2], &c[1])),
                cartesian_cross(&c[0], &c[2]),
            );
            return spherical(&cartesian_normalize(&v));
        })
        .collect();
}
