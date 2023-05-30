use core::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;

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
    /// Returns a Multiline string assoicated with the input geometry.
    ///
    /// # Panics
    ///  The delauanay object must be valid when this function is called.
    pub fn cell_mesh(mut self, data: Option<Geometry<T>>) -> Option<MultiLineString<T>> {
        if let Some(data) = data {
            match Self::new(Some(data)) {
                Ok(s) => self = s,
                Err(_) => {
                    return None;
                }
            }
        }

        // Return early maybe?
        self.delaunay.as_ref()?;

        let delaunay = self.delaunay.unwrap();
        let polygons = delaunay.polygons;
        let centers = delaunay.centers;
        // Here can only supply an underestimate of the capacity
        // but if the number of polygons is large it will provide
        // some relief from constant rellocation.
        let mut coordinates: Vec<LineString<T>> = Vec::with_capacity(polygons.len());
        for p in polygons {
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

        Some(MultiLineString(coordinates))
    }
}
