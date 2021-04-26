use delaunator::Point;
use geo::CoordFloat;
use num_traits::{AsPrimitive, Float, FloatConst};
use rust_d3_delaunay::delaunay::Delaunay;
use std::ops::AddAssign;

pub fn geo_triangles<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + FloatConst>(
    delaunay: &Delaunay<T>,
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
    return geo_triangles;
}
