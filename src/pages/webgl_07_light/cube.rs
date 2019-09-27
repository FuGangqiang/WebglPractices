use cgmath::{Matrix, Matrix4, SquareMatrix, Transform, Vector3, Vector4};
use std::collections::HashMap;
use std::mem::size_of;
use web_sys::{WebGl2RenderingContext as GL, *};

use super::Shader;
use super::State;

pub struct Cube {
    vertices: Vec<f32>,
    indices: Vec<u16>,
    shader: Shader,
    vao: Option<WebGlVertexArrayObject>,
    vbos: HashMap<String, WebGlBuffer>,
}

impl Cube {
    pub fn new(shader: Shader) -> Self {
        let vertices = vec![
            // position    // normals
            1.0, 1.0, 1.0, 0.0, 0.0, 1.0, //  top
            -1.0, 1.0, 1.0, 0.0, 0.0, 1.0, //
            -1.0, -1.0, 1.0, 0.0, 0.0, 1.0, //
            1.0, -1.0, 1.0, 0.0, 0.0, 1.0, //
            1.0, 1.0, -1.0, 0.0, 0.0, -1.0, // bottom
            -1.0, 1.0, -1.0, 0.0, 0.0, -1.0, //
            -1.0, -1.0, -1.0, 0.0, 0.0, -1.0, //
            1.0, -1.0, -1.0, 0.0, 0.0, -1.0, //
            -1.0, -1.0, 1.0, -1.0, 0.0, 0.0, // left
            -1.0, 1.0, 1.0, -1.0, 0.0, 0.0, //
            -1.0, 1.0, -1.0, -1.0, 0.0, 0.0, //
            -1.0, -1.0, -1.0, -1.0, 0.0, 0.0, //
            1.0, -1.0, 1.0, 1.0, 0.0, 0.0, // right
            1.0, 1.0, 1.0, 1.0, 0.0, 0.0, //
            1.0, 1.0, -1.0, 1.0, 0.0, 0.0, //
            1.0, -1.0, -1.0, 1.0, 0.0, 0.0, //
            -1.0, -1.0, 1.0, 0.0, 1.0, 0.0, // front
            -1.0, -1.0, -1.0, 0.0, 1.0, 0.0, //
            1.0, -1.0, -1.0, 0.0, 1.0, 0.0, //
            1.0, -1.0, 1.0, 0.0, 1.0, 0.0, //
            -1.0, 1.0, 1.0, 0.0, -1.0, 0.0, // back
            -1.0, 1.0, -1.0, 0.0, -1.0, 0.0, //
            1.0, 1.0, -1.0, 0.0, -1.0, 0.0, //
            1.0, 1.0, 1.0, 0.0, -1.0, 0.0, //
        ];
        let indices = vec![
            0, 1, 2, // top
            0, 2, 3, //
            4, 6, 5, // bottom
            4, 7, 6, //
            8, 9, 10, // left
            8, 10, 11, //
            12, 15, 14, // right
            12, 14, 13, //
            16, 17, 18, // front
            16, 18, 19, //
            20, 23, 22, // back
            20, 22, 21, //
        ];
        Self {
            vertices,
            indices,
            shader,
            vao: None,
            vbos: HashMap::new(),
        }
    }

    pub fn vertices(&self) -> &[f32] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u16] {
        &self.indices
    }

    pub fn prepare_for_render(&mut self, gl: &GL) {
        gl.use_program(Some(&self.shader.program));
        self.vao = gl.create_vertex_array();
        gl.bind_vertex_array(self.vao.as_ref());

        let vert_buffer = self.array_buffer_f32_data(&gl, self.vertices(), GL::STATIC_DRAW);
        self.set_attrib_location_f32(&gl, "aVertexPosition", 3, 6 * size_of::<f32>(), 0);
        self.set_attrib_location_f32(&gl, "aVertexNormal", 3, 6 * size_of::<f32>(), 3 * size_of::<f32>());
        let indices_buffer = self.index_buffer_u16_data(&gl, self.indices());
        self.vbos.insert("vert".into(), vert_buffer);
        self.vbos.insert("index".into(), indices_buffer);

        gl.bind_vertex_array(None);
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, None);
        gl.bind_buffer(GL::ARRAY_BUFFER, None);
    }

    pub fn render(&self, gl: &GL, state: &State) {
        gl.use_program(Some(&self.shader.program));

        let model_matrix: Matrix4<f32> = Matrix4::one();
        let normal_matrix: Matrix4<f32> = model_matrix.invert().unwrap().transpose();
        self.set_uniform_location_matrix4(&gl, "uModelMatrix", model_matrix);
        self.set_uniform_location_matrix4(&gl, "uViewMatrix", state.camera().view_matrix());
        self.set_uniform_location_matrix4(&gl, "uProjectiveMatrix", state.camera().projection_matrix());
        self.set_uniform_location_matrix4(&gl, "uNormalMatrix", normal_matrix);

        self.set_uniform_location_vector3(&gl, "uLightDirection", Vector3::new(10.0, 25.0, 10.0));
        self.set_uniform_location_vector4(&gl, "uLightAmbient", Vector4::new(1.0, 1.0, 1.0, 1.0));
        self.set_uniform_location_vector4(&gl, "uLightDiffuse", Vector4::new(1.0, 1.0, 1.0, 1.0));
        self.set_uniform_location_vector4(&gl, "uMaterialAmbient", Vector4::new(0.4, 0.4, 0.4, 1.0));
        self.set_uniform_location_vector4(&gl, "uMaterialDiffuse", Vector4::new(0.2, 0.2, 0.2, 1.0));

        gl.bind_vertex_array(self.vao.as_ref());
        gl.draw_elements_with_i32(GL::TRIANGLES, self.indices().len() as i32, GL::UNSIGNED_SHORT, 0);
        gl.bind_vertex_array(None);
    }

    pub fn set_uniform_location_matrix4(&self, gl: &GL, uniform_name: &str, matrix: Matrix4<f32>) {
        let location = self.shader.get_uniform_location(&gl, uniform_name);
        let array: &[f32; 16] = matrix.as_ref();
        gl.uniform_matrix4fv_with_f32_array(location.as_ref(), false, &array[..]);
    }

    pub fn set_uniform_location_vector3(&self, gl: &GL, uniform_name: &str, vector: Vector3<f32>) {
        let location = self.shader.get_uniform_location(&gl, uniform_name);
        let array: &[f32; 3] = vector.as_ref();
        gl.uniform3fv_with_f32_array(location.as_ref(), &array[..]);
    }

    pub fn set_uniform_location_vector4(&self, gl: &GL, uniform_name: &str, vector: Vector4<f32>) {
        let location = self.shader.get_uniform_location(&gl, uniform_name);
        let array: &[f32; 4] = vector.as_ref();
        gl.uniform4fv_with_f32_array(location.as_ref(), &array[..]);
    }

    pub fn array_buffer_f32_data(&self, gl: &GL, data: &[f32], usage: u32) -> WebGlBuffer {
        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vbo));
        unsafe {
            let data_array = js_sys::Float32Array::view(data);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &data_array, usage);
        }
        vbo
    }

    pub fn set_attrib_location_f32(&self, gl: &GL, attrib_name: &str, size: i32, stride: usize, offset: usize) {
        let location = self.shader.get_attrib_location(&gl, attrib_name);
        gl.enable_vertex_attrib_array(location as u32);
        gl.vertex_attrib_pointer_with_i32(location as u32, size, GL::FLOAT, false, stride as i32, offset as i32);
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
