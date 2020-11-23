#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::needless_return)]
extern crate rust_d3_array;
extern crate rust_d3_geo;

pub mod delaunay;
pub mod math;
pub mod voronoi;

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust-d3-geo-voronoi!");
}