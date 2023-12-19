use core::borrow::Borrow;
use core::fmt::Display;
use core::ops::AddAssign;

use approx::AbsDiffEq;
use float_next_after::NextAfter;
use geo::Coord;
use geo::CoordFloat;
use geo::Geometry;
use geo::HasKernel;
use geo::LineString;
use num_traits::AsPrimitive;
use num_traits::Bounded;
use num_traits::FloatConst;
use num_traits::FromPrimitive;
use num_traits::Signed;

use d3_geo_rs::data_object::FeatureCollection;
use d3_geo_rs::data_object::FeatureProperty;
use d3_geo_rs::data_object::Features;
use d3_geo_rs::distance::distance;

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
    /// Given a `GeometryObject` return an annotated Feature collection labelled with distance urquhart etc.
    ///
    /// # Errors
    ///
    /// Will return error if a Voronoi object could not be created
    /// from the input.
    ///
    /// For example if an insufficient number of point was supplied.
    pub fn links_with_data(data: Geometry<T>) -> Result<FeatureCollection<T>, ConstructionError> {
        let voronoi = Self::try_from(data)?;
        let links = voronoi.links();
        Ok(links)
    }

    /// Returns an annotated Feature collection labelled with distance urquhart etc.
    pub fn links(&self) -> FeatureCollection<T> {
        // if let Some(data) = data {
        //     match Self::try_from(data) {
        //         Ok(s) => *self = s,
        //         Err(_) => return None,
        //     }
        // }

        let points: &Vec<Coord<T>> = self.points.borrow();
        let distances: Vec<T> = self
            .delaunay
            .edges
            .iter()
            .map(|e| distance(&points[e.0], &points[e.1]))
            .collect();
        let urquhart = (self.delaunay.urquhart)(&distances);
        let features: Vec<Features<T>> = self
            .delaunay
            .edges
            .iter()
            .enumerate()
            .map(|(i, e)| {
                let ls: LineString<T> = vec![points[0], points[e.1]].into();
                Features {
                    properties: vec![
                        FeatureProperty::Source(self.valid[e.0]),
                        FeatureProperty::Target(self.valid[e.1]),
                        FeatureProperty::Length(distances[i]),
                        FeatureProperty::Urquhart(urquhart[i]),
                    ],
                    geometry: vec![Geometry::LineString(ls)],
                }
            })
            .collect();
        FeatureCollection(features)
    }
}
