#![allow(clippy::many_single_char_names)]
mod cartesian;
pub mod excess;
mod geo_circumcenters;
pub mod geo_delaunay_from;
mod geo_edges;
mod geo_find;
mod geo_hull;
mod geo_mesh;
mod geo_neighbors;
mod geo_polygons;
mod geo_triangles;
mod geo_urquhart;
mod o_midpoint;

// use core::future::poll_fn;
use std::cell::RefCell;
/// Delaunay triangulation
use std::collections::HashMap;
use std::fmt;
use std::ops::AddAssign;
use std::rc::Rc;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;
use num_traits::{float::Float, FromPrimitive};

use rust_d3_delaunay::delaunay::Delaunay;
use rust_d3_geo::projection::projection_mutator::ProjectionMutator;

use geo_circumcenters::geo_circumcenters;
use geo_delaunay_from::geo_delaunay_from;
use geo_edges::geo_edges;
use geo_find::geo_find;
use geo_hull::geo_hull;
use geo_mesh::geo_mesh;
use geo_neighbors::geo_neighbors;
use geo_polygons::geo_polygons;
use geo_triangles::geo_triangles;
use geo_urquhart::geo_urquhart;

// #[derive(Default, Debug)]
// pub struct Delaunay {
//     /// The coordinates of the points as an array [x0, y0, x1, y1, ...].
//     /// TyPIcally, this is a Float64Array, however you can use any array-like type in the constructor.
//     ///
//     pub points: Vec<f64>,

//     ///
//     /// The halfedge indices as an Int32Array [j0, j1, ...].
//     /// For each index 0 <= i < half_edges.length, there is a halfedge from triangle vertex j = half_edges[i] to triangle vertex i.
//     ///
//     pub half_edges: Vec<i32>,

//     ///
//     /// An arbitrary node on the convex hull.
//     /// The convex hull is represented as a circular doubly-linked list of nodes.
//     ///
//     // hull: Node,

//     /// The triangle vertex indices as an Uint32Array [i0, j0, k0, i1, j1, k1, ...].
//     /// Each contiguous triplet of indices i, j, k forms a counterclockwise triangle.
//     /// The coordinates of the triangle's points can be found by going through 'points'.
//     ///
//     pub triangles: Vec<usize>,

//     pub centers: Option<Vec<Point>>,

//     /// The incoming halfedge indexes as a Int32Array [e0, e1, e2, ...].
//     /// For each point i, inedges[i] is the halfedge index e of an incoming halfedge.
//     /// For coincident points, the halfedge index is -1; for points on the convex hull, the incoming halfedge is on the convex hull; for other points, the choice of incoming halfedge is arbitrary.
//     ///
//     pub inedges: Vec<i32>,

//     /// The outgoing halfedge indexes as a Int32Array [e0, e1, e2, ...].
//     /// For each point i on the convex hull, outedges[i] is the halfedge index e of the corresponding outgoing halfedge; for other points, the halfedge index is -1.
//     ///
//     outedges: Vec<i32>,

//     pub projection: Option<ProjectionMutator>,
// }
// #[derive(Clone, Copy, Debug)]
pub struct GeoDelaunay<'a, T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + FloatConst,
{
    pub delaunay: Delaunay<T>,
    // The edges and triangles properties need RC because the values are close over in the urquhart function.
    pub edges: Rc<Vec<[usize; 2]>>,
    pub triangles: Rc<Vec<Vec<usize>>>,
    pub centers: Vec<Coordinate<T>>,
    // neighbours:  passes to Voronoi::polygon() where it is consumed.
    pub neighbors: Rc<RefCell<HashMap<usize, Vec<usize>>>>,
    pub polygons: Vec<Vec<usize>>,
    pub mesh: Vec<[usize; 2]>,
    pub hull: Vec<usize>,
    pub urquhart: Box<dyn Fn(&Vec<T>) -> Vec<bool> + 'a>,
    pub find: Box<dyn Fn(Coordinate<T>, Option<usize>) -> Option<usize> + 'a>,
}

impl<'a, T> GeoDelaunay<'a, T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + FloatConst + FromPrimitive,
{
    pub fn delaunay(points: Rc<Vec<Coordinate<T>>>) -> Option<GeoDelaunay<'a, T>> {
        let p = points.clone();
        match geo_delaunay_from(p) {
            Some(delaunay) => {
                // RC is needed here as tri and e are both closed over in the urquhart function an is part of the Delaunay return.
                let tri = Rc::new(geo_triangles(&delaunay));
                let e = Rc::new(geo_edges(&tri, &points));

                let circumcenters = geo_circumcenters(&tri, &points);
                println!("returned circumcenters");

                let (polys, centers) = geo_polygons(circumcenters, &tri, &points);

                // RC is needed here as it is both closed over in the find function an is part of the Delaunay return.
                let n = Rc::new(RefCell::new(geo_neighbors(&tri, points.len())));

                // Borrow and release polys.
                let m;
                {
                    m = geo_mesh(&polys);
                }

                // Borrow and release tri.
                let h;
                {
                    h = geo_hull(&tri, &points);
                }

                // Borrow and release e, tri.
                let u;
                {
                    u = geo_urquhart(e.clone(), tri.clone());
                }

                let f = geo_find(n.clone(), points);

                return Some(Self {
                    delaunay,
                    edges: e,
                    triangles: tri,
                    centers,
                    neighbors: n,
                    polygons: polys,
                    mesh: m,
                    hull: h,
                    urquhart: u,
                    find: f,
                });
            }
            None => {
                panic!("return none");
                // return None;
            }
        }
    }
}
