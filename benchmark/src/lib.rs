#![allow(clippy::pedantic)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
//! # rust d3 geo voronoi
//!
//! See the README.md.
extern crate js_sys;
extern crate rand;
extern crate web_sys;

use std::cell::RefCell;
use std::f64::consts::TAU;
use std::iter::repeat_with;
use std::rc::Rc;

use geo::Coordinate;
use geo::Geometry;
use geo::Geometry::Polygon;
use geo::MultiPoint;
use js_sys::try_iter;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::HtmlElement;
use web_sys::PerformanceMeasure;

use rust_d3_geo::clip::circle::pv::PV;
use rust_d3_geo::data_object::DataObject;
use rust_d3_geo::data_object::FeatureCollection;
use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::path::context::Context;
use rust_d3_geo::path::context_stream::ContextStream;
use rust_d3_geo::projection::orthographic::Orthographic;
use rust_d3_geo::projection::Raw;
use rust_d3_geo::projection::Rotate;
use rust_d3_geo::stream::StreamDrainStub;
use rust_d3_geo::Transform;
use rust_d3_geo_voronoi::voronoi::GeoVoronoi;

mod dom_macros;

type Result<T> = std::result::Result<T, JsValue>;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);

    fn alert(s: &str);
}

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn get_document() -> Result<Document> {
    let window = web_sys::window().unwrap();
    Ok(window.document().unwrap())
}

/// Entry point.
#[wasm_bindgen]
pub fn run() -> Result<()> {
    console_log!("run() - wasm entry point");
    let document = get_document()?;
    let body = document.body().expect("Could not get body");

    attach_listener(&document)?;

    Ok(())
}

// Draw dot.
fn update_canvas(document: &Document, size: u32) -> Result<()> {
    // Grab canvas.
    let canvas = document
        .get_element_by_id("c")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context_raw = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let context = Rc::new(context_raw);

    // TODO can this be defined statically
    let scheme_category10: [JsValue; 10] = [
        JsValue::from_str("#1f77b4"),
        JsValue::from_str("#ff7f0e"),
        JsValue::from_str("#2ca02c"),
        JsValue::from_str("#d62728"),
        JsValue::from_str("#9467bd"),
        JsValue::from_str("#8c564b"),
        JsValue::from_str("#e377c2"),
        JsValue::from_str("#7f7f7f"),
        JsValue::from_str("#bcbd22"),
        JsValue::from_str("#17becf"),
    ];

    let window = web_sys::window().expect("should have a window in this context");
    let performance = window
        .performance()
        .expect("performance should be available");

    let width = canvas.width().into();
    let height = canvas.height().into();
    context.set_fill_style(&"black".into());
    context.set_stroke_style(&"black".into());
    context.fill_rect(0.0, 0.0, width, height);

    let cs: ContextStream<f64> = ContextStream::C(Context::new(context.clone()));
    let pb: PathBuilder<Orthographic<ContextStream<f64>, f64>, PV<f64>, f64> =
        PathBuilder::new(Rc::new(RefCell::new(cs)));

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

    let mut gv: GeoVoronoi<StreamDrainStub<f64>, f64> =
        GeoVoronoi::new(Some(Geometry::MultiPoint(sites.clone())));

    performance.mark("render_start")?;
    let ortho = Rc::new(ortho_builder.rotate(&[0_f64, 0_f64, 0_f64]).build());
    let mut path = pb.build(ortho.clone());
    // this is not quite proejction rebuilt.
    performance.mark("projection_rebuilt")?;
    match gv.polygons(None) {
        None => {
            console_log!("failed to get polygons");
        }
        Some(FeatureCollection(fc)) => {
            performance.mark("computed_polygons")?;
            context.set_stroke_style(&"black".into());
            // console_log!("{:?}",fc);
            for (i, features) in fc.iter().enumerate() {
                // console_log!("i {}",i%10);
                context.set_fill_style(&scheme_category10[i % 10]);
                // console_log!("{:?}",features.geometry[0]);
                match &features.geometry[0] {
                    Polygon(polygon) => {
                        context.begin_path();
                        path.object(&DataObject::Geometry(Geometry::Polygon(polygon.clone())));
                        context.fill();
                        context.stroke();
                    }
                    _ => {
                        console_log!("polygon not found");
                    }
                }
            }
            performance.mark("polygons_rendered")?;
            // Render points.
            context.set_fill_style(&"white".into());
            for p in sites {
                // console_log!("{:?}", p);
                let pt = ortho.clone().transform(&p.into());
                context.begin_path();
                context.arc(
                    pt.x, pt.y, 5.0, // radius
                    0.0, TAU,
                )?;
                context.fill();
                context.stroke();
            }
            performance.mark("points_rendered")?;
            performance.measure_with_start_mark_and_end_mark(
                "rebuilding_projection",
                "render_start",
                "projection_rebuilt",
            )?;
            performance.measure_with_start_mark_and_end_mark(
                "computing_polygons",
                "projection_rebuilt",
                "computed_polygons",
            )?;
            performance.measure_with_start_mark_and_end_mark(
                "rendering_polygons",
                "computed_polygons",
                "polygons_rendered",
            )?;
            performance.measure_with_start_mark_and_end_mark(
                "rendering_points",
                "polygons_rendered",
                "points_rendered",
            )?;
            performance.measure_with_start_mark_and_end_mark(
                "total",
                "render_start",
                "points_rendered",
            )?;
            let entries = performance.get_entries_by_type("measure");
            let iter = try_iter(&entries)?;
            for e in iter.unwrap() {
                let eu = e.unwrap();
                let pm = eu.dyn_into::<PerformanceMeasure>()?;
                console_log!(" {:?} {:.3} ms", pm.name(), pm.duration());
            }
        }
    }
    Ok(())
}

// Update the size-output span.
fn update_span(document: &Document, new_size: u32) -> Result<()> {
    let span = document.get_element_by_id("size-output").unwrap();
    span.set_text_content(Some(&format!("{}", new_size)));
    Ok(())
}

// Given a new size, sets all relevant DOM elements.
fn update_all() -> Result<()> {
    // get new size
    let document = get_document()?;
    let new_size = document
        .get_element_by_id("size")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()?
        .value()
        .parse::<u32>()
        .expect("Could not parse slider value");
    update_canvas(&document, new_size)?;
    update_span(&document, new_size)?;
    Ok(())
}

fn attach_listener(document: &Document) -> Result<()> {
    update_all()?; // call once for initial render before any changes

    let callback = Closure::wrap(Box::new(move |_evt: web_sys::Event| {
        update_all().expect("Could not update");
    }) as Box<dyn Fn(_)>);

    document
        .get_element_by_id("size")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()?
        .set_onchange(Some(callback.as_ref().unchecked_ref()));

    callback.forget();

    Ok(())
}
