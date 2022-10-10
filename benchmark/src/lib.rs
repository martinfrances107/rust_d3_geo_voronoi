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
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use geo::Coordinate;
use geo::Geometry;
use geo::Geometry::Polygon;
use geo::MultiPoint;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::console_log;
use web_sys::window;
use web_sys::CanvasRenderingContext2d;
use web_sys::Document;
use web_sys::Event;
use web_sys::HtmlCanvasElement;
use web_sys::HtmlInputElement;
use web_sys::PerformanceMeasure;

use rust_d3_geo::data_object::FeatureCollection;
use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::path::context::Context;
use rust_d3_geo::projection::orthographic::Orthographic;
use rust_d3_geo::projection::Build;
use rust_d3_geo::projection::ProjectionRawBase;
use rust_d3_geo::projection::RotateSet;
use rust_d3_geo::stream::StreamDrainStub;
use rust_d3_geo_voronoi::voronoi::GeoVoronoi;
#[cfg(not(tarpaulin_include))]
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .expect("should have a window in this context")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[cfg(not(tarpaulin_include))]
fn get_document() -> Result<Document, JsValue> {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            Ok(document)
        } else {
            Err(JsValue::from_str("unable to get document"))
        }
    } else {
        Err(JsValue::from_str("Unable to get window."))
    }
}

/// Entry point.
#[cfg(not(tarpaulin_include))]
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_log!("run() - wasm entry point");
    let document = get_document()?;

    update_all()?; // call once for initial render before any changes

    attach_listener(&document)?;

    Ok(())
}

fn perf_to_system(amt: f64) -> SystemTime {
    let secs = (amt as u64) / 1_000;
    let nanos = (((amt as u64) % 1_000) as u32) * 1_000_000;
    UNIX_EPOCH + Duration::new(secs, nanos)
}

// Draw dot.
#[cfg(not(tarpaulin_include))]
fn update_canvas(document: &Document, size: u32) -> Result<(), JsValue> {
    let perf = document
        .get_element_by_id("perf")
        .unwrap()
        .dyn_into::<web_sys::HtmlParagraphElement>()?;

    // Grab canvas.
    let canvas = match document.get_element_by_id("c") {
        Some(element) => match element.dyn_into::<HtmlCanvasElement>() {
            Ok(canvas) => canvas,
            Err(_) => return Err(JsValue::from_str("#c is not a canvas element.")),
        },
        None => {
            return Err(JsValue::from_str("Did not find #c on the page."));
        }
    };

    let context = match canvas.get_context("2d") {
        Ok(o) => match o {
            Some(c) => match c.dyn_into::<CanvasRenderingContext2d>() {
                Ok(c) => c,
                Err(_) => {
                    return Err(JsValue::from_str("hello"));
                }
            },
            None => {
                return Err(JsValue::from_str("did not receive a context"));
            }
        },
        Err(_) => {
            return Err(JsValue::from_str("unable to get context"));
        }
    };

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

    let window = window().expect("should have a window in this context");
    let performance = window
        .performance()
        .expect("performance should be available");

    let width = canvas.width().into();
    let height = canvas.height().into();
    context.set_fill_style(&"black".into());
    context.set_stroke_style(&"black".into());
    context.fill_rect(0.0, 0.0, width, height);

    let mut ob = Orthographic::builder();

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

    let mut gv: GeoVoronoi<'_, _, _, StreamDrainStub<f64>, _, _, _, _, _> =
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

        let cs: Context = Context::new(context.clone());
        let pb = PathBuilder::new(cs);

        let t0 = performance.now();
        ob.rotate_set(&[t0 / 150_f64, 0_f64, 0_f64]);
        let ortho = ob.build();
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
                    context.begin_path();
                    path.object(&features.geometry[0]);
                    context.fill();
                    context.stroke();
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
    if let Some(span) = document.get_element_by_id("size-output") {
        span.set_text_content(Some(&format!("{}", new_size)));
        Ok(())
    } else {
        Err(JsValue::from_str("did not find #size-output on the page."))
    }
}

// Given a new size, sets all relevant DOM elements.
#[cfg(not(tarpaulin_include))]
fn update_all() -> Result<(), JsValue> {
    // get new size

    let document = get_document()?;

    if let Some(element) = document.get_element_by_id("size-range") {
        if let Ok(input_element) = element.dyn_into::<HtmlInputElement>() {
            if let Ok(new_size) = input_element.value().parse::<u32>() {
                update_canvas(&document, new_size)?;
                update_span(&document, new_size)?;
                Ok(())
            } else {
                Err(JsValue::from_str("Could not parse input to number."))
            }
        } else {
            Err(JsValue::from_str("Could not find #size-range on page."))
        }
    } else {
        Err(JsValue::from_str("Could not find #size-range on page."))
    }
}

#[cfg(not(tarpaulin_include))]
fn attach_listener(document: &Document) -> Result<(), JsValue> {
    let callback = Closure::wrap(Box::new(move |_evt: Event| {
        update_all().expect("Could not update");
    }) as Box<dyn Fn(_)>);

    match document.get_element_by_id("size-range") {
        Some(sr) => match sr.dyn_into::<HtmlInputElement>() {
            Ok(ie) => {
                ie.set_onchange(Some(callback.as_ref().unchecked_ref()));
            }
            _ => return Err(JsValue::from_str("Could not attach onchange")),
        },
        None => {
            return Err(JsValue::from_str(
                "aborting attach_listener: Could not find element.",
            ));
        }
    }

    callback.forget();
    Ok(())
}
