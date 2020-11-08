use std::cmp;
use std::rc::Rc;

use delaunator::Point;

use rust_d3_geo::projection::projection::Projection;
use rust_d3_geo::projection::stereographic::StereographicRaw;
use rust_d3_geo::rotation::rotation::Rotation;
use rust_d3_geo::Transform;

use super::Delaunay;

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
    // for (i, elem) in points.iter().enumerate() {
    for i in 0..points.len() {
        let m = points[i].x * points[i].x + points[i].y * points[i].y;
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

    let points = Rc::new(points);

    // const delaunay = Delaunay.from(points);
    let delaunay: Option<Delaunay> = delaunay_from(points.clone());

    match delaunay {
        Some(mut delaunay) => {
            delaunay.projection = Some(projection);

            // clean up the triangulation
            // let  {triangles, halfedges, inedges} = delaunay;
            // let triangles: &mut Vec<usize> = &mut delaunay.triangles;
            // let halfedges: &mut Vec<i32> = &mut delaunay.halfedges;
            // let mut inedges = delaunay.inedges;

            // const degenerate = [];
            let mut degenerate: Vec<usize> = Vec::new();
            // for (let i = 0, l = halfedges.length; i < l; i++) {
            for i in 0..delaunay.halfedges.len() {
                if delaunay.halfedges[i] < 0 {
                    let j = match i % 3 == 2 {
                        true => i - 2,
                        false => i + 1,
                    };
                    let k = match i % 3 == 0 {
                        true => i + 2,
                        false => i - 1,
                    };
                    let a = delaunay.halfedges[j] as usize;
                    let b = delaunay.halfedges[k] as usize;
                    delaunay.halfedges[a] = b as i32;
                    delaunay.halfedges[b] = a as i32;
                    delaunay.halfedges[j] = -1;
                    delaunay.halfedges[k] = -1;
                    delaunay.triangles[i] = pivot;
                    delaunay.triangles[j] = pivot;
                    delaunay.triangles[k] = pivot;
                    match a % 3 == 0 {
                        true => {
                            delaunay.inedges[delaunay.triangles[a]] = a as i32 + 2;
                            delaunay.inedges[delaunay.triangles[b]] = b as i32 + 2;
                        }
                        false => {
                            delaunay.inedges[delaunay.triangles[a]] = a as i32 - 1;
                            delaunay.inedges[delaunay.triangles[b]] = b as i32 - 1;
                        }
                    };
                    let m = cmp::min(i, j);
                    let m = cmp::min(m, k);
                    degenerate.push(m);

                // TODO must rework loop
                // i += 2 - i % 3;
                } else if delaunay.triangles[i] > points.len() - 3 - 1 {
                    delaunay.triangles[i] = pivot;
                }
            }

            // // there should always be 4 degenerate triangles
            // // console.warn(degenerate);
            return Some(delaunay);
        }
        None => {
            return None;
        }
    }
}
