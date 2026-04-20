use crate::gl::{core::GL, types::*};
use std::{ffi, sync::Arc};

pub struct VAO {
    gl: Arc<GL>,
    id: GLuint,
}

impl VAO {
    pub fn new(gl: Arc<GL>) -> Self {
        let mut id: GLuint = 0;
        gl.gen_vertex_arrays(1, &mut id);

        Self { gl, id }
    }

    pub fn bind(&self) {
        self.gl.bind_vertex_array(self.id);
    }

    pub fn attrib_pointer(
        &self,
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const ffi::c_void,
    ) {
        self.gl
            .vertex_attrib_pointer(index, size, type_, normalized, stride, pointer);
        self.gl.enable_vertex_attrib_array(index);
    }
}
impl Drop for VAO {
    fn drop(&mut self) {
        self.gl.delete_vertex_arrays(1, &self.id);
    }
}
