#![allow(clippy::many_single_char_names)]

use core::cmp;

use delaunator::EMPTY;
use geo::CoordFloat;
use geo_types::Coord;
use num_traits::float::FloatConst;
use num_traits::FromPrimitive;

use d3_delaunay_rs::delaunay::Delaunay;
use d3_geo_rs::projection::stereographic::Stereographic;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::RawBase as ProjectionRawBase;
use d3_geo_rs::projection::RotateSet;
use d3_geo_rs::projection::ScaleSet;
use d3_geo_rs::projection::TranslateSet;
use d3_geo_rs::rot::rotation::Rotation;
use d3_geo_rs::stream::DrainStub;
use d3_geo_rs::Transform;

/// Creates a delaunay object from a set of points.
///
/// # Panics
///  Will never happen as constants will always be converted into T.
#[must_use]
pub fn from_points<T>(points: &[Coord<T>]) -> Option<Delaunay<T>>
where
    T: 'static + CoordFloat + Default + FloatConst + FromPrimitive,
{
    if points.len() < 2 {
        return None;
    }

    // Find a valid Pivot point to send to infinity.
    // The index of the first acceptable point in
    // which the x or y component is not infinity.
    let pivot: usize =
        points.iter().position(|p| (p.x + p.y).is_finite()).unwrap();

    let r = Rotation::new(points[pivot].x, points[pivot].y, T::zero());
    let r_invert = r.invert(&Coord {
        x: T::from(180_f64).unwrap(),
        y: T::zero(),
    });

    let mut builder = Stereographic::builder::<DrainStub<T>>();
    builder.translate_set(&Coord {
        x: T::zero(),
        y: T::zero(),
    });
    builder.scale_set(T::one());
    builder.rotate2_set(&[r_invert.x, r_invert.y]);
    let projection = builder.build();

    let mut points: Vec<Coord<T>> =
        points.iter().map(|p| projection.transform(p)).collect();

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

    // Clean up the triangulation.
    let mut degenerate: Vec<usize> = Vec::new();
    let mut i: usize = 0;
    let l = delaunay.delaunator.halfedges.len();

    'he_loop: loop {
        if delaunay.delaunator.halfedges[i] == EMPTY {
            let j = if i % 3 == 2 { i - 2 } else { i + 1 };
            let k = if i % 3 == 0 { i + 2 } else { i - 1 };
            let a = delaunay.delaunator.halfedges[j];
            let b = delaunay.delaunator.halfedges[k];
            delaunay.delaunator.halfedges[a] = b;
            delaunay.delaunator.halfedges[b] = a;
            delaunay.delaunator.halfedges[j] = EMPTY;
            delaunay.delaunator.halfedges[k] = EMPTY;
            delaunay.delaunator.triangles[i] = pivot;
            delaunay.delaunator.triangles[j] = pivot;
            delaunay.delaunator.triangles[k] = pivot;
            delaunay.inedges[delaunay.delaunator.triangles[a]] =
                if a % 3 == 0 { a + 2 } else { a - 1 };
            delaunay.inedges[delaunay.delaunator.triangles[b]] =
                if b % 3 == 0 { b + 2 } else { b - 1 };

            let mut m = cmp::min(i, j);
            m = cmp::min(m, k);
            degenerate.push(m);

            i += 2 - i % 3;
        } else if delaunay.delaunator.triangles[i] > point_len - 3 - 1 {
            delaunay.delaunator.triangles[i] = pivot;
        }

        i += 1;
        if i >= l {
            break 'he_loop;
        }
    }
    // // there should always be 4 degenerate triangles
    debug_assert_eq!(degenerate.len(), 4);
    Some(delaunay)
}
