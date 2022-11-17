#![allow(clippy::many_single_char_names)]

use std::cmp;
use std::fmt::Debug;
use std::rc::Rc;

use approx::AbsDiffEq;
use delaunator::EMPTY;
use geo::CoordFloat;
use geo_types::Coord;
use num_traits::float::FloatConst;
use num_traits::FromPrimitive;

use rust_d3_delaunay::delaunay::Delaunay;
use rust_d3_geo::clip::circle::ClipCircleC;
use rust_d3_geo::clip::circle::ClipCircleU;
use rust_d3_geo::projection::builder::template::NoPCNU;
use rust_d3_geo::projection::builder::template::ResampleNoPCNC;
use rust_d3_geo::projection::builder::template::ResampleNoPCNU;
use rust_d3_geo::projection::stereographic::Stereographic;
use rust_d3_geo::projection::Build;
use rust_d3_geo::projection::RawBase as ProjectionRawBase;
use rust_d3_geo::projection::RotateSet;
use rust_d3_geo::projection::ScaleSet;
use rust_d3_geo::projection::TranslateSet;
use rust_d3_geo::rot::rotation::Rotation;
use rust_d3_geo::stream::Stream;
use rust_d3_geo::Transform;

type DReturn<DRAIN, PCNU, PR, RC, RU, T> =
    Delaunay<ClipCircleC<RC, T>, ClipCircleU<RC, T>, DRAIN, PCNU, PR, RC, RU, T>;

/// Creates a delaunay object from a set of points.
///
/// # Panics
///  Will never happen as constants will always be converted into T.
#[allow(clippy::type_complexity)]
#[must_use]
pub fn geo_delaunay_from<DRAIN, PCNC, PCNU, RC, RU, T>(
    points: &Rc<Vec<Coord<T>>>,
) -> Option<
    DReturn<
        DRAIN,
        NoPCNU,
        Stereographic<DRAIN, T>,
        ResampleNoPCNC<DRAIN, Stereographic<DRAIN, T>, T>,
        ResampleNoPCNU<Stereographic<DRAIN, T>, T>,
        T,
    >,
>
where
    DRAIN: Clone + Debug + Default + Stream<EP = DRAIN, T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + Default + FloatConst + FromPrimitive,
{
    if points.len() < 2 {
        return None;
    };

    // Find a valid Pivot point to send to infinity.
    // The index of the first acceptable point in
    // which the x or y component is not inifinty.
    let pivot: usize = points.iter().position(|p| (p.x + p.y).is_finite()).unwrap();

    let r = Rotation::new(points[pivot].x, points[pivot].y, T::zero());
    let r_invert = r.invert(&Coord {
        x: T::from(180_f64).unwrap(),
        y: T::zero(),
    });

    let mut builder = Stereographic::builder();
    builder.translate_set(&Coord {
        x: T::zero(),
        y: T::zero(),
    });
    builder.scale_set(T::one());
    builder.rotate_set(&[r_invert.x, r_invert.y, T::zero()]);
    let projection = builder.build();

    let mut points: Vec<Coord<T>> = points.iter().map(|p| projection.transform(p)).collect();

    let mut zeros = Vec::new();
    let mut max2 = T::one();
    let m_threshold = T::from(1e32f64).unwrap();
    for (i, point) in points.iter().enumerate() {
        let m = point.x * point.x + point.y * point.y;
        if !m.is_finite() || m > m_threshold {
            zeros.push(i);
        } else if m > max2 {
            max2 = m;
        }
    }
    let far = T::from(1e6_f64).unwrap() * (max2).sqrt();

    for i in zeros {
        points[i] = Coord {
            x: far,
            y: T::zero(),
        }
    }

    // Add infinite horizon points
    points.push(Coord {
        x: T::zero(),
        y: far,
    });
    points.push(Coord {
        x: -far,
        y: T::zero(),
    });
    points.push(Coord {
        x: T::zero(),
        y: -far,
    });

    let point_len = points.len();
    let mut delaunay = Delaunay::new(&points);
    delaunay.projection = Some(projection);

    // clean up the triangulation
    let mut degenerate: Vec<usize> = Vec::new();
    let mut i: usize = 0;
    let l = delaunay.half_edges.len();

    'he_loop: loop {
        if delaunay.half_edges[i] == EMPTY {
            let j = if i % 3 == 2 { i - 2 } else { i + 1 };
            let k = if i % 3 == 0 { i + 2 } else { i - 1 };
            let a = delaunay.half_edges[j];
            let b = delaunay.half_edges[k];
            delaunay.half_edges[a] = b;
            delaunay.half_edges[b] = a;
            delaunay.half_edges[j] = EMPTY;
            delaunay.half_edges[k] = EMPTY;
            delaunay.triangles[i] = pivot;
            delaunay.triangles[j] = pivot;
            delaunay.triangles[k] = pivot;
            delaunay.inedges[delaunay.triangles[a]] = if a % 3 == 0 { a + 2 } else { a - 1 };
            delaunay.inedges[delaunay.triangles[b]] = if b % 3 == 0 { b + 2 } else { b - 1 };

            let mut m = cmp::min(i, j);
            m = cmp::min(m, k);
            degenerate.push(m);

            i += 2 - i % 3;
        } else if delaunay.triangles[i] > point_len - 3 - 1 {
            delaunay.triangles[i] = pivot;
        }

        i += 1;
        if i >= l {
            break 'he_loop;
        }
    }
    // // there should always be 4 degenerate triangles
    // // console.warn(degenerate);
    Some(delaunay)
}
