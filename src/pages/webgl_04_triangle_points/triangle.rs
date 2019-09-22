pub struct Triangle {
    vertices: Vec<f32>,
    colors: Vec<f32>,
    indices: Vec<u16>,
}

impl Triangle {
    pub fn new() -> Self {
        let vertices = vec![
            -0.5, -0.5, 0.0, //
            0.5, -0.5, 0.0, //
            0.0, 0.5, 0.0, //
        ];
        let colors = vec![
            1.0, 0.0, 0.0, 1.0, //
            0.0, 1.0, 0.0, 1.0, //
            0.0, 0.0, 1.0, 1.0, //
        ];
        let indices = vec![0, 1, 2];
        Self { vertices, colors, indices }
    }

    pub fn vertices(&self) -> &[f32] {
        &self.vertices
    }

    pub fn colors(&self) -> &[f32] {
        &self.colors
    }

    pub fn indices(&self) -> &[u16] {
        &self.indices
    }
}
