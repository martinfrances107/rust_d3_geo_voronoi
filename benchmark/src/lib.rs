#![allow(clippy::pedantic)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
//! # rust d3 geo voronoi
//!
//! Know bugs.
//!
//! When I convert this benchmark to run on f32's
//! The polygons are mis-shaped
//!
//! See the README.md.
extern crate js_sys;
extern crate rand;
extern crate wasm_bindgen_test;
extern crate web_sys;

use std::cell::RefCell;
use std::iter::repeat_with;
use std::rc::Rc;

use geo::Coordinate;
use geo::Geometry;
use geo::Geometry::Polygon;
use geo::MultiPoint;
use js_sys::try_iter;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::console_log;
use web_sys::Document;
use web_sys::PerformanceMeasure;

use rust_d3_geo::clip::buffer::Buffer;
use rust_d3_geo::clip::circle::interpolate::Interpolate as InterpolateCircle;
use rust_d3_geo::clip::circle::line::Line as LineCircle;
use rust_d3_geo::clip::circle::pv::PV as PVCircle;
use rust_d3_geo::data_object::FeatureCollection;
use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::path::context::Context;
use rust_d3_geo::projection::builder::template::NoClipU;
use rust_d3_geo::projection::builder::template::ResampleNoClipC;
use rust_d3_geo::projection::builder::template::ResampleNoClipU;
use rust_d3_geo::projection::orthographic::Orthographic;
use rust_d3_geo::projection::stereographic::Stereographic;
use rust_d3_geo::projection::Build;
use rust_d3_geo::projection::ProjectionRawBase;
use rust_d3_geo::projection::RotateSet;
use rust_d3_geo::stream::Connected;
use rust_d3_geo::stream::StreamDrainStub;
use rust_d3_geo::stream::Unconnected;
use rust_d3_geo_voronoi::voronoi::GeoVoronoi;

#[cfg(not(tarpaulin_include))]
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .expect("should have a window in this context")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[cfg(not(tarpaulin_include))]
fn get_document() -> Result<Document, JsValue> {
    let window = web_sys::window().unwrap();
    Ok(window.document().unwrap())
}

/// Entry point.
#[cfg(not(tarpaulin_include))]
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_log!("run() - wasm entry point");
    let document = get_document()?;

    attach_listener(&document)?;

    Ok(())
}

