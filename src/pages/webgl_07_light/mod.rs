use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext as GL, *};

use canvas::{create_canvas, create_webgl_context};
use cube::Cube;
use events::attach_event_handlers;
use shader::Shader;
use state::{Msg, State};

mod canvas;
mod cube;
mod events;
mod shader;
mod state;

pub fn run() -> Result<(), JsValue> {
    let app = Rc::new(App::new());
    attach_event_handlers(Rc::clone(&app))?;
    Rc::clone(&app).render()?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        app.render().unwrap();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub struct App {
    canvas: HtmlCanvasElement,
    gl: GL,
    state: RefCell<State>,
    cube: Cube,
}

impl App {
    pub fn new() -> Self {
        let canvas = create_canvas().unwrap();
        let gl = create_webgl_context(&canvas).unwrap();
        let state = RefCell::new(State::new());
        state.borrow_mut().msg(Msg::ViewportResize(canvas.width() as f64, canvas.height() as f64));
        let shader = Shader::default_shader(&gl).expect("shader new error");
        let mut cube = Cube::new(shader);
        cube.prepare_for_render(&gl);
        Self { canvas, gl, state, cube }
    }

    pub fn render(&self) -> Result<(), JsValue> {
        let state = self.state.borrow();
        let clear_color = state.clear_color();
        let viewport = state.viewport();

        self.gl.clear_color(clear_color.r, clear_color.g, clear_color.b, clear_color.a);
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        self.gl.viewport(0, 0, viewport.width() as i32, viewport.height() as i32);

        self.cube.render(&self.gl, &self.state.borrow());
        Ok(())
    }
}
