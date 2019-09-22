use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext as GL, *};

use canvas::{create_canvas, create_webgl_context};
use events::attach_event_handlers;
use shader::Shader;
use state::{Msg, State};
use triangle::Triangle;

mod canvas;
mod events;
mod shader;
mod state;
mod triangle;

pub fn run() -> Result<(), JsValue> {
    let app = Rc::new(App::new());
    attach_event_handlers(Rc::clone(&app))?;
    Rc::clone(&app).render()?;
    Ok(())
}

pub struct App {
    canvas: HtmlCanvasElement,
    gl: GL,
    shader: Shader,
    state: RefCell<State>,
    triangle: Triangle,
}

impl App {
    fn new() -> Self {
        let canvas = create_canvas().unwrap();
        let gl = create_webgl_context(&canvas).unwrap();
        let shader = Shader::default_shader(&gl).expect("shader new error");
        let state = RefCell::new(State::new());
        state.borrow_mut().msg(Msg::ViewportResize(canvas.width() as f64, canvas.height() as f64));
        let triangle = Triangle::new();
        Self {
            canvas,
            gl,
            shader,
            state,
            triangle,
        }
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
            let vert_data_array = js_sys::Float32Array::view(self.triangle.vertices());
            self.gl
                .buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_data_array, GL::STATIC_DRAW);
        }
        let a_vertex_postion = self.shader.get_attrib_location(&self.gl, "aVertexPosition");
        self.gl.enable_vertex_attrib_array(a_vertex_postion as u32);
        self.gl.vertex_attrib_pointer_with_i32(a_vertex_postion as u32, 3, GL::FLOAT, false, 0, 0);

        let color_buffer = self.gl.create_buffer().unwrap();
        self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&color_buffer));
        unsafe {
            let color_data_array = js_sys::Float32Array::view(self.triangle.colors());
            self.gl
                .buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &color_data_array, GL::STATIC_DRAW);
        }
        let a_vertex_color = self.shader.get_attrib_location(&self.gl, "aVertexColor");
        self.gl.enable_vertex_attrib_array(a_vertex_color as u32);
        self.gl.vertex_attrib_pointer_with_i32(a_vertex_color as u32, 4, GL::FLOAT, false, 0, 0);

        let indices_buffer = self.gl.create_buffer().unwrap();
        self.gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&indices_buffer));
        unsafe {
            let indices_data_array = js_sys::Uint16Array::view(self.triangle.indices());
            self.gl
                .buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &indices_data_array, GL::STATIC_DRAW);
        }

        self.gl.draw_elements_with_i32(GL::TRIANGLES, 3, GL::UNSIGNED_SHORT, 0);

        Ok(())
    }
}
