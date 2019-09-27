use super::state::Msg;
use super::App;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn attach_event_handlers(app: Rc<App>) -> Result<(), JsValue> {
    attach_onresize_handler(Rc::clone(&app))?;
    attach_mouse_down_handler(Rc::clone(&app))?;
    attach_mouse_up_handler(Rc::clone(&app))?;
    attach_mouse_move_handler(Rc::clone(&app))?;
    attach_mouse_wheel_handler(Rc::clone(&app))?;
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

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    body.set_onresize(Some(handler.as_ref().unchecked_ref()));
    handler.forget();

    Ok(())
}

fn attach_mouse_down_handler(app: Rc<App>) -> Result<(), JsValue> {
    let app2 = Rc::clone(&app);
    let handler = move |event: web_sys::MouseEvent| {
        let x = event.client_x();
        let y = event.client_y();
        app2.state.borrow_mut().msg(Msg::MouseDown(x, y));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    app.canvas
        .add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_mouse_up_handler(app: Rc<App>) -> Result<(), JsValue> {
    let app2 = Rc::clone(&app);
    let handler = move |_event: web_sys::MouseEvent| {
        app2.state.borrow_mut().msg(Msg::MouseUp);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    app.canvas.add_event_listener_with_callback("mouseup", handler.as_ref().unchecked_ref())?;
    handler.forget();
    Ok(())
}

fn attach_mouse_move_handler(app: Rc<App>) -> Result<(), JsValue> {
    let app2 = Rc::clone(&app);
    let handler = move |event: web_sys::MouseEvent| {
        event.prevent_default();
        let x = event.client_x();
        let y = event.client_y();
        app2.state.borrow_mut().msg(Msg::MouseMove(x, y));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    app.canvas
        .add_event_listener_with_callback("mousemove", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_mouse_wheel_handler(app: Rc<App>) -> Result<(), JsValue> {
    let app2 = Rc::clone(&app);
    let handler = move |event: web_sys::WheelEvent| {
        event.prevent_default();
        let zoom_amount = event.delta_y() / 50.;
        app2.state.borrow_mut().msg(Msg::Zoom(zoom_amount as f32));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    app.canvas.add_event_listener_with_callback("wheel", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}
