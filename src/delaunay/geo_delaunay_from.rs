#![allow(clippy::many_single_char_names)]
use std::cmp;
use std::ops::AddAssign;
use std::rc::Rc;

use geo::Point;
use geo::{CoordFloat, Coordinate};
use num_traits::{float::Float, float::FloatConst, AsPrimitive, FromPrimitive};

use rust_d3_delaunay::delaunay::Delaunay;

use rust_d3_geo::projection::projection::Projection;
use rust_d3_geo::projection::stereographic::StereographicRaw;
use rust_d3_geo::rotation::rotation::Rotation;
use rust_d3_geo::Transform;

use delaunator::EMPTY;

pub fn geo_delaunay_from<T>(points: Rc<Vec<Coordinate<T>>>) -> Option<Delaunay<T>>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + FloatConst + FromPrimitive,
{
    if points.len() < 2 {
        return None;
    };

    // Find a valid PIvot point.
    // The index of the first acceptable point in
    // which the x or y component is not inifinty.
    let pivot: usize = points.iter().position(|p| (p.x + p.y).is_finite()).unwrap();

    let r = Rotation::new(points[pivot].x, points[pivot].y, T::zero());

    let angles2 = r.invert(&Coordinate {
        x: T::from(180).unwrap(),
        y: T::zero(),
    });
    let angles: [T; 3] = [angles2.x, angles2.y, T::zero()];

    let projection = StereographicRaw::gen_projection_mutator()
        .translate(&Coordinate {
            x: T::zero(),
            y: T::zero(),
        })
        .scale(T::one())
        .rotate(angles);

    let mut points: Vec<Coordinate<T>> = points.iter().map(|p| projection.transform(&p)).collect();

    let mut zeros = Vec::new();
    let mut max2 = T::one();
    for (i, point) in points.iter().enumerate() {
        let m = point.x * point.x + point.y * point.y;
        if !m.is_finite() || m > T::from(1e32f64).unwrap() {
            zeros.push(i);
        } else if m > max2 {
            max2 = m;
        }
    }
    let far = T::from(1e6).unwrap() * (max2).sqrt();

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
    let mut delaunay = Delaunay::new(points);
    delaunay.projection = Some(projection);

    // clean up the triangulation
    // let  {triangles, half_edges, inedges} = delaunay;
    // let triangles: &mut Vec<usize> = &mut delaunay.triangles;
    // let half_edges: &mut Vec<i32> = &mut delaunay.half_edges;
    // let mut inedges = delaunay.inedges;

    let mut degenerate: Vec<usize> = Vec::new();
    let mut i: usize = 0;
    let l = delaunay.half_edges.len();

    'he_loop: loop {
        if delaunay.half_edges[i] == EMPTY {
            println!("empty");
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
    return Some(delaunay);
}
