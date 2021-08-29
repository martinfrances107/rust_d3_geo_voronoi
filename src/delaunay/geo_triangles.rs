use rust_d3_geo::clip::Line;
use rust_d3_geo::clip::PointVisible;
use rust_d3_geo::projection::Raw;
use rust_d3_geo::stream::Stream;
use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use num_traits::{AsPrimitive, FloatConst};
use rust_d3_delaunay::delaunay::Delaunay;

pub fn geo_triangles<
    DRAIN: Stream<T = T>,
    L: Line,
    PR: Raw<T>,
    PV: PointVisible<T = T>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + FloatConst,
>(
    delaunay: &Delaunay<DRAIN, L, PR, PV, T>,
) -> Vec<Vec<usize>> {
    let Delaunay { triangles, .. } = delaunay;
    if triangles.is_empty() {
        return Vec::new();
    }

    let mut geo_triangles: Vec<Vec<usize>> = Vec::new();
    let n: usize = triangles.len() / 3usize;

    for i in 0..n {
        let a = triangles[3 * i];
        let b = triangles[3 * i + 1];
        let c = triangles[3 * i + 2];
        if a != b && b != c {
            geo_triangles.push(vec![a, c, b]);
        }
    }
    geo_triangles
}
