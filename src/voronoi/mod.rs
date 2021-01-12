use std::rc::Rc;
use std::{borrow::Borrow, collections::HashMap};

use geo::centroid::Centroid;
use geo::prelude::*;
use geo::Coordinate;
use geo::CoordinateType;
use geo::Geometry;
use geo::GeometryCollection;
use geo::LineString;
use geo::MultiLineString;
use geo::Point;
use geo::Polygon;
use geo::{line_string, MultiPoint};
use num_traits::{AsPrimitive, Float, FloatConst, FromPrimitive};
use rust_d3_geo::data_object::feature_collection::FeatureCollection;
use rust_d3_geo::data_object::feature_geometry::FeatureGeometry;
use rust_d3_geo::data_object::feature_property::FeatureProperty;
use rust_d3_geo::data_object::feature_struct::FeatureStruct;
use rust_d3_geo::data_object::features_struct::FeaturesStruct;
use rust_d3_geo::data_object::DataObject;
use rust_d3_geo::distance::distance;

use crate::delaunay::excess::excess;

use super::delaunay::GeoDelaunay;

/// Return type used by .x() and .y()
enum XYReturn<'a, T>
where
    T: Float + AsPrimitive<T>,
{
    Voronoi(Voronoi<'a, T>),
    Func(Box<dyn Fn(&dyn Centroid<Output = Point<T>>) -> T>),
}

// #[derive(Debug)]
struct TriStruct<T>
where
    T: CoordinateType,
{
    tri_points: Vec<Coordinate<T>>,
    center: Coordinate<T>,
}

// #[derive(Debug)]
pub struct Voronoi<'a, T>
where
    T: AsPrimitive<T> + CoordinateType + Float,
{
    geo_delaunay: Option<GeoDelaunay<'a, T>>,
    data: Option<Geometry<T>>,
    found: Option<usize>,
    //Points: Rc needed here as the egdes, triangles, neigbours etc all index into thts vec.
    points: Rc<Vec<Coordinate<T>>>,
    valid: Vec<Coordinate<T>>,
    // Option<Box<impl Fn(&dyn Centroid<Output = Coordinate<T>>) -> T>>
    vx: Box<dyn Fn(&dyn Centroid<Output = Point<T>>) -> T>,
    vy: Box<dyn Fn(&dyn Centroid<Output = Point<T>>) -> T>,
}

impl<'a, T> Default for Voronoi<'a, T>
where
    T: AsPrimitive<T> + CoordinateType + Float,
{
    fn default() -> Voronoi<'a, T> {
        return Voronoi {
            data: None,
            geo_delaunay: None,
            found: None,
            points: Rc::new(Vec::new()),
            valid: Vec::new(),
            vx: Box::new(|d: &dyn Centroid<Output = Point<T>>| d.centroid().x()),
            vy: Box::new(|d: &dyn Centroid<Output = Point<T>>| d.centroid().y()),
        };
    }
}

