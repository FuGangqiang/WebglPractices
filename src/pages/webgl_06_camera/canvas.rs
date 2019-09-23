use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext as GL, *};

pub fn create_canvas() -> Result<HtmlCanvasElement, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let body_style = body.style();
    body_style.set_property("margin", "0")?;
    body_style.set_property("width", "100%")?;
    body_style.set_property("height", "100%")?;
    body_style.set_property("overflow", "hidden")?;

    let canvas_node = document.create_element("canvas")?;
    canvas_node.set_attribute("id", "webgl-canvas")?;
    body.append_child(&canvas_node)?;
    let canvas = canvas_node.dyn_into::<HtmlCanvasElement>()?;
    canvas.set_width(window.inner_width()?.as_f64().unwrap() as u32);
    canvas.set_height(window.inner_height()?.as_f64().unwrap() as u32);
    Ok(canvas)
}

pub fn create_webgl_context(canvas: &HtmlCanvasElement) -> Result<GL, JsValue> {
    let gl = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<GL>().unwrap();
    gl.enable(GL::DEPTH_TEST);
    Ok(gl)
}
