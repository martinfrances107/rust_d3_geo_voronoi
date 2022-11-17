use geo::CoordFloat;
use geo_types::Coord;

use num_traits::FloatConst;
use rust_d3_geo::cartesian::add;
use rust_d3_geo::cartesian::cross;
use rust_d3_geo::cartesian::dot;
use rust_d3_geo::cartesian::normalize;
use rust_d3_geo::cartesian::scale;
use rust_d3_geo::cartesian::spherical;

use super::cartesian::cartesian;

pub fn o_midpoint<T>(a: &Coord<T>, b: &Coord<T>, c: &Coord<T>) -> Coord<T>
where
    T: CoordFloat + FloatConst,
{
    let a = &cartesian(a);
    let b = &cartesian(b);
    let c = &cartesian(c);
    let s = (dot(&cross(b, a), c)).signum();

    let norm = normalize(&add(*a, *b));
    let signed_norm = scale(&norm, s);
    spherical(&signed_norm)
}
