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
use generate::from_points;
use hull::hull;
use mesh::mesh;
use neighbors::neighbors;
use polygons::gen;
use triangles::triangles;
use urquhart::urquhart;

use d3_delaunay_rs::delaunay::Delaunay as DelaunayInner;

/// A Pair of indices pointing into a dataset identifying a edge.
type EdgeIndex = (usize, usize);

/// Three indices pointing into a dataset identifying a triangle.
type TriIndex = [usize; 3];

type UTransform<T> = Box<dyn Fn(&Vec<T>) -> Vec<bool>>;

/// Wraps data associated with a delaunay object.
pub struct Delaunay<T>
where
    T: CoordFloat,
{
    /// The underlying delaunay object
    pub delaunay: DelaunayInner<T>,
    /// The edges and triangles properties need RC because the values are close over in the urquhart function.
    pub edges: Rc<HashSet<EdgeIndex>>,
    /// A set of triangles as defined by set of indices.
    pub triangles: Rc<Vec<TriIndex>>,
    /// A list of centers associated with the cells.
    pub centers: Vec<Coord<T>>,
    /// Passes to Voronoi::polygon() where it is consumed.
    pub neighbors: Rc<HashMap<usize, Vec<usize>>>,
    /// A set pf polygons as defined by a set of indices.
    pub polygons: Vec<Vec<usize>>,
    /// The mesh as identified by a pair of indices.
    pub mesh: Vec<EdgeIndex>,
    /// The hull.
    pub hull: Vec<usize>,
    /// Urquhart graph .. by index the set the of points in the plane.
    pub urquhart: UTransform<T>,
    // /// Returns the indexes of the points.
    // pub find: FindReturn<'a, T>,
}

impl<T> Debug for Delaunay<T>
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

impl<T> Default for Delaunay<T>
where
    T: CoordFloat + FloatConst + FromPrimitive,
{
    fn default() -> Self {
        Self {
            delaunay: DelaunayInner::new(&[]),
            edges: Rc::new(HashSet::new()),
            triangles: Rc::new(vec![]),
            centers: vec![],
            neighbors: Rc::new(HashMap::new()),
            polygons: vec![],
            mesh: vec![],
            urquhart: Box::new(|_: &Vec<T>| vec![]),
            hull: vec![],
        }
    }
}
#[derive(Debug)]
pub struct NotEnoughPointsError {}

impl<T> TryFrom<&Vec<Coord<T>>> for Delaunay<T>
where
    T: 'static + CoordFloat + Default + FloatConst + FromPrimitive,
{
    type Error = NotEnoughPointsError;

    /// Creates a `GeoDelaunay` object from a set of points.
    fn try_from(points: &Vec<Coord<T>>) -> Result<Self, NotEnoughPointsError> {
        match from_points(points) {
            Some(delaunay) => {
                // RC is needed here as tri and e are both closed over in the urquhart function an is part of the Delaunay return.
                let tri = Rc::new(triangles(&delaunay));
                let e = Rc::new(edges(&tri, points));
                let circumcenters = circumcenters(&tri, points);
                let (polys, centers) = gen(circumcenters.collect(), tri.clone(), points);

                // RC is needed here as it is both closed over in the find function an is part of the Delaunay return.
                let neighbors = Rc::new(neighbors(&tri, points.len()));

                return Ok(Self {
                    delaunay,
                    edges: e.clone(),
                    centers,
                    hull: hull(&tri, points),
                    // find: find(n.clone(), points),
                    neighbors,
                    mesh: mesh(&polys),
                    polygons: polys,
                    urquhart: urquhart(e, tri.clone()),
                    triangles: tri,
                });
            }
            None => Err(NotEnoughPointsError {}),
        }
    }
}
