use d3_geo_rs::cartesian::add;
use d3_geo_rs::cartesian::cross;
use d3_geo_rs::cartesian::normalize;
use d3_geo_rs::cartesian::spherical;
use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use super::cartesian::cartesian;

pub fn circumcenters<'a, T>(
    triangles: &'a [[usize; 3]],
    points: &'a [Coord<T>],
) -> impl Iterator<Item = Coord<T>> + 'a
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
