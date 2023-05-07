#![allow(clippy::many_single_char_names)]
mod cartesian;

mod circumcenters;
mod edges;
/// A helper function.
pub mod excess;
mod find;
/// Helper function.
pub mod generate;
mod hull;
mod mesh;
mod neighbors;
mod o_midpoint;
mod polygons;
mod triangles;
mod urquhart;

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

use circumcenters::circumcenters;
use edges::edges;
use find::find;
use generate::from_points;
use hull::hull;
use mesh::mesh;
use neighbors::neighbors;
use polygons::Polygons;
use triangles::triangles;
use urquhart::urquhart;

use d3_delaunay_rs::delaunay::Delaunay as DelaunayInner;
use d3_geo_rs::clip::circle::ClipCircleC;
use d3_geo_rs::clip::circle::ClipCircleU;
use d3_geo_rs::projection::builder::template::NoPCNC;
use d3_geo_rs::projection::builder::template::NoPCNU;
use d3_geo_rs::projection::builder::template::ResampleNoPCNC;
use d3_geo_rs::projection::builder::template::ResampleNoPCNU;
use d3_geo_rs::projection::stereographic::Stereographic;
use d3_geo_rs::stream::Stream;

type FindReturn<'a, T> = Box<dyn Fn(&Coord<T>, Option<usize>) -> Option<usize> + 'a>;

/// A Pair of indicies pointing into a dataset identifying a edge.
type EdgeIndex = [usize; 2];

/// Three indicies pointing into a dataset identifying a triangle.
type TriIndex = [usize; 3];

type UTransform<T> = Box<dyn Fn(&Vec<T>) -> Vec<bool>>;
/// Wraps data associated with a delaunay object.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Delaunay<'a, CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + FloatConst,
{
    /// The wrapped delaunay object.
    #[derivative(Debug = "ignore")]
    pub delaunay: DelaunayInner<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T>,
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
    Delaunay<
        'a,
        ClipCircleC<ResampleNoPCNC<DRAIN, Stereographic<DRAIN, T>, T>, T>,
        ClipCircleU<ResampleNoPCNC<DRAIN, Stereographic<DRAIN, T>, T>, T>,
        DRAIN,
        NoPCNU,
        Stereographic<DRAIN, T>,
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
    pub fn new(points: Rc<Vec<Coord<T>>>) -> Option<Self> {
        let p = points.clone();
        match from_points::<
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
                let tri = Rc::new(triangles(&delaunay));
                let e = Rc::new(edges(&tri, &points));
                let circumcenters = circumcenters(&tri, &points);
                let (polys, centers) =
                    Polygons::default().gen(circumcenters.collect(), tri.clone(), &points);

                // RC is needed here as it is both closed over in the find function an is part of the Delaunay return.
                let n = Rc::new(RefCell::new(neighbors(&tri, points.len())));

                return Some(Self {
                    delaunay,
                    edges: e.clone(),
                    centers,
                    hull: hull(&tri, &points),
                    find: find(n.clone(), points),
                    neighbors: n,
                    mesh: mesh(&polys),
                    polygons: polys,
                    urquhart: urquhart(e, tri.clone()),
                    triangles: tri,
                });
            }
            None => None,
        }
    }
}
