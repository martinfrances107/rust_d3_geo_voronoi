#![allow(clippy::many_single_char_names)]

use std::cmp;
use std::fmt::Debug;
use std::ops::AddAssign;
use std::rc::Rc;

use approx::AbsDiffEq;
use delaunator::EMPTY;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;
use num_traits::AsPrimitive;
use num_traits::FromPrimitive;

use rust_d3_delaunay::delaunay::Delaunay;
use rust_d3_geo::clip::buffer::Buffer;
use rust_d3_geo::clip::circle::interpolate::Interpolate as InterpolateCircle;
use rust_d3_geo::clip::circle::line::Line as LineCircle;
use rust_d3_geo::clip::circle::pv::PV as PVCircle;
use rust_d3_geo::projection::builder::template::NoClipC;
use rust_d3_geo::projection::builder::template::NoClipU;
use rust_d3_geo::projection::builder::template::ResampleNoClipC;
use rust_d3_geo::projection::builder::template::ResampleNoClipU;
use rust_d3_geo::projection::stereographic::Stereographic;
use rust_d3_geo::projection::Build;
use rust_d3_geo::projection::ProjectionRawBase;
use rust_d3_geo::projection::Rotate;
use rust_d3_geo::projection::Scale;
use rust_d3_geo::projection::Translate;
use rust_d3_geo::rot::rotation::Rotation;
use rust_d3_geo::stream::Connected;
use rust_d3_geo::stream::Stream;
use rust_d3_geo::stream::Unconnected;
use rust_d3_geo::Transform;

type DReturn<DRAIN, PCNU, PR, RC, RU, T> = Delaunay<
    DRAIN,
    InterpolateCircle<T>,
    LineCircle<Buffer<T>, Connected<Buffer<T>>, T>,
    LineCircle<RC, Connected<RC>, T>,
    LineCircle<RC, Unconnected, T>,
    PCNU,
    PR,
    PVCircle<T>,
    RC,
    RU,
    T,
>;

/// Creates a delaunay object from a set of points.
pub fn geo_delaunay_from<DRAIN, PCNC, PCNU, RC, RU, T>(
    points: Rc<Vec<Coordinate<T>>>,
) -> Option<
    DReturn<
        DRAIN,
        NoClipU<DRAIN>,
        Stereographic<DRAIN, T>,
        ResampleNoClipC<DRAIN, Stereographic<DRAIN, T>, T>,
        ResampleNoClipU<DRAIN, Stereographic<DRAIN, T>, T>,
        T,
    >,
>
where
    DRAIN: Clone + Debug + Default + Stream<EP = DRAIN, T = T>,
    T: AbsDiffEq<Epsilon = T>
        + AddAssign
        + AsPrimitive<T>
        + CoordFloat
        + FloatConst
        + FromPrimitive,
{
    if points.len() < 2 {
        return None;
    };

    // Find a valid Pivot point to send to infinity.
    // The index of the first acceptable point in
    // which the x or y component is not inifinty.
    let pivot: usize = points.iter().position(|p| (p.x + p.y).is_finite()).unwrap();

    let r = Rotation::new(points[pivot].x, points[pivot].y, T::zero());
    let r_invert = r.invert(&Coordinate {
        x: T::from(180_f64).unwrap(),
        y: T::zero(),
    });

    // let builder: Builder<DRAIN, LineCircle<T>, Stereographic<DRAIN, T>, PV<T>, T> =
    let builder = Stereographic::builder();
    let projection = builder
        .translate(&Coordinate {
            x: T::zero(),
            y: T::zero(),
        })
        .scale(T::one())
        .rotate(&[r_invert.x, r_invert.y, T::zero()])
        .build();

    let mut points: Vec<Coordinate<T>> = points.iter().map(|p| projection.transform(p)).collect();

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

    zeros.iter().for_each(|i| {
        points[*i] = Coordinate {
            x: far,
            y: T::zero(),
        }
    });

    // Add infinite horizon points
    points.push(Coordinate {
        x: T::zero(),
        y: far,
    });
    points.push(Coordinate {
        x: -far,
        y: T::zero(),
    });
    points.push(Coordinate {
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
            let j = match i % 3 == 2 {
                true => i - 2,
                false => i + 1,
            };
            let k = match i % 3 == 0 {
                true => i + 2,
                false => i - 1,
            };
            let a = delaunay.half_edges[j];
            let b = delaunay.half_edges[k];
            delaunay.half_edges[a] = b;
            delaunay.half_edges[b] = a;
            delaunay.half_edges[j] = EMPTY;
            delaunay.half_edges[k] = EMPTY;
            delaunay.triangles[i] = pivot;
            delaunay.triangles[j] = pivot;
            delaunay.triangles[k] = pivot;
            delaunay.inedges[delaunay.triangles[a]] = match a % 3 == 0 {
                true => a + 2,
                false => a - 1,
            };

            delaunay.inedges[delaunay.triangles[b]] = match b % 3 == 0 {
                true => b + 2,
                false => b - 1,
            };

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
