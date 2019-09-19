use std::cell::RefCell;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext as GL, *};

pub struct Shader {
    pub program: WebGlProgram,
    #[allow(dead_code)]
    uniforms: RefCell<HashMap<String, WebGlUniformLocation>>,
    attribs: RefCell<HashMap<String, i32>>,
}

impl Shader {
    pub fn new(gl: &GL, vert_shader: &str, frag_shader: &str) -> Result<Shader, JsValue> {
        let vert_shader = compile_shader(&gl, GL::VERTEX_SHADER, vert_shader)?;
        let frag_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, frag_shader)?;
        let program = link_program(&gl, &vert_shader, &frag_shader)?;
        let uniforms = RefCell::new(HashMap::new());
        let attribs = RefCell::new(HashMap::new());
        Ok(Self { program, uniforms, attribs })
    }

    pub fn get_attrib_location(&self, gl: &GL, name: &str) -> i32 {
        let mut attribs = self.attribs.borrow_mut();
        if attribs.get(name).is_none() {
            attribs.insert(name.into(), gl.get_attrib_location(&self.program, name));
        }
        attribs.get(name).unwrap().clone()
    }

    #[allow(dead_code)]
    pub fn get_uniform_location(&self, gl: &GL, uniform_name: &str) -> Option<WebGlUniformLocation> {
        let mut uniforms = self.uniforms.borrow_mut();
        if uniforms.get(uniform_name).is_none() {
            uniforms.insert(
                uniform_name.to_string(),
                gl.get_uniform_location(&self.program, uniform_name)
                    .expect(&format!(r#"Uniform '{}' not found"#, uniform_name)),
            );
        }
        Some(uniforms.get(uniform_name).expect("loc").clone())
    }
}

fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl.create_shader(shader_type).ok_or_else(|| "Could not create shader".to_string())?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);
    if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false) {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".to_string()))
    }
}

fn link_program(gl: &GL, vert_shader: &WebGlShader, frag_shader: &WebGlShader) -> Result<WebGlProgram, String> {
    let program = gl.create_program().ok_or_else(|| "Unable to create shader program".to_string())?;
    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);
    if gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap_or(false) {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program".to_string()))
    }
}
