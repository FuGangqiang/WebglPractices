use nalgebra::Vector4;
use shader::Shader;
use std::mem::size_of;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext as GL, *};

mod shader;

static VERT_SHADER_SRC: &'static str = include_str!("./shader/vertex-shader.essl");
static FRAG_SHADER_SRC: &'static str = include_str!("./shader/fragment-shader.essl");

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
    gl: GL,
    shader: Shader,
    vertices: [f32; 21],
    triangle_indices: [u16; 3],
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
        let gl = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<GL>().unwrap();
        let shader = Shader::new(&gl, VERT_SHADER_SRC, FRAG_SHADER_SRC).expect("shader new error");
        let vertices = [
            // points         // color
            -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0, //
            0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, //
            0.0, 0.5, 0.0, 0.0, 0.0, 1.0, 1.0,
        ];
        let triangle_indices = [0, 1, 2];
        Self {
            viewport,
            clear_color,
            canvas,
            gl,
            shader,
            vertices,
            triangle_indices,
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
        self.gl
            .clear_color(self.clear_color[0], self.clear_color[1], self.clear_color[2], self.clear_color[3]);
        self.gl.clear(GL::COLOR_BUFFER_BIT);
        self.gl.viewport(0, 0, self.viewport.width as i32, self.viewport.height as i32);
        self.gl.use_program(Some(&self.shader.program));

        let vert_buffer = self.gl.create_buffer().unwrap();
        self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vert_buffer));
        unsafe {
            let vert_data_array = js_sys::Float32Array::view(&self.vertices);
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
            let indices_data_array = js_sys::Uint16Array::view(&self.triangle_indices);
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
