#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
//! A port of [d3/d3-geo-voronoi](<https://github.com/Fil/d3-geo-voronoi>).
//!
//! Voronoi / Delaunay tessellations on the sphere.
//!
//! <hr>
//!
//! Repository [`rust_d3_geo`](<https://github.com/martinfrances107/rust_d3_geo_voronoi>)

extern crate d3_geo_rs;
/// Allows debug to be auto-derived from complex structs.
extern crate float_next_after;
extern crate geo;
/// Delaunay helper functions.
pub mod delaunay;

/// Wrapper for Delaunay contains helper methods.
pub mod voronoi;

mod extent;
