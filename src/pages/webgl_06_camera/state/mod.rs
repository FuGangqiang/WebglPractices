use camera::Camera;
use clear_color::ClearColor;
use mouse::Mouse;
use viewport::Viewport;

mod camera;
mod clear_color;
mod mouse;
mod viewport;

pub struct State {
    viewport: Viewport,
    clear_color: ClearColor,
    camera: Camera,
    mouse: Mouse,
}

impl State {
    pub fn new() -> Self {
        let viewport = Viewport::default();
        let clear_color = ClearColor::default();
        let camera = Camera::new();
        let mouse = Mouse::default();
        Self {
            viewport,
            clear_color,
            camera,
            mouse,
        }
    }

    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    pub fn clear_color(&self) -> &ClearColor {
        &self.clear_color
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn msg(&mut self, msg: Msg) {
        match msg {
            Msg::ViewportResize(width, height) => self.viewport.set(width, height),
            Msg::ClearColorChange(r, g, b, a) => self.clear_color.set(r, g, b, a),
            Msg::MouseDown(x, y) => {
                self.mouse.set_pressed(true);
                self.mouse.set_pos(x, y);
            }
            Msg::MouseUp => self.mouse.set_pressed(false),
            Msg::MouseMove(x, y) => {
                if !self.mouse.pressed() {
                    return;
                }
                let (old_x, old_y) = self.mouse.pos();
                let dx = old_x as i32 - x;
                let dy = y - old_y as i32;
                self.camera.orbit_left_right(dx as f32);
                self.camera.orbit_up_down(dy as f32);
                self.mouse.set_pos(x, y);
            }
            Msg::Zoom(zoom) => self.camera.zoom(zoom),
        }
    }
}

pub enum Msg {
    ViewportResize(f64, f64),
    #[allow(dead_code)]
    ClearColorChange(f32, f32, f32, f32),
    MouseDown(i32, i32),
    MouseUp,
    MouseMove(i32, i32),
    Zoom(f32),
}
