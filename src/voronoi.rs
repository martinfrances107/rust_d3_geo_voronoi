//
// (c) 2018 Philippe Riviere
//
// https://github.com/Fil/
//
// This software is distributed under the terms of the MIT License

// use geo::algorithm::centroid::Centroid;
// use geo::{Point, Polygon};

// use { extent } from "d3-array";
// use { geoCentroid, geoDistance } from "d3-geo";
// use { geoDelaunay, excess } from "./delaunay.js";
// use crate::delaunay::Delaunay;
// use geo::Point;
// use geo::Polygon;

// // use crate::math::TAU;

// // The data struct must implement .x() .y()
// trait D  {
//   fn x(&self) -> f64;
//   fn y(&self) -> f64;
// }


// pub struct GeoVoronoi {
//   delaunay: f64,
//   _data: Vec<f64>,
//   // _vx: fn(p: Polygon<f64>) -> Option<f64>,
//   // _vy: fn(p: Polygon<f64>) -> Option<f64>,
//   points: Vec<Point<f64>>,
//   polygons: Vec<Polygon<f64>>,
//   valid: Vec<f64>,
// }

// impl GeoVoronoi {
//   pub fn new<T: D>(data:Vec<T>) -> Self {
//     // Filter out invalid input.
//     let temp = data
//       .into_iter()
//       .filter(|d| (d.x() + d.y()).is_finite());

//     // From filtered input store Point<f64>
//     let points = temp.map(|d| (d.x(), d.y()).into());

//     // From filtered input store those keep.
//     let valid = temp.map(|d| d);

//     // let delaunay = geoDelaunay(points);
//     let delaunay = Delaunay::from_points(points);

//     return GeoVoronoi {
//       delaunay,
//       // _data: data,
//       points,
//       // _vx: x,
//       // _vy: y,
//       // polygons,
//       valid,

//     };

//   }
// }
