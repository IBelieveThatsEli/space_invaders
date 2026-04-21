use rmath::{Vec3, common::to_radians};

use super::{camera::*, mesh::*, shader::*};
use crate::gl::core::*;
use std::sync::Arc;

pub trait Scene {
    fn load(&mut self, gl: Arc<GL>);
    fn update(&mut self, dt: f64);
    fn render(&self, gl: &GL, shader: &Shader);
    fn unload(&mut self);
}

pub struct GameScene {
    meshes: Vec<Mesh>,
    loaded: bool,
    camera: Camera,
}

impl GameScene {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            loaded: false,
            camera: Camera::new(
                Vec3::init(3.0, -3.0, -20.0),
                to_radians(45.0),
                16.0 / 9.0,
                0.1,
                100.0,
                Vec3::init(0.0, 0.0, 0.0),
                Vec3::init(0.0, 1.0, 0.0),
            ),
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }
}

impl Scene for GameScene {
    fn load(&mut self, gl: Arc<GL>) {
        if self.loaded {
            return;
        }
        let vertices: [f32; 120] = [
            // positions        // tex coords

            // FRONT (z = 0.5)
            1.0, 1.0, 0.5, 1.0, 1.0, 1.0, -1.0, 0.5, 1.0, 0.0, -1.0, -1.0, 0.5, 0.0, 0.0, -1.0, 1.0,
            0.5, 0.0, 1.0, // BACK (z = -0.5)
            1.0, 1.0, -0.5, 1.0, 1.0, 1.0, -1.0, -0.5, 1.0, 0.0, -1.0, -1.0, -0.5, 0.0, 0.0, -1.0,
            1.0, -0.5, 0.0, 1.0, // LEFT
            -1.0, 1.0, 0.5, 1.0, 1.0, -1.0, -1.0, 0.5, 1.0, 0.0, -1.0, -1.0, -0.5, 0.0, 0.0, -1.0,
            1.0, -0.5, 0.0, 1.0, // RIGHT
            1.0, 1.0, 0.5, 1.0, 1.0, 1.0, -1.0, 0.5, 1.0, 0.0, 1.0, -1.0, -0.5, 0.0, 0.0, 1.0, 1.0,
            -0.5, 0.0, 1.0, // TOP
            1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, -0.5, 1.0, 0.0, -1.0, 1.0, -0.5, 0.0, 0.0, -1.0,
            1.0, 0.5, 0.0, 1.0, // BOTTOM
            1.0, -1.0, 0.5, 1.0, 1.0, 1.0, -1.0, -0.5, 1.0, 0.0, -1.0, -1.0, -0.5, 0.0, 0.0, -1.0,
            -1.0, 0.5, 0.0, 1.0,
        ];
        let indices: [u32; 36] = [
            0, 1, 3, 1, 2, 3, // front
            4, 5, 7, 5, 6, 7, // back
            8, 9, 11, 9, 10, 11, // left
            12, 13, 15, 13, 14, 15, // right
            16, 17, 19, 17, 18, 19, // top
            20, 21, 23, 21, 22, 23, // bottom
        ];
        let mesh = Mesh::new(gl.clone(), &vertices, &indices, "assets/box_texture.png");
        self.meshes.push(mesh);

        self.loaded = true;
    }

    fn update(&mut self, _dt: f64) {
        // Update game logic here
        // For example: move meshes, check collisions, etc.
    }

    fn render(&self, gl: &GL, shader: &Shader) {
        for mesh in &self.meshes {
            mesh.render(gl, shader);
        }
        shader.bind();
        let pv = self.camera.get_pv();
        shader.set_uniform_mat4fv("pv", 1, gl.boolean.false_, pv.value_ptr());
    }

    fn unload(&mut self) {
        self.meshes.clear();
        self.loaded = false;
    }
}

pub struct MenuScene {
    meshes: Vec<Mesh>,
    loaded: bool,
}

impl MenuScene {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            loaded: false,
        }
    }
}

impl Scene for MenuScene {
    fn load(&mut self, _gl: Arc<GL>) {
        if self.loaded {
            return;
        }

        // Load menu-specific meshes here
        // For example: buttons, title screen, etc.

        self.loaded = true;
    }

    fn update(&mut self, _dt: f64) {
        // Update menu logic here
    }

    fn render(&self, gl: &GL, shader: &Shader) {
        for mesh in &self.meshes {
            mesh.render(gl, shader);
        }
    }

    fn unload(&mut self) {
        self.meshes.clear();
        self.loaded = false;
    }
}
