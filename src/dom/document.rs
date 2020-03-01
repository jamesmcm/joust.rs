use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const TARGET_DIV_ID: &str = "wasm_target";
const DEFAULT_CANVAS_WIDTH: usize = 800;
const DEFAULT_CANVAS_HEIGHT: usize = 600;

pub fn build_ui() -> Result<
    ((
        web_sys::HtmlCanvasElement,
        web_sys::CanvasRenderingContext2d,
    )),
    JsValue,
> {
    let document = document();

    let targetdiv = wasm_target_div();

    // Add canvas object
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    targetdiv.append_child(&canvas)?;

    // TODO: Handle dynamic canvas size on window resize
    canvas.set_width(DEFAULT_CANVAS_WIDTH as u32);
    canvas.set_height(DEFAULT_CANVAS_HEIGHT as u32);

    canvas.style().set_property("border", "solid")?;
    canvas.set_id("canvas");

    let mut context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    Ok((canvas, context))
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn wasm_target_div() -> web_sys::Element {
    document()
        .get_element_by_id(TARGET_DIV_ID)
        .expect("target div should exist")
}

// TODO
// fn request_animation_frame(f: &Closure<dyn FnMut()>) {
//     unsafe {
//         ANIMATION_ID = Some(
//             window()
//                 .request_animation_frame(f.as_ref().unchecked_ref())
//                 .expect("should register `requestAnimationFrame` OK"),
//         );
//     }
// }
