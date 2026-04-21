use crate::input::types::{action::Action, key::Key};
use crate::math::math::*;
use crate::renderer::{entity::Entity, mesh::Mesh, transform::Transform};
use crate::window::x11::events::Event;

pub struct Player {
    pub entity: Entity,
    pub speed: f32,
    pub movement: Vec3,
}

impl Player {
    pub fn new(mesh: Mesh, position: Vec3) -> Self {
        let transform =
            Transform::new(position, Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));

        Self {
            entity: Entity::new(mesh, transform),
            speed: 5.0,
            movement: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn handle_input(&mut self, event: &Event) {
        match event {
            Event::Key(key, _, action, _) => {
                let value = match action {
                    Action::Press => 1.0,
                    Action::Release => 0.0,
                    _ => return,
                };

                match key {
                    Key::W => self.movement.y = value,
                    Key::S => self.movement.y = -value,
                    Key::A => self.movement.x = value,
                    Key::D => self.movement.x = -value,
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn update(&mut self, dt: f64) {
        // Normalize movement vector to prevent faster diagonal movement
        let mut move_vec = Vec3::new(self.movement.x, self.movement.y, self.movement.z);
        let length = (move_vec.x * move_vec.x + move_vec.y * move_vec.y).sqrt();

        if length > 0.0 {
            move_vec.x /= length;
            move_vec.y /= length;
        }

        // Apply movement
        let delta = Vec3::new(
            move_vec.x * self.speed * dt as f32,
            move_vec.y * self.speed * dt as f32,
            0.0,
        );

        self.entity.transform.translate(delta);

        // Clamp position to screen bounds (adjust as needed)
        let x_bounds = 10.0;
        let y_bounds = 2.0;
        self.entity.transform.position.x =
            self.entity.transform.position.x.clamp(-x_bounds, x_bounds);
        self.entity.transform.position.y =
            self.entity.transform.position.y.clamp(-y_bounds, y_bounds);
    }
}
