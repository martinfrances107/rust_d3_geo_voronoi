#![allow(clippy::many_single_char_names)]
use std::cmp;
use std::rc::Rc;

use delaunator::Point;
use rust_d3_delaunay::delaunay::Delaunay;

use rust_d3_geo::projection::projection::Projection;
use rust_d3_geo::projection::stereographic::StereographicRaw;
use rust_d3_geo::rotation::rotation::Rotation;
use rust_d3_geo::Transform;

use delaunator::EMPTY;

pub fn delaunay_from(points: Rc<Vec<Point>>) -> Option<Delaunay> {
    if points.len() < 2 {
        return None;
    };

    // Find a valid PIvot point.
    // The index of the first acceptable point in
    // which the x or y component is not inifinty.
    let pivot: usize = points.iter().position(|p| (p.x + p.y).is_finite()).unwrap();

    // TODO must fix this
    // let r = Rotation::new(points[pivot][0], points[pivot][1], points[pivot][2]);
    let r = Rotation::new(points[pivot].x, points[pivot].y, 0f64);

    let mut projection = StereographicRaw::gen_projection_mutator();
    projection.translate(Some(&Point { x: 0f64, y: 0f64 }));
    projection.scale(Some(&1f64));
    let angles2: Point = r.invert(&Point { x: 180f64, y: 0f64 });
    let angles: [f64; 3] = [angles2.x, angles2.y, 0f64];
    projection.rotate(Some(angles));

    let mut points: Vec<Point> = points.iter().map(|p| projection.transform(&p)).collect();

    let mut zeros = Vec::new();
    let mut max2 = 1f64;
    for (i, point) in points.iter().enumerate() {
        let m = point.x * point.x + point.y * point.y;
        if !m.is_finite() || m > 1e32f64 {
            zeros.push(i);
        } else {
            if m > max2 {
                max2 = m;
            }
        }
    }
    let far = 1e6 * (max2).sqrt();

    zeros
        .iter()
        .for_each(|i| points[*i] = Point { x: far, y: 0f64 });

    // Add infinite horizon points
    points.push(Point { x: 0f64, y: far });
    points.push(Point { x: -far, y: 0f64 });
    points.push(Point { x: 0f64, y: -far });

    let point_len = points.len();

    let mut delaunay = Delaunay::new(points);

    delaunay.projection = Box::new(projection);

    // clean up the triangulation
    // let  {triangles, half_edges, inedges} = delaunay;
    // let triangles: &mut Vec<usize> = &mut delaunay.triangles;
    // let half_edges: &mut Vec<i32> = &mut delaunay.half_edges;
    // let mut inedges = delaunay.inedges;

    let mut degenerate: Vec<usize> = Vec::new();
    for i in 0..delaunay.half_edges.len() {
        if delaunay.half_edges[i] < 0 {
            let j = match i % 3 == 2 {
                true => i - 2,
                false => i + 1,
            };
            let k = match i % 3 == 0 {
                true => i + 2,
                false => i - 1,
            };
            let a = delaunay.half_edges[j] as usize;
            let b = delaunay.half_edges[k] as usize;
            delaunay.half_edges[a] = b;
            delaunay.half_edges[b] = a;
            delaunay.half_edges[j] = EMPTY;
            delaunay.half_edges[k] = EMPTY;
            delaunay.triangles[i] = pivot;
            delaunay.triangles[j] = pivot;
            delaunay.triangles[k] = pivot;
            match a % 3 == 0 {
                true => {
                    delaunay.inedges[delaunay.triangles[a]] = a + 2;
                    delaunay.inedges[delaunay.triangles[b]] = b + 2;
                }
                false => {
                    delaunay.inedges[delaunay.triangles[a]] = a - 1;
                    delaunay.inedges[delaunay.triangles[b]] = b - 1;
                }
            };
            let m = cmp::min(i, j);
            let m = cmp::min(m, k);
            degenerate.push(m);

        // TODO must rework loop
        // i += 2 - i % 3;
        } else if delaunay.triangles[i] > point_len - 3 - 1 {
            delaunay.triangles[i] = pivot;
        }
    }

    // // there should always be 4 degenerate triangles
    // // console.warn(degenerate);
    return Some(delaunay);
}
