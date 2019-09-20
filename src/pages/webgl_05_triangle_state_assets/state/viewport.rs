pub struct Viewport {
    width: f64,
    height: f64,
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new(200.0, 300.0)
    }
}

impl Viewport {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn set(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }
}
