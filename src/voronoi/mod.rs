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

use crate::delaunay::excess::excess;
use crate::delaunay::DelaunayReturn;
use crate::delaunay::Delaunay;

/// Return type used by .x() and .y()
enum XYReturn<'a, F>
where
  F: Float + FloatConst + FromPrimitive,
{
  Voronoi(Voronoi<'a, F>),
  Func(Box<dyn Fn(DataType<F>) -> Option<F>>),
}

#[derive(Debug)]
struct TriStruct<F>
where
  F: Float,
{
  tri_points: Vec<[F; 2]>,
  center: [F; 2],
}

pub struct Voronoi<'a, F>
where
  F: Float + FloatConst + FromPrimitive,
{
  delaunay_return: Option<DelaunayReturn<'a, F>>,
  data: DataType<F>,
  found: Option<usize>,
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
      found: None,
      points: Rc::new(Vec::new()),
      valid: Vec::new(),
      vx: Box::new(|_| None),
      vy: Box::new(|_| None),
    };
  }
}

impl<'a, F> Voronoi<'a, F>
where
  F: Float + FloatConst + FromPrimitive + 'static
{
  pub fn new(mut data: DataType<F>) -> Voronoi<'a, F>
  where
    F: Float + FloatConst + FromPrimitive + 'static,
  {
    let mut v: Voronoi<'a, F>;


    let delaunay_return: Option<DelaunayReturn<F>> = None;

    // On finding a Features Collection take the first element only, drop other elements.
    match data {
      DataType::Object(DataObject::FeatureCollection{mut features}) => {

        // TODO: .remove() panics it it can't complete - consider trapping.
        let mut first_feature = features.remove(0);
        let geometry = first_feature.geometry.remove(0);
        let feature = FeatureStruct {
          properties: Vec::new(),
          geometry,
        };
        data =  DataType::Object(DataObject::Feature { feature });

      }
      _ => {
            // Other DataTypes variants.
      }
    };

    println!("received ");
    v = Voronoi {
      data,
      ..Voronoi::default()
    };

    let vx = Box::new(|d: &DataType<F>| -> Option<F> {
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

    let vy = Box::new(|d: &DataType<F>| -> Option<F> {
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

    match v.data {
      DataType::Vec(ref data) => {
        let temp: Vec<(Option<F>, Option<F>, F)> = data
          .iter()
          .map(|d| {
            return (vx(&v.data), vy(&v.data), *d);
          })
          .filter(|d| {
            match d {
              (Some(d0), Some(d1), _) => { return (*d0 + *d1).is_finite();}
              _ => { return false;}
              }
            }
          )
          .collect();

        let points: Vec<[F; 2]> = temp.iter().map(|d| {
          match d {
            (Some(d0), Some(d1), _) => {return [*d0, *d1]; }
            _ => { panic!("Unexpected Vec has been filtered ");}
            }
          }
        ).collect();
        v.points = Rc::new(points);
        v.valid = temp.iter().map(|d| (*d).2 ).collect();
        v.delaunay_return = Delaunay::delaunay(v.points.clone());
      }
      _ => {}
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

  pub fn polygons(mut self, data: DataType<F>) -> Option<DataObject<F>> {
    match data {
      DataType::Blank => {
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
        let features: Vec<FeaturesStruct<F>> = Vec::new();
        for (i, ref poly) in dr.polygons.iter().enumerate() {
          let first = poly[0].clone();
          let mut coordinates_i: Vec<usize> = poly.to_vec();
          coordinates_i.push(first);
          let coordinates: Vec<Vec<[F; 2]>> =
            vec![coordinates_i.iter().map(|i| dr.centers[*i]).collect()];

          let geometry = FeatureGeometry::Polygon { coordinates };
          let mut neighbors = dr.neighbors.borrow_mut();
          let n: Vec<usize> = (neighbors.remove(&i)).unwrap();
          let properties = vec![
            // FeatureProperty::<F>::Site(self.valid[i]),
            // FeatureProperty::<F>::Sitecoordinates(self.points[i]),
            // The endpoint for neighbors.
            // Consume neighbours here. Remove, and thereby destroy neighbours.
            FeatureProperty::<F>::Neighbors(n),
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

  fn triangles(mut self, data: DataType<F>) -> Option<DataObject<F>> {
    match data {
      DataType::Blank => {
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

        return Some(DataObject::FeatureCollection { features });
      }
    }
  }

  fn link(mut self, data: DataType<F>) -> Option<DataType<F>> {
    match data {
      DataType::Blank => {
        // No op
      }
      _ => {
        self = Voronoi::new(data);
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
          return Some(DataType::Object(DataObject::FeatureCollection { features }));
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
        self = Voronoi::new(data);
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

  fn cell_mesh(mut self, data: DataType<F>) -> Option<DataObject<F>> {
    match data {
      DataType::Blank => {
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

  fn find(mut self, x: F, y: F, radius: Option<F>) -> Option<usize> {
    return match self.delaunay_return {
      None => None,
      Some(delaunay_return) => {
        self.found = (delaunay_return.find)(x, y, self.found);
        match self.found {
          Some(found) => {
            return match radius {
              Some(radius) => {
                if distance(&[x, y], &self.points[found]) < radius {
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

  fn hull(mut self, data: DataType<F>) -> Option<DataObject<F>> {
    match data {
      DataType::Blank => {
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

}
