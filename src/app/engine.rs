use crate::gl::core::*;
use crate::renderer::scene::{GameScene, Scene};
use crate::renderer::shader::Shader;
use crate::window::x11::{events::*, window::*};
use std::sync::Arc;

pub struct Engine {
    window: X11Window,
    gl: Arc<GL>,
    scene: GameScene,
    shader: Shader,
}

impl Engine {
    pub fn new() -> Self {
        let window = X11Window::new(800, 600, "Space_Invaders").unwrap();
        let gl = Arc::new(GL::load_with(|s| window.get_proc_address(s)));

        gl.enable(gl.buffer.depth_test);

        let mut scene = GameScene::new();
        scene.load(gl.clone());

        let mut shader = Shader::new(gl.clone());
        shader.load_from_files("src/glsl/vert.glsl", "src/glsl/frag.glsl");

        Self {
            window,
            gl,
            scene,
            shader,
        }
    }
    pub fn update(&mut self, _dt: f64) {
        self.gl.clear_color(0.2, 0.3, 0.4, 1.0);
        self.gl
            .clear(self.gl.buffer_bit.color | self.gl.buffer_bit.depth);

        let event = self.window.poll_events();

        match event {
            Some(e) => match e {
                Event::Close => {}
                Event::Resize(width, height) => {
                    self.gl.viewport(0, 0, width as i32, height as i32);
                }
                Event::Iconified(_iconified) => {}
                Event::Focused(_focused) => {}
                Event::Maximized(_maximized) => {}
                Event::Key(_key, _scancode, _action, _mods) => {}
                Event::MouseButton(_button, _action) => {}
                Event::MouseScroll(_x, _y) => {}
                Event::CursorPos(_x, _y) => {}
                Event::CursorEnter(_entered) => {}
            },
            _ => {}
        }

        self.scene.render(&self.gl, &self.shader);

        self.window.swap_buffers();
    }
    pub fn is_open(&self) -> bool {
        !self.window.should_close
    }
}
