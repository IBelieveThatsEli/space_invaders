use crate::gl::{core::GL, types::*};
use std::{ffi, sync::Arc};

pub struct VBO {
    gl: Arc<GL>,
    id: GLuint,
}

pub struct EBO {
    gl: Arc<GL>,
    id: GLuint,
}

impl VBO {
    pub fn new(gl: Arc<GL>, data: &[f32]) -> Self {
        let mut id: GLuint = 0;
        gl.gen_buffers(1, &mut id);

        gl.bind_buffer(gl.buffer.array, id);
        gl.buffer_data(
            gl.buffer.array,
            (data.len() * std::mem::size_of::<f32>()) as isize,
            data.as_ptr() as *const ffi::c_void,
            gl.buffer.static_draw,
        );

        Self { gl, id }
    }

    pub fn bind(&self) {
        self.gl.bind_buffer(self.gl.buffer.array, self.id);
    }
}
impl Drop for VBO {
    fn drop(&mut self) {
        self.gl.delete_vertex_arrays(1, &self.id);
    }
}

impl EBO {
    pub fn new(gl: Arc<GL>, indices: &[u32]) -> Self {
        let mut id: GLuint = 0;

        gl.gen_buffers(1, &mut id);

        gl.bind_buffer(gl.buffer.element_array, id);
        gl.buffer_data(
            gl.buffer.element_array,
            (indices.len() * std::mem::size_of::<u32>()) as isize,
            indices.as_ptr() as *const ffi::c_void,
            gl.buffer.static_draw,
        );

        Self { gl, id }
    }

    pub fn bind(&self) {
        self.gl.bind_buffer(self.gl.buffer.element_array, self.id);
    }
}
impl Drop for EBO {
    fn drop(&mut self) {
        self.gl.delete_buffers(1, &self.id);
    }
}
