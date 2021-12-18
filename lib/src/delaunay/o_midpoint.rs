// use delaunator::Point;
use geo::{CoordFloat, Coordinate};

use rust_d3_geo::cartesian::add;
use rust_d3_geo::cartesian::cross;
use rust_d3_geo::cartesian::dot;
use rust_d3_geo::cartesian::normalize;
use rust_d3_geo::cartesian::scale;
use rust_d3_geo::cartesian::spherical;

use super::cartesian::cartesian;

pub fn o_midpoint<T: CoordFloat>(
    a: &Coordinate<T>,
    b: &Coordinate<T>,
    c: &Coordinate<T>,
) -> Coordinate<T> {
    let a = &cartesian(a);
    let b = &cartesian(b);
    let c = &cartesian(c);
    let s = (dot(&cross(b, a), c)).signum();

    let norm = normalize(&add(*a, *b));
    let signed_norm = scale(&norm, s);
    spherical(&signed_norm)
}