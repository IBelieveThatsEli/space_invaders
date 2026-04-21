use rmath::{Vec3, common::to_radians};

use super::{camera::*, mesh::*, shader::*, transform::*};
use crate::{gl::core::*, renderer::entity::Entity};
use std::sync::Arc;

pub trait Scene {
    fn load(&mut self, gl: Arc<GL>);
    fn update(&mut self, dt: f64);
    fn render(&self, gl: &GL, shader: &Shader);
    fn unload(&mut self);
}

pub struct GameScene {
    entities: Vec<Entity>,
    loaded: bool,
    camera: Camera,
    time: f64,
}

impl GameScene {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
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
            time: 0.0,
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn add_mesh_with_transform(&mut self, mesh: Mesh, transform: Transform) {
        self.entities.push(Entity::new(mesh, transform));
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
        let mesh1 = Mesh::new(gl.clone(), &vertices, &indices, "assets/box_texture.png");
        let transform1 = Transform::new(
            Vec3::init(-3.0, 0.0, 0.0),
            Vec3::init(0.0, 0.0, 0.0),
            Vec3::init(1.0, 1.0, 1.0),
        );
        self.add_entity(Entity::new(mesh1, transform1));

        let mesh2 = Mesh::new(gl.clone(), &vertices, &indices, "assets/box_texture.png");
        let transform2 = Transform::new(
            Vec3::init(0.0, 0.0, 0.0),
            Vec3::init(0.0, 0.0, 0.0),
            Vec3::init(1.5, 1.5, 1.5),
        );
        self.add_entity(Entity::new(mesh2, transform2));

        let mesh3 = Mesh::new(gl.clone(), &vertices, &indices, "assets/box_texture.png");
        let transform3 = Transform::new(
            Vec3::init(3.0, 0.0, 0.0),
            Vec3::init(0.0, 0.0, 0.0),
            Vec3::init(0.5, 2.0, 0.5),
        );
        self.add_entity(Entity::new(mesh3, transform3));

        self.loaded = true;
    }

    fn update(&mut self, dt: f64) {
        self.time += dt;

        for (i, entity) in self.entities.iter_mut().enumerate() {
            // Rotate each entity at different speeds
            let rotation_speed = (i * 5) as f64;
            entity.transform.rotate(Vec3::init(
                dt as f32 * rotation_speed as f32,
                dt as f32 * rotation_speed as f32 * 0.5,
                0.0,
            ));

            // Calculate wave offset for this entity
            let offset = i as f32 * 2.0;
            let y_offset = (self.time as f32 + offset).sin() * 2.0;

            // Set position based on original X position
            let original_x = -3.0 + (i as f32 * 3.0); // -3.0, 0.0, 3.0
            entity.transform.position = Vec3::init(original_x, y_offset, 0.0);
        }
    }

    fn render(&self, gl: &GL, shader: &Shader) {
        shader.bind();

        // Set projection-view matrix once
        let pv = self.camera.get_pv();
        shader.set_uniform_mat4fv("pv", 1, gl.boolean.false_, pv.value_ptr());

        // Render each entity with its own model matrix
        for entity in &self.entities {
            entity.render(gl, shader);
        }
    }

    fn unload(&mut self) {
        self.entities.clear();
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
