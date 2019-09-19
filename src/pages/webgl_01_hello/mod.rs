use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let body_style = body.style();
    body_style.set_property("margin", "0")?;
    body_style.set_property("width", "100%")?;
    body_style.set_property("height", "100%")?;
    body_style.set_property("overflow", "hidden")?;

    let canvas_node = document.create_element("canvas")?;
    canvas_node.set_attribute("id", "webgl-canvas")?;
    body.append_child(&canvas_node)?;
    let canvas = canvas_node.dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_width(window.inner_width()?.as_f64().unwrap() as u32);
    canvas.set_height(window.inner_height()?.as_f64().unwrap() as u32);

    let gl = canvas.get_context("webgl2")?.unwrap().dyn_into::<WebGl2RenderingContext>()?;

    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    let onresize = Closure::wrap(Box::new(move |_evt: web_sys::Event| {
        let window = web_sys::window().expect("no global `window` exists");
        let width = window.inner_width().unwrap().as_f64().unwrap();
        let height = window.inner_height().unwrap().as_f64().unwrap();
        canvas.set_width(width as u32);
        canvas.set_height(height as u32);
        gl.viewport(0, 0, width as i32, height as i32);
    }) as Box<dyn Fn(_)>);
    body.set_onresize(Some(onresize.as_ref().unchecked_ref()));
    onresize.forget();

    Ok(())
}
