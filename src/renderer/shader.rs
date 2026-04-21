use crate::gl::{core::GL, types::*};
use std::{ffi, fs, sync::Arc};

pub struct Shader {
    id: GLuint,
    gl: Arc<GL>,
}

impl Shader {
    pub fn new(gl: Arc<GL>) -> Self {
        Self { id: 0, gl: gl }
    }
    pub fn load_from_files(&mut self, vpath: &str, fpath: &str) {
        let vert_source = Self::load(vpath);
        let frag_source = Self::load(fpath);

        let vs = Self::compile(&self.gl, &vert_source, self.gl.shader.vertex);
        let fs = Self::compile(&self.gl, &frag_source, self.gl.shader.fragment);

        self.id = Self::link(&self.gl, vs, fs);
    }

    pub fn bind(&self) {
        self.gl.use_program(self.id);
    }

    pub fn set_uniform_4f(&self, location: &str, v0: f32, v1: f32, v2: f32, v3: f32) {
        let cstr = ffi::CString::new(location).expect("Failed to read uniform location");

        let location = self.gl.get_uniform_location(self.id, cstr.as_ptr());

        self.gl.uniform_4f(location, v0, v1, v2, v3);
    }

    pub fn set_uniform_mat4fv(
        &self,
        location: &str,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        let cstr = ffi::CString::new(location).expect("Failed to read uniform location");

        let location = self.gl.get_uniform_location(self.id, cstr.as_ptr());

        self.gl.uniform_mat4fv(location, count, transpose, value);
    }

    fn load(path: &str) -> ffi::CString {
        let src = fs::read_to_string(path).expect("Failed to read shader file");

        ffi::CString::new(src).expect("Shader contains a null byte")
    }

    fn compile(gl: &GL, src: &ffi::CString, shader_type: GLenum) -> GLuint {
        let shader = gl.create_shader(shader_type);
        gl.shader_source(shader, 1, &src.as_ptr(), std::ptr::null());
        gl.compile_shader(shader);
        shader
    }

    fn link(gl: &GL, vs: GLuint, fs: GLuint) -> GLuint {
        let program = gl.create_program();

        gl.attach_shader(program, vs);
        gl.attach_shader(program, fs);
        gl.link_program(program);

        gl.delete_shader(vs);
        gl.delete_shader(fs);

        program
    }
}
impl Drop for Shader {
    fn drop(&mut self) {
        self.gl.delete_shader(self.id);
    }
}
