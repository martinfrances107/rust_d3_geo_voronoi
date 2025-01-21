use float_next_after::NextAfter;
use geo::line_string;
use geo::CoordFloat;
use geo::GeoNum;
use geo::Geometry;
use geo::MultiLineString;
use num_traits::Bounded;
use num_traits::FloatConst;
use num_traits::FromPrimitive;
use num_traits::Signed;

use super::ConstructionError;
use super::Voronoi;

impl<T> Voronoi<T>
where
    T: 'static
        + Bounded
        + CoordFloat
        + Default
        + FloatConst
        + FromPrimitive
        + GeoNum
        + NextAfter
        + Signed,
{
    /// Return a mesh from the supplied geometry.
    ///
    /// # Errors
    ///
    /// Will return error if a Voronoi object could not be created
    /// from the input.
    ///
    /// For example if an insufficient number of point was supplied.
    pub fn mesh_from_data(
        data: Geometry<T>,
    ) -> Result<MultiLineString<T>, ConstructionError> {
        let voronoi = Self::try_from(data)?;
        Ok(voronoi.mesh())
    }
    /// Returns the mesh in the form of a multi-line string.
    pub fn mesh(self) -> MultiLineString<T> {
        self.delaunay
            .edges
            .iter()
            .map(|e| line_string![(self.points)[e.0], (self.points)[e.1]])
            .collect::<MultiLineString<T>>()
    }
}
