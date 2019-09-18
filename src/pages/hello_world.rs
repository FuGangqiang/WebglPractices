use wasm_bindgen::prelude::*;

pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("p")?;
    val.set_inner_html("Webgl2 Practices from hello world");
    body.append_child(&val)?;

    Ok(())
}
