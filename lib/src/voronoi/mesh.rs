use core::fmt::Display;
use core::ops::AddAssign;

use approx::AbsDiffEq;
use float_next_after::NextAfter;
use geo::line_string;
use geo::CoordFloat;
use geo::Geometry;
use geo::HasKernel;
use geo::MultiLineString;
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
    /// Returns the mesh in the form of a mutli-line string.
    pub fn mesh(mut self, data: Option<Geometry<T>>) -> Option<MultiLineString<T>> {
        if let Some(data) = data {
            match Self::new(Some(data)) {
                Ok(s) => self = s,
                Err(_) => return None,
            }
        }

        self.delaunay.as_ref().map(|delaunay_return| {
            delaunay_return
                .edges
                .iter()
                .map(|e| line_string![(self.points)[e[0]], (self.points)[e[1]]])
                .collect()
        })
    }
}
