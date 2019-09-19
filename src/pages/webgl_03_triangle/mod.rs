use nalgebra::Vector4;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL, WebGlProgram, WebGlShader};

static VERTEX_SHADER_SRC: &'static str = include_str!("./vertex-shader.essl");
static FRAGMENT_SHADER_SRC: &'static str = include_str!("./fragment-shader.essl");

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
    vertices: [f32; 9],
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
        let vertices = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
        let triangle_indices = [0, 1, 2];
        Self {
            viewport,
            clear_color,
            canvas,
            gl,
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

        let vertex_shader = compile_shader(&self.gl, GL::VERTEX_SHADER, VERTEX_SHADER_SRC)?;
        let fragment_shader = compile_shader(&self.gl, GL::FRAGMENT_SHADER, FRAGMENT_SHADER_SRC)?;
        let program = link_program(&self.gl, &vertex_shader, &fragment_shader)?;
        self.gl.use_program(Some(&program));

        let vertices_buffer = self.gl.create_buffer().unwrap();
        self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertices_buffer));
        unsafe {
            let vertices_data_array = js_sys::Float32Array::view(&self.vertices);
            self.gl
                .buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices_data_array, GL::STATIC_DRAW);
        }

        let vertex_position_attrib = self.gl.get_attrib_location(&program, "aVertexPosition");
        self.gl.enable_vertex_attrib_array(vertex_position_attrib as u32);
        self.gl
            .vertex_attrib_pointer_with_i32(vertex_position_attrib as u32, 3, GL::FLOAT, false, 0, 0);

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

fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl.create_shader(shader_type).ok_or_else(|| "Could not create shader".to_string())?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);
    if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false) {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".to_string()))
    }
}

fn link_program(gl: &GL, vert_shader: &WebGlShader, frag_shader: &WebGlShader) -> Result<WebGlProgram, String> {
    let program = gl.create_program().ok_or_else(|| "Unable to create shader program".to_string())?;
    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);
    if gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap_or(false) {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program".to_string()))
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
