use geo::CoordFloat;
use geo_types::Coord;

use d3_geo_rs::cartesian::add;
use d3_geo_rs::cartesian::cross;
use d3_geo_rs::cartesian::dot;
use d3_geo_rs::cartesian::normalize;
use d3_geo_rs::cartesian::scale;
use d3_geo_rs::cartesian::spherical;
use num_traits::FloatConst;

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
