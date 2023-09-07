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
    /// Returns an annotated Feature collection labelled with distance urquhart etc.
    pub fn links(&mut self, data: Option<Geometry<T>>) -> Option<FeatureCollection<T>> {
        if let Some(data) = data {
            match Self::new(Some(data)) {
                Ok(s) => *self = s,
                Err(_) => return None,
            }
        }

        return match &self.delaunay {
            None => None,
            Some(delaunay_return) => {
                let points: &Vec<Coord<T>> = self.points.borrow();
                let distances: Vec<T> = delaunay_return
                    .edges
                    .iter()
                    .map(|e| distance(&points[e.0], &points[e.1]))
                    .collect();
                let urquhart = (delaunay_return.urquhart)(&distances);
                let features: Vec<Features<T>> = delaunay_return
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
                return Some(FeatureCollection(features));
            }
        };
    }
}
