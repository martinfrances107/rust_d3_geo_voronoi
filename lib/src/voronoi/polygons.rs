use core::fmt::Display;
use core::ops::AddAssign;

use approx::AbsDiffEq;
use float_next_after::NextAfter;
use geo::CoordFloat;
use geo::GeoNum;
use geo::Geometry;
use geo::LineString;
use geo::Polygon;
use num_traits::AsPrimitive;
use num_traits::Bounded;
use num_traits::FloatConst;
use num_traits::FromPrimitive;
use num_traits::Signed;

use d3_geo_rs::data_object::FeatureCollection;
use d3_geo_rs::data_object::FeatureProperty;
use d3_geo_rs::data_object::Features;

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
        + GeoNum
        + Signed
        + NextAfter,
{
    /// Returns polygons in the form of a feature collection.
    ///
    /// # Errors
    ///
    /// Will return error if a Voronoi object could not be created
    /// from the input.
    ///
    /// For example if an insufficient number of point was supplied.
    pub fn polygons_from_data(
        data: Geometry<T>,
    ) -> Result<FeatureCollection<T>, ConstructionError> {
        let voronoi = Self::try_from(data)?;
        Ok(voronoi.polygons())
    }
    /// Returns polygons in the form of a feature collection.
    ///
    /// None when either
    /// the constructor fails, or
    /// the delaunay instance is None.
    pub fn polygons(&self) -> FeatureCollection<T> {
        // if let Some(data) = data {
        //     match Self::try_from(data) {
        //         Ok(s) => *self = s,
        //         Err(_) => return None,
        //     }
        // };

        if self.valid.is_empty() {
            return FeatureCollection(Vec::new());
        }

        let len = self.delaunay.polygons.len();
        let mut features: Vec<Features<T>> = Vec::with_capacity(len);
        for (i, poly) in self.delaunay.polygons.iter().enumerate() {
            let mut poly_closed: Vec<usize> = poly.clone();
            poly_closed.push(poly[0]);
            let exterior: LineString<T> = poly_closed
                .iter()
                .map(|&i| self.delaunay.centers[i])
                .collect();

            let geometry = Geometry::Polygon(Polygon::new(exterior, vec![]));
            let n = self.delaunay.neighbors.get(&i).unwrap_or(&vec![]).clone();
            let properties: Vec<FeatureProperty<T>> = vec![
                FeatureProperty::Site(self.valid[i]),
                FeatureProperty::Sitecoordinates(self.points[i]),
                FeatureProperty::Neighbors(n),
            ];
            let fs = Features {
                geometry: vec![geometry],
                properties,
            };
            features.push(fs);
        }
        FeatureCollection(features)
    }
}
