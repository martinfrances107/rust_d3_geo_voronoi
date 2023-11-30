use core::fmt::Display;
use core::ops::AddAssign;

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

use super::Voronoi;

impl<T> Voronoi<T>
where
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
            match Self::try_from(data) {
                Ok(s) => self = s,
                Err(_) => {
                    return None;
                }
            }
        }

        if self.delaunay.hull.is_empty() {
            None
        } else {
            let hull = &self.delaunay.hull;
            let mut coordinates: Vec<Coord<T>> =
                self.delaunay.hull.iter().map(|i| self.points[*i]).collect();
            coordinates.push(self.points[hull[0]]);
            Some(Polygon::new(coordinates.into(), vec![]))
        }
    }
}
