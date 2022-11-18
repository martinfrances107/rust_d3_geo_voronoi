#![cfg(not(tarpaulin_include))]

use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;
use std::iter::repeat_with;

#[macro_use]
extern crate lazy_static;
extern crate rand;

use geo::Geometry;
use geo::Geometry::Polygon;
use geo::MultiPoint;
use geo_types::Coord;

use rust_d3_geo::clip::circle::ClipCircleC;
use rust_d3_geo::clip::circle::ClipCircleU;
use rust_d3_geo::data_object::FeatureCollection;
use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::projection::builder::template::NoPCNU;
use rust_d3_geo::projection::builder::template::ResampleNoPCNC;
use rust_d3_geo::projection::builder::template::ResampleNoPCNU;
use rust_d3_geo::projection::orthographic::Orthographic;
use rust_d3_geo::projection::stereographic::Stereographic;
use rust_d3_geo::projection::Build;
use rust_d3_geo::projection::RawBase as ProjectionRawBase;
use rust_d3_geo::projection::RotateSet;
use rust_d3_geo::stream::DrainStub;
use rust_d3_geo_voronoi::voronoi::ConstructionError;
use rust_d3_geo_voronoi::voronoi::GeoVoronoi;

type GV<'a> = GeoVoronoi<
    'a,
    ClipCircleC<ResampleNoPCNC<DrainStub<f64>, Stereographic<DrainStub<f64>, f64>, f64>, f64>,
    ClipCircleU<ResampleNoPCNC<DrainStub<f64>, Stereographic<DrainStub<f64>, f64>, f64>, f64>,
    DrainStub<f64>,
    NoPCNU,
    Stereographic<DrainStub<f64>, f64>,
    ResampleNoPCNC<DrainStub<f64>, Stereographic<DrainStub<f64>, f64>, f64>,
    ResampleNoPCNU<Stereographic<DrainStub<f64>, f64>, f64>,
    f64,
>;

#[cfg(not(tarpaulin_include))]
lazy_static! {
    static ref SCHEME_CATEGORY10: [String; 10] = [
        String::from("#1f77b4"),
        String::from("#ff7f0e"),
        String::from("#2ca02c"),
        String::from("#d62728"),
        String::from("#9467bd"),
        String::from("#8c564b"),
        String::from("#e377c2"),
        String::from("#7f7f7f"),
        String::from("#bcbd22"),
        String::from("#17becf"),
    ];
}

#[cfg(not(tarpaulin_include))]
fn draw() -> Result<String, ConstructionError> {
    // size is the number of voronoi

    let size = 6000;
    let mut ortho_builder = Orthographic::builder();

    let sites = MultiPoint(
        repeat_with(rand::random)
            .map(|(x, y): (f64, f64)| {
                Coord {
                    x: 360_f64 * x,
                    y: 180_f64 * y - 90_f64,
                }
                .into()
            })
            .take(size as usize)
            .collect(),
    );

    let mut gv: GV = GeoVoronoi::new(Some(Geometry::MultiPoint(sites.clone())))?;

    ortho_builder.rotate_set(&[0_f64, 0_f64, 0_f64]);
    let ortho = ortho_builder.build();
    let mut path = PathBuilder::context_pathstring().build(ortho);

    let mut out = match gv.polygons(None) {
        None => {
            panic!("failed to get polygons");
        }
        Some(FeatureCollection(fc)) => {
            let mut paths = String::from("");
            for (i, features) in fc.iter().enumerate() {
                let d = match &features.geometry[0] {
                    Polygon(polygon) => path.object(&Geometry::Polygon(polygon.clone())),
                    _ => {
                        panic!("polygon not found");
                    }
                };
                let line = format!(
                    "<path d={:?} fill=\"{}\" stroke=\"black\" />",
                    d,
                    SCHEME_CATEGORY10[i % 10]
                );
                paths.push_str(&line);
            }
            paths
        }
    };

    for p in sites {
        let d = path.object(&Geometry::Point(p));
        let line = format!("<path d={:?} fill=\"white\" stroke=\"black\" />", d,);
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
