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

mod utils;

use std::iter::repeat_with;

use geo::Coordinate;
use geo::Geometry;
use geo::MultiPoint;
use rust_d3_geo::clip::circle::ClipCircleC;
use rust_d3_geo::clip::circle::ClipCircleU;
use rust_d3_geo::projection::builder::template::NoPCNU;
use rust_d3_geo::projection::builder::template::ResampleNoPCNC;
use rust_d3_geo::projection::builder::template::ResampleNoPCNU;
use rust_d3_geo::projection::builder::types::BuilderCircleResampleNoClip;
use rust_d3_geo::projection::stereographic::Stereographic;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::console_log;
use web_sys::window;
use web_sys::CanvasRenderingContext2d;
use web_sys::Document;
use web_sys::Performance;
// use web_sys::Event;
use web_sys::HtmlCanvasElement;
// use web_sys::HtmlInputElement;

use rust_d3_geo::data_object::FeatureCollection;
use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::path::context::Context;
use rust_d3_geo::projection::orthographic::Orthographic;
use rust_d3_geo::projection::Build;
use rust_d3_geo::projection::ProjectionRawBase;
use rust_d3_geo::projection::RotateSet;
use rust_d3_geo_voronoi::voronoi::GeoVoronoi;

type GV = GeoVoronoi<
    'static,
    ClipCircleC<ResampleNoPCNC<Context, Stereographic<Context, f64>, f64>, f64>,
    ClipCircleU<ResampleNoPCNC<Context, Stereographic<Context, f64>, f64>, f64>,
    Context,
    NoPCNU<Context>,
    Stereographic<Context, f64>,
    ResampleNoPCNC<Context, Stereographic<Context, f64>, f64>,
    ResampleNoPCNU<Context, Stereographic<Context, f64>, f64>,
    f64,
>;

#[wasm_bindgen]
#[derive(Debug)]
/// State associated with render call.
pub struct Renderer {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    ob: BuilderCircleResampleNoClip<Context, Orthographic<Context, f64>, f64>,
    performance: Performance,
    scheme_category10: [JsValue; 10],
    sites: MultiPoint<f64>,
    gv: GV,
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
impl Renderer {
    /// size is the point of points generated at random.
    pub fn new(size: u32) -> Result<Renderer, JsValue> {
        utils::set_panic_hook();
        log!("entry: new()");

        let document = get_document()?;
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
                        return Err(JsValue::from_str("Could not convert context."));
                    }
                },
                None => {
                    return Err(JsValue::from_str("Did not receive a context."));
                }
            },
            Err(_) => {
                return Err(JsValue::from_str("Unable to get context."));
            }
        };

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

        let w = match window() {
            Some(w) => w,
            None => {
                return Err(JsValue::from_str("Could not get window."));
            }
        };

        let performance = match w.performance() {
            Some(p) => p,
            None => {
                return Err(JsValue::from_str("Could not get performance."));
            }
        };

        let ob = Orthographic::builder();

        console_log!("size {:?}", size);
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

        let gp = Geometry::MultiPoint(sites.clone());

        let gv = match GeoVoronoi::new(Some(gp)) {
            Ok(gv) => gv,
            Err(_) => {
                return Err(JsValue::from_str("Could not GeoVoronoi mesh."));
            }
        };

        Ok(Self {
            canvas,
            context,
            gv,
            ob,
            performance,
            sites,
            scheme_category10,
        })
    }

    /// Render the next frame.
    pub fn render(&mut self) {
        utils::set_panic_hook();

        let width = self.canvas.width().into();
        let height = self.canvas.height().into();
        self.context.set_fill_style(&"black".into());
        self.context.set_stroke_style(&"black".into());
        self.context.fill_rect(0.0, 0.0, width, height);

        let cs: Context = Context::new(self.context.clone());
        let t0 = self.performance.now();
        self.ob.rotate_set(&[t0 / 150_f64, 0_f64, 0_f64]);
        let ortho = self.ob.build();

        let pb = PathBuilder::new(cs);

        let mut path = pb.build(ortho);

        match self.gv.polygons(None) {
            None => {
                console_log!("Failed to get polygons.");
            }
            Some(FeatureCollection(fc)) => {
                if self.performance.mark("computed_polygons").is_err() {
                    log!("Failed to compute polygons.");
                }

                self.context.set_stroke_style(&"black".into());
                for (i, features) in fc.iter().enumerate() {
                    self.context.set_fill_style(&self.scheme_category10[i % 10]);
                    self.context.begin_path();
                    path.object(&features.geometry[0]);
                    self.context.fill();
                    self.context.stroke();
                }
            }
        }

        // Render points.
        self.context.set_fill_style(&"white".into());
        self.context.set_stroke_style(&"black".into());
        for p in &self.sites {
            self.context.begin_path();
            path.object(&Geometry::Point(*p));
            self.context.fill();
            self.context.stroke();
        }
    }
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
