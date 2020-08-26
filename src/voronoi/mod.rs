use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::delaunay::excess::excess;

use crate::data_object::DataObject;
use crate::data_object::DataType;
use crate::data_object::GeometryType;
use crate::data_object::PropertyType;
use crate::data_object::FeaturesStruct;
use crate::delaunay::delaunay_from::delaunay_from;
use crate::delaunay::Delaunay;

struct Voronoi<F>
where
  F: Float + FloatConst + FromPrimitive,
{
  cellMesh:
  delaunay: Option<Delaunay<F>>,
  data: DataType<F>,
  find: Box<dyn Fn(x: F, y: F, radius: Option<F>) -> Option<DataObject<F>>,
  points: Vec<[F; 2]>,
  mesh: Box<dyn Fn(DataObject<F>) -> Option<DataObject<F>>>,
  link: Box<dyn Fn(DataObject<F>) -> Option<DataObject<F>>>,
  hull: Box<dyn Fn(DataObject<F>) -> Option<DataObject<F>>>,
  triangles: Box< dyn Fn(DataObject<F>) -> Option<DataObject<F>>>,
  valid: Vec<F>,
  vx: Box<dyn Fn(DataObject<F>) -> Option<usize>>,
  vy: Box<dyn Fn(DataObject<F>) -> Option<usize>>,
}

