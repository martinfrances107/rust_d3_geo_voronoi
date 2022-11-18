use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;

use approx::AbsDiffEq;
use float_next_after::NextAfter;
use geo::CoordFloat;
use geo::Geometry;
use geo::HasKernel;
use geo::LineString;
use geo::Polygon;
use num_traits::AsPrimitive;
use num_traits::Bounded;
use num_traits::FloatConst;
use num_traits::FromPrimitive;
use num_traits::Signed;

use rust_d3_geo::clip::circle::ClipCircleC;
use rust_d3_geo::clip::circle::ClipCircleU;
use rust_d3_geo::data_object::FeatureCollection;
use rust_d3_geo::data_object::FeatureProperty;
use rust_d3_geo::data_object::Features;
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
    /// Returns polygons in the form of a feature collection.
    ///
    /// None when either
    /// the constructor fails, or
    /// the delaunay instance is None.
    pub fn polygons(&mut self, data: Option<Geometry<T>>) -> Option<FeatureCollection<T>> {
        if let Some(data) = data {
            match Self::new(Some(data)) {
                Ok(s) => *self = s,
                Err(_) => return None,
            }
        };

        match &self.delaunay {
            None => None,
            Some(dr) => {
                if self.valid.is_empty() {
                    return Some(FeatureCollection(Vec::new()));
                }

                let mut features: Vec<Features<T>> = Vec::new();
                for (i, poly) in dr.polygons.iter().enumerate() {
                    let mut poly_closed: Vec<usize> = poly.clone();
                    poly_closed.push(poly[0]);
                    let exterior: LineString<T> =
                        poly_closed.iter().map(|&i| (dr.centers[i])).collect();

                    let geometry = Geometry::Polygon(Polygon::new(exterior, vec![]));
                    // TODO why does this need to be borrow_mut
                    let neighbors = dr.neighbors.borrow_mut();
                    let n = neighbors.get(&i).unwrap().clone();
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
                Some(FeatureCollection(features))
            }
        }
    }
}
