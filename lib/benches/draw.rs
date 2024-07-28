use core::iter::repeat_with;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use d3_geo_rs::data_object::FeatureCollection;
use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::projection::orthographic::Orthographic;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::RawBase as ProjectionRawBase;
use d3_geo_rs::projection::RotateSet;
use d3_geo_voronoi_rs::voronoi::ConstructionError;
use d3_geo_voronoi_rs::voronoi::Voronoi;
use geo::Geometry;
use geo::Geometry::Polygon;
use geo::MultiPoint;
use geo_types::Coord;

static SCHEME_CATEGORY10: [&'static str; 10] = [
    "#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd", "#8c564b",
    "#e377c2", "#7f7f7f", "#bcbd22", "#17becf",
];

fn draw(size: usize) -> Result<String, ConstructionError> {
    // size is the number of voronoi

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

fn criterion_benchmark(c: &mut Criterion) {
    let size = 600_usize;
    c.bench_function("draw600", |b| b.iter(|| draw(size)));
    let size = 6000_usize;
    c.bench_function("draw6000", |b| b.iter(|| draw(size)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
