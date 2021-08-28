# Project: rust d3 geo voronoi

This is a port of the [d3-geo-voronoi](https://github.com/Fil/d3-geo-voronoi) library into a RUST library crate/package. It is in a very early development phase.

## Phase 1

Early draft port - sub module by submodule. Sub module porting means the test have also been ported.
No API stability guarentees.

delaunay first then  voronoi.

Currently we have 78% test coverage ( as reported by cargo tarpaulin -o Html )

## Phase 2

API finialization. There maybe optimisation in the area of generics. So the API only gets locked down in phase 2.
 The code will be profiled and bottlenecks identified.

Modules, passing test ready for phase 2 evaluation :-

## Instructions

* To Run the test suite :-

        cargo test

This module contains a benchmark library crate. That crate is used in a wasm-pack web application. A web site which can be deployed.

* To build and start the web server:-

        cd benchmark
        wasm-pack build
        cd www
        npm install
        npm run start

<br/>
<br/>
<br/>
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
