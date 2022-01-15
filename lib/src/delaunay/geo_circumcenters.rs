use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;
use rust_d3_geo::cartesian::add;
use rust_d3_geo::cartesian::cross;
use rust_d3_geo::cartesian::normalize;
use rust_d3_geo::cartesian::spherical;

use super::cartesian::cartesian;

pub fn geo_circumcenters<'a, T>(
    triangles: &'a [[usize; 3]],
    points: &'a [Coordinate<T>],
) -> impl Iterator<Item = Coordinate<T>> + 'a
where
    T: CoordFloat + FloatConst,
{
    return triangles.iter().map(|tri| {
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
    });
}
