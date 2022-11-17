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
use std::fmt::Debug;
use std::ops::AddAssign;
use std::rc::Rc;

use approx::AbsDiffEq;
use derivative::Derivative;
use geo::CoordFloat;
use geo_types::Coord;
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
use rust_d3_geo::clip::circle::ClipCircleC;
use rust_d3_geo::clip::circle::ClipCircleU;
use rust_d3_geo::projection::builder::template::NoPCNC;
use rust_d3_geo::projection::builder::template::NoPCNU;
use rust_d3_geo::projection::builder::template::ResampleNoPCNC;
use rust_d3_geo::projection::builder::template::ResampleNoPCNU;
use rust_d3_geo::projection::stereographic::Stereographic;
use rust_d3_geo::stream::Stream;

type FindReturn<'a, T> = Box<dyn Fn(&Coord<T>, Option<usize>) -> Option<usize> + 'a>;

/// A Pair of indicies pointing into a dataset identifying a edge.
type EdgeIndex = [usize; 2];

/// Three indicies pointing into a dataset identifying a triangle.
type TriIndex = [usize; 3];

type UTransform<T> = Box<dyn Fn(&Vec<T>) -> Vec<bool>>;
/// Wraps data associated with a delaunay object.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct GeoDelaunay<'a, CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + FloatConst,
{
    /// The wrapped delaunay object.
    #[derivative(Debug = "ignore")]
    pub delaunay: Delaunay<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>,
    /// The edges and triangles properties need RC because the values are close over in the urquhart function.
    pub edges: Rc<HashSet<EdgeIndex>>,
    /// A set of triangles as defined by set of indicies.
    pub triangles: Rc<Vec<[usize; 3]>>,
    /// A list of centers associated with the cells.
    pub centers: Vec<Coord<T>>,
    /// Passes to Voronoi::polygon() where it is consumed.
    pub neighbors: Rc<RefCell<HashMap<usize, Vec<usize>>>>,
    /// A set pf polygons as defined by a set of indicies.
    pub polygons: Vec<Vec<usize>>,
    /// The mesh as identified by a pair of indicies.
    pub mesh: Vec<EdgeIndex>,
    /// The hull.
    pub hull: Vec<usize>,
    /// Urquhart graph .. by index the set the of points in the plane.
    #[derivative(Debug = "ignore")]
    pub urquhart: UTransform<T>,
    /// Returns the indexes of the points.
    #[derivative(Debug = "ignore")]
    pub find: FindReturn<'a, T>,
}

impl<'a, DRAIN, T>
    GeoDelaunay<
        'a,
        ClipCircleC<ResampleNoPCNC<DRAIN, Stereographic<DRAIN, T>, T>, T>,
        ClipCircleU<ResampleNoPCNC<DRAIN, Stereographic<DRAIN, T>, T>, T>,
        DRAIN,
        NoPCNU,
        Stereographic<DRAIN, T>,
        ResampleNoPCNC<DRAIN, Stereographic<DRAIN, T>, T>,
        ResampleNoPCNU<Stereographic<DRAIN, T>, T>,
        T,
    >
where
    DRAIN: Clone + Debug + Stream<EP = DRAIN, T = T> + Default,
    T: AbsDiffEq<Epsilon = T>
        + AddAssign
        + AsPrimitive<T>
        + CoordFloat
        + Default
        + FloatConst
        + FromPrimitive,
{
    /// Creates a `GeoDelaunay` object from a set of points.
    #[must_use]
    pub fn delaunay(points: Rc<Vec<Coord<T>>>) -> Option<Self> {
        let p = points.clone();
        match geo_delaunay_from::<
            DRAIN,
            NoPCNC<DRAIN>,
            NoPCNC<DRAIN>,
            ResampleNoPCNC<DRAIN, Stereographic<DRAIN, T>, T>,
            ResampleNoPCNU<Stereographic<DRAIN, T>, T>,
            T,
        >(&p)
        {
            Some(delaunay) => {
                // RC is needed here as tri and e are both closed over in the urquhart function an is part of the Delaunay return.
                let tri = Rc::new(geo_triangles(&delaunay));
                let e = Rc::new(geo_edges(&tri, &points));
                let circumcenters = geo_circumcenters(&tri, &points);
                let (polys, centers) =
                    GeoPolygons::default().gen(circumcenters.collect(), tri.clone(), &points);

                // RC is needed here as it is both closed over in the find function an is part of the Delaunay return.
                let n = Rc::new(RefCell::new(geo_neighbors(&tri, points.len())));

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
            None => None,
        }
    }
}
