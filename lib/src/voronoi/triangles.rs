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

use d3_geo_rs::data_object::FeatureCollection;
use d3_geo_rs::data_object::FeatureProperty;
use d3_geo_rs::data_object::Features;

use crate::delaunay::excess::excess;

use super::Voronoi;

#[derive(Debug)]
struct TriStruct<T>
where
    T: CoordFloat,
{
    tri_points: [Coord<T>; 3],
    center: Coord<T>,
}

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
    /// Returns a feature collection representing the triangularization of the input object.
    pub fn triangles(mut self, data: Option<Geometry<T>>) -> Option<FeatureCollection<T>> {
        if let Some(data) = data {
            match Self::try_from(data) {
                Ok(s) => {
                    self = s;
                }
                Err(_) => return None,
            }
        }

        match self.delaunay {
            None => None,

            Some(delaunay_return) => {
                let points = self.points;
                let features: Vec<Features<T>> = delaunay_return
                    .triangles
                    .iter()
                    .enumerate()
                    .map(|(index, tri)| TriStruct {
                        tri_points: [points[tri[0]], points[tri[1]], points[tri[2]]],
                        center: (delaunay_return.centers[index]),
                    })
                    .filter(|tri_struct| excess(&tri_struct.tri_points) > T::zero())
                    .map(|tri_struct| {
                        let first = tri_struct.tri_points[0];
                        let mut coordinates: Vec<Coord<T>> = tri_struct.tri_points.into();
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
