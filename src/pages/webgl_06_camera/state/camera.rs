use cgmath::{Matrix4, PerspectiveFov, Point3, Rad, Vector3};
use std::f32::consts::PI;

pub struct Camera {
    projection: PerspectiveFov<f32>,
    up_down_angle: f32,    // pitch
    left_right_angle: f32, // yaw
    r: f32,
    focus: Point3<f32>,
}

// todo: add Ortho

impl Camera {
    pub fn new() -> Camera {
        let fovy = Rad(PI / 3.0);

        Camera {
            projection: PerspectiveFov {
                fovy,
                aspect: 1.0,
                near: 0.1,
                far: 50.0,
            },
            left_right_angle: 45.0f32.to_radians(),
            up_down_angle: 80.0f32.to_radians(),
            r: 15.0,
            focus: Point3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        let eye = self.get_eye();
        Matrix4::look_at(eye, self.focus.clone(), Vector3::unit_y())
    }

    pub fn projection_matrix(&self) -> Matrix4<f32> {
        self.projection.into()
    }

    pub fn get_eye(&self) -> Point3<f32> {
        let pitch = self.up_down_angle;
        let yaw = self.left_right_angle;
        let y = self.r * pitch.sin();
        let x = self.r * pitch.cos() * yaw.sin();
        let z = self.r * pitch.cos() * yaw.cos();
        Point3::new(x, y, z)
    }

    pub fn orbit_left_right(&mut self, delta: f32) {
        self.left_right_angle += delta / 50.0;
    }

    pub fn orbit_up_down(&mut self, delta: f32) {
        self.up_down_angle += delta / 50.0;
        log::debug!("up_down_angle: {}", self.up_down_angle % (2.0 * PI) / (2.0 * PI) * 360.0);
    }

    pub fn zoom(&mut self, zoom: f32) {
        self.r += zoom;

        if self.r > 50.0 {
            self.r = 50.0;
        } else if self.r < 0.1 {
            self.r = 0.1;
        }
    }
}
