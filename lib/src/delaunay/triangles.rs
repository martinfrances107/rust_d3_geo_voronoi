use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use rust_d3_delaunay::delaunay::Delaunay;

pub fn triangles<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>(
    delaunay: &Delaunay<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>,
) -> Vec<[usize; 3]>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    let Delaunay { triangles, .. } = delaunay;
    if triangles.is_empty() {
        return Vec::new();
    }

    let n: usize = triangles.len() / 3usize;
    let mut t: Vec<[usize; 3]> = Vec::with_capacity(n);

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
