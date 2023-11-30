use float_next_after::NextAfter;
use geo::line_string;
use geo::CoordFloat;
use geo::Geometry;
use geo::HasKernel;
use geo::MultiLineString;
use num_traits::Bounded;
use num_traits::FloatConst;
use num_traits::FromPrimitive;
use num_traits::Signed;

use super::Voronoi;

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
    /// Returns the mesh in the form of a multi-line string.
    pub fn mesh(mut self, data: Option<Geometry<T>>) -> Option<MultiLineString<T>> {
        if let Some(data) = data {
            match Self::try_from(data) {
                Ok(s) => self = s,
                Err(_) => return None,
            }
        }

        Some(MultiLineString(
            self.delaunay
                .edges
                .iter()
                .map(|e| line_string![(self.points)[e.0], (self.points)[e.1]])
                .collect::<Vec<_>>(),
        ))
    }
}
