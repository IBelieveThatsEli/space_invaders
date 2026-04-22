use crate::math::math::*;
use crate::renderer::{entity::Entity, mesh::Mesh, transform::Transform};
use std::sync::Arc;

pub struct Enemy {
    pub entity: Entity,
    pub speed: f32,
    pub base_x: f32,
    pub time: f64,
}

impl Enemy {
    pub fn new(mesh: Arc<Mesh>, position: Vec3) -> Self {
        let transform = Transform::new(
            Vec3::new(position.x, position.y, position.z),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 1.0),
        );

        Self {
            entity: Entity::new(mesh.clone(), transform),
            speed: 0.5,
            base_x: position.x,
            time: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.time += dt;
        let offset = (self.time * self.speed as f64).sin() as f32 * 3.0;
        self.entity.transform.position.x = self.base_x + offset;
    }

    pub fn reflow(&mut self, new_base_x: f32) {
        self.base_x = new_base_x;
    }
}
