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

use core::cell::RefCell;
use core::fmt::Debug;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;
use num_traits::FromPrimitive;

use circumcenters::circumcenters;
use edges::edges;
// use find::find;
use generate::from_points;
use hull::hull;
use mesh::mesh;
use neighbors::neighbors;
use polygons::Polygons;
use triangles::triangles;
use urquhart::urquhart;

use d3_geo_rs::projection::builder::template::NoPCNC;
use d3_geo_rs::projection::builder::template::ResampleNoPCNC;
use d3_geo_rs::projection::builder::template::ResampleNoPCNU;
use d3_geo_rs::projection::projector_commom::types::ProjectorCircleResampleNoClip;
use d3_geo_rs::projection::stereographic::Stereographic;
use d3_geo_rs::stream::Stream;

use d3_delaunay_rs::delaunay::Delaunay as DelaunayInner;

/// A Pair of indicies pointing into a dataset identifying a edge.
type EdgeIndex = [usize; 2];

/// Three indicies pointing into a dataset identifying a triangle.
type TriIndex = [usize; 3];

type UTransform<T> = Box<dyn Fn(&Vec<T>) -> Vec<bool>>;

/// Wraps data associated with a delaunay object.
pub struct Delaunay<PROJECTOR, T>
where
    T: CoordFloat,
{
    pub delaunay: DelaunayInner<PROJECTOR, T>,
    /// The edges and triangles properties need RC because the values are close over in the urquhart function.
    pub edges: Rc<HashSet<EdgeIndex>>,
    /// A set of triangles as defined by set of indicies.
    pub triangles: Rc<Vec<TriIndex>>,
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
    pub urquhart: UTransform<T>,
    // /// Returns the indexes of the points.
    // pub find: FindReturn<'a, T>,
    points: Vec<Coord<T>>,
}

impl<PROJECTOR, T> Debug for Delaunay<PROJECTOR, T>
where
    T: CoordFloat,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Delaunay")
            .field(&self.edges)
            .field(&self.triangles)
            .field(&self.centers)
            .field(&self.neighbors)
            .field(&self.polygons)
            .field(&self.mesh)
            .field(&self.hull)
            .finish()
    }
}

type ProjectorSterographic<DRAIN, T> = ProjectorCircleResampleNoClip<DRAIN, Stereographic<T>, T>;

impl<DRAIN, T> Delaunay<ProjectorSterographic<DRAIN, T>, T>
where
    DRAIN: Clone + Debug + Stream<EP = DRAIN, T = T> + Default,
    T: 'static + CoordFloat + Default + FloatConst + FromPrimitive,
{
    /// Creates a `GeoDelaunay` object from a set of points.
    #[must_use]
    pub fn new(points: Vec<Coord<T>>) -> Option<Self> {
        let p = points.clone();
        match from_points::<
            DRAIN,
            NoPCNC<DRAIN>,
            NoPCNC<DRAIN>,
            ResampleNoPCNC<DRAIN, Stereographic<T>, T>,
            ResampleNoPCNU<Stereographic<T>, T>,
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
                    // find: find(n.clone(), points),
                    neighbors: n,
                    mesh: mesh(&polys),
                    polygons: polys,
                    urquhart: urquhart(e, tri.clone()),
                    triangles: tri,
                    points,
                });
            }
            None => None,
        }
    }
}
