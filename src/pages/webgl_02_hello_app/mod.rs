use nalgebra::Vector4;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

pub fn run() -> Result<(), JsValue> {
    let app = Rc::new(App::new());
    app.clone().attach_event_handlers()?;
    app.clone().render()?;
    Ok(())
}

struct App {
    viewport: Viewport,
    clear_color: Vector4<f32>,
    canvas: HtmlCanvasElement,
    gl: WebGl2RenderingContext,
}

struct Viewport {
    width: f64,
    height: f64,
}

impl App {
    fn new() -> Self {
        let canvas = create_canvas().unwrap();

        let viewport = Viewport {
            width: canvas.width() as f64,
            height: canvas.height() as f64,
        };
        let clear_color = Vector4::new(0.0, 0.0, 0.0, 1.0);
        let gl = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();
        Self {
            viewport,
            clear_color,
            canvas,
            gl,
        }
    }

    fn attach_event_handlers(self: Rc<Self>) -> Result<(), JsValue> {
        self.attach_onresize_handler()?;
        Ok(())
    }

    fn attach_onresize_handler(self: Rc<Self>) -> Result<(), JsValue> {
        let handler = move |_evt: web_sys::Event| {
            let window = web_sys::window().expect("no global `window` exists");
            let width = window.inner_width().unwrap().as_f64().unwrap();
            let height = window.inner_height().unwrap().as_f64().unwrap();
            self.canvas.set_width(width as u32);
            self.canvas.set_height(height as u32);
            self.gl.viewport(0, 0, width as i32, height as i32);
        };

        let handler = Closure::wrap(Box::new(handler) as Box<dyn Fn(_)>);
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        body.set_onresize(Some(handler.as_ref().unchecked_ref()));
        handler.forget();

        Ok(())
    }

    fn render(self: Rc<Self>) -> Result<(), JsValue> {
        self.gl.clear_color(
            self.clear_color[0],
            self.clear_color[1],
            self.clear_color[2],
            self.clear_color[3],
        );
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        self.gl
            .viewport(0, 0, self.viewport.width as i32, self.viewport.height as i32);
        Ok(())
    }
}

fn create_canvas() -> Result<HtmlCanvasElement, JsValue> {
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
