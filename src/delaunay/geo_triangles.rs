use approx::AbsDiffEq;
use rust_d3_geo::clip::Line;
use rust_d3_geo::clip::PointVisible;
use rust_d3_geo::projection::Raw;
use rust_d3_geo::stream::Stream;
use std::ops::AddAssign;

use geo::CoordFloat;
use num_traits::{AsPrimitive, FloatConst};
use rust_d3_delaunay::delaunay::Delaunay;

pub fn geo_triangles<
    DRAIN: Stream<T = T>,
    PR: Raw<T>,
    PV: PointVisible<T = T>,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + FloatConst,
>(
    delaunay: &Delaunay<DRAIN, PR, PV, T>,
) -> Vec<[usize; 3]> {
    let Delaunay { triangles, .. } = delaunay;
    if triangles.is_empty() {
        // panic!("empty triangles");
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
