# [d3_geo_voronoi_rs](<https://github.com/martinfrances107/rust_d3_geo_voronoi>)

rust 2021 edition.

<div align="center">

<a href="https://crates.io/crates/d3_geo_voronoi_rs"><img alt="crates.io" src="https://img.shields.io/crates/v/d3_geo_voronoi_rs.svg"/></a>
<a href="https://docs.rs/d3_geo_voronoi_rs" rel="nofollow noopener noreferrer"><img src="https://docs.rs/d3_geo_voronoi_rs/badge.svg" alt="Documentation"></a>
<a href="https://crates.io/crates/d3_geo_voronoi_rs"><img src="https://img.shields.io/crates/d/d3_geo_voronoi_rs.svg" alt="Download" /></a>

</div>

## About

This is a port of the [d3-geo-voronoi](<https://github.com/Fil/d3-geo-voronoi>) library into a [RUST](<https://www.rust-lang.org/>) library.

 As a example the library can be used to compute the following delaunay mesh.
![Delaunay mesh from a set of random points on a sphere](./lamp.png "Delaunay mesh from a set of random points on a sphere")

( computed from 6,000 points on a sphere selected at random )

Currently we have 84% test coverage ( as reported by cargo tarpaulin -o Html )

A collection of d3 sub packages is being ported to rust.

* [d3_geo_rs](https://crates.io/crates/d3_geo_rs)
* [d3_delaunay_rs](https://crates.io/crates/d3_delaunay_rs)
* d3_geo_voronoi_rs

## Performance Profiling

### Demo Page

The original javascript library has a benchmark in the form of web page which records the number of frames displayed per second. For comparison the benchmark has been ported.

* javascript -  d3-geo_voronoi/src/benchmark/sphereCanvas.html
* rust -  rust_d3_geo_vornoi/benchmark.

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

from rust_d3_geo_voronoi

```console
cargo install  flamegraph
cargo flamegraph -- profile_target
```

## Further Development

Known bugs

Currently the demo works only then f64 is used as the floating point type. [For f32 the cell integrity is broken as some of the cell appear to takeup the whole sphere.]
To be ported:-

contour.js and geo-contour-test.js

Currenly there is a failing test suite
geo_voronoi_test.rs "geoVoronoi.hull does not break on difficult polygons"

## Instructions for building the benchmark

I'm using [wasm pack](<https://github.com/rustwasm/wasm-pack>) to package the
 following packages into a single wasm file.
 [rust_d3_geo](<https://github.com/martinfrances107/rust_d3_geo>),
 [rust_d3_delaunay](<https://github.com/martinfrances107/rust_d3_delaunay>).

The application displays a delaunay mesh of a large number of cells onto a sphere.

To build and start the web server:-

```console
cd benchmark\www
npm install
npm run start
```

The last command automatically starts your web browser.

For benchmarking, to obtain best performance

```console
cd benchmark\www
npm run build
npm run serve
```
