use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};

pub type JsResult = Result<(), JsValue>;

pub fn window() -> Window {
    web_sys::window().expect("could not find window")
}

pub fn document() -> Document {
    window()
        .document()
        .expect("could not find document for window")
}

pub fn canvas() -> HtmlCanvasElement {
    let canvas = document()
        .get_element_by_id("canvas")
        .expect("could not find canvas element");
    canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .expect("could not cast elment with id 'canvas' into a canvas element")
}

pub fn canvas_rendering_context_2d() -> CanvasRenderingContext2d {
    canvas()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap()
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("could not register request animation frame");
}

pub fn set_interval(f: &Closure<dyn FnMut()>, timeout: i32) {
    window()
        .set_interval_with_callback_and_timeout_and_arguments_0(f.as_ref().unchecked_ref(), timeout)
        .expect("could not register interval");
}
