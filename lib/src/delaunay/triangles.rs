use geo::CoordFloat;
use num_traits::FloatConst;

use d3_delaunay_rs::delaunay::Delaunay;

use super::TriIndex;

pub fn triangles<PROJECTOR, T>(delaunay: &Delaunay<PROJECTOR, T>) -> Vec<TriIndex>
where
    T: CoordFloat + FloatConst,
{
    let Delaunay { triangles, .. } = delaunay;
    if triangles.is_empty() {
        return Vec::new();
    }

    let n: usize = triangles.len() / 3usize;
    let mut t: Vec<TriIndex> = Vec::with_capacity(n);

    for i in 0..n {
        let a = triangles[3 * i];
        let b = triangles[3 * i + 1];
        let c = triangles[3 * i + 2];
        if a != b && b != c {
            t.push([a, c, b]);
        }
    }
    t
}
