use core::fmt::Debug;
use core::fmt::Display;
use std::rc::Rc;

use float_next_after::NextAfter;
use geo::centroid::Centroid;
use geo::kernels::HasKernel;
use geo::CoordFloat;
use geo::Geometry;
use geo::Point;
use geo_types::Coord;
use num_traits::Bounded;
use num_traits::FloatConst;
use num_traits::FromPrimitive;
use num_traits::Signed;

use super::delaunay::Delaunay;

mod cell_mesh;
mod find;
mod hull;
mod links;
mod mesh;
mod polygons;
mod triangles;

/// Return type used by .x() and .y()
#[allow(missing_debug_implementations)]
pub enum XYReturn<T>
where
    T: CoordFloat,
{
    /// Voronoi
    Voronoi(Voronoi<T>),
    /// Function
    Func(VTransform<T>),
}

type XYReturnDefault<T> = XYReturn<T>;

#[derive(Debug)]
struct TriStruct<T>
where
    T: CoordFloat,
{
    tri_points: Vec<Coord<T>>,
    center: Coord<T>,
}

/// Velocity Transform.
pub type VTransform<T> = Box<dyn Fn(&dyn Centroid<Output = Point<T>>) -> T>;

/// Holds data centered on a [`Delaunay`] instance.
pub struct Voronoi<T>
where
    T: CoordFloat,
{
    /// The wrapped GeoDelaunay instance.
    #[allow(clippy::type_complexity)]
    pub delaunay: Option<Delaunay<T>>,
    data: Option<Geometry<T>>,
    found: Option<usize>,
    //Points: Rc needed here as the edges, triangles, neighbors etc all index into that vec.
    points: Rc<Vec<Coord<T>>>,
    valid: Vec<Coord<T>>,
    // Option<Box<impl Fn(&dyn Centroid<Output = Coord<T>>) -> T>>
    vx: VTransform<T>,
    vy: VTransform<T>,
}

impl<T> Debug for Voronoi<T>
where
    T: CoordFloat + Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Centroid<T>")
            .field(&self.delaunay)
            .field(&self.data)
            .field(&self.found)
            .field(&self.points)
            .field(&self.valid)
            .finish()
    }
}

impl<T> Default for Voronoi<T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Self {
            data: None,
            delaunay: None,
            found: None,
            points: Rc::new(Vec::new()),
            valid: Vec::new(),
            vx: Box::new(|d: &dyn Centroid<Output = Point<T>>| d.centroid().x()),
            vy: Box::new(|d: &dyn Centroid<Output = Point<T>>| d.centroid().y()),
        }
    }
}

/// Geo-Voronoi construction error.
///
/// Unexpected Geometry input.
#[derive(Debug, Clone)]
pub struct ConstructionError;

impl core::fmt::Display for ConstructionError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Must implement Voronoi::new for other DataObject<T> types"
        )
    }
}

impl<T> Voronoi<T>
where
    T: 'static
        + Bounded
        + CoordFloat
        + Default
        + FloatConst
        + FromPrimitive
        + HasKernel
        + NextAfter
        + Signed,
{
    /// If the input is a collection we act only on the first element in the collection.
    /// by copying over the data into a new single element before proceeding.
    ///
    /// # Errors
    ///  A `Geometry::Multipoint` object must be input.
    pub fn new(data: Option<Geometry<T>>) -> Result<Self, ConstructionError> {
        // let delaunay_return: Option<GeoDelaunay> = None;

        // On finding a Features Collection take the first element only, drop other elements.
        // match data {
        //     DataObject<T>::FeatureCollection { mut features } => {
        //         // TODO: .remove() panics it it can't complete - consider trapping.
        //         let mut first_feature = features.remove(0);
        //         let geometry = first_feature.geometry.remove(0);
        //         let feature = FeatureStruct {
        //             properties: Vec::new(),
        //             geometry,
        //         };
        //         data = DataObject<T>::Feature { feature };
        //     }
        //     _ => {
        //         // Other DataTypes variants.
        //     }
        // };

        let mut v = Self {
            data,
            ..Self::default()
        };

        // Data sanitization:-
        // Transform points using vx() and vy().
        // Remove infinities, store list of untransformed - valid points.

        match v.data {
            // Some(FeatureCollection { features: f }) => {
            //     f.iter()
            //         .map(|d| ((v.vx)(d), (v.vy)(d), d.clone()))
            //         .filter(|t| (t.0 + t.1).is_finite());
            // }
            Some(Geometry::MultiPoint(ref data)) => {
                let temp: Vec<(T, T, Point<T>)> = data
                    .iter()
                    .map(|d| ((Self::default().vx)(d), (Self::default().vy)(d), *d))
                    .filter(|(d0, d1, _)| (*d0 + *d1).is_finite())
                    .collect();
                let points: Vec<Coord<T>> = temp
                    .iter()
                    .map(|(d0, d1, _)| Coord { x: *d0, y: *d1 })
                    .collect();
                v.points = points.clone().into();
                v.valid = temp
                    .iter()
                    .map(|d| Coord {
                        x: d.2.x(),
                        y: d.2.y(),
                    })
                    .collect();
                v.delaunay = Delaunay::new(points);
            }
            None => {
                v = Self::default();
            }
            _ => return Err(ConstructionError),
        }

        Ok(v)
    }

    /// Sets the y() override function.
    pub fn x(
        mut self,
        f: Option<Box<impl Fn(&dyn Centroid<Output = Point<T>>) -> T + 'static>>,
    ) -> XYReturnDefault<T> {
        match f {
            None => XYReturn::Func(self.vx),
            Some(f) => {
                self.vx = f;
                XYReturn::Voronoi(self)
            }
        }
    }

    /// Sets the y() override function.
    pub fn y(
        mut self,
        f: Option<Box<impl Fn(&dyn Centroid<Output = Point<T>>) -> T + 'static>>,
    ) -> XYReturnDefault<T> {
        match f {
            None => XYReturn::Func(self.vy),
            Some(f) => {
                self.vy = f;
                XYReturn::Voronoi(self)
            }
        }
    }
}
