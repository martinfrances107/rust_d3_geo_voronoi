use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

use approx::AbsDiffEq;
use derivative::Derivative;
use float_next_after::NextAfter;
use geo::centroid::Centroid;
use geo::kernels::HasKernel;
use geo::CoordFloat;
use geo::Geometry;
use geo::Point;
use geo_types::Coord;
use num_traits::AsPrimitive;
use num_traits::Bounded;
use num_traits::FloatConst;
use num_traits::FromPrimitive;
use num_traits::Signed;

use rust_d3_geo::clip::circle::ClipCircleC;
use rust_d3_geo::clip::circle::ClipCircleU;
use rust_d3_geo::projection::builder::template::NoPCNU;
use rust_d3_geo::projection::builder::template::ResampleNoPCNC;
use rust_d3_geo::projection::builder::template::ResampleNoPCNU;
use rust_d3_geo::projection::stereographic::Stereographic;
use rust_d3_geo::stream::Stream;

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
pub enum XYReturn<'a, CLIPC, CLIPU, DRAIN, PCNU, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + Display + CoordFloat + FloatConst,
{
    /// Voronoi
    Voronoi(Voronoi<'a, CLIPC, CLIPU, DRAIN, PCNU, Stereographic<DRAIN, T>, RC, RU, T>),
    /// Function
    Func(VTransform<T>),
}

type XYReturnDefault<'a, DRAIN, T> = XYReturn<
    'a,
    ClipCircleC<ResampleNoPCNC<DRAIN, Stereographic<DRAIN, T>, T>, T>,
    ClipCircleU<ResampleNoPCNC<DRAIN, Stereographic<DRAIN, T>, T>, T>,
    DRAIN,
    NoPCNU,
    ResampleNoPCNC<DRAIN, Stereographic<DRAIN, T>, T>,
    ResampleNoPCNU<Stereographic<DRAIN, T>, T>,
    T,
>;

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

#[derive(Derivative)]
#[derivative(Debug)]
/// Holds data centered on a `GeoDelauany` instance.
pub struct Voronoi<'a, CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    /// The wrapped GeoDelaunay instance.
    #[allow(clippy::type_complexity)]
    pub delaunay: Option<Delaunay<'a, CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>>,
    data: Option<Geometry<T>>,
    found: Option<usize>,
    //Points: Rc needed here as the egdes, triangles, neigbours etc all index into thts vec.
    points: Rc<Vec<Coord<T>>>,
    valid: Vec<Coord<T>>,
    // Option<Box<impl Fn(&dyn Centroid<Output = Coord<T>>) -> T>>
    #[derivative(Debug = "ignore")]
    vx: VTransform<T>,
    #[derivative(Debug = "ignore")]
    vy: VTransform<T>,
}

impl<'a, CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T> Default
    for Voronoi<'a, CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    fn default() -> Self {
        Voronoi {
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

/// Geovoronoi construction error.
///
/// Unexpected Geometry input.
#[derive(Debug, Clone)]
pub struct ConstructionError;

impl std::fmt::Display for ConstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Must implement Voronoi::new for other DataObject<T> types"
        )
    }
}

impl<'a, DRAIN, T>
    Voronoi<
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
        + Bounded
        + CoordFloat
        + Display
        + Default
        + FloatConst
        + FromPrimitive
        + HasKernel
        + Signed
        + NextAfter<T>,
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

        let mut v = Voronoi {
            data,
            ..Voronoi::default()
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
                v.points = Rc::new(points);
                v.valid = temp
                    .iter()
                    .map(|d| Coord {
                        x: d.2.x(),
                        y: d.2.y(),
                    })
                    .collect();
                v.delaunay = Delaunay::new(v.points.clone());
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
    ) -> XYReturnDefault<'a, DRAIN, T> {
        return match f {
            None => XYReturn::Func(self.vx),
            Some(f) => {
                self.vx = f;
                return XYReturn::Voronoi(self);
            }
        };
    }

    /// Sets the y() override function.
    pub fn y(
        mut self,
        f: Option<Box<impl Fn(&dyn Centroid<Output = Point<T>>) -> T + 'static>>,
    ) -> XYReturnDefault<'a, DRAIN, T> {
        return match f {
            None => XYReturn::Func(self.vy),
            Some(f) => {
                self.vy = f;
                return XYReturn::Voronoi(self);
            }
        };
    }
}
