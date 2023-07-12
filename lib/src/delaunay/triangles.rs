use geo::CoordFloat;
use num_traits::FloatConst;

use d3_delaunay_rs::delaunay::Delaunay;

use super::TriIndex;

pub fn triangles<T>(delaunay: &Delaunay<T>) -> Vec<TriIndex>
where
    T: CoordFloat + FloatConst,
{
    let Delaunay { triangles, .. } = delaunay;

    triangles
        .chunks_exact(3)
        .filter(|t| t[0] != t[1] && t[1] != t[2])
        .map(|t| [t[0], t[2], t[1]])
        .collect()
}
