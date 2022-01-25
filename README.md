# [Rust D3 geo voronoi](<https://github.com/martinfrances107/rust_d3_geo_voronoi>)

( rust 2021 edition.)

This is a port of the [d3-geo-voronoi](<https://github.com/Fil/d3-geo-voronoi>) library into a [RUST](<https://www.rust-lang.org/>) library.

 As a example the library can be used to compute the following delaunay mesh.
![Delaunay mesh from a set of random points on a sphere](./lamp.png "Delaunay mesh from a set of random points on a sphere")

( computed from 6,000 points on a sphere selected at random )

Currently we have 82% test coverage ( as reported by cargo tarpaulin -o Html )

## Performance Profiling.

### Demo Page
The original javascript library has a benchmark in the form of web page which records the number of frames displayed per second. For comparison the benchmark has been ported.

 - javascript -  d3-geo_voronoi/src/benchmark/sphereCanvas.html
 - rust -  rust_d3_geo_vornoi/benchmark.


Measuring the performance of a library is compilcated, as different applications employing the library may see different results. Desktop and mobile perfomance may differ.

There are traditionally two way of measuring increases in performance :-

A) Speed: By asking how long it takes to perform a certain computation?

B) Throughput:  By asking how much more work can I perfom in a given time?

A and B are ideally linearly related, but often high throughput requires more memory which may increase page swapping. Javascipt uses garbage collection, rust does not. In garbage collected environments, the dynamics of a collectors behaviours may cause the figures reported by A and B to diverge.

Here are the results for the benchmark :-

A) When I tune the benchmark for 60ps, I find the javascript can render 600 points in 16ms. The rust version performs the same workload in 11ms. ( a 31% speedup ).

B) When I increase the number of points given to the RUST version to render, I find I can render 826 in 16ms. An increase in throughput of 37%.

I am currently looking to add more benchmarks, a diverse collection of example application code, will give greater confidence in what to expect.

### Profile Target
This workspace contain a binary "profie_target" which outputs the result of a computation similar to that of the demo_page. Before API finialization - I am currently  using cargo flamegraph to identify any obvious optimizations.

## Further Development.

To be ported:-

contour.js and geo-contour-test.js
geo_voronoi_test "geoVoronoi.hull does not break on difficult polygons"


## Instructions for building the benchmark.

I'm using [wasm pack](<https://github.com/rustwasm/wasm-pack>) to package the
 following packages into a single wasm file.
 [rust_d3_geo](<https://github.com/martinfrances107/rust_d3_geo>),
 [rust_d3_delaunay](<https://github.com/martinfrances107/rust_d3_delaunay>).

The application displays a delaunay mesh of a large number of cells onto a sphere.

* To build and start the web server:-

        cd benchmark
        npm install
        npm run start

the last command automatically start you web browser.

For benchmarking, to obtain best performance

         cd benchmark
         npm install
         npm run build

and then host rust_d3_geo_voronoi/benchmark/dist directory before viewing in a browser.
Timing specific information is output to the console.

<br/>

## Module Dependencies

* [`delaunator`]("https://github.com/mourner/delaunator-rs.git") for generation
   of the delaunay mesh.

* [`rust_d3_array`]("https://github.com/martinfrances107/rust_d3_array.git")
   for a common set of helpers.

* [`rust_d3_delaunay`]("https://github.com/martinfrances107/rust_d3_delaunay.git")

* [`rust_d3_geo`]("https://github.com/martinfrances107/rust_d3_geo.git")
