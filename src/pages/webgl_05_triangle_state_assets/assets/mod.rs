pub struct Assets {
    pub triangle_vertices: [f32; 21],
    pub triangle_indices: [u16; 3],
}

impl Assets {
    pub fn new() -> Self {
        let triangle_vertices = Self::get_triangle_vertices();
        let triangle_indices = Self::get_triangle_indices();
        Assets {
            triangle_vertices,
            triangle_indices,
        }
    }

    fn get_triangle_vertices() -> [f32; 21] {
        [
            // points         // color
            -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0, //
            0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, //
            0.0, 0.5, 0.0, 0.0, 0.0, 1.0, 1.0,
        ]
    }

    fn get_triangle_indices() -> [u16; 3] {
        [0, 1, 2]
    }
}