// Draw dot.
#[cfg(not(tarpaulin_include))]
fn update_canvas(document: &Document, size: u32) -> Result<(), JsValue> {
    let size_range = document.get_element_by_id("size-range");
    let size_label = document.get_element_by_id("size-label");
    let perf = document
        .get_element_by_id("perf")
        .unwrap()
        .dyn_into::<web_sys::HtmlParagraphElement>()?;

    // Grab canvas.
    let canvas = document
        .get_element_by_id("c")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context_raw = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let context = context_raw;

    // Holds elapsed samples (use to compute the standard deviation).
    let mut elapsed_array: [f64; 200] = [0_f64; 200];
    // index into the elapsedArray 0..199
    let mut index = 0;

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

    let mut gv: GeoVoronoi<'_, StreamDrainStub<f64>, _, _, _, _, _, _, _, _, _, _> =
        GeoVoronoi::new(Some(Geometry::MultiPoint(sites.clone())));

    // let ortho = ortho_builder.rotate(&[0_f64, 0_f64, 0_f64]).build();
    // let mut path = pb.build(ortho);

    // let mut path;
    //     // Here we want to call `requestAnimationFrame` in a loop, but only a fixed
    //     // number of times. After it's done we want all our resources cleaned up. To
    //     // achieve this we're using an `Rc`. The `Rc` will eventually store the
    //     // closure we want to execute on each frame, but to start out it contains
    //     // `None`.
    //     //
    //     // After the `Rc` is made we'll actually create the closure, and the closure
    //     // will reference one of the `Rc` instances. The other `Rc` reference is
    //     // used to store the closure, request the first frame, and then is dropped
    //     // by this function.
    //     //
    //     // Inside the closure we've got a persistent `Rc` reference, which we use
    //     // for all future iterations of the loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        performance
            .mark("render_start")
            .expect("Failed render start");

        let cs: Context<f64> = Context::new(context.clone());
        let path_builder = PathBuilder::new(cs);

        let ob = ortho_builder.clone();
        let pb = path_builder;
        let t0 = performance.now();
        let ortho = ob.rotate_set(&[t0 / 150_f64, 0_f64, 0_f64]).build();
        let mut path = pb.build(ortho);

        performance
            .mark("projection_rebuilt")
            .expect("Failed projection rebuilt");
        match gv.polygons(None) {
            None => {
                console_log!("failed to get polygons");
            }
            Some(FeatureCollection(fc)) => {
                performance
                    .mark("computed_polygons")
                    .expect("Failed computed polygons");
                context.set_stroke_style(&"black".into());
                for (i, features) in fc.iter().enumerate() {
                    context.set_fill_style(&scheme_category10[i % 10]);
                    match &features.geometry[0] {
                        Polygon(polygon) => {
                            context.begin_path();
                            path.object(&Geometry::Polygon(polygon.clone()));
                            context.fill();
                            context.stroke();
                        }
                        _ => {
                            console_log!("polygon not found");
                        }
                    }
                }
            }
        }
        performance
            .mark("polygons_rendered")
            .expect("failed polgons rendered");

        // Render points.
        context.set_fill_style(&"white".into());
        context.set_stroke_style(&"black".into());
        for p in &sites {
            context.begin_path();
            path.object(&Geometry::Point(*p));
            context.fill();
            context.stroke();
        }
        let t1 = performance.now();
        performance
            .mark("points_rendered")
            .expect("Failed points rendered");
        performance
            .measure_with_start_mark_and_end_mark(
                "rebuilding_projection",
                "render_start",
                "projection_rebuilt",
            )
            .expect("failed measure_with_start_mark_and_end_mark");
        performance
            .measure_with_start_mark_and_end_mark(
                "computing_polygons",
                "projection_rebuilt",
                "computed_polygons",
            )
            .expect("Failed computed polygons");
        performance
            .measure_with_start_mark_and_end_mark(
                "rendering_polygons",
                "computed_polygons",
                "polygons_rendered",
            )
            .expect("Failed polygons rendered.");
        performance
            .measure_with_start_mark_and_end_mark(
                "rendering_points",
                "polygons_rendered",
                "points_rendered",
            )
            .expect("Failed points rendered.");
        performance
            .measure_with_start_mark_and_end_mark("total", "render_start", "points_rendered")
            .expect("failed points rendered.");

        // let entries = performance.get_entries_by_type("measure");
        // let iter = try_iter(&entries).expect("failed entries iter");
        // for e in iter.unwrap() {
        //     let eu = e.unwrap();
        //     let pm = eu
        //         .dyn_into::<PerformanceMeasure>()
        //         .expect("failed getting e");
        //     console_log!(" {:?} {:.3} ms", pm.name(), pm.duration());
        // }

        // Compute the mean elapsed time and compute the standard deviation based on the
        // // the last 200 samples.
        let elapsed = t1 - t0;
        index = (index + 1) % 200;
        elapsed_array[index] = elapsed;

        let n = elapsed_array.len() as f64;

        let mean: f64 = elapsed_array.iter().sum::<f64>() / n;

        let std_dev = (elapsed_array
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / n)
            .sqrt();

        perf.set_inner_html(&format!(
            "{} Mean Render Time: {} +/- {} ms",
            index, mean, std_dev
        ));

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

// Update the size-output span.
#[cfg(not(tarpaulin_include))]
fn update_span(document: &Document, new_size: u32) -> Result<(), JsValue> {
    let span = document.get_element_by_id("size-output").unwrap();
    span.set_text_content(Some(&format!("{}", new_size)));
    Ok(())
}

// Given a new size, sets all relevant DOM elements.
#[cfg(not(tarpaulin_include))]
fn update_all() -> Result<(), JsValue> {
    // get new size
    let document = get_document()?;
    let new_size = document
        .get_element_by_id("size-range")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()?
        .value()
        .parse::<u32>()
        .expect("Could not parse slider value");
    update_canvas(&document, new_size)?;
    update_span(&document, new_size)?;
    Ok(())
}

#[cfg(not(tarpaulin_include))]
fn attach_listener(document: &Document) -> Result<(), JsValue> {
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
