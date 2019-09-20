use clear_color::ClearColor;
use viewport::Viewport;

mod clear_color;
mod viewport;

pub struct State {
    viewport: Viewport,
    clear_color: ClearColor,
}

impl State {
    pub fn new() -> Self {
        let viewport = Viewport::default();
        let clear_color = ClearColor::default();
        Self { viewport, clear_color }
    }

    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    pub fn clear_color(&self) -> &ClearColor {
        &self.clear_color
    }

    pub fn msg(&mut self, msg: Msg) {
        match msg {
            Msg::ViewportResize(width, height) => self.viewport.set(width, height),
            Msg::ClearColorChange(r, g, b, a) => self.clear_color.set(r, g, b, a),
        }
    }
}

pub enum Msg {
    ViewportResize(f64, f64),
    #[allow(dead_code)]
    ClearColorChange(f32, f32, f32, f32),
}
