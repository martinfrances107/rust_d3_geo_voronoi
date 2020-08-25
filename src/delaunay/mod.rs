mod circumcenters;
mod edges;
mod excess;
mod delaunay_from;
mod find;
mod hull;
mod mesh;
mod neighbors;
mod o_midpoint;
mod polygons;
mod triangles;
mod urquhart;

/// Delaunay triangulation
use std::collections::HashMap;

use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

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

#[derive(Default)]
pub struct Delaunay<F>
where
  F: Float + FloatConst + FromPrimitive,
{
  /// The coordinates of the points as an array [x0, y0, x1, y1, ...].
  /// TyPIcally, this is a Float64Array, however you can use any array-like type in the constructor.
  ///
  points: Vec<F>,

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

  centers: Option<Vec<[F;2]>>,

  /// The incoming halfedge indexes as a Int32Array [e0, e1, e2, ...].
  /// For each point i, inedges[i] is the halfedge index e of an incoming halfedge.
  /// For coincident points, the halfedge index is -1; for points on the convex hull, the incoming halfedge is on the convex hull; for other points, the choice of incoming halfedge is arbitrary.
  ///
  pub inedges: Vec<i32>,

  /// The outgoing halfedge indexes as a Int32Array [e0, e1, e2, ...].
  /// For each point i on the convex hull, outedges[i] is the halfedge index e of the corresponding outgoing halfedge; for other points, the halfedge index is -1.
  ///
  outedges: Vec<i32>,

  pub projection: Option<ProjectionMutator<F>>,

}

struct DelaunayReturn<F>
where
  F: Float + FloatConst + FromPrimitive,
{
  delaunay: Delaunay<F>,
  edges: Vec<[usize; 2]>,
  triangles: Vec<[usize; 3]>,
  centers: Vec<[F; 2]>,
  neghbors: HashMap<usize, Vec<usize>>,
  polygons: Vec<Vec<usize>>,
  mesh: Vec<[usize; 2]>,
  hull: Vec<usize>,
  urquhart: Box<dyn Fn(Vec<F>) -> Vec<bool>>,
  find: Box<dyn Fn(F, F, Option<usize>) -> Option<usize>>,
}

impl<F> Delaunay<F>
where
  F: Float + FloatConst + FromPrimitive + 'static,
{
  fn delaunay(points: &Vec<[F; 2]>) -> Option<DelaunayReturn<F>> {
    match delaunay_from(&points) {
      Some(delaunay) => {
        let tri = triangles(delaunay);
        let e = edges(&tri, &points);
        let pr = polygons(circumcenters(&tri, &points), tri, &points);
        let polys = pr.0;
        let centers = pr.1;
        let n = neighbors(tri, points.len());

        return Some(DelaunayReturn {
          delaunay: delaunay,
          edges: e,
          triangles: tri,
          centers,
          neghbors: n,
          polygons: polys,
          mesh: mesh(polys),
          hull: hull(&tri, &points),
          urquhart: urquhart(e, tri),
          find: find(n, &points),
        });
      }
      None => {
        return None;
      }
    }
  }
}