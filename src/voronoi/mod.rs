use std::borrow::Borrow;
use std::rc::Rc;

use delaunator::Point;

use rust_d3_geo::data_object::DataObject;

use rust_d3_geo::data_object::FeatureGeometry;
use rust_d3_geo::data_object::FeatureProperty;
use rust_d3_geo::data_object::FeatureStruct;
use rust_d3_geo::data_object::FeaturesStruct;
use rust_d3_geo::distance::distance;

use crate::delaunay::excess::excess;
use crate::delaunay::Delaunay;
use crate::delaunay::DelaunayReturn;

/// Return type used by .x() and .y()
enum XYReturn<'a> {
    Voronoi(Voronoi<'a>),
    Func(Box<dyn Fn(&DataObject) -> Option<f64>>),
}

#[derive(Debug)]
struct TriStruct {
    tri_points: Vec<Point>,
    center: Point,
}
// #[derive(Debug)]
pub struct Voronoi<'a> {
    delaunay_return: Option<DelaunayReturn<'a>>,
    data: DataObject,
    found: Option<usize>,
    //Points: Rc needed here as the egdes, triangles, neigbours etc all index into thts vec.
    points: Rc<Vec<Point>>,
    valid: Vec<Point>,
    vx: Box<dyn Fn(&DataObject) -> Option<f64>>,
    vy: Box<dyn Fn(&DataObject) -> Option<f64>>,
}

impl<'a> Default for Voronoi<'a> {
    fn default() -> Voronoi<'a> {
        return Voronoi {
            data: DataObject::Blank,
            delaunay_return: None,
            found: None,
            points: Rc::new(Vec::new()),
            valid: Vec::new(),
            vx: Box::new(|_| None),
            vy: Box::new(|_| None),
        };
    }
}

impl<'a> Voronoi<'a> {
    pub fn new(mut data: DataObject) -> Voronoi<'a> {
        let mut v: Voronoi<'a>;

        let delaunay_return: Option<DelaunayReturn> = None;

        // On finding a Features Collection take the first element only, drop other elements.
        match data {
            DataObject::FeatureCollection { mut features } => {
                // TODO: .remove() panics it it can't complete - consider trapping.
                let mut first_feature = features.remove(0);
                let geometry = first_feature.geometry.remove(0);
                let feature = FeatureStruct {
                    properties: Vec::new(),
                    geometry,
                };
                data = DataObject::Feature { feature };
            }
            _ => {
                // Other DataTypes variants.
            }
        };

        v = Voronoi {
            data,
            ..Voronoi::default()
        };

        let vx = Box::new(|d: &DataObject| -> Option<f64> {
            match d {
                DataObject::Vec(d) => {
                    match d.first() {
                        Some(d) => {
                            return Some(d.x);
                        }
                        None => {
                            // Could panic here with
                            //panic!("given a emtpy vector ");
                            return None;
                        }
                    }
                }
                DataObject::Blank => {
                    return None;
                }
                _ => {
                    // TODO untested code slip centroid calc.
                    // return Some(centroid(d)[0]);
                    return None;
                }
            }
        });

        let vy = Box::new(|d: &DataObject| -> Option<f64> {
            match d {
                DataObject::Vec(d) => {
                    if d.len() > 1 {
                        return Some(d[0].y);
                    }
                    return None;
                }
                DataObject::Blank => {
                    return None;
                }
                _ => {
                    // TODO untested code slip centroid calc.
                    // return Some(centroid(d)[1]);
                    return None;
                }
            }
        });

        match v.data {
            DataObject::Vec(ref data) => {
                let temp: Vec<(Option<f64>, Option<f64>, Point)> = data
                    .iter()
                    .map(|d| {
                        return (vx(&v.data), vy(&v.data), d.clone());
                    })
                    .filter(|d| match d {
                        (Some(d0), Some(d1), _) => {
                            return (*d0 + *d1).is_finite();
                        }
                        _ => {
                            return false;
                        }
                    })
                    .collect();

                let points: Vec<Point> = temp
                    .iter()
                    .map(|d| match d {
                        (Some(d0), Some(d1), _) => {
                            return Point { x: *d0, y: *d1 };
                        }
                        _ => {
                            panic!("Unexpected Vec has been filtered ");
                        }
                    })
                    .collect();
                v.points = Rc::new(points);
                v.valid = temp.iter().map(|d| (d.2).clone()).collect();
                v.delaunay_return = Delaunay::delaunay(v.points.clone());
            }
            _ => {
                panic!("Must implement Voronoi::new for other DataObject types");
            }
        }

