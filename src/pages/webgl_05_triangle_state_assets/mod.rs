use std::cell::RefCell;
use std::mem::size_of;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext as GL, *};

use assets::Assets;
use shader::Shader;
use state::{Msg, State};

mod assets;
mod shader;
mod state;

pub fn run() -> Result<(), JsValue> {
    let app = Rc::new(App::new());
    app.clone().attach_event_handlers()?;
    app.clone().render()?;
    Ok(())
}

struct App {
    canvas: HtmlCanvasElement,
    gl: GL,
    shader: Shader,
    assets: Assets,
    state: RefCell<State>,
}

impl App {
    fn new() -> Self {
        let canvas = create_canvas().unwrap();
        let gl = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<GL>().unwrap();
        let shader = Shader::triangle_shader(&gl).expect("shader new error");
        let assets = Assets::new();
        let state = RefCell::new(State::new());
        state.borrow_mut().msg(Msg::ViewportResize(canvas.width() as f64, canvas.height() as f64));
        Self {
            canvas,
            gl,
            shader,
            assets,
            state,
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
            self.state.borrow_mut().msg(Msg::ViewportResize(width, height));
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
        let state = self.state.borrow();
        let clear_color = state.clear_color();
        let viewport = state.viewport();
        self.gl.clear_color(clear_color.r, clear_color.g, clear_color.b, clear_color.a);
        self.gl.clear(GL::COLOR_BUFFER_BIT);
        self.gl.viewport(0, 0, viewport.width() as i32, viewport.height() as i32);
        self.gl.use_program(Some(&self.shader.program));

        let vert_buffer = self.gl.create_buffer().unwrap();
        self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vert_buffer));
        unsafe {
            let vert_data_array = js_sys::Float32Array::view(&self.assets.triangle_vertices);
            self.gl
                .buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_data_array, GL::STATIC_DRAW);
        }

        let a_vertex_postion = self.shader.get_attrib_location(&self.gl, "aVertexPosition");
        self.gl.enable_vertex_attrib_array(a_vertex_postion as u32);
        self.gl
            .vertex_attrib_pointer_with_i32(a_vertex_postion as u32, 3, GL::FLOAT, false, 7 * size_of::<f32>() as i32, 0);

        let a_vertex_color = self.shader.get_attrib_location(&self.gl, "aVertexColor");
        self.gl.enable_vertex_attrib_array(a_vertex_color as u32);
        self.gl.vertex_attrib_pointer_with_i32(
            a_vertex_color as u32,
            4,
            GL::FLOAT,
            false,
            7 * size_of::<f32>() as i32,
            3 * size_of::<f32>() as i32,
        );

        let indices_buffer = self.gl.create_buffer().unwrap();
        self.gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&indices_buffer));
        unsafe {
            let indices_data_array = js_sys::Uint16Array::view(&self.assets.triangle_indices);
            self.gl
                .buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &indices_data_array, GL::STATIC_DRAW);
        }

        self.gl.draw_elements_with_i32(GL::TRIANGLES, 3, GL::UNSIGNED_SHORT, 0);

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
