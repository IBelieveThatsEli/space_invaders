use crate::math::math::*;
use crate::renderer::{entity::Entity, mesh::Mesh, transform::Transform};

pub struct Enemy {
    pub entity: Entity,
    pub speed: f32,
}

impl Enemy {
    pub fn new(mesh: Mesh, position: Vec3) -> Self {
        let transform =
            Transform::new(position, Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));

        Self {
            entity: Entity::new(mesh, transform),
            speed: 2.0,
        }
    }

    pub fn update(&mut self, _dt: f64) {
        // Enemy is stationary for now
        // Future: Add AI movement patterns here
    }
}
