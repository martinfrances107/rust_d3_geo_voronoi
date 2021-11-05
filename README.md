# [Rust D3 geo voronoi](<https://github.com/martinfrances107/rust_d3_geo_voronoi>)

( rust 2021 edition.)

This is a port of the [d3-geo-voronoi](<https://github.com/Fil/d3-geo-voronoi>) library into a [RUST](<https://www.rust-lang.org/>) library.

 It is in a very early development phase, but to show progress. The library can be used to compute the following delaunay mesh.
![Delaunay mesh from a set of random points on a sphere](./lamp.png "Delaunay mesh from a set of random points on a sphere")

( computed from 6,000 points on a sphere selected at random )

Currently we have 82% test coverage ( as reported by cargo tarpaulin -o Html )

## Phase 1: Porting

Early draft port - sub module by submodule. Sub module porting means the test have also been ported.
No API stability guarentees.

## Phase 2: Performance Testing

API finialization. There maybe optimisation in the area of generics. So the API only gets locked down in phase 2.
 The code will be profiled and bottlenecks identified.

Modules, passing test ready for phase 2 evaluation :-

## Phase 3: Other Development.

To be ported:-

contour.js and geo-contour-test.js
geo_voronoi_test "geoVoronoi.hull does not break on difficult polygons"

## Demo Page
A demonstration webpage is supplied which acts as both a benchmark and a template for running this library in a browser.

## Instructions
It used [wasm pack](<https://github.com/rustwasm/wasm-pack>) to package the following packages into a single wasm file. [rust_d3_geo](<https://github.com/martinfrances107/rust_d3_geo>), [rust_d3_delaunay](<https://github.com/martinfrances107/rust_d3_delaunay>). From this,  a simple demo application constructed.
The application displays a delaunay mesh of a large number of cells onto a sphere.

* To build and start the web server:-

        cd benchmark
        npm install
        npm run start

the last command automatically start you web browser.

<br/>

## Module Dependencies

* [`delaunator`]("https://github.com/mourner/delaunator-rs.git") for generation of the delaunay mesh.

* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)

* [`rust_d3_array`]("https://github.com/martinfrances107/rust_d3_array.git") for a common set of helpers.

* [`rust_d3_delaunay`]("https://github.com/martinfrances107/rust_d3_delaunay.git")

* [`rust_d3_geo`]("https://github.com/martinfrances107/rust_d3_geo.git")

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating between WebAssembly and JavaScript. For logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.

* [`wasm-pack`](https://github.com/rustwasm/wasm-pack), A generator used to created all the glue code to create the rust benchamrk web-app.
