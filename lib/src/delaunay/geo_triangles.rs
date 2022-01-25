use std::ops::AddAssign;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::{AsPrimitive, FloatConst};

use rust_d3_delaunay::delaunay::Delaunay;
use rust_d3_geo::clip::buffer::Buffer;
use rust_d3_geo::clip::post_clip_node::PostClipNode;
use rust_d3_geo::clip::Line;
use rust_d3_geo::clip::PointVisible;
use rust_d3_geo::projection::resample::ResampleNode;
use rust_d3_geo::projection::stream_node::StreamNode;
use rust_d3_geo::projection::Raw;
use rust_d3_geo::stream::Stream;

pub fn geo_triangles<DRAIN, LINE, PR, PV, T>(
    delaunay: &Delaunay<DRAIN, LINE, PR, PV, T>,
) -> Vec<[usize; 3]>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    LINE: Line,
    PR: Raw<T>,
    PV: PointVisible<T = T>,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + FloatConst,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    StreamNode<DRAIN, LINE, ResampleNode<DRAIN, PR, PostClipNode<DRAIN, DRAIN, T>, T>, T>:
        Stream<EP = DRAIN, T = T>,
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
