use cgmath::{Matrix4, Transform};
use std::collections::HashMap;
use web_sys::{WebGl2RenderingContext as GL, *};

use super::Shader;
use super::State;

pub struct Cube {
    vertices: Vec<f32>,
    colors: Vec<f32>,
    indices: Vec<u16>,
    shader: Shader,
    vao: Option<WebGlVertexArrayObject>,
    vbos: HashMap<String, WebGlBuffer>,
}

impl Cube {
    pub fn new(shader: Shader) -> Self {
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
        Self {
            vertices,
            colors,
            indices,
            shader,
            vao: None,
            vbos: HashMap::new(),
        }
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

    pub fn prepare_for_render(&mut self, gl: &GL) {
        gl.use_program(Some(&self.shader.program));
        self.vao = gl.create_vertex_array();
        gl.bind_vertex_array(self.vao.as_ref());

        let vert_buffer = self.array_buffer_f32_data(&gl, self.vertices(), "aVertexPosition", 3);
        let color_buffer = self.array_buffer_f32_data(&gl, self.colors(), "aVertexColor", 4);
        let indices_buffer = self.index_buffer_u16_data(&gl, self.indices());
        self.vbos.insert("vert".into(), vert_buffer);
        self.vbos.insert("color".into(), color_buffer);
        self.vbos.insert("index".into(), indices_buffer);

        gl.bind_vertex_array(None);
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, None);
        gl.bind_buffer(GL::ARRAY_BUFFER, None);
    }

    pub fn render(&self, gl: &GL, state: &State) {
        gl.use_program(Some(&self.shader.program));

        self.set_uniform_location_matrix4(&gl, "uModelMatrix", Matrix4::one());
        self.set_uniform_location_matrix4(&gl, "uViewMatrix", state.camera().view_matrix());
        self.set_uniform_location_matrix4(&gl, "uProjectiveMatrix", state.camera().projection_matrix());

        gl.bind_vertex_array(self.vao.as_ref());
        gl.draw_elements_with_i32(GL::TRIANGLES, self.indices().len() as i32, GL::UNSIGNED_SHORT, 0);
        gl.bind_vertex_array(None);
    }

    pub fn set_uniform_location_matrix4(&self, gl: &GL, uniform_name: &str, matrix: Matrix4<f32>) {
        let location = self.shader.get_uniform_location(&gl, uniform_name);
        let array: &[f32; 16] = matrix.as_ref();
        gl.uniform_matrix4fv_with_f32_array(location.as_ref(), false, &array[..]);
    }

    pub fn array_buffer_f32_data(&self, gl: &GL, data: &[f32], attrib_name: &str, size: i32) -> WebGlBuffer {
        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vbo));
        unsafe {
            let data_array = js_sys::Float32Array::view(data);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &data_array, GL::STATIC_DRAW);
        }
        let location = self.shader.get_attrib_location(&gl, attrib_name);
        gl.enable_vertex_attrib_array(location as u32);
        gl.vertex_attrib_pointer_with_i32(location as u32, size, GL::FLOAT, false, 0, 0);
        vbo
    }

    pub fn index_buffer_u16_data(&self, gl: &GL, data: &[u16]) -> WebGlBuffer {
        let ebo = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&ebo));
        unsafe {
            let data_array = js_sys::Uint16Array::view(data);
            gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &data_array, GL::STATIC_DRAW);
        }
        ebo
    }
}
