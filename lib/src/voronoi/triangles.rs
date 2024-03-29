use core::fmt::Display;
use core::ops::AddAssign;

use approx::AbsDiffEq;
use float_next_after::NextAfter;
use geo::Coord;
use geo::CoordFloat;
use geo::GeoNum;
use geo::Geometry;
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

use super::ConstructionError;
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
        + GeoNum
        + Signed
        + NextAfter,
{
    /// Returns a feature collection representing the triangularization of the input object.
    ///
    /// # Errors
    ///
    /// Will return error if a Voronoi object could not be created
    /// from the input.
    ///
    /// For example if an insufficient number of point was supplied.
    pub fn triangles_with_data(
        data: Geometry<T>,
    ) -> Result<FeatureCollection<T>, ConstructionError> {
        let voronoi = Self::try_from(data)?;
        Ok(voronoi.triangles())
    }
    /// Returns a feature collection representing the triangularization of the input object.
    pub fn triangles(&self) -> FeatureCollection<T> {
        let points = self.points.clone();
        let features: Vec<Features<T>> = self
            .delaunay
            .triangles
            .iter()
            .enumerate()
            .map(|(index, tri)| TriStruct {
                tri_points: [points[tri[0]], points[tri[1]], points[tri[2]]],
                center: (self.delaunay.centers[index]),
            })
            .filter(|tri_struct| excess(&tri_struct.tri_points) > T::zero())
            .map(|tri_struct| {
                let first = tri_struct.tri_points[0];
                let mut coordinates: Vec<Coord<T>> =
                    tri_struct.tri_points.into();
                coordinates.push(first);
                Features {
                    properties: vec![FeatureProperty::Circumecenter(
                        tri_struct.center,
                    )],
                    geometry: vec![Geometry::Polygon(Polygon::new(
                        coordinates.into(),
                        vec![],
                    ))],
                }
            })
            .collect();

        FeatureCollection(features)
    }
}
