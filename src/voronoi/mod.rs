use std::rc::Rc;

use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use rust_d3_geo::data_object::DataObject;
use rust_d3_geo::data_object::DataType;
use rust_d3_geo::data_object::FeatureGeometry;
use rust_d3_geo::data_object::FeatureProperty;
use rust_d3_geo::data_object::FeatureStruct;
use rust_d3_geo::data_object::FeaturesStruct;
use rust_d3_geo::distance::distance;

use crate::delaunay::delaunay_from::delaunay_from;
use crate::delaunay::excess::excess;
use crate::delaunay::Delaunay;
use crate::delaunay::DelaunayReturn;

/// Return type used by .x() and .y()
enum XYReturn<'a, F>
where
  F: Float + FloatConst + FromPrimitive,
{
  Voronoi(Voronoi<'a, F>),
  Func(Box<dyn Fn(DataType<F>) -> Option<F>>),
}

struct TriStruct<F>
where
  F: Float,
{
  tri_points: Vec<[F; 2]>,
  center: [F; 2],
}

struct Voronoi<'a, F>
where
  F: Float + FloatConst + FromPrimitive,
{
  // cellMesh:
  delaunay_return: Option<DelaunayReturn<'a, F>>,
  data: DataType<F>,
  found: Vec<F>,
  //Points: Rc needed here as the egdes, triangles, neigbours etc all index into thts vec.
  points: Rc<Vec<[F; 2]>>,
  valid: Vec<F>,
  vx: Box<dyn Fn(DataType<F>) -> Option<F>>,
  vy: Box<dyn Fn(DataType<F>) -> Option<F>>,
}

impl<'a, F> Default for Voronoi<'a, F>
where
  F: Float + FloatConst + FromPrimitive,
{
  fn default() -> Voronoi<'a, F> {
    return Voronoi {
      data: DataType::Blank,
      delaunay_return: None,
      found: Vec::new(),
      points: Rc::new(Vec::new()),
      valid: Vec::new(),
      vx: Box::new(|_| None),
      vy: Box::new(|_| None),
    };
  }
}

