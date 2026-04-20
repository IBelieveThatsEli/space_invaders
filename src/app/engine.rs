use crate::gl::core::*;
use crate::renderer::{buffer::*, camera::*, shader::*, texture::*, vertex_array::*};
use crate::window::x11::{events::Event, window::X11Window};
use rmath::common::to_radians;
use rmath::{Mat4, Vec3};
use std::ffi;
use std::sync::Arc;

pub struct Engine {
    window: X11Window,
    gl: Arc<GL>,
    shader: Shader,
    texture: Texture,
    camera: Camera,
    vao: VAO,
    vbo: VBO,
    ebo: EBO,
}

impl Engine {
    pub fn new() -> Self {
        let window = X11Window::new(800, 600, "Space_Invaders").unwrap();
        let gl = Arc::new(GL::load_with(|s| window.get_proc_address(s)));

        let mut shader = Shader::new(gl.clone());
        shader.load_from_files("src/glsl/vert.glsl", "src/glsl/frag.glsl");

        let texture = Texture::new(gl.clone(), "assets/box_texture.png");

        let vertices: [f32; _] = [
            1.0, 1.0, 0.0, 1.0, 1.0, // top right
            1.0, -1.0, 0.0, 1.0, 0.0, // bottom right
            -1.0, -1.0, 0.0, 0.0, 0.0, // bottom left
            -1.0, 1.0, 0.0, 0.0, 1.0, // top left
        ];
        let indices: [u32; _] = [0, 1, 3, 1, 2, 3];

        let vao = VAO::new(gl.clone());
        vao.bind();

        let ebo = EBO::new(gl.clone(), &indices);
        let vbo = VBO::new(gl.clone(), &vertices);

        vao.attrib_pointer(
            0,
            3,
            gl.data_type.float,
            gl.boolean.false_,
            5 * std::mem::size_of::<f32>() as i32,
            std::ptr::null(),
        );

        vao.attrib_pointer(
            1,
            2,
            gl.data_type.float,
            gl.boolean.false_,
            5 * std::mem::size_of::<f32>() as i32,
            (3 * std::mem::size_of::<f32>()) as *const ffi::c_void,
        );

        let mut model = Mat4::identity();
        model.rotate(&Vec3::init(0.0, 0.0, 0.0));

        let camera = Camera::new(
            Vec3::init(0.0, 0.0, -3.0),
            to_radians(45.0),
            16.0 / 9.0,
            0.1,
            100.0,
        );

        shader.bind();
        shader.set_uniform_mat4fv("model", 1, gl.boolean.false_, model.value_ptr());
        shader.set_uniform_mat4fv("pv", 1, gl.boolean.false_, camera.get_pv().value_ptr());

        Self {
            window,
            gl,
            shader,
            texture,
            camera,
            vao,
            vbo,
            ebo,
        }
    }
    pub fn update(&mut self, _dt: f32) {
        self.gl.clear_color(0.2, 0.3, 0.4, 1.0);
        self.gl.clear(self.gl.buffer_bit.color);

        let event = self.window.poll_events();

        match event {
            Some(e) => match e {
                Event::Close => {}
                Event::Resize(_width, _height) => {}
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

        self.shader.bind();
        self.texture.bind();
        self.vao.bind();
        self.ebo.bind();
        self.gl.draw_elements(
            self.gl.primitive.triangles,
            6,
            self.gl.data_type.unsigned_int,
            std::ptr::null(),
        );

        self.window.swap_buffers();
    }
    pub fn is_open(&self) -> bool {
        !self.window.should_close
    }
}