impl<F> Voronoi<F>
where
  F: Float + FloatConst + FromPrimitive,
{
  fn voronoi(data: DataType<F>) -> Voronoi<F>
  where
    F: Float + FloatConst + FromPrimitive,
  {
    let mut v: Voronoi<F>;
    v.data = data;

    let points: Vec<[F; 2]>;
    let valid: Vec<F>;
    let delaunay: Option<Delaunay<F>> = None;

    //   if (typeof v._data === "object" && v._data.type === "FeatureCollection") {
    //     v._data = v._data.features;
    //   }

    // On finding a Features Collection take the first element only, drop everything else.
    match v.data {
      DataType::Object(obj) => match obj {
        DataObject::FeaturesCollection { features: f } => match f.first() {
          Some(f) => {
            v.data = DataType::Object(DataObject::Feature(*f));
          }
          None => {
            panic!("Found an empty feature Collection!");
          }
        },
      },
    }

    let vx = Box::new(|d: DataType<F>| -> Option<usize> {
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

    //   v._vy = function(d) {
    //     if (typeof d == "object" && "type" in d) {
    //       return geoCentroid(d)[1];
    //     }
    //     if (1 in d) return d[1];
    //   };

    let vy = Box::new(|d: DataType<F>| -> Option<usize> {
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


    // v.links = function(data) {
    //   if (data !== undefined) {
    //     v(data);
    //   }
    //   if (!v.delaunay) return false;
    //   const _distances = v.delaunay.edges.map(e =>
    //       geoDistance(v.points[e[0]], v.points[e[1]])
    //     ),
    //     _urquart = v.delaunay.urquhart(_distances);
    //   return {
    //     type: "FeatureCollection",
    //     features: v.delaunay.edges.map((e, i) => ({
    //       type: "Feature",
    //       properties: {
    //         source: v.valid[e[0]],
    //         target: v.valid[e[1]],
    //         length: _distances[i],
    //         urquhart: !!_urquart[i]
    //       },
    //       geometry: {
    //         type: "LineString",
    //         coordinates: [v.points[e[0]], v.points[e[1]]]
    //       }
    //     }))
    //   };
    // };
    let link = Box::new(|data: DataType<F>| -> Option<DataType<F>> {
      match data {
        DataType::Blank => {},
        _ => {
          Voronoi::voronoi(data);
        }
      }

      return match v.delaunay {
        None =>  None,
        Some(delaunay) => {
          let features: Vec<FeaturesStruct<F>> = v.delaunay.edges.iter.enumerate.map(|(i, e)| FeaturesStruct {
            properties: vec![
              PropertyType::Source(v.valid[e[0]]),
              PropertyType::Target(v.valid[e[0]]),
              PropertyType::Length(v.valid[e[0]]),
              PropertyType::Urquhart(v.valid[e[0]]),
            ],
            geometry: GeometryType::LineString{
              coordinate: [ v.points[0], v.points[e[1]] ],
            }
          }).collect();

          return Some(DataType::Object(DataObject::FeaturesCollection { features }));

      }
    }
  });


    let triangles = Box::new(|data :DataType<F>| -> Option<DataObject<F>> {

      match data {
        DataType::Blank => {}
        _ => {
          Voronoi::voronoi(data);
        }
      }

      if v.delaunay.is_none() {
        return None;
      }

      return Some(DataObject::FeaturesCollection {
        features:

          v.delaunay.triangles.iter().enumerate()
          .map(|(tri, index)| {
            tri = tri.map(|i|  v.points[i]);
            tri.center = v.delaunay.centers[index];
            return tri;
          })
          .filter(|tri| excess(tri) > F::zero())
          .map(|tri| DataObject::Feature{
            properties: PropertyType::Circumecenter( tri.center ),
            geometry: GeometryType::Polygon{ coordinates:  [[..tri, tri[0]]] }
            }
        ).collect()
        }
      );
    });

    //   v.mesh = function(data) {
    //     if (data !== undefined) {
    //       v(data);
    //     }
    //     if (!v.delaunay) return false;
    //     return {
    //       type: "MultiLineString",
    //       coordinates: v.delaunay.edges.map(e => [v.points[e[0]], v.points[e[1]]])
    //     };
    //   };

    let mesh = Box::new(|data: DataType<F>| -> Option<DataObject<F>> {
      match data {
        DataType::Blank => {}
        _ => {
          Voronoi::voronoi(data);
        }
      }

      match v.delaunay {
        None => {
          return None;
        }
        Some(delaunay) => {
          let coordinates = delaunay
            .iter()
            .map(|e| {
              [v.points[*e[0]], v.points[*e[1]]];
            })
            .collect();
          return Some(DataObject::MultiLineString { coordinates });
        }
      }
    });


    //   v._found = undefined;
//   v.find = function(x, y, radius) {
//     v._found = v.delaunay.find(x, y, v._found);
//     if (!radius || geoDistance([x, y], v.points[v._found]) < radius)
//       return v._found;
//   };

    let found: Option<F>;
    let find = Box::new(|x: F, y: F, radius: Option<F>| -> Option<DataObject<F>> {
      if radius.is_none() ||  distance([x,y], v.points[v.found ] < radius.unwrap()) {
        return v.found;
      }
      else {
        return None;
      }
    });

    let hull = Box::new(|data: DataType<F>| -> Option<DataObject<F>> {
      match data {
        DataType::Blank => {}
        _ => {
          Voronoi::voronoi(data);
        }
      }

      match Some(data) {
        voronoi(data);
        return None;
      },
      None => {
        const hull = v.delaunay.hull;
        const points = v.points;
        return match hull.len() {
          0 => { None },
          _ => {
            Some(DataObject::Polygon {
              coordinates: [ [..hull.map(|i| =>  points[i]).collect(), points[hull[0]]]];
            });

          }
        }
    }
  });

      // v.hull = function(data) {
      //   if (data !== undefined) {
      //     v(data);
      //   }
      //   const hull = v.delaunay.hull,
      //     points = v.points;
      //   return hull.length === 0
      //     ? null
      //     : {
      //         type: "Polygon",
      //         coordinates: [[...hull.map(i => points[i]), points[hull[0]]]]
      //       };
      // };





    //   if (typeof v._data === "object") {
    //     const temp = v._data
    //       .map(d => [v._vx(d), v._vy(d), d])
    //       .filter(d => isFinite(d[0] + d[1]));
    //     v.points = temp.map(d => [d[0], d[1]]);
    //     v.valid = temp.map(d => d[2]);
    //     v.delaunay = geoDelaunay(v.points);obj{d, ..}
    //   }
    //   return v;
    // };

    match v.data {
      DataType::Vec(data) => {
        let temp = data
          .iter()
          .map(|d| {
            return [vx(d), vy(d), d];
          })
          .filter(|d| (d[0] + d[1]).is_finite())
          .collect();

        points = temp.iter().map(|d| [d[0], d[1]]).collect();
        valid = temp.map(|d| d[2]).collect();
        delaunay = delaunay_from(&points);
      }
      _ => {}
    }

    let v = Voronoi {
      data,
      delaunay,
      points,
      mesh,
      link,
      triangles,
      valid,
      vx,
      vy,
      // fn x(self f: Option<F>) {
      //   match f {
      //     Some(f) {
      //       return _vx;
      //     }
      //     _vx = f;
      //     return self;
      //   }
      // }

      // v.x = function(f) {
      //   if (!f) return v._vx;
      //   v._vx = f;
      //   return v;
      // };
      // v.y = function(f) {
      //   if (!f) return v._vy;
      //   v._vy = f;
      //   return v;
      // };

      // v.triangles = function(data) {
      //   if (data !== undefined) {
      //     v(data);
      //   }
      //   if (!v.delaunay) return false;

      //   return {
      //     type: "FeatureCollection",
      //     features: v.delaunay.triangles
      //       .map((tri, index) => {
      //         tri = tri.map(i => v.points[i]);
      //         tri.center = v.delaunay.centers[index];
      //         return tri;
      //       })
      //       .filter(tri => excess(tri) > 0)
      //       .map(tri => ({
      //         type: "Feature",
      //         properties: {
      //           circumcenter: tri.center
      //         },
      //         geometry: {
      //           type: "Polygon",
      //           coordinates: [[...tri, tri[0]]]
      //         }
      //       }))
      //   };
      // };



      // v.hull = function(data) {
      //   if (data !== undefined) {
      //     v(data);
      //   }
      //   const hull = v.delaunay.hull,
      //     points = v.points;
      //   return hull.length === 0
      //     ? null
      //     : {
      //         type: "Polygon",
      //         coordinates: [[...hull.map(i => points[i]), points[hull[0]]]]
      //       };
      // };


    };

    return match data {
      DataType::Blank => v,
      _ => v(data),
    };
  }
}
// export function geoVoronoi(data) {
//   const v = function(data) {
//     v.delaunay = null;
//     v._data = data;

//     if (typeof v._data === "object" && v._data.type === "FeatureCollection") {
//       v._data = v._data.features;
//     }
//     if (typeof v._data === "object") {
//       const temp = v._data
//         .map(d => [v._vx(d), v._vy(d), d])
//         .filter(d => isFinite(d[0] + d[1]));
//       v.points = temp.map(d => [d[0], d[1]]);
//       v.valid = temp.map(d => d[2]);
//       v.delaunay = geoDelaunay(v.points);
//     }
//     return v;
//   };

//   v._vx = function(d) {
//     if (typeof d == "object" && "type" in d) {
//       return geoCentroid(d)[0];
//     }
//     if (0 in d) return d[0];
//   };

//   v._vy = function(d) {
//     if (typeof d == "object" && "type" in d) {
//       return geoCentroid(d)[1];
//     }
//     if (1 in d) return d[1];
//   };

//   v.x = function(f) {
//     if (!f) return v._vx;
//     v._vx = f;
//     return v;
//   };
//   v.y = function(f) {
//     if (!f) return v._vy;
//     v._vy = f;
//     return v;
//   };

//   v.polygons = function(data) {
//     if (data !== undefined) {
//       v(data);
//     }

//     if (!v.delaunay) return false;
//     const coll = {
//       type: "FeatureCollection",
//       features: []
//     };
//     if (v.valid.length === 0) return coll;
//     v.delaunay.polygons.forEach((poly, i) =>
//       coll.features.push({
//         type: "Feature",
//         geometry: !poly
//           ? null
//           : {
//               type: "Polygon",
//               coordinates: [[...poly, poly[0]].map(i => v.delaunay.centers[i])]
//             },
//         properties: {
//           site: v.valid[i],
//           sitecoordinates: v.points[i],
//           neighbours: v.delaunay.neighbors[i] // not part of the public API
//         }
//       })
//     );
//     if (v.valid.length === 1)
//       coll.features.push({
//         type: "Feature",
//         geometry: { type: "Sphere" },
//         properties: {
//           site: v.valid[0],
//           sitecoordinates: v.points[0],
//           neighbours: []
//         }
//       });
//     return coll;
//   };

//   v.triangles = function(data) {
//     if (data !== undefined) {
//       v(data);
//     }
//     if (!v.delaunay) return false;

//     return {
//       type: "FeatureCollection",
//       features: v.delaunay.triangles
//         .map((tri, index) => {
//           tri = tri.map(i => v.points[i]);
//           tri.center = v.delaunay.centers[index];
//           return tri;
//         })
//         .filter(tri => excess(tri) > 0)
//         .map(tri => ({
//           type: "Feature",
//           properties: {
//             circumcenter: tri.center
//           },
//           geometry: {
//             type: "Polygon",
//             coordinates: [[...tri, tri[0]]]
//           }
//         }))
//     };
//   };

//   v.links = function(data) {
//     if (data !== undefined) {
//       v(data);
//     }
//     if (!v.delaunay) return false;
//     const _distances = v.delaunay.edges.map(e =>
//         geoDistance(v.points[e[0]], v.points[e[1]])
//       ),
//       _urquart = v.delaunay.urquhart(_distances);
//     return {
//       type: "FeatureCollection",
//       features: v.delaunay.edges.map((e, i) => ({
//         type: "Feature",
//         properties: {
//           source: v.valid[e[0]],
//           target: v.valid[e[1]],
//           length: _distances[i],
//           urquhart: !!_urquart[i]
//         },
//         geometry: {
//           type: "LineString",
//           coordinates: [v.points[e[0]], v.points[e[1]]]
//         }
//       }))
//     };
//   };

//   v.mesh = function(data) {
//     if (data !== undefined) {
//       v(data);
//     }
//     if (!v.delaunay) return false;
//     return {
//       type: "MultiLineString",
//       coordinates: v.delaunay.edges.map(e => [v.points[e[0]], v.points[e[1]]])
//     };
//   };

//   v.cellMesh = function(data) {
//     if (data !== undefined) {
//       v(data);
//     }
//     if (!v.delaunay) return false;
//     const { centers, polygons } = v.delaunay;
//     const coordinates = [];
//     for (const p of polygons) {
//       if (!p) continue;
//       for (
//         let n = p.length, p0 = p[n - 1], p1 = p[0], i = 0;
//         i < n;
//         p0 = p1, p1 = p[++i]
//       ) {
//         if (p1 > p0) {
//           coordinates.push([centers[p0], centers[p1]]);
//         }
//       }
//     }
//     return {
//       type: "MultiLineString",
//       coordinates
//     };
//   };

//   v._found = undefined;
//   v.find = function(x, y, radius) {
//     v._found = v.delaunay.find(x, y, v._found);
//     if (!radius || geoDistance([x, y], v.points[v._found]) < radius)
//       return v._found;
//   };

//   v.hull = function(data) {
//     if (data !== undefined) {
//       v(data);
//     }
//     const hull = v.delaunay.hull,
//       points = v.points;
//     return hull.length === 0
//       ? null
//       : {
//           type: "Polygon",
//           coordinates: [[...hull.map(i => points[i]), points[hull[0]]]]
//         };
//   };

//   return data ? v(data) : v;
// }
