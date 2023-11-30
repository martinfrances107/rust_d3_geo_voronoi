use core::fmt::Display;
use core::ops::AddAssign;

use approx::AbsDiffEq;
use float_next_after::NextAfter;
use geo::line_string;
use geo::CoordFloat;
use geo::Geometry;
use geo::HasKernel;
use geo::LineString;
use geo::MultiLineString;
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
    /// Returns a Multiline string associated with the input geometry.
    ///
    /// # Errors
    ///  The delaunay object must be valid when this function is called.
    pub fn cell_mesh_with_data(data: Geometry<T>) -> Result<MultiLineString<T>, ConstructionError> {
        let out = Self::try_from(data)?;
        Ok(out.cell_mesh())
    }

    /// Returns all the cells.
    /// # Panics
    ///   If polygons must have a least one value, when called.
    pub fn cell_mesh(self) -> MultiLineString<T> {
        let delaunay = self.delaunay;
        let polygons = delaunay.polygons;
        let centers = delaunay.centers;
        // Here can only supply an underestimate of the capacity
        // but if the number of polygons is large it will provide
        // some relief from constant reallocation.
        let mut coordinates: Vec<LineString<T>> = Vec::with_capacity(polygons.len());
        for p in polygons {
            //   TODO: remove panic and return a sensible default.
            let mut p0 = *p.last().unwrap();
            let mut p1 = p[0];
            for pi in p {
                if p1 > p0 {
                    coordinates.push(line_string![centers[p0], centers[p1]]);
                }
                p0 = p1;
                p1 = pi;
            }
        }

        MultiLineString(coordinates)
    }
}
