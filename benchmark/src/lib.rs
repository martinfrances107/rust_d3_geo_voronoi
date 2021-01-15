extern crate web_sys;

use geo::Coordinate;
use geo::Geometry;
use geo::MultiPoint;
use geo::Point;
use rand::prelude::*;
use rust_d3_geo::data_object::feature_collection::FeatureCollection;
use rust_d3_geo::projection::orthographic::OrthographicRaw;
use rust_d3_geo::Transform;
use rust_d3_geo_voronoi::voronoi::Voronoi;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::HtmlElement;

mod dom_macros;

const TWO_PI: f64 = 2.0 * std::f64::consts::PI;

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

#[wasm_bindgen]
pub fn run() -> Result<()> {
    console_log!("Hello {}!", "world");
    let document = get_document()?;
    let body = document.body().expect("Could not get body");

    mount_app(&document, &body)?;
    attach_listener(&document)?;

    Ok(())
}

fn mount_app(_document: &Document, _body: &HtmlElement) -> Result<()> {
    // mount_controls(&document, &body)?;
    Ok(())
}

// draw dot
fn update_canvas(document: &Document, size: u32) -> Result<()> {
    // grab canvas
    let canvas = document
        .get_element_by_id("c")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    // TODO can this be defined statically
    let scheme_category10 = vec![
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

    let width = canvas.width().into();
    let height = canvas.height().into();
    context.set_fill_style(&"black".into());
    context.set_stroke_style(&"black".into());
    context.fill_rect(0.0, 0.0, width, height);
    let mut rng = rand::thread_rng();
    let ortho = OrthographicRaw::gen_projection_mutator::<f64>();

    let mut sites: Vec<Point<f64>> = Vec::new();
    for _i in 0..size {
        let point = Coordinate {
            x: rng.gen_range(0., 360f64),
            y: rng.gen_range(-90f64, 90f64),
        };
        let t = ortho.transform(&point);
        
        sites.push(t.into());
    }

    let sites = MultiPoint(sites);

    match Voronoi::new(Some(Geometry::MultiPoint(sites.clone()))).polygons(None) {
        None => {
            console_log!("failed to get polygons");
        }
        Some(FeatureCollection(features)) => {
            context.set_stroke_style(&"black".into());
            for (i, g) in features[0].geometry.iter().enumerate() {
                context.set_fill_style(&scheme_category10[i % 10]);
                match g {
                    Geometry::Polygon(polygon) => {
                        let ls = polygon.exterior();
                        let l_iter = ls.lines();
                        context.begin_path();
                        for line in l_iter {
                            context.line_to(line.start.x, line.start.y);
                            context.line_to(line.end.x, line.end.x);
                        }
                        context.close_path();
                        context.fill();
                    }
                    _ => {
                        console_log!("expecting a polygon");
                    }
                }
            }

            context.set_fill_style(&"white".into());
            for p in sites {
                context.begin_path();
                context.arc(
                    p.x(),
                    p.y(),
                    5.0, // radius
                    0.0,
                    TWO_PI,
                )?;
                context.fill();
                context.stroke();
            }
            // }
        }
    }
    Ok(())
}

// update the size-output span
fn update_span(document: &Document, new_size: u32) -> Result<()> {
    let span = document.get_element_by_id("size-output").unwrap();
    span.set_text_content(Some(&format!("{}", new_size)));
    Ok(())
}

// given a new size, sets all relevant DOM elements
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
    // listen for size change events

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
