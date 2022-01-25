#![allow(clippy::many_single_char_names)]
mod cartesian;

/// A helper function.
pub mod excess;
mod geo_circumcenters;
/// Helper function.
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

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::AddAssign;
use std::rc::Rc;

use approx::AbsDiffEq;
use derivative::*;
use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;
use num_traits::FromPrimitive;

use geo_circumcenters::geo_circumcenters;
use geo_delaunay_from::geo_delaunay_from;
use geo_edges::geo_edges;
use geo_find::geo_find;
use geo_hull::geo_hull;
use geo_mesh::geo_mesh;
use geo_neighbors::geo_neighbors;
use geo_polygons::GeoPolygons;
use geo_triangles::geo_triangles;
use geo_urquhart::geo_urquhart;

use rust_d3_delaunay::delaunay::Delaunay;
use rust_d3_geo::clip::buffer::Buffer;
use rust_d3_geo::clip::circle::line::Line as LineCircle;
use rust_d3_geo::clip::circle::pv::PV;
use rust_d3_geo::clip::post_clip_node::PostClipNode;
use rust_d3_geo::clip::Line;
use rust_d3_geo::projection::resample::ResampleNode;
use rust_d3_geo::projection::stereographic::Stereographic;
use rust_d3_geo::projection::stream_node::StreamNode;
use rust_d3_geo::stream::Stream;

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

/// Wraps data associated with a delaunay object.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct GeoDelaunay<'a, DRAIN, LINE, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    LINE: Line,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + FloatConst,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    StreamNode<
        DRAIN,
        LINE,
        ResampleNode<DRAIN, Stereographic<DRAIN, T>, PostClipNode<DRAIN, DRAIN, T>, T>,
        T,
    >: Stream<EP = DRAIN, T = T>,
{
    /// The wrapped delaunay object.
    #[derivative(Debug = "ignore")]
    pub delaunay: Delaunay<DRAIN, LINE, Stereographic<DRAIN, T>, PV<T>, T>,
    /// The edges and triangles properties need RC because the values are close over in the urquhart function.
    pub edges: Rc<HashSet<[usize; 2]>>,
    /// A set of triangles as defined by set of indicies.
    pub triangles: Rc<Vec<[usize; 3]>>,
    /// A list of centers associated with the cells.
    pub centers: Vec<Coordinate<T>>,
    /// Passes to Voronoi::polygon() where it is consumed.
    pub neighbors: Rc<RefCell<HashMap<usize, Vec<usize>>>>,
    /// A set pf polygons as defined by a set of indicies.
    pub polygons: Vec<Vec<usize>>,
    /// The mesh as identified by a pair of indicies.
    pub mesh: Vec<[usize; 2]>,
    /// The hull.
    pub hull: Vec<usize>,
    /// Urquhart graph .. by index the set the of points in the plane.
    #[derivative(Debug = "ignore")]
    pub urquhart: Box<dyn Fn(&Vec<T>) -> Vec<bool> + 'a>,
    /// Returns the indexes of the points.
    #[derivative(Debug = "ignore")]
    pub find: Box<dyn Fn(&Coordinate<T>, Option<usize>) -> Option<usize> + 'a>,
}

impl<'a, DRAIN, T> GeoDelaunay<'a, DRAIN, LineCircle<T>, T>
where
    DRAIN: Stream<EP = DRAIN, T = T> + Default,
    T: AbsDiffEq<Epsilon = T>
        + AddAssign
        + AsPrimitive<T>
        + CoordFloat
        + FloatConst
        + FromPrimitive,
    StreamNode<Buffer<T>, LineCircle<T>, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    StreamNode<
        DRAIN,
        LineCircle<T>,
        ResampleNode<DRAIN, Stereographic<DRAIN, T>, PostClipNode<DRAIN, DRAIN, T>, T>,
        T,
    >: Stream<EP = DRAIN, T = T>,
{
    /// Creates a GeoDelaunay object from a set of points.
    pub fn delaunay(
        points: Rc<Vec<Coordinate<T>>>,
    ) -> Option<GeoDelaunay<'a, DRAIN, LineCircle<T>, T>> {
        let p = points.clone();
        match geo_delaunay_from(p) {
            Some(delaunay) => {
                // RC is needed here as tri and e are both closed over in the urquhart function an is part of the Delaunay return.
                let tri = Rc::new(geo_triangles(&delaunay));
                let e = Rc::new(geo_edges(&tri, &points));
                let circumcenters = geo_circumcenters(&tri, &points);
                let (polys, centers) =
                    GeoPolygons::default().gen(circumcenters.collect(), tri.clone(), &points);

                // RC is needed here as it is both closed over in the find function an is part of the Delaunay return.
                let n = Rc::new(RefCell::new(geo_neighbors(tri.clone(), points.len())));

                return Some(Self {
                    delaunay,
                    edges: e.clone(),
                    centers,
                    hull: geo_hull(&tri, &points),
                    find: geo_find(n.clone(), points),
                    neighbors: n,
                    mesh: geo_mesh(&polys),
                    polygons: polys,
                    urquhart: geo_urquhart(e, tri.clone()),
                    triangles: tri,
                });
            }
            None => {
                panic!("return none");
            }
        }
    }
}
