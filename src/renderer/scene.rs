use super::{camera::*, mesh::*, shader::*};
use crate::audio::Audio;
use crate::game::bullet::Bullet;
use crate::game::{enemy::Enemy, player::Player};
use crate::math::math::*;
use crate::{gl::core::*, window::x11::events::Event};
use std::sync::Arc;

pub trait Scene {
    fn load(&mut self, gl: Arc<GL>);
    fn update(&mut self, dt: f64);
    fn render(&self, gl: &GL, shader: &Shader);
    fn unload(&mut self);
    fn handle_input(&mut self, event: &Event);
}

pub struct GameScene {
    player: Option<Player>,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    meshes: Vec<Arc<Mesh>>,
    loaded: bool,
    camera: Camera,
    audio: Option<Audio>, // time: f64,
}

impl GameScene {
    pub fn new() -> Self {
        Self {
            player: None,
            enemies: Vec::new(),
            bullets: Vec::new(),
            meshes: Vec::new(),
            loaded: false,
            camera: Camera::new(
                Vec3::new(0.0, -5.0, -11.0),
                to_radians(45.0),
                16.0 / 9.0,
                0.1,
                100.0,
                Vec3::new(0.0, -1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            audio: None,
            // time: 0.0,
        }
    }
}

impl Scene for GameScene {
    fn load(&mut self, gl: Arc<GL>) {
        if self.loaded {
            self.audio = Audio::new("assets/subway_theme.mp3");
            if let Some(audio) = &mut self.audio {
                audio.play();
            }
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

        self.meshes.push(Arc::new(Mesh::new(
            gl.clone(),
            &vertices,
            &indices,
            "assets/box_texture.png",
        )));

        self.player = Some(Player::new(
            self.meshes[0].clone(),
            Vec3::new(0.0, 0.0, 0.0),
        ));

        let num_enemies = 8;
        let spacing = 4.0;
        let start_x = -(num_enemies as f32 - 1.0) * spacing / 2.0;

        for i in 0..num_enemies {
            let x = start_x + i as f32 * spacing;
            self.enemies
                .push(Enemy::new(self.meshes[0].clone(), Vec3::new(x, 20.0, 0.0)));
        }

        self.loaded = true;
    }

    fn update(&mut self, dt: f64) {
        if let Some(player) = &mut self.player {
            player.update(dt);
        }

        for enemy in &mut self.enemies {
            enemy.update(dt);
        }

        for bullet in &mut self.bullets {
            bullet.update(dt);
        }

        self.enemies
            .retain(|e| e.entity.transform.position.y > -10.0);
        self.reflow_enemies();

        self.bullets.retain(|b| !b.should_despawn());
    }

    fn render(&self, gl: &GL, shader: &Shader) {
        shader.bind();

        // Set projection-view matrix once
        let pv = self.camera.get_pv();
        shader.set_uniform_mat4fv("pv", 1, gl.boolean.false_, pv.value_ptr());

        if let Some(player) = &self.player {
            player.entity.render(gl, shader);
        }

        for enemy in &self.enemies {
            enemy.entity.render(gl, shader);
        }

        for bullet in &self.bullets {
            bullet.entity.render(gl, shader);
        }
    }

    fn unload(&mut self) {
        self.player = None;
        self.enemies.clear();
        self.bullets.clear();
        self.meshes.clear();
        self.loaded = false;
    }

    fn handle_input(&mut self, event: &Event) {
        if let Some(player) = &mut self.player {
            let should_shoot = player.handle_input(event);

            if should_shoot {
                let pos = player.get_position();
                self.bullets.push(Bullet::new(self.meshes[0].clone(), pos));
            }
        }
    }
}

impl GameScene {
    fn reflow_enemies(&mut self) {
        let num_enemies = self.enemies.len();
        if num_enemies == 0 {
            return;
        }

        let spacing = 4.0;
        let start_x = -(num_enemies as f32 - 1.0) * spacing / 2.0;

        for (i, enemy) in self.enemies.iter_mut().enumerate() {
            let new_x = start_x + i as f32 * spacing;
            enemy.reflow(new_x);
        }
    }
}

// pub struct MenuScene {
//     meshes: Vec<Mesh>,
//     loaded: bool,
// }

// impl MenuScene {
//     pub fn new() -> Self {
//         Self {
//             meshes: Vec::new(),
//             loaded: false,
//         }
//     }
// }

// impl Scene for MenuScene {
//     fn load(&mut self, _gl: Arc<GL>) {
//         if self.loaded {
//             return;
//         }

//         // Load menu-specific meshes here
//         // For example: buttons, title screen, etc.

//         self.loaded = true;
//     }

//     fn update(&mut self, _dt: f64) {
//         // Update menu logic here
//     }

//     fn render(&self, gl: &GL, shader: &Shader) {
//         for mesh in &self.meshes {
//             mesh.render(gl, shader);
//         }
//     }

//     fn unload(&mut self) {
//         self.meshes.clear();
//         self.loaded = false;
//     }
// }
