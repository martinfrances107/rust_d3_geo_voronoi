#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
//! # rust d3 geo voronoi
//!
//! Know bugs.
//!
//! When I convert this benchmark to run on f32's
//! The polygons are mis-shaped
//!
//! See the README.md.
extern crate js_sys;
extern crate wasm_bindgen_test;
extern crate web_sys;

mod utils;

use core::iter::repeat_with;

use geo::Geometry;
use geo::MultiPoint;
use geo_types::Coord;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::window;
use web_sys::CanvasRenderingContext2d;
use web_sys::Document;
use web_sys::HtmlCanvasElement;
use web_sys::Path2d;
use web_sys::Performance;

use d3_geo_rs::data_object::FeatureCollection;
use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::path::path2d_endpoint::Path2dEndpoint;
use d3_geo_rs::path::Result as PathResult;
use d3_geo_rs::projection::builder::types::BuilderCircleResampleNoClip;
use d3_geo_rs::projection::orthographic::Orthographic;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::RawBase as ProjectionRawBase;
use d3_geo_rs::projection::RotateSet;
use d3_geo_voronoi_rs::voronoi::Voronoi;

#[wasm_bindgen]
#[derive(Debug)]
/// State associated with render call.
pub struct Renderer {
    context2d: CanvasRenderingContext2d,
    ep: Path2dEndpoint,
    ob: BuilderCircleResampleNoClip<Path2dEndpoint, Orthographic<f64>, f64>,
    performance: Performance,
    scheme_category10: [&'static str; 10],
    sites: MultiPoint<f64>,
    black: &'static str,
    white: &'static str,
    gv: Voronoi<f64>,
}

#[wasm_bindgen]
impl Renderer {
    /// size is the point of points generated at random.
    ///
    /// # Panics
    ///
    /// If the document object is now available.
    ///
    /// # Errors
    ///
    /// If the canvas element in not in the DOM.
    /// If the CanvasRenderingContext2d object is not available.
    /// If the window is not available.
    ///
    pub fn new(size: u32) -> Result<Renderer, JsValue> {
        utils::set_panic_hook();

        let document = get_document()?;
        // Grab canvas.
        let canvas = match document.get_element_by_id("c") {
            Some(element) => match element.dyn_into::<HtmlCanvasElement>() {
                Ok(canvas) => canvas,
                Err(_) => {
                    return Err(JsValue::from_str(
                        "#c is not a canvas element.",
                    ))
                }
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
                        return Err(JsValue::from_str(
                            "Could not convert context.",
                        ));
                    }
                },
                None => {
                    return Err(JsValue::from_str(
                        "Did not receive a context.",
                    ));
                }
            },
            Err(_) => {
                return Err(JsValue::from_str("Unable to get context."));
            }
        };

        context2d.clear_rect(0_f64, 0_f64, 960_f64, 600_f64);

        let path2d = Path2d::new().unwrap();
        let ep = Path2dEndpoint::new(path2d);

        let scheme_category10: [&'static str; 10] = [
            "#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd", "#8c564b",
            "#e377c2", "#7f7f7f", "#bcbd22", "#17becf",
        ];

        let Some(w) = window() else {
            return Err(JsValue::from_str("new() Could not get window."));
        };

        let Some(performance) = w.performance() else {
            return Err(JsValue::from_str("new() Could not get performance."));
        };

        let mut out = Self {
            context2d,
            ep,
            black: "black",
            gv: Voronoi::default(),
            ob: Orthographic::builder(),
            performance,
            sites: MultiPoint(vec![]),
            scheme_category10,
            white: "white",
        };

        out.update(size)?;

        Ok(out)
    }

    ///Regenerate mesh points and associated data structures.
    ///
    /// This function is designed to be called as part of a
    /// HTML element onchange event, so I am using a
    /// update in-place strategy.
    ///
    /// # Errors
    ///
    /// If the generated points cannot be lead to malformed Mesh.
    ///
    pub fn update(&mut self, size: u32) -> Result<(), JsValue> {
        utils::set_panic_hook();
        self.sites = repeat_with(rand::random)
            .map(|(x, y): (f64, f64)| Coord {
                x: 360_f64 * x,
                y: 180_f64 * y - 90_f64,
            })
            .take(size as usize)
            .collect();

        let gp = Geometry::MultiPoint(self.sites.clone());

        self.gv = match Voronoi::try_from(gp) {
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
        self.context2d.clear_rect(0_f64, 0_f64, 960_f64, 600_f64);
        let t0 = self.performance.now();

        self.ob.rotate2_set(&[t0 / 150_f64, 0_f64]);
        let ortho = self.ob.build();

        let pb = PathBuilder::new(self.ep.clone());

        let mut path = pb.build(ortho);

        let FeatureCollection(fc) = self.gv.polygons();

        self.context2d.set_stroke_style_str(self.black);
        for (i, features) in fc.iter().enumerate() {
            self.context2d
                .set_fill_style_str(self.scheme_category10[i % 10]);
            let _ = path.object(&features.geometry[0]);
            let path2d = path.context.result();
            self.context2d.fill_with_path_2d(&path2d);
            self.context2d.stroke_with_path(&path2d);
        }

        // Render points.
        self.context2d.set_fill_style_str(self.white);
        self.context2d.set_stroke_style_str(self.black);
        for p in &self.sites {
            let _ = path.object(&Geometry::Point(*p));
            let path2d = path.context.result();
            self.context2d.fill_with_path_2d(&path2d);
            self.context2d.stroke_with_path(&path2d);
        }
    }
}

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