impl<'a, F> Voronoi<'a, F>
where
  F: Float + FloatConst + FromPrimitive,
{
  fn voronoi(data: DataType<F>) -> Voronoi<'a, F>
  where
    F: Float + FloatConst + FromPrimitive,
  {
    let mut v: Voronoi<'a, F> = Voronoi {
      data,
      ..Voronoi::default()
    };
    // v.data = data;

    let points: Vec<[F; 2]>;
    let delaunay_return: Option<DelaunayReturn<F>> = None;

    // On finding a Features Collection take the first element only, drop everything else.
    // match v.data {
    //   DataType::Object(obj) => match obj {
    //     DataObject::FeaturesCollection { features } => {
    //       let featureMaybe = features.first();
    //       match featureMaybe {
    //         Some(feature) => {
    //           v.data = DataType::Object(DataObject::Feature {
    //             feature: FeatureStruct {
    //               // TODO do I want to drop the assocated properites? No.
    //               properties: Vec::new(),
    //               geometry: *(feature.geometry.first().unwrap()),
    //             },
    //           });
    //         }
    //         None => {
    //           panic!("found a collection with not elements");
    //         }
    //       }
    //     }
    //     _ => { // Other Data Objects.
    //     }
    //   },
    //   _ => { // Other Data Types
    //   }
    // }

    let vx = Box::new(|d: DataType<F>| -> Option<F> {
      match d {
        DataType::Object(d) => {
          // TODO untested code slip centroid calc.
          // return Some(centroid(d)[0]);
          return None;
        }
        DataType::Vec(d) => {
          match d.first() {
            Some(d) => {
              return Some(*d);
            }
            None => {
              // Could panic here with
              //panic!("given a emtpy vector ");
              return None;
            }
          }
        }
        DataType::Blank => {
          return None;
        }
      }
    });

    let vy = Box::new(|d: DataType<F>| -> Option<F> {
      match d {
        DataType::Object(d) => {
          // TODO untested code slip centroid calc.
          // return Some(centroid(d)[1]);
          return None;
        }

        DataType::Vec(d) => {
          if d.len() > 2 {
            return Some(d[1]);
          }
          return None;
        }
        DataType::Blank => {
          return None;
        }
      }
    });

    // match v.data {
    //   DataType::Vec(data) => {
    //     let temp = data
    //       .iter()
    //       .map(|d| {
    //         return [vx(v.data), vy(v.data), Some(*d)];
    //       })
    //       .filter(|d| (d[0] + d[1]).is_finite())
    //       .collect();

    //     points = temp.iter().map(|d| [d[0], d[1]]).collect();
    //     valid = temp.map(|d| d[2]).collect();
    //     delaunay_return = Delaunay::delaunay(&points);
    //   }
    //   _ => {}
    // }

    // v = Voronoi {
    //   delaunay_return,
    //   found: Vec::new(),
    //   // valid,
    //   vx,
    //   vy,
    //   ..v
    // };

    v.delaunay_return = delaunay_return;
    v.found = Vec::new();
    v.vx = vx;
    v.vy = vy;

    // let v = Voronoi {
    //   data,
    //   delaunay_return,
    //   found: Vec::new(),
    //   points,
    //   valid,
    //   vx,
    //   vy,
    // };

    return match v.data {
      DataType::Blank => v,
      _ => Voronoi::voronoi(v.data),
    };
  }

  fn cell_mesh(mut self, data: DataType<F>) -> Option<DataObject<F>> {
    match data {
      DataType::Blank => {
        // No op
      }
      _ => {
        self = Voronoi::voronoi(data);
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
                coordinates.push(vec![centers[p0], centers[p1]]);
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

  //   v._found = undefined;
  //   v.find = function(x, y, radius) {
  //     v._found = v.delaunay.find(x, y, v._found);
  //     if (!radius || geoDistance([x, y], v.points[v._found]) < radius)
  //       return v._found;
  //   };

  // fn find(&self, x: F, y: F, radius: Option<F>) -> Option<DataObject<F>> {
  //   return match self.delaunay_return {
  //     None => {None},
  //     Some(delaunay_return) => {
  //       self.found = (delaunay_return.find)(x, y, self.found);
  //       return match radius {
  //       Some(radius) => {
  //         if distance(&[x, y], &self.points[self.found]) < radius {
  //           return self.found;
  //         }
  //         else {
  //           return None;
  //         }
  //       }
  //       None => None,
  //     };
  //   }
  //   }
  // }

  fn triangles(mut self, data: DataType<F>) -> Option<DataObject<F>> {
    match data {
      DataType::Blank => {
        // No op
      }
      _ => {
        self = Voronoi::voronoi(data);
      }
    }

    match self.delaunay_return {
      None => {
        return None;
      }

      Some(delaunay_return) => {
        let points = self.points.clone();
        let features: Vec<FeaturesStruct<F>> = delaunay_return
          .triangles
          .iter()
          .enumerate()
          .map(|(index, tri)| {
            let tri_points: Vec<[F; 2]> = tri.iter().map(|i| points[*i]).collect();
            let tri_struct = TriStruct {
              tri_points,
              center: delaunay_return.centers[index],
            };
            return tri_struct;
          })
          .filter(|tri_struct| return excess(&tri_struct.tri_points) > F::zero())
          .map(|tri_struct| {
            let first = tri_struct.tri_points[0].clone();
            let mut coordinates: Vec<[F; 2]> = tri_struct.tri_points;
            coordinates.push(first);
            FeaturesStruct::<F> {
              properties: vec![FeatureProperty::Circumecenter(tri_struct.center)],
              geometry: vec![FeatureGeometry::Polygon {
                coordinates: vec![coordinates],
              }],
            }
          })
          .collect();

        return Some(DataObject::FeaturesCollection { features });
      }
    }
  }

  fn link(mut self, data: DataType<F>) -> Option<DataType<F>> {
    match data {
      DataType::Blank => {
        // No op
      }
      _ => {
        self = Voronoi::voronoi(data);
      }
    }

    return match &self.delaunay_return {
      None => None,
      Some(delaunay_return) => {
        let points = self.points.clone();
        let distances: Rc<Vec<F>> = Rc::new(
          delaunay_return
            .edges
            .iter()
            .map(|e| distance(&(points)[e[0]], &(points)[e[0]]))
            .collect(),
        );

        {
          let urquhart = (delaunay_return.urquhart)(&distances);
          let features: Vec<FeaturesStruct<F>> = delaunay_return
            .edges
            .iter()
            .enumerate()
            .map(|(i, e)| {
              let coordinates = vec![points[0], points[e[1]]];
              return FeaturesStruct {
                properties: vec![
                  FeatureProperty::Source(self.valid[e[0]]),
                  FeatureProperty::Target(self.valid[e[1]]),
                  FeatureProperty::Length(distances[i]),
                  FeatureProperty::Urquhart(urquhart[i]),
                ],
                geometry: vec![FeatureGeometry::LineString { coordinates }],
              };
            })
            .collect();
          return Some(DataType::Object(DataObject::FeaturesCollection {
            features,
          }));
        }
      }
    };
  }

  fn mesh(mut self, data: DataType<F>) -> Option<DataObject<F>> {
    match data {
      DataType::Blank => {
        // No op
      }
      _ => {
        self = Voronoi::voronoi(data);
      }
    }

    match &self.delaunay_return {
      None => {
        return None;
      }
      Some(delaunay_return) => {
        let coordinates: Vec<Vec<[F; 2]>> = delaunay_return
          .edges
          .iter()
          .map(|e| vec![(self.points)[e[0]], (self.points)[e[1]]])
          .collect();
        return Some(DataObject::MultiLineString { coordinates });
      }
    }
  }

  fn hull(mut self, data: DataType<F>) -> Option<DataObject<F>> {
    match data {
      DataType::Blank => {
        // No op
      }
      _ => {
        self = Voronoi::voronoi(data);
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
            let mut coordinates: Vec<[F; 2]> = delaunay_return
              .hull
              .iter()
              .map(|i| {
                return self.points[*i];
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

  fn x(mut self, f: Option<Box<dyn Fn(DataType<F>) -> Option<F>>>) -> XYReturn<'a, F> {
    return match f {
      None => XYReturn::Func(self.vx),
      Some(f) => {
        self.vx = f;
        return XYReturn::Voronoi(self);
      }
    };
  }

  fn y(mut self, f: Option<Box<dyn Fn(DataType<F>) -> Option<F>>>) -> XYReturn<'a, F> {
    return match f {
      None => XYReturn::Func(self.vy),
      Some(f) => {
        self.vy = f;
        return XYReturn::Voronoi(self);
      }
    };
  }
}
