use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;

use approx::AbsDiffEq;
use float_next_after::NextAfter;
use geo::Coord;
use geo::CoordFloat;
use geo::Geometry;
use geo::HasKernel;
use geo::Polygon;
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

use super::Voronoi;

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
    /// Returns the hull for a given geometry.
    pub fn hull(mut self, data: Option<Geometry<T>>) -> Option<Polygon<T>> {
        if let Some(data) = data {
            match Self::new(Some(data)) {
                Ok(s) => self = s,
                Err(_) => {
                    return None;
                }
            }
        }

        match self.delaunay {
            None => None,
            Some(ref delaunay_return) => {
                if delaunay_return.hull.is_empty() {
                    None
                } else {
                    let hull = &delaunay_return.hull;
                    let mut coordinates: Vec<Coord<T>> =
                        hull.iter().map(|i| self.points[*i]).collect();
                    coordinates.push(self.points[hull[0]]);
                    Some(Polygon::new(coordinates.into(), vec![]))
                }
            }
        }
    }
}
