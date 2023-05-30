use core::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;

use approx::AbsDiffEq;
use d3_geo_rs::projection::projector_commom::types::ProjectorCircleResampleNoClip;
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

use d3_geo_rs::data_object::FeatureCollection;
use d3_geo_rs::data_object::FeatureProperty;
use d3_geo_rs::data_object::Features;
use d3_geo_rs::projection::stereographic::Stereographic;
use d3_geo_rs::stream::Stream;

use crate::delaunay::excess::excess;

use super::TriStruct;
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
    /// Returns a freature collection representing the triangularization of the input object.
    pub fn triangles(mut self, data: Option<Geometry<T>>) -> Option<FeatureCollection<T>> {
        if let Some(data) = data {
            match Self::new(Some(data)) {
                Ok(s) => {
                    self = s;
                }
                Err(_) => return None,
            }
        }

        match self.delaunay {
            None => None,

            Some(delaunay_return) => {
                let points = self.points.clone();
                let features: Vec<Features<T>> = delaunay_return
                    .triangles
                    .iter()
                    .enumerate()
                    .map(|(index, tri)| {
                        let tri_points: Vec<Coord<T>> = tri.iter().map(|i| (points[*i])).collect();
                        TriStruct {
                            tri_points,
                            center: (delaunay_return.centers[index]),
                        }
                    })
                    .filter(|tri_struct| excess(&tri_struct.tri_points) > T::zero())
                    .map(|tri_struct| {
                        let first = tri_struct.tri_points[0];
                        let mut coordinates: Vec<Coord<T>> = tri_struct.tri_points;
                        coordinates.push(first);
                        Features {
                            properties: vec![FeatureProperty::Circumecenter(tri_struct.center)],
                            geometry: vec![Geometry::Polygon(Polygon::new(
                                coordinates.into(),
                                vec![],
                            ))],
                        }
                    })
                    .collect();

                Some(FeatureCollection(features))
            }
        }
    }
}