        // v = Voronoi {
        //   delaunay_return,
        //   found: Vec::new(),
        //   // valid,
        //   vx,
        //   vy,
        //   ..v
        // };

        // v.delaunay_return = delaunay_return;
        // v.found = None;
        // v.vx = vx;
        // v.vy = vy;

        // let v = Voronoi {
        //   data,
        //   delaunay_return,
        //   found: Vec::new(),
        //   points,
        //   valid,
        //   vx,
        //   vy,
        // };

        return v;

        // TODO break recursion here.
        // return match v.data {
        //   DataType::Blank => v,
        //   _ => Voronoi::new(v.data),
        // };
    }

    fn x(mut self, f: Option<Box<dyn Fn(&DataObject) -> Option<f64>>>) -> XYReturn<'a> {
        return match f {
            None => XYReturn::Func(self.vx),
            Some(f) => {
                self.vx = f;
                return XYReturn::Voronoi(self);
            }
        };
    }

    fn y(mut self, f: Option<Box<dyn Fn(&DataObject) -> Option<f64>>>) -> XYReturn<'a> {
        return match f {
            None => XYReturn::Func(self.vy),
            Some(f) => {
                self.vy = f;
                return XYReturn::Voronoi(self);
            }
        };
    }

    pub fn polygons(mut self, data: DataObject) -> Option<DataObject> {
        match data {
            DataObject::Blank => {
                // No op
            }
            _ => {
                self = Voronoi::new(data);
            }
        }

        match self.delaunay_return {
            None => {
                panic!("the delaunay return is None");
                return None;
            }
            Some(dr) => {
                let features: Vec<FeaturesStruct> = Vec::new();
                for (i, ref poly) in dr.polygons.iter().enumerate() {
                    let first = poly[0].clone();
                    let mut coordinates_i: Vec<usize> = poly.to_vec();
                    coordinates_i.push(first);
                    let coordinates: Vec<Vec<Point>> = vec![coordinates_i
                        .iter()
                        .map(|i| (dr.centers[*i]).clone())
                        .collect()];

                    let geometry = FeatureGeometry::Polygon { coordinates };
                    let mut neighbors = dr.neighbors.borrow_mut();
                    let n: Vec<usize> = (neighbors.remove(&i)).unwrap();
                    let properties = vec![
                        // FeatureProperty::<F>::Site(self.valid[i]),
                        // FeatureProperty::<F>::Sitecoordinates(self.points[i]),
                        // The endpoint for neighbors.
                        // Consume neighbours here. Remove, and thereby destroy neighbours.
                        FeatureProperty::Neighbors(n),
                    ];
                    let f = DataObject::Feature {
                        feature: FeatureStruct {
                            geometry,
                            properties: Vec::new(),
                        },
                    };
                    //   coll.features.push();
                    // }
                }
                return Some(DataObject::FeatureCollection { features });
            }
        }
    }

    fn triangles(mut self, data: DataObject) -> Option<DataObject> {
        match data {
            DataObject::Blank => {
                // No op
            }
            _ => {
                self = Voronoi::new(data);
            }
        }

        match self.delaunay_return {
            None => {
                return None;
            }

            Some(delaunay_return) => {
                let points = self.points.clone();
                let features: Vec<FeaturesStruct> = delaunay_return
                    .triangles
                    .iter()
                    .enumerate()
                    .map(|(index, tri)| {
                        let tri_points: Vec<Point> =
                            tri.iter().map(|i| (points[*i]).clone()).collect();
                        let tri_struct = TriStruct {
                            tri_points,
                            center: (delaunay_return.centers[index]).clone(),
                        };
                        return tri_struct;
                    })
                    .filter(|tri_struct| return excess(&tri_struct.tri_points) > 0f64)
                    .map(|tri_struct| {
                        let first = tri_struct.tri_points[0].clone();
                        let mut coordinates: Vec<Point> = tri_struct.tri_points;
                        coordinates.push(first);
                        FeaturesStruct {
                            properties: vec![FeatureProperty::Circumecenter(tri_struct.center)],
                            geometry: vec![FeatureGeometry::Polygon {
                                coordinates: vec![coordinates],
                            }],
                        }
                    })
                    .collect();

                return Some(DataObject::FeatureCollection { features });
            }
        }
    }

    fn link(mut self, data: DataObject) -> Option<DataObject> {
        match data {
            DataObject::Blank => {
                // No op
            }
            _ => {
                self = Voronoi::new(data);
            }
        }

        return match &self.delaunay_return {
            None => None,
            Some(delaunay_return) => {
                let points: &Vec<Point> = self.points.borrow();
                let distances: Rc<Vec<f64>> = Rc::new(
                    delaunay_return
                        .edges
                        .iter()
                        .map(|e| distance(&(points)[e[0]], &(points)[e[0]]))
                        .collect(),
                );

                {
                    let urquhart = (delaunay_return.urquhart)(&distances);
                    let features: Vec<FeaturesStruct> = delaunay_return
                        .edges
                        .iter()
                        .enumerate()
                        .map(|(i, e)| {
                            let coordinates = vec![points[0].clone(), points[e[1]].clone()];
                            return FeaturesStruct {
                                properties: vec![
                                    FeatureProperty::Source(self.valid[e[0]].clone()),
                                    FeatureProperty::Target(self.valid[e[1]].clone()),
                                    FeatureProperty::Length(distances[i]),
                                    FeatureProperty::Urquhart(urquhart[i]),
                                ],
                                geometry: vec![FeatureGeometry::LineString { coordinates }],
                            };
                        })
                        .collect();
                    return Some(DataObject::FeatureCollection { features });
                }
            }
        };
    }

    fn mesh(mut self, data: DataObject) -> Option<DataObject> {
        match data {
            DataObject::Blank => {
                // No op
            }
            _ => {
                self = Voronoi::new(data);
            }
        }

        match &self.delaunay_return {
            None => {
                return None;
            }
            Some(delaunay_return) => {
                let coordinates: Vec<Vec<Point>> = delaunay_return
                    .edges
                    .iter()
                    .map(|e| vec![(self.points)[e[0]].clone(), (self.points)[e[1]].clone()])
                    .collect();
                return Some(DataObject::MultiLineString { coordinates });
            }
        }
    }

    fn cell_mesh(mut self, data: DataObject) -> Option<DataObject> {
        match data {
            DataObject::Blank => {
                // No op
            }
            _ => {
                self = Voronoi::new(data);
            }
        }

        match self.delaunay_return {
            None => {
                return None;
            }
            Some(delaunay_return) => match delaunay_return.delaunay.centers {
                None => {
                    panic!("Expected to be able to access centers here.");
                }
                Some(centers) => {
                    let polygons = delaunay_return.polygons;
                    let mut coordinates = vec![vec![]];
                    for p in polygons {
                        let n = p.len();
                        let mut p0 = *p.last().unwrap();
                        let mut p1 = p[0];
                        for i in 0..n {
                            if p1 > p0 {
                                coordinates.push(vec![centers[p0].clone(), centers[p1].clone()]);
                            }
                            p0 = p1;
                            p1 = p[i + 1];
                        }
                    }

                    return Some(DataObject::MultiLineString { coordinates });
                }
            },
        }
    }

    fn find(mut self, x: f64, y: f64, radius: Option<f64>) -> Option<usize> {
        return match self.delaunay_return {
            None => None,
            Some(delaunay_return) => {
                self.found = (delaunay_return.find)(x, y, self.found);
                match self.found {
                    Some(found) => {
                        return match radius {
                            Some(radius) => {
                                if distance(&Point { x, y }, &self.points[found]) < radius {
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

    fn hull(mut self, data: DataObject) -> Option<DataObject> {
        match data {
            DataObject::Blank => {
                // No op
            }
            _ => {
                self = Voronoi::new(data);
            }
        }

        match self.delaunay_return {
            None => {
                return None;
            }
            Some(ref delaunay_return) => {
                match delaunay_return.hull.len() {
                    0usize => {
                        return None;
                    }
                    _ => {
                        let mut coordinates: Vec<Point> = delaunay_return
                            .hull
                            .iter()
                            .map(|i| {
                                return self.points[*i].clone();
                            })
                            .collect();
                        coordinates.push(self.points[0].clone());
                        return Some(DataObject::Polygon {
                            coordinates: vec![coordinates],
                        });
                    }
                };
            }
        }
    }
}
