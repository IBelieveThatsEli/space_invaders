use crate::gl::core::*;
use crate::renderer::{buffer::*, shader::*, texture::*, vertex_array::*};
use rmath::{Mat4, Vec3};
use std::ffi;
use std::sync::Arc;

pub struct Mesh {
    pub vao: VAO,
    pub vbo: VBO,
    pub ebo: EBO,
    pub texture: Texture,
    pub model: Mat4,
    pub index_count: i32,
}

impl Mesh {
    pub fn new(gl: Arc<GL>, vertices: &[f32], indices: &[u32], texture_path: &str) -> Self {
        let vao = VAO::new(gl.clone());
        vao.bind();

        let vbo = VBO::new(gl.clone(), vertices);
        let ebo = EBO::new(gl.clone(), indices);

        // Position attribute (3 floats)
        vao.attrib_pointer(
            0,
            3,
            gl.data_type.float,
            gl.boolean.false_,
            5 * std::mem::size_of::<f32>() as i32,
            std::ptr::null(),
        );

        // Texture coordinate attribute (2 floats)
        vao.attrib_pointer(
            1,
            2,
            gl.data_type.float,
            gl.boolean.false_,
            5 * std::mem::size_of::<f32>() as i32,
            (3 * std::mem::size_of::<f32>()) as *const ffi::c_void,
        );

        let texture = Texture::new(gl.clone(), texture_path);
        let model = Mat4::identity();

        Self {
            vao,
            vbo,
            ebo,
            texture,
            model,
            index_count: indices.len() as i32,
        }
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.model = Mat4::identity();
        self.model.translate(&position);
    }

    pub fn set_rotation(&mut self, rotation: Vec3) {
        self.model.rotate(&rotation);
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.model.scale(&scale);
    }

    pub fn render(&self, gl: &GL, shader: &Shader) {
        shader.bind();
        shader.set_uniform_mat4fv("model", 1, gl.boolean.false_, self.model.value_ptr());
        self.texture.bind();
        self.vao.bind();
        self.ebo.bind();

        gl.draw_elements(
            gl.primitive.triangles,
            self.index_count,
            gl.data_type.unsigned_int,
            std::ptr::null(),
        );
    }
}
