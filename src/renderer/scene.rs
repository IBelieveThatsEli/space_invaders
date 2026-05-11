use super::gltf_mesh::GltfRenderable;
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
            // time: 0.0,
        }
    }
}

impl Scene for GameScene {
    fn load(&mut self, gl: Arc<GL>) {
        if self.loaded {
            return;
        }
        let vertices: [f32; 120] = [
            // positions (3); tex coords (2);
            // FRONT
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 0.0, -1.0, -1.0, 1.0, 0.0, 0.0, -1.0, 1.0,
            1.0, 0.0, 1.0, // BACK
            1.0, 1.0, -1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 0.0, -1.0, -1.0, -1.0, 0.0, 0.0, -1.0,
            1.0, -1.0, 0.0, 1.0, // LEFT
            -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 0.0, -1.0, -1.0, -1.0, 0.0, 0.0, -1.0,
            1.0, -1.0, 0.0, 1.0, // RIGHT
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 0.0, 1.0, -1.0, -1.0, 0.0, 0.0, 1.0, 1.0,
            -1.0, 0.0, 1.0, // TOP
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 0.0, -1.0, 1.0, -1.0, 0.0, 0.0, -1.0,
            1.0, 1.0, 0.0, 1.0, // BOTTOM
            1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 0.0, -1.0, -1.0, -1.0, 0.0, 0.0, -1.0,
            -1.0, 1.0, 0.0, 1.0,
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
        std::thread::spawn(|| {
            if let Some(mut audio) = Audio::new("assets/subway_theme.mp3") {
                audio.play();
            }
        });

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

        let mut bullets_to_remove = Vec::new();
        // let mut enemies_to_remove = Vec::new();
        for (bullet_idx, bullet) in self.bullets.iter().enumerate() {
            let bullet_pos = bullet.get_position();
            for enemy in &mut self.enemies {
                if enemy.collides_with(&bullet_pos) {
                    let was_alive = !enemy.is_dead();
                    enemy.take_damage(5);

                    if was_alive {
                        if enemy.is_dead() {
                            // Play death sound
                            std::thread::spawn(|| {
                                if let Some(mut audio) = Audio::new("assets/ohno.mp3") {
                                    audio.play();
                                }
                            });

                            // enemies_to_remove.push(enemy_idk);
                        } else {
                            // Play hit sound
                            std::thread::spawn(|| {
                                if let Some(mut audio) = Audio::new("assets/oof.mp3") {
                                    audio.play();
                                }
                            });
                        }
                    }

                    bullets_to_remove.push(bullet_idx);
                    break;
                }
            }
        }

        // Remove bullets that hit enemies (in reverse order to maintain indices)
        for &idx in bullets_to_remove.iter().rev() {
            self.bullets.remove(idx);
        }
        self.enemies.retain(|enemy| !enemy.is_dead());
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

                std::thread::spawn(|| {
                    if let Some(mut audio) = Audio::new("assets/lazer.mp3") {
                        audio.play();
                    }
                });
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

impl Drop for GameScene {
    fn drop(&mut self) {
        self.unload();
    }
}

pub struct MeshScene {
    gltf_models: Vec<GltfRenderable>,
    loaded: bool,
    camera: Camera,
}

impl MeshScene {
    pub fn new() -> Self {
        Self {
            gltf_models: Vec::new(),
            loaded: false,
            camera: Camera::new(
                Vec3::new(0.0, 0.0, -5.0),
                to_radians(45.0),
                16.0 / 9.0,
                0.1,
                100.0,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
        }
    }
    pub fn resize(&mut self, width: u32, height: u32) {
        if height != 0 {
            self.camera.aspect = width as f32 / height as f32;
        }
    }
}

impl Scene for MeshScene {
    fn load(&mut self, gl: Arc<GL>) {
        if self.loaded {
            return;
        }

        match GltfRenderable::load(gl.clone(), "assets/monkey.glb") {
            Ok(gltf_model) => {
                gltf_model.model.scale(&Vec3::new(1.5, 1.5, 1.5));
                self.gltf_models.push(gltf_model);
            }
            Err(e) => eprintln!("Failed to load GLTF model: {}", e),
        }

        self.loaded = true;
    }

    fn update(&mut self, dt: f64) {
        // Rotation amount for this frame
        let angle = 2.0 * dt as f32;

        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        // Translate camera position so target becomes the origin
        let dx = self.camera.position.x - self.camera.target.x;
        let dz = self.camera.position.z - self.camera.target.z;

        // Rotate around the Y axis
        let rotated_x = dx * cos_angle - dz * sin_angle;
        let rotated_z = dx * sin_angle + dz * cos_angle;

        // Translate back to world space
        self.camera.position.x = self.camera.target.x + rotated_x;
        self.camera.position.z = self.camera.target.z + rotated_z;
    }

    fn render(&self, gl: &GL, shader: &Shader) {
        shader.bind();

        shader.set_uniform_3f("lightDir", 0.2, -1.0, -0.3);
        shader.set_uniform_3f("lightColor", 1.0, 1.0, 1.0);
        shader.set_uniform_3f("ambientColor", 0.3, 0.3, 0.3);

        let pv = self.camera.get_pv();
        shader.set_uniform_mat4fv("pv", 1, gl.boolean.false_, pv.value_ptr());

        for gltf_model in &self.gltf_models {
            shader.set_uniform_mat4fv("model", 1, gl.boolean.false_, gltf_model.model.value_ptr());
            gltf_model.render(gl, shader);
        }
    }

    fn handle_input(&mut self, _event: &Event) {
        // no-op for now
    }

    fn unload(&mut self) {
        self.gltf_models.clear();
        self.loaded = false;
    }
}

impl Drop for MeshScene {
    fn drop(&mut self) {
        self.unload();
    }
}
