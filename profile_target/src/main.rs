#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![cfg(not(tarpaulin_include))]

//! A benchmark used to profile the library.

use core::iter::repeat_with;
use std::fs::File;
use std::io::LineWriter;
use std::io::Write;

extern crate rand;

use geo::Geometry;
use geo::Geometry::Polygon;
use geo::MultiPoint;
use geo_types::Coord;

use d3_geo_rs::data_object::FeatureCollection;
use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::projection::orthographic::Orthographic;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::RawBase as ProjectionRawBase;
use d3_geo_rs::projection::RotateSet;
use d3_geo_voronoi_rs::voronoi::ConstructionError;
use d3_geo_voronoi_rs::voronoi::Voronoi;

static SCHEME_CATEGORY10: [&'static str; 10] = [
    "#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd", "#8c564b",
    "#e377c2", "#7f7f7f", "#bcbd22", "#17becf",
];

#[cfg(not(tarpaulin_include))]
fn draw() -> Result<String, ConstructionError> {
    // size is the number of voronoi

    let size = 6000_u32;
    let mut ortho_builder = Orthographic::builder();

    let sites: MultiPoint = repeat_with(rand::random)
        .map(|(x, y): (f64, f64)| Coord {
            x: 360_f64 * x,
            y: 180_f64 * y - 90_f64,
        })
        .take(size as usize)
        .collect();

    // TODO this is a pofile code, can I remove the clone.
    let gv = Voronoi::try_from(Geometry::MultiPoint(sites.clone()))?;

    ortho_builder.rotate2_set(&[0_f64, 0_f64]);
    let ortho = ortho_builder.build();
    let mut path = PathBuilder::pathstring().build(ortho);

    let mut out = String::new();
    let FeatureCollection(fc) = gv.polygons();

    for (i, features) in fc.iter().enumerate() {
        let d = match &features.geometry[0] {
            Polygon(polygon) => {
                path.object(&Geometry::Polygon(polygon.clone()))
            }
            _ => {
                panic!("polygon not found");
            }
        };

        if !d.is_empty() {
            let line = format!(
                "<path d={:?} fill=\"{}\" stroke=\"black\" />",
                d,
                SCHEME_CATEGORY10[i % 10]
            );
            out.push_str(&line);
        }
    }

    for p in sites {
        let d = path.object(&Geometry::Point(p));
        let line = format!("<path d={d:?} fill=\"white\" stroke=\"black\" />");
        out.push_str(&line);
    }
    Ok(out)
}

#[cfg(not(tarpaulin_include))]
fn main() -> std::io::Result<()> {
    let file = File::create("profile_output.html")?;
    let mut file = LineWriter::new(file);

    file.write_all(b"
    <!DOCTYPE html>
    <html lang=\"en\">
    <head>
    <title>Profile Target</title>
    <meta charset=\"utf-8\">
    <meta name=\"description\" content=\"Complex output used for profiling.\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
    </head>
    <body>
    <h1>Project: rust_d3_geo_voronoi</h1>
    <p>
     A Complex SVG used for profiling.
    </p>
    <?xml version=\"1.0\" standalone=\"no\"?><!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">
    <svg version=\"1.1\"
      width=\"1280\"
      height=\"640\"
      viewBox=\"0 0 1200 518\"
      xmlns=\"http://www.w3.org/2000/svg\"
    >")?;

    // file.write_all(draw().as_bytes())?;
    match draw() {
        Ok(d) => {
            file.write_all(d.as_bytes())?;
            file.write_all(b"</svg></body></html>")?;

            file.flush()?;

            Ok(())
        }
        Err(_) => Ok(()),
    }
}
