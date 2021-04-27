#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::needless_return)]
extern crate derivative;
extern crate geo;
extern crate rust_d3_array;
extern crate rust_d3_geo;

pub mod delaunay;
pub mod math;
mod utils;
pub mod voronoi;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
