pub struct Cube {
    vertices: Vec<f32>,
    colors: Vec<f32>,
    indices: Vec<u16>,
}

impl Cube {
    pub fn new() -> Self {
        let vertices = vec![
            1.0, 1.0, 1.0, // top points
            -1.0, 1.0, 1.0, //
            -1.0, -1.0, 1.0, //
            1.0, -1.0, 1.0, //
            1.0, 1.0, -1.0, // bottom points
            -1.0, 1.0, -1.0, //
            -1.0, -1.0, -1.0, //
            1.0, -1.0, -1.0, //
        ];
        let colors = vec![
            0.0, 0.0, 0.0, 1.0, //
            1.0, 0.0, 0.0, 1.0, //
            0.0, 1.0, 0.0, 1.0, //
            1.0, 1.0, 0.0, 1.0, //
            0.0, 0.0, 1.0, 1.0, //
            1.0, 0.0, 1.0, 1.0, //
            0.0, 1.0, 1.0, 1.0, //
            1.0, 1.0, 1.0, 1.0, //
        ];
        let indices = vec![
            0, 1, 2, // top
            0, 2, 3, //
            4, 6, 5, // bottom
            4, 7, 6, //
            1, 5, 2, // left
            2, 5, 6, //
            0, 3, 4, // right
            3, 7, 4, //
            2, 6, 3, // front
            3, 6, 7, //
            0, 4, 5, // back
            0, 5, 1, //
        ];
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
