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

use super::ConstructionError;
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
    /// Returns the hull for a given geometry for a given geometry object.
    ///
    /// # Errors
    ///
    /// Will return error if a Voronoi object could not be created
    /// from the input.
    ///
    /// For example if an insufficient number of point was supplied.
    pub fn hull_with_data(data: Geometry<T>) -> Result<Option<Polygon<T>>, ConstructionError> {
        let v = Self::try_from(data)?;
        Ok(v.hull())
    }

    /// Returns the hull for a given geometry.
    pub fn hull(self) -> Option<Polygon<T>> {
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