impl<'a, T> Voronoi<'a, T>
where
    T: CoordinateType + AsPrimitive<T> + Float + FloatConst + FromPrimitive,
{
    /// If the input is a collection we act only on the first element in the collection.
    /// by copying over the data into a new single element before proceeding.
    pub fn new(data: Option<Geometry<T>>) -> Voronoi<'a, T> {
        let mut v: Voronoi<'a, T>;

        // let delaunay_return: Option<GeoDelaunay> = None;

        // On finding a Features Collection take the first element only, drop other elements.
        // match data {
        //     DataObject<T>::FeatureCollection { mut features } => {
        //         // TODO: .remove() panics it it can't complete - consider trapping.
        //         let mut first_feature = features.remove(0);
        //         let geometry = first_feature.geometry.remove(0);
        //         let feature = FeatureStruct {
        //             properties: Vec::new(),
        //             geometry,
        //         };
        //         data = DataObject<T>::Feature { feature };
        //     }
        //     _ => {
        //         // Other DataTypes variants.
        //     }
        // };

        v = Voronoi {
            data,
            ..Voronoi::default()
        };

        // Data sanitization:-
        // Transform points using vx() and vy().
        // Remove infinities, store list of untransformed - valid points.
        let mut temp: Vec<(T, T, Point<T>)> = Vec::new();
        match v.data {
            // Some(FeatureCollection { features: f }) => {
            //     f.iter()
            //         .map(|d| ((v.vx)(d), (v.vy)(d), d.clone()))
            //         .filter(|t| (t.0 + t.1).is_finite());
            // }
            Some(Geometry::MultiPoint(ref data)) => {
                temp = data
                    .iter()
                    .map(|d| {
                        return (
                            (Self::default().vx)(&d.clone()),
                            (Self::default().vy)(&d.clone()),
                            *d,
                        );
                    })
                    .filter(|(d0, d1, _)| (*d0 + *d1).is_finite())
                    .collect();

                let points: Vec<Coordinate<T>> = temp
                    .iter()
                    .map(|(d0, d1, _)| Coordinate { x: *d0, y: *d1 })
                    .collect();
                v.points = Rc::new(points);
                v.valid = temp
                    .iter()
                    .map(|d| Coordinate {
                        x: d.2.x(),
                        y: d.2.y(),
                    })
                    .collect();
                let pclone = v.points.clone();
                v.geo_delaunay = GeoDelaunay::delaunay(pclone);
            }
            None => {
                v = Self::default();
            }
            _ => {
                panic!("Must implement Voronoi::new for other DataObject<T> types");
            }
        }

        return v;
    }

    fn x(
        mut self,
        f: Option<Box<impl Fn(&dyn Centroid<Output = Point<T>>) -> T + 'static>>,
    ) -> XYReturn<'a, T> {
        return match f {
            None => XYReturn::Func(self.vx),
            Some(f) => {
                self.vx = f;
                return XYReturn::Voronoi(self);
            }
        };
    }

    fn y(
        mut self,
        f: Option<Box<impl Fn(&dyn Centroid<Output = Point<T>>) -> T + 'static>>,
    ) -> XYReturn<'a, T> {
        return match f {
            None => XYReturn::Func(self.vy),
            Some(f) => {
                self.vy = f;
                return XYReturn::Voronoi(self);
            }
        };
    }

    pub fn polygons(&mut self, data: Option<Geometry<T>>) -> Option<FeatureCollection<T>> {
        match data {
            None => {}
            Some(_) => {
                *self = Self::new(data);
            }
        }

        match &self.geo_delaunay {
            None => {
                return None;
            }
            Some(dr) => {
                if self.valid.is_empty() {
                    return Some(FeatureCollection(Vec::new()));
                }

                let mut features: Vec<FeaturesStruct<T>> = Vec::new();
                println!("dr.polygons.len: {:?}", dr.polygons.len());
                for (i, poly) in dr.polygons.iter().enumerate() {
                    let mut poly_closed: Vec<usize> = poly.to_vec();
                    poly_closed.push(poly[0]);
                    let coordinates: Vec<Coordinate<T>> =
                        poly_closed.iter().map(|&i| (dr.centers[i])).collect();

                    let geometry = Geometry::Polygon(Polygon::new(coordinates.into(), vec![]));
                    // TODO why does this need to be borrow_mut
                    let neighbors = dr.neighbors.borrow_mut();
                    let n = neighbors.get(&i).unwrap().to_vec();
                    let properties: Vec<FeatureProperty<T>> = vec![
                        FeatureProperty::Site(self.valid[i]),
                        FeatureProperty::Sitecoordinates(self.points[i]),
                        FeatureProperty::Neighbors(n),
                    ];
                    let fs = FeaturesStruct {
                        geometry: vec![geometry],
                        properties: Vec::new(),
                    };
                    features.push(fs);
                }
                return Some(FeatureCollection(features));
            }
        }
    }

    fn triangles(mut self, data: Option<Geometry<T>>) -> Option<FeatureCollection<T>> {
        match data {
            None => {
                // No op
            }
            Some(_) => {
                self = Self::new(data);
            }
        }

        match self.geo_delaunay {
            None => {
                return None;
            }

            Some(delaunay_return) => {
                let points = self.points.clone();
                let features: Vec<FeaturesStruct<T>> = delaunay_return
                    .triangles
                    .iter()
                    .enumerate()
                    .map(|(index, tri)| {
                        let tri_points: Vec<Coordinate<T>> =
                            tri.iter().map(|i| (points[*i])).collect();
                        let tri_struct = TriStruct {
                            tri_points,
                            center: (delaunay_return.centers[index]),
                        };
                        return tri_struct;
                    })
                    .filter(|tri_struct| return excess(&tri_struct.tri_points) > T::zero())
                    .map(|tri_struct| {
                        let first = tri_struct.tri_points[0];
                        let mut coordinates: Vec<Coordinate<T>> = tri_struct.tri_points;
                        coordinates.push(first);
                        FeaturesStruct {
                            properties: vec![FeatureProperty::Circumecenter(tri_struct.center)],
                            geometry: vec![Geometry::Polygon(Polygon::new(
                                coordinates.into(),
                                vec![],
                            ))],
                        }
                    })
                    .collect();

                return Some(FeatureCollection(features));
            }
        }
    }

    fn link(mut self, data: Option<Geometry<T>>) -> Option<FeatureCollection<T>> {
        match data {
            None => {
                // No op
            }
            _ => {
                self = Self::new(data);
            }
        }

        return match &self.geo_delaunay {
            None => None,
            Some(delaunay_return) => {
                let points: &Vec<Coordinate<T>> = self.points.borrow();
                let distances: Rc<Vec<T>> = Rc::new(
                    delaunay_return
                        .edges
                        .iter()
                        .map(|e| distance(&(points)[e[0]], &(points)[e[0]]))
                        .collect(),
                );

                {
                    let urquhart = (delaunay_return.urquhart)(&distances);
                    let features: Vec<FeaturesStruct<T>> = delaunay_return
                        .edges
                        .iter()
                        .enumerate()
                        .map(|(i, e)| {
                            let ls: LineString<T> = vec![points[0], points[e[1]]].into();
                            return FeaturesStruct {
                                properties: vec![
                                    FeatureProperty::Source(self.valid[e[0]]),
                                    FeatureProperty::Target(self.valid[e[1]]),
                                    FeatureProperty::Length(distances[i]),
                                    FeatureProperty::Urquhart(urquhart[i]),
                                ],
                                geometry: vec![Geometry::LineString(ls)],
                            };
                        })
                        .collect();
                    return Some(FeatureCollection(features));
                }
            }
        };
    }

    pub fn mesh(mut self, data: Option<Geometry<T>>) -> Option<MultiLineString<T>> {
        match data {
            None => {
                // No op
            }
            _ => {
                self = Self::new(data);
            }
        }

        match &self.geo_delaunay {
            None => {
                return None;
            }
            Some(delaunay_return) => {
                let coordinates: Vec<LineString<T>> = delaunay_return
                    .edges
                    .iter()
                    .map(|e| line_string![(self.points)[e[0]], (self.points)[e[1]]])
                    .collect();
                return Some(MultiLineString(coordinates));
            }
        }
    }

    pub fn cell_mesh(mut self, data: Option<Geometry<T>>) -> Option<MultiLineString<T>> {
        match data {
            None => {
                // No op
            }
            Some(_) => {
                self = Self::new(data);
            }
        }

        if self.geo_delaunay.is_none() {
            return None;
        }
        let delaunay = self.geo_delaunay.unwrap();
        let polygons = delaunay.polygons;
        let centers = delaunay.centers;
        let mut coordinates: Vec<LineString<T>> = Vec::new();
        for p in polygons {
            let n = p.len();
            let mut p0 = *p.last().unwrap();
            let mut p1 = p[0];
            for i in 0..n {
                if p1 > p0 {
                    coordinates.push(line_string![centers[p0], centers[p1]]);
                }
                p0 = p1;
                p1 = p[i];
            }
        }

        return Some(MultiLineString(coordinates));
    }

    fn find(mut self, p: Coordinate<T>, radius: Option<T>) -> Option<usize> {
        return match self.geo_delaunay {
            None => None,
            Some(delaunay_return) => {
                self.found = (delaunay_return.find)(p, self.found);
                match self.found {
                    Some(found) => {
                        return match radius {
                            Some(radius) => {
                                // TODO confirm the eclidean_distance is the same as the rust_geo::distance....
                                if distance(&p, &self.points[found]) < radius {
                                    // if p.euclidean_distance(&self.points[found]) < radius {
                                    return Some(found);
                                } else {
                                    return None;
                                }
                            }
                            None => None,
                        };
                    }
                    None => {
                        return None;
                    }
                }
            }
        };
    }

    pub fn hull(mut self, data: Option<Geometry<T>>) -> Option<Polygon<T>> {
        match data {
            None => {
                // No op
            }
            _ => {
                self = Self::new(data);
            }
        }

        match self.geo_delaunay {
            None => {
                return None;
            }
            Some(ref delaunay_return) => {
                match delaunay_return.hull.len() {
                    0usize => {
                        return None;
                    }
                    _ => {
                        let hull = &delaunay_return.hull;
                        let mut coordinates: Vec<Coordinate<T>> =
                            hull.iter().map(|i| self.points[*i]).collect();
                        coordinates.push(self.points[hull[0]]);
                        return Some(Polygon::new(coordinates.into(), vec![]));
                    }
                };
            }
        }
    }
}
