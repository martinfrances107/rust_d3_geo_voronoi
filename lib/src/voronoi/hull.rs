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

use d3_geo_rs::projection::projector_commom::types::ProjectorCircleResampleNoClip;
use d3_geo_rs::projection::stereographic::Stereographic;
use d3_geo_rs::stream::Stream;

use super::Voronoi;

type ProjectorSterographic<DRAIN, T> = ProjectorCircleResampleNoClip<DRAIN, Stereographic<T>, T>;

impl<'a, DRAIN, T> Voronoi<'a, ProjectorSterographic<DRAIN, T>, T>
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
        + NextAfter,
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
