#![allow(clippy::many_single_char_names)]
use std::collections::HashMap;
use std::rc::Rc;

use geo::{CoordFloat, Coordinate};

use rust_d3_geo::cartesian::add;
use rust_d3_geo::cartesian::cross;
use rust_d3_geo::cartesian::normalize;
use rust_d3_geo::cartesian::spherical;

use super::cartesian::cartesian;
use super::o_midpoint::o_midpoint;

// let mut tuple_vec: Vec<(usize, usize, usize, (usize, usize, usize))>;
type TupleVec = Vec<(usize, usize, usize, (usize, usize, usize))>;

#[derive(Debug)]
pub struct GeoPolygons<T>
where
    T: CoordFloat,
{
    centers: Vec<Coordinate<T>>,
    polygons: Vec<Vec<usize>>,
    triangles: Rc<Vec<[usize; 3]>>,
}

impl<T> Default for GeoPolygons<T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Self {
            centers: Vec::new(),
            polygons: Vec::new(),
            triangles: Rc::new(Vec::new()),
        }
    }
}

impl<T> GeoPolygons<T>
where
    T: CoordFloat,
{
    fn supplement(&mut self, point: &Coordinate<T>) -> usize {
        let mut f = None;
        self.centers[self.triangles.len()..]
            .iter()
            .enumerate()
            .for_each(|(i, p)| {
                if p == point {
                    f = Some(i + self.triangles.len())
                }
            });
        match f {
            None => {
                let f_out: usize = self.centers.len();
                self.centers.push(*point);
                f_out
            }
            Some(f) => f,
        }
    }

    pub fn gen(
        mut self,
        circumcenter: Vec<Coordinate<T>>,
        triangles_p: Rc<Vec<[usize; 3]>>,
        points: &[Coordinate<T>],
    ) -> (Vec<Vec<usize>>, Vec<Coordinate<T>>) {
        let mut polygons: Vec<Vec<usize>> = Vec::new();
        self.centers = circumcenter;
        self.triangles = triangles_p;

        if self.triangles.is_empty() {
            if points.len() < 2 {
                return (polygons, self.centers);
            }
            // // WARNING in the original javascript this block is never tested.
            if points.len() == 2 {
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
                .map(|p| self.supplement(&p))
                .collect();
                polygons.push(poly.clone());
                let rev: Vec<usize> = poly.iter().rev().copied().collect();
                polygons.push(rev);
                return (polygons, self.centers);
            }
        }

        let mut polygons_map: HashMap<usize, TupleVec> =
            HashMap::with_capacity(self.triangles.len());
        for (t, tri) in self.triangles.iter().enumerate() {
            for j in 0..3 {
                let a = tri[j];
                let b = tri[(j + 1) % 3];
                let c = tri[(j + 2) % 3];
                let mut tuple_vec: TupleVec = match polygons_map.get(&a) {
                    Some(t) => (*t).clone(),
                    None => Vec::new(),
                };
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
                    0 | 1 => Vec::new(),
                    2 => {
                        let i0;
                        let i1;

                        let r0 = o_midpoint(
                            &points[(poly[0].3).0],
                            &points[(poly[0].3).1],
                            &self.centers[p[0]],
                        );
                        let r1 = o_midpoint(
                            &points[(poly[0].3).2],
                            &points[(poly[0].3).0],
                            &self.centers[p[0]],
                        );

                        i0 = self.supplement(&r0);
                        i1 = self.supplement(&r1);

                        return vec![p[0], i1, p[1], i0];
                    }
                    _ => p,
                }
            })
            .collect();

        ((*reordered).to_vec(), self.centers)
    }
}
