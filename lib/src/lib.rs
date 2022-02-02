#![allow(clippy::pedantic)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
//! A port of [d3/d3-delaunay](<https://github.com/Fil/d3-geo-voronoi>).
//!
//! Voronoi / Delaunay tessellations on the sphere.
//!
//! <hr>
//!
//! Repository [rust_d3_geo](<https://github.com/martinfrances107/rust_d3_geo_voronoi>)

/// Allows debug to be autoderived from complex structs.
extern crate derivative;
extern crate geo;
extern crate rust_d3_array;
extern crate rust_d3_geo;

/// delaunay helper functions.
pub mod delaunay;

/// Wrapper for Delaunay contains helper methods.
pub mod voronoi;
