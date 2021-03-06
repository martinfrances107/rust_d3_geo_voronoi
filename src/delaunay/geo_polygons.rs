#![allow(clippy::many_single_char_names)]
use std::collections::HashMap;
use std::rc::Rc;

// use delaunator::Point;
use delaunator::EMPTY;
use geo::{CoordFloat, Coordinate};
use num_traits::Float;
use rust_d3_geo::cartesian::cartesian_add;
use rust_d3_geo::cartesian::cartesian_cross;
use rust_d3_geo::cartesian::cartesian_normalize;
use rust_d3_geo::cartesian::spherical;

use crate::math::{EPSILON, EPSILON2};

use super::cartesian::cartesian;
use super::o_midpoint::o_midpoint;

pub fn geo_polygons<'a, T: CoordFloat>(
    circumcenter: Vec<Coordinate<T>>,
    triangles: &[Vec<usize>],
    points: &'a [Coordinate<T>],
) -> (Vec<Vec<usize>>, Vec<Coordinate<T>>) {
    let epsilon_t = T::from(EPSILON).unwrap();
    let mut polygons: Vec<Vec<usize>> = Vec::new();
    let mut centers = circumcenter;

    let supplement = |point: &Coordinate<T>, centers: &mut Vec<Coordinate<T>>| -> usize {
        let mut f = None;
        centers[triangles.len()..]
            .iter()
            .enumerate()
            .for_each(|(i, p)| {
                if p == point {
                    f = Some(triangles.len() + 1)
                }
            });
        match f {
            None => {
                let f_out: usize = centers.len();
                centers.push(*point);
                f_out
            }
            Some(f) => f,
        }
    };

    if triangles.is_empty() {
        if points.is_empty() {
            return (polygons, centers);
        }
        if points.len() == 1 {
            // // WARNING in the original javascript this block is never tested.
            if points.len() == 2 {
                // Two hemispheres.
                let a = cartesian(&points[0]);
                let b = cartesian(&points[0]);
                let m = cartesian_normalize(&cartesian_add(a, b));

                let d = cartesian_normalize(&cartesian_cross(&a, &b));
                let c = cartesian_cross(&m, &d);

                let poly: Vec<usize> = [
                    m,
                    cartesian_cross(&m, &c),
                    cartesian_cross(&cartesian_cross(&m, &c), &c),
                    cartesian_cross(&cartesian_cross(&cartesian_cross(&m, &c), &c), &c),
                ]
                .iter()
                .map(|p| spherical(p))
                .map(|p| supplement(&p, &mut centers))
                .collect();
                polygons.push(poly);
                // let rev: Vec<usize> = poly.iter().rev().map(|x| *x).collect();
                // polygons.push(rev);
                return (polygons, centers);
            }
        }
    }

    let mut polygons_map: HashMap<usize, Vec<(usize, usize, usize, (usize, usize, usize))>> =
        HashMap::new();
    for (t, tri) in triangles.iter().enumerate() {
        for j in 0..3 {
            let a = tri[j];
            let b = tri[(j + 1) % 3];
            let c = tri[(j + 2) % 3];
            let mut tuple_vec: Vec<(usize, usize, usize, (usize, usize, usize))>;
            match polygons_map.get(&a) {
                Some(t) => {
                    tuple_vec = (*t).clone();
                }
                None => {
                    tuple_vec = Vec::new();
                }
            }
            tuple_vec.push((b, c, t, (a, b, c)));
            polygons_map.insert(a, tuple_vec);
        }
    }

    // Reorder each polygon.
    let reordered: Vec<Vec<usize>> = polygons_map
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
                0 | 1 => {
                    return Vec::new();
                }
                2 => {
                    let i0;
                    let i1;

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

                    i0 = supplement(&r0, &mut centers);
                    i1 = supplement(&r1, &mut centers);

                    return vec![p[0], i1, p[1], i0];
                }
                _ => {
                    return p;
                }
            }
        })
        .collect();

    return ((*reordered).to_vec(), centers);
}
