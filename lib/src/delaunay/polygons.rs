#![allow(clippy::many_single_char_names)]
use core::fmt::Debug;
use std::collections::HashMap;
use std::rc::Rc;

use d3_geo_rs::cartesian::add;
use d3_geo_rs::cartesian::cross;
use d3_geo_rs::cartesian::normalize;
use d3_geo_rs::cartesian::spherical;
use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use super::cartesian::cartesian;
use super::o_midpoint::o_midpoint;
use super::TriIndex;

type TupleVec = Vec<(usize, usize, usize, (usize, usize, usize))>;

fn supplement<T>(
    point: &Coord<T>,
    centers: &mut Vec<Coord<T>>,
    triangles_len: usize,
) -> usize
where
    T: CoordFloat + FloatConst,
{
    let mut f = None;
    centers[triangles_len..]
        .iter()
        .enumerate()
        .for_each(|(i, p)| {
            if p == point {
                f = Some(i + triangles_len);
            }
        });
    f.map_or_else(
        || {
            let f_out: usize = centers.len();
            centers.push(*point);
            f_out
        },
        |f| f,
    )
}

/// Looking at the flamegraph generated with `profile_target`.
///
/// This is on the hot path, please profile after modifying
/// this functions.
pub fn gen<T>(
    circumcenter: Vec<Coord<T>>,
    triangles_p: Rc<Vec<TriIndex>>,
    points: &[Coord<T>],
) -> (Vec<Vec<usize>>, Vec<Coord<T>>)
where
    T: CoordFloat + Debug + FloatConst,
{
    let mut centers = circumcenter;
    let triangles = triangles_p;

    if triangles.is_empty() {
        if points.len() < 2 {
            return (vec![], centers);
        }
        // // WARNING in the original javascript this block is never tested.
        if points.len() == 2 {
            let mut polygons: Vec<Vec<usize>> = Vec::new();
            // Two hemispheres.
            let a = cartesian(&points[0]);
            let b = cartesian(&points[1]);
            let m = normalize(&add(a, b));

            let d = normalize(&cross(&a, &b));
            let c = cross(&m, &d);
            let poly: Vec<usize> = [
                m,
                cross(&m, &c),
                cross(&cross(&m, &c), &c),
                cross(&cross(&cross(&m, &c), &c), &c),
            ]
            .iter()
            .map(|p| spherical(p))
            .map(|p| supplement(&p, &mut centers, triangles.len()))
            .collect();
            let rev: Vec<usize> = poly.iter().rev().copied().collect();
            polygons.push(poly);
            polygons.push(rev);
            return (polygons, centers);
        }
    }

    let mut polygons: HashMap<usize, TupleVec> =
        HashMap::with_capacity(triangles.len());
    for (t, tri) in triangles.iter().enumerate() {
        for j in 0..3 {
            let a = tri[j];
            let b = tri[(j + 1) % 3];
            let c = tri[(j + 2) % 3];

            let next = (b, c, t, (a, b, c));

            // This "nursey" clippy condition gives compilation errors.
            #[allow(clippy::option_if_let_else)]
            match polygons.get_mut(&a) {
                Some(polygon) => {
                    polygon.push(next);
                }
                None => {
                    polygons.insert(a, vec![next]);
                }
            };
        }
    }

    // Reorder each polygon.
    let reordered: Vec<Vec<usize>> = polygons
        .iter()
        .map(|poly_ind| {
            let poly = poly_ind.1;
            let mut p = vec![poly[0].2]; // t
            let mut k = poly[0].1; // k = c

            for _i in 0..poly.len() {
                // look for b = k
                for pj in poly {
                    if pj.0 == k {
                        k = pj.1;
                        p.push(pj.2);
                        break;
                    }
                }
            }

            match p.len() {
                0 | 1 => Vec::new(),
                2 => {
                    let r0 = o_midpoint(
                        &points[(poly[0].3).0],
                        &points[(poly[0].3).1],
                        &centers[p[0]],
                    );
                    let r1 = o_midpoint(
                        &points[(poly[0].3).2],
                        &points[(poly[0].3).0],
                        &centers[p[0]],
                    );

                    let i0 = supplement(&r0, &mut centers, triangles.len());
                    let i1 = supplement(&r1, &mut centers, triangles.len());

                    vec![p[0], i1, p[1], i0]
                }
                _ => p,
            }
        })
        .collect();

    (reordered, centers)
}
