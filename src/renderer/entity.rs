use super::shader::Shader;
use super::{mesh::Mesh, transform::Transform};
use crate::gl::core::GL;
use std::sync::Arc;

pub struct Entity {
    pub mesh: Arc<Mesh>,
    pub transform: Transform,
    pub active: bool,
}

impl Entity {
    pub fn new(mesh: Arc<Mesh>, transform: Transform) -> Self {
        Self {
            mesh: mesh.clone(),
            transform,
            active: true,
        }
    }

    pub fn render(&self, gl: &GL, shader: &Shader) {
        if !self.active {
            return;
        }

        shader.bind();
        let model = self.transform.get_model_matrix();
        shader.set_uniform_mat4fv("model", 1, gl.boolean.false_, model.value_ptr());

        self.mesh.render(gl, shader);
    }

    pub fn update(&mut self, _dt: f64) {
        // Override this in game logic for per-entity updates
    }
}
