use super::state::Msg;
use super::App;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn attach_event_handlers(app: Rc<App>) -> Result<(), JsValue> {
    attach_onresize_handler(Rc::clone(&app))?;
    Ok(())
}

fn attach_onresize_handler(app: Rc<App>) -> Result<(), JsValue> {
    let handler = move |_evt: web_sys::Event| {
        let window = web_sys::window().expect("no global `window` exists");
        let width = window.inner_width().unwrap().as_f64().unwrap();
        let height = window.inner_height().unwrap().as_f64().unwrap();
        app.canvas.set_width(width as u32);
        app.canvas.set_height(height as u32);
        app.state.borrow_mut().msg(Msg::ViewportResize(width, height));
        app.gl.viewport(0, 0, width as i32, height as i32);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn Fn(_)>);
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    body.set_onresize(Some(handler.as_ref().unchecked_ref()));
    handler.forget();

    Ok(())
}
