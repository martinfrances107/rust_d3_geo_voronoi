use rust_d3_geo::Transform;
use std::ops::AddAssign;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::{AsPrimitive, FloatConst};

use rust_d3_delaunay::delaunay::Delaunay;
use rust_d3_geo::stream::Stream;

pub fn geo_triangles<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>(
    delaunay: &Delaunay<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>,
) -> Vec<[usize; 3]>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    I: Clone,
    LB: Clone,
    LC: Clone,
    LU: Clone,
    PCNC: Clone,
    PCNU: Clone,
    PR: Transform<T = T>,
    PV: Clone,
    RC: Clone,
    RU: Clone,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + FloatConst,
{
    let Delaunay { triangles, .. } = delaunay;
    if triangles.is_empty() {
        return Vec::new();
    }

    let mut geo_triangles: Vec<[usize; 3]> = Vec::new();
    let n: usize = triangles.len() / 3usize;

    for i in 0..n {
        let a = triangles[3 * i];
        let b = triangles[3 * i + 1];
        let c = triangles[3 * i + 2];
        if a != b && b != c {
            geo_triangles.push([a, c, b]);
        }
    }
    geo_triangles
}
