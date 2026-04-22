use crate::math::math::*;
use crate::renderer::{entity::Entity, mesh::Mesh, transform::Transform};
use std::sync::Arc;

pub struct Bullet {
    pub entity: Entity,
    pub speed: f32,
    pub spawn_y: f32,
    pub max_range: f32,
}

impl Bullet {
    pub fn new(mesh: Arc<Mesh>, position: Vec3) -> Self {
        let transform = Transform::new(
            Vec3::new(position.x, position.y, position.z),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.5, 0.5, 0.5),
        );

        Self {
            entity: Entity::new(mesh, transform),
            speed: 15.0,
            spawn_y: position.y,
            max_range: 25.0,
        }
    }

    pub fn update(&mut self, dt: f64) {
        // Move upward
        self.entity.transform.position.y += self.speed * dt as f32;

        // Rotate continuously
        self.entity.transform.rotation.y += 3.0 * dt as f32;
        self.entity.transform.rotation.x += 2.0 * dt as f32;
    }

    pub fn should_despawn(&self) -> bool {
        (self.entity.transform.position.y - self.spawn_y).abs() > self.max_range
    }
}
