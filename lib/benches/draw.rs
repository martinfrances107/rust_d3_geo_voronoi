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
use lazy_static::lazy_static;

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

    let mut gv = Voronoi::new(Some(Geometry::MultiPoint(sites.clone())))?;

    ortho_builder.rotate2_set(&[0_f64, 0_f64]);
    let ortho = ortho_builder.build();
    let mut path = PathBuilder::pathstring().build(ortho);

    let mut out = match gv.polygons(None) {
        None => {
            panic!("failed to get polygons");
        }
        Some(FeatureCollection(fc)) => {
            let mut paths = String::new();
            for (i, features) in fc.iter().enumerate() {
                let d = match &features.geometry[0] {
                    Polygon(polygon) => path.object(&Geometry::Polygon(polygon.clone())),
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
                    paths.push_str(&line);
                }
            }
            paths
        }
    };

    for p in sites {
        let d = path.object(&Geometry::Point(p));
        let line = format!("<path d={d:?} fill=\"white\" stroke=\"black\" />");
        out.push_str(&line);
    }
    Ok(out)
}

fn criterion_benchmark(c: &mut Criterion) {
    let size = 6000_usize;
    c.bench_function("draw", |b| b.iter(|| draw(size)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
