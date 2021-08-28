#![allow(clippy::pedantic)]
// #![warn(missing_debug_implementations)]
// #![warn(missing_docs)]
///! # rust d3 geo voronoi
///
/// See the README.md.

/// Allows debug to be autoderived from complex structs.
extern crate derivative;
extern crate geo;
extern crate rust_d3_array;
extern crate rust_d3_geo;

/// delaunay helper functions.
pub mod delaunay;
/// A set of helper methods.
pub mod math;

pub mod voronoi;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
