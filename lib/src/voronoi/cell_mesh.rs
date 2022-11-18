use std::fmt::Debug;
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

use rust_d3_geo::clip::circle::ClipCircleC;
use rust_d3_geo::clip::circle::ClipCircleU;
use rust_d3_geo::projection::builder::template::NoPCNU;
use rust_d3_geo::projection::builder::template::ResampleNoPCNC;
use rust_d3_geo::projection::builder::template::ResampleNoPCNU;
use rust_d3_geo::projection::stereographic::Stereographic;
use rust_d3_geo::stream::Stream;

use super::GeoVoronoi;

impl<'a, DRAIN, T>
    GeoVoronoi<
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
