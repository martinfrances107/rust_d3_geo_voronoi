use core::fmt::Debug;
use core::fmt::Display;
use core::ops::AddAssign;

use approx::AbsDiffEq;
use float_next_after::NextAfter;
use geo::Coord;
use geo::CoordFloat;
use geo::HasKernel;
use num_traits::AsPrimitive;
use num_traits::Bounded;
use num_traits::FloatConst;
use num_traits::FromPrimitive;
use num_traits::Signed;

use d3_geo_rs::distance::distance;
use d3_geo_rs::projection::projector_commom::types::ProjectorCircleResampleNoClip;
use d3_geo_rs::projection::stereographic::Stereographic;
use d3_geo_rs::stream::Stream;

use super::Voronoi;

type ProjectorSterographic<DRAIN, T> = ProjectorCircleResampleNoClip<DRAIN, Stereographic<T>, T>;

impl<DRAIN, T> Voronoi<ProjectorSterographic<DRAIN, T>, T>
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
    /// Returns the index associated with the given point.
    pub fn find(&mut self, p: &Coord<T>, radius: Option<T>) -> Option<usize> {
        match &self.delaunay {
            None => None,
            Some(delaunay_return) => {
                self.found = delaunay_return.find(p, self.found);
                match radius {
                    Some(radius) => match self.found {
                        Some(found) => {
                            if distance(p, &self.points[found]) < radius {
                                Some(found)
                            } else {
                                None
                            }
                        }
                        None => None,
                    },
                    None => self.found,
                }
            }
        }
    }
}
