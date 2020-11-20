use delaunator::Point;

use rust_d3_geo::cartesian::cartesian_add;
use rust_d3_geo::cartesian::cartesian_cross;
use rust_d3_geo::cartesian::cartesian_dot;
use rust_d3_geo::cartesian::cartesian_normalize;
use rust_d3_geo::cartesian::cartesian_scale;
use rust_d3_geo::cartesian::spherical;

use super::cartesian::cartesian;

pub fn o_midpoint(a: &Point, b: &Point, c: &Point) -> Point {
    let a = &cartesian(a);
    let b = &cartesian(b);
    let c = &cartesian(c);
    let s = (cartesian_dot(&cartesian_cross(b, a), c)).signum();

    let norm = cartesian_normalize(&cartesian_add(*a, *b));
    let signed_norm = cartesian_scale(&norm, s);
    return spherical(&signed_norm);
}
