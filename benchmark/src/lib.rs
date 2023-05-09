#![allow(clippy::pedantic)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![cfg(not(tarpaulin_include))]

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

use d3_geo_rs::clip::circle::ClipCircleC;
use d3_geo_rs::clip::circle::ClipCircleU;
use d3_geo_rs::data_object::FeatureCollection;
use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::path::context::Context;
use d3_geo_rs::projection::builder::template::NoPCNU;
use d3_geo_rs::projection::builder::template::ResampleNoPCNC;
use d3_geo_rs::projection::builder::template::ResampleNoPCNU;
use d3_geo_rs::projection::builder::types::BuilderCircleResampleNoClip;
use d3_geo_rs::projection::orthographic::Orthographic;
use d3_geo_rs::projection::stereographic::Stereographic;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::RawBase as ProjectionRawBase;
use d3_geo_rs::projection::RotateSet;
use d3_geo_voronoi_rs::voronoi::Voronoi;

use geo::Geometry;
use geo::MultiPoint;
use geo_types::Coord;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;
use web_sys::CanvasRenderingContext2d;
use web_sys::Document;
use web_sys::HtmlCanvasElement;
use web_sys::Performance;

type GV = Voronoi<
    'static,
    ClipCircleC<ResampleNoPCNC<Context, Stereographic<Context, f64>, f64>, f64>,
    ClipCircleU<ResampleNoPCNC<Context, Stereographic<Context, f64>, f64>, f64>,
    Context,
    NoPCNU,
    Stereographic<Context, f64>,
    ResampleNoPCNU<Stereographic<Context, f64>, f64>,
    f64,
>;

#[wasm_bindgen]
#[derive(Debug)]
/// State associated with render call.
pub struct Renderer {
    context2d: CanvasRenderingContext2d,
    context: Context,
    ob: BuilderCircleResampleNoClip<Context, Orthographic<Context, f64>, f64>,
    performance: Performance,
    scheme_category10: [JsValue; 10],
    sites: MultiPoint<f64>,
    black: JsValue,
    white: JsValue,
    gv: GV,
}

#[wasm_bindgen]
impl Renderer {
    /// size is the point of points generated at random.
    pub fn new(size: u32) -> Result<Renderer, JsValue> {
        utils::set_panic_hook();

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

        let context2d = match canvas.get_context("2d") {
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

        let context: Context = Context::new(context2d.clone());

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
                return Err(JsValue::from_str("new() Could not get window."));
            }
        };

        let performance = match w.performance() {
            Some(p) => p,
            None => {
                return Err(JsValue::from_str("new() Could not get performance."));
            }
        };

        let ob = Orthographic::builder();

        // Insert dummy values.
        let sites = MultiPoint(vec![]);
        let gp = Geometry::MultiPoint(sites.clone());
        let gv = match Voronoi::new(Some(gp)) {
            Ok(gv) => gv,
            Err(_) => {
                return Err(JsValue::from_str("new() Could not GeoVoronoi mesh."));
            }
        };

        let mut out = Self {
            context2d,
            context,
            black: JsValue::from_str("black"),
            gv,
            ob,
            performance,
            sites,
            scheme_category10,
            white: JsValue::from_str("white"),
        };

        out.update(size)?;

        Ok(out)
    }

    ///Regenerate mesh points and associated data structures.
    ///
    /// This function is designed to be called as part of a
    /// HTML element onchange event, so I am using a
    /// update in-place stratergy.
    pub fn update(&mut self, size: u32) -> Result<(), JsValue> {
        utils::set_panic_hook();
        self.sites = MultiPoint(
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

        let gp = Geometry::MultiPoint(self.sites.clone());

        self.gv = match Voronoi::new(Some(gp)) {
            Ok(gv) => gv,
            Err(_) => {
                return Err(JsValue::from_str(
                    "update() Could not construct the GeoVoronoi mesh.",
                ));
            }
        };

        Ok(())
    }

    /// Render the next frame.
    pub fn render(&mut self) {
        let t0 = self.performance.now();
        self.ob.rotate2_set(&[t0 / 150_f64, 0_f64]);
        let ortho = self.ob.build();

        let pb = PathBuilder::new(self.context.clone());

        let mut path = pb.build(ortho);

        match self.gv.polygons(None) {
            None => {
                panic!("Failed to get polygons.");
            }
            Some(FeatureCollection(fc)) => {
                self.context2d.set_stroke_style(&"black".into());
                for (i, features) in fc.iter().enumerate() {
                    self.context2d
                        .set_fill_style(&self.scheme_category10[i % 10]);
                    self.context2d.begin_path();
                    path.object(&features.geometry[0]);
                    self.context2d.fill();
                    self.context2d.stroke();
                }
            }
        }

        // Render points.
        self.context2d.set_fill_style(&self.white);
        self.context2d.set_stroke_style(&self.black);
        for p in &self.sites {
            self.context2d.begin_path();
            path.object(&Geometry::Point(*p));
            self.context2d.fill();
            self.context2d.stroke();
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
