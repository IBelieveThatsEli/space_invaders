use std::{ffi::c_void, sync::Arc};

use crate::gl::{core::GL, types::*};
use image::GenericImageView;

pub struct Texture {
    gl: Arc<GL>,
    id: GLuint,
}

impl Texture {
    pub fn new(gl: Arc<GL>, image_path: &str) -> Self {
        let mut id: GLuint = 0;

        gl.gen_textures(1, &mut id);
        gl.bind_texture(gl.texture.texture_2d, id);

        gl.tex_parameteri(
            gl.texture.texture_2d,
            gl.texture.wrap_s,
            gl.texture.repeat as i32,
        );
        gl.tex_parameteri(
            gl.texture.texture_2d,
            gl.texture.wrap_t,
            gl.texture.repeat as i32,
        );
        gl.tex_parameteri(
            gl.texture.texture_2d,
            gl.texture.min_filter,
            gl.texture.linear_mipmap_linear as i32,
        );
        gl.tex_parameteri(
            gl.texture.texture_2d,
            gl.texture.mag_filter,
            gl.texture.linear as i32,
        );

        let img = image::open(image_path).unwrap();
        let data = img.to_rgba8();

        let (width, height) = img.dimensions();

        gl.tex_image_2d(
            gl.texture.texture_2d,
            0,
            gl.format.rgba as i32,
            width as i32,
            height as i32,
            0,
            gl.format.rgba,
            gl.data_type.unsigned_byte,
            data.as_ptr() as *const _,
        );
        gl.generate_mipmap(gl.texture.texture_2d);

        Self { id, gl }
    }

    pub fn bind(&self) {
        self.gl.bind_texture(self.gl.texture.texture_2d, self.id);
    }
}
