mod circumcenters;
pub mod delaunay_from;
mod edges;
pub mod excess;
mod find;
mod hull;
mod mesh;
mod neighbors;
mod o_midpoint;
mod polygons;
mod triangles;
mod urquhart;

use std::cell::RefCell;
/// Delaunay triangulation
use std::collections::HashMap;
use std::rc::Rc;
use std::fmt;


use delaunator::Point;


use rust_d3_geo::projection::projection_mutator::ProjectionMutator;

use circumcenters::circumcenters;
use delaunay_from::delaunay_from;
use edges::edges;
use find::find;
use hull::hull;
use mesh::mesh;
use neighbors::neighbors;
use polygons::polygons;
use triangles::triangles;
use urquhart::urquhart;

// #[derive(Default, Debug)]
pub struct Delaunay
{
  /// The coordinates of the points as an array [x0, y0, x1, y1, ...].
  /// TyPIcally, this is a Float64Array, however you can use any array-like type in the constructor.
  ///
  pub points: Vec<f64>,

  ///
  /// The halfedge indices as an Int32Array [j0, j1, ...].
  /// For each index 0 <= i < halfedges.length, there is a halfedge from triangle vertex j = halfedges[i] to triangle vertex i.
  ///
  pub halfedges: Vec<i32>,

  ///
  /// An arbitrary node on the convex hull.
  /// The convex hull is represented as a circular doubly-linked list of nodes.
  ///
  // hull: Node,

  /// The triangle vertex indices as an Uint32Array [i0, j0, k0, i1, j1, k1, ...].
  /// Each contiguous triplet of indices i, j, k forms a counterclockwise triangle.
  /// The coordinates of the triangle's points can be found by going through 'points'.
  ///
  pub triangles: Vec<usize>,

  pub centers: Option<Vec<Point>>,

  /// The incoming halfedge indexes as a Int32Array [e0, e1, e2, ...].
  /// For each point i, inedges[i] is the halfedge index e of an incoming halfedge.
  /// For coincident points, the halfedge index is -1; for points on the convex hull, the incoming halfedge is on the convex hull; for other points, the choice of incoming halfedge is arbitrary.
  ///
  pub inedges: Vec<i32>,

  /// The outgoing halfedge indexes as a Int32Array [e0, e1, e2, ...].
  /// For each point i on the convex hull, outedges[i] is the halfedge index e of the corresponding outgoing halfedge; for other points, the halfedge index is -1.
  ///
  outedges: Vec<i32>,

  pub projection: Option<ProjectionMutator>,
}

// #[derive(Clone)]
pub struct DelaunayReturn<'a>
{
  pub delaunay: Delaunay,
  // The edges and triangles properties need RC because the values are close over in the urquhart function.
  pub edges: Rc<Vec<[usize; 2]>>,
  pub triangles: Rc<Vec<Vec<usize>>>,
  pub centers: Vec<Point>,
  // neighbours:  passes to Voronoi::polygon() where it is consumed.
  pub neighbors: Rc<RefCell<HashMap<usize, Vec<usize>>>>,
  pub polygons: Vec<Vec<usize>>,
  pub mesh: Vec<[usize; 2]>,
  pub hull: Vec<usize>,
  pub urquhart: Box<dyn Fn(&Vec<f64>) -> Vec<bool> + 'a>,
  pub find: Box<dyn Fn(f64, f64, Option<usize>) -> Option<usize> + 'a>,
}

impl fmt::Display for DelaunayReturn<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.centers, self.hull)
    }
}

impl<'a> Delaunay
{
  pub fn delaunay(points: Rc<Vec<Point>>) -> Option<DelaunayReturn<'a>> {
    match delaunay_from(points.clone()) {
      Some(delaunay) => {
        // RC is needed here as tri and e are both closed over in the urquhart function an is part of the Delaunay return.
        let tri = Rc::new(triangles(&delaunay));
        let e = Rc::new(edges(&tri, &points));
        let polys;
        let centers;
        {
          let pr = polygons(circumcenters(&tri, &points), &tri, &points);
          polys = pr.0;
          centers = pr.1;
        }

        // RC is needed here as it is both closed over in the find function an is part of the Delaunay return.
        let n = Rc::new(RefCell::new(neighbors(&tri, points.len())));

        // Borrow and release polys.
        let m;
        {
          m = mesh(&polys);
        }

        // Borrow and release tri.
        let h;
        {
          h = hull(&tri, &points);
        }

        // Borrow and release e, tri.
        let u;
        {
          u = urquhart(e.clone(), tri.clone());
        }

        let f;
        {
          f = find(n.clone(), points);
        }

        return Some(DelaunayReturn {
          delaunay: delaunay,
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
        return None;
      }
    }
  }
}
