use geo::MultiPoint;
use geo::Point;
use rand::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::Element;
use web_sys::HtmlElement;

mod dom_macros;

const STARTING_SIZE: u32 = 5;
const TWO_PI: f64 = 2.0 * std::f64::consts::PI;

type Result<T> = std::result::Result<T, JsValue>;

fn get_document() -> Result<Document> {
    let window = web_sys::window().unwrap();
    Ok(window.document().unwrap())
}

#[wasm_bindgen]
pub fn run() -> Result<()> {
    // get window/document/body
    // let window = web_sys::window().expect("Could not get window");
    let document = get_document()?;
    let body = document.body().expect("Could not get body");

    mount_app(&document, &body)?;
    attach_listener(&document)?;

    Ok(())
}

fn mount_canvas(document: &Document, parent: &Element) -> Result<()> {
    let p = create_element_attrs!(document, "p",);
    append_element_attrs!(
        document,
        p,
        "canvas",
        ("id", "dot-canvas"),
        ("width", "200"),
        ("height", "200")
    );
    parent.append_child(&p)?;
    Ok(())
}

fn mount_controls(document: &Document, parent: &HtmlElement) -> Result<()> {
    // containing div
    let div = create_element_attrs!(document, "div", ("id", "rxcanvas"));
    // span
    append_text_element_attrs!(
        document,
        div,
        "span",
        &format!("{}", STARTING_SIZE),
        ("id", "size-output")
    );
    // input
    append_element_attrs!(
        document,
        div,
        "input",
        ("id", "size"),
        ("type", "range"),
        ("min", "5"),
        ("max", "100"),
        ("step", "5")
    );
    // label
    append_text_element_attrs!(document, div, "label", "- Size", ("for", "size"));
    // canvas
    mount_canvas(&document, &div)?;
    parent.append_child(&div)?;
    Ok(())
}

fn mount_app(document: &Document, body: &HtmlElement) -> Result<()> {
    append_text_element_attrs!(document, body, "h1", "DOT",);
    mount_controls(&document, &body)?;
    Ok(())
}

// draw dot
fn update_canvas(document: &Document, size: u32) -> Result<()> {
    // grab canvas
    let canvas = document
        .get_element_by_id("dot-canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    // draw

    let width = canvas.width().into();
    let height = canvas.height().into();
    context.clear_rect(0.0, 0.0, width, height);

    let mut rng = rand::thread_rng();
    let mut sites: Vec<Point<f64>> = Vec::new();
    for _i in 0..size {
        sites.push(Point::new(
            rng.gen_range(0., width),
            rng.gen_range(0., height),
        ));
    }

    let mp = MultiPoint(sites);

    for p in mp {
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
