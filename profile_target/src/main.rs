use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;
use std::iter::repeat_with;

#[macro_use]
extern crate lazy_static;
extern crate rand;

use geo::Geometry::Polygon;
use geo::{Coordinate, Geometry, MultiPoint};

use rust_d3_geo::clip::buffer::Buffer;
use rust_d3_geo::clip::circle::interpolate::Interpolate as InterpolateCircle;
use rust_d3_geo::clip::circle::line::Line as LineCircle;
use rust_d3_geo::clip::circle::pv::PV as PVCircle;
use rust_d3_geo::data_object::FeatureCollection;
use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::projection::builder::template::NoClipC;
use rust_d3_geo::projection::builder::template::NoClipU;
use rust_d3_geo::projection::builder::template::ResampleNoClipC;
use rust_d3_geo::projection::builder::template::ResampleNoClipU;
use rust_d3_geo::projection::orthographic::Orthographic;
use rust_d3_geo::projection::stereographic::Stereographic;
use rust_d3_geo::projection::Build;
use rust_d3_geo::projection::ProjectionRawBase;
use rust_d3_geo::projection::Rotate;
use rust_d3_geo::stream::Connected;
use rust_d3_geo::stream::StreamDrainStub;
use rust_d3_geo::stream::Unconnected;
use rust_d3_geo_voronoi::voronoi::GeoVoronoi;

type GV<'a> = GeoVoronoi<
    'a,
    StreamDrainStub<f64>,
    InterpolateCircle<f64>,
    LineCircle<Buffer<f64>, Buffer<f64>, Connected<Buffer<f64>>, f64>,
    LineCircle<
        StreamDrainStub<f64>,
        ResampleNoClipC<StreamDrainStub<f64>, Stereographic<StreamDrainStub<f64>, f64>, f64>,
        Connected<
            ResampleNoClipC<StreamDrainStub<f64>, Stereographic<StreamDrainStub<f64>, f64>, f64>,
        >,
        f64,
    >,
    LineCircle<
        StreamDrainStub<f64>,
        ResampleNoClipC<StreamDrainStub<f64>, Stereographic<StreamDrainStub<f64>, f64>, f64>,
        Unconnected,
        f64,
    >,
    NoClipC<StreamDrainStub<f64>, f64>,
    NoClipU<StreamDrainStub<f64>, f64>,
    Stereographic<StreamDrainStub<f64>, f64>,
    PVCircle<f64>,
    ResampleNoClipC<StreamDrainStub<f64>, Stereographic<StreamDrainStub<f64>, f64>, f64>,
    ResampleNoClipU<StreamDrainStub<f64>, Stereographic<StreamDrainStub<f64>, f64>, f64>,
    f64,
>;
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

fn draw() -> String {
    // size is the number of voronoi
    let size = 6000;
    // let s = path.object(&object);
    let ortho_builder = Orthographic::builder();

    let sites = MultiPoint(
        repeat_with(rand::random)
            .map(|(x, y): (f64, f64)| {
                Coordinate {
                    x: 360_f64 * x,
                    y: 180_f64 * y - 90_f64,
                }
                .into()
            })
            .take(size as usize)
            .collect(),
    );

    let mut gv: GV = GeoVoronoi::new(Some(Geometry::MultiPoint(sites)));

    let ortho = ortho_builder.rotate(&[0_f64, 0_f64, 0_f64]).build();
    let mut path = PathBuilder::context_pathstring().build(ortho);

    match gv.polygons(None) {
        None => {
            panic!("failed to get polygons");
        }
        Some(FeatureCollection(fc)) => {
            let mut paths = String::from("");
            for (i, features) in fc.iter().enumerate() {
                let d = match &features.geometry[0] {
                    Polygon(polygon) => {
                        // todo
                        path.object(&Geometry::Polygon(polygon.clone()))
                    }
                    _ => {
                        panic!("polygon not found");
                    }
                };
                let line = format!(
                    "<path d={:?} fill=\"{}\" stroke=\"white\" />",
                    d,
                    SCHEME_CATEGORY10[i % 10]
                );
                paths.push_str(&line);
            }
            paths
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::create("profile_output.html")?;
    let mut file = LineWriter::new(file);

    let header = b"
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
    >";

    file.write_all(header)?;

    let d_string = draw();
    let d = d_string.as_bytes();

    file.write_all(d)?;

    let tail = b"</svg></body></html>";

    file.write_all(tail)?;

    file.flush()?;

    Ok(())
}
