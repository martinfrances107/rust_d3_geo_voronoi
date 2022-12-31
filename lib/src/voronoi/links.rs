use std::borrow::Borrow;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;

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

use d3_geo_rs::clip::circle::ClipCircleC;
use d3_geo_rs::clip::circle::ClipCircleU;
use d3_geo_rs::data_object::FeatureCollection;
use d3_geo_rs::data_object::FeatureProperty;
use d3_geo_rs::data_object::Features;
use d3_geo_rs::distance::distance;
use d3_geo_rs::projection::builder::template::NoPCNU;
use d3_geo_rs::projection::builder::template::ResampleNoPCNC;
use d3_geo_rs::projection::builder::template::ResampleNoPCNU;
use d3_geo_rs::projection::stereographic::Stereographic;
use d3_geo_rs::stream::Stream;

use super::Voronoi;

impl<'a, DRAIN, T>
    Voronoi<
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
                    .map(|e| distance(&points[e[0]], &points[e[1]]))
                    .collect();
                let urquhart = (delaunay_return.urquhart)(&distances);
                let features: Vec<Features<T>> = delaunay_return
                    .edges
                    .iter()
                    .enumerate()
                    .map(|(i, e)| {
                        let ls: LineString<T> = vec![points[0], points[e[1]]].into();
                        Features {
                            properties: vec![
                                FeatureProperty::Source(self.valid[e[0]]),
                                FeatureProperty::Target(self.valid[e[1]]),
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
