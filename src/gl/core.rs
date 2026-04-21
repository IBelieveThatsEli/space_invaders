use super::{constants::*, function::*, types::*};
use std::ffi;

struct Functions {
    clear: GLClear,
    clear_color: GLClearColor,
    viewport: GLViewport,
    create_shader: GLCreateShader,
    delete_shader: GLDeleteShader,
    shader_source: GLShaderSource,
    compile_shader: GLCompileShader,
    create_program: GLCreateProgram,
    attach_shader: GLAttachShader,
    link_program: GLLinkProgram,
    use_program: GLUseProgram,
    gen_buffers: GLGenBuffers,
    delete_buffers: GLDeleteBuffers,
    bind_buffer: GLBindBuffer,
    buffer_data: GLBufferData,
    gen_vertex_arrays: GLGenVertexArrays,
    delete_vertex_arrays: GLDeleteVertexArrays,
    bind_vertex_array: GLBindVertexArray,
    vertex_attrib_pointer: GLVertexAttribPointer,
    enable_vertex_attrib_array: GLEnableVertexAttribArray,
    draw_arrays: GLDrawArrays,
    draw_elements: GLDrawElements,
    get_uniform_location: GLGetUniformLocation,
    uniform_4f: GLUniform4f,
    uniform_mat4fv: GLUniformMatrix4fv,
    tex_parameteri: GLTexParameteri,
    tex_parameterfv: GLTexParameterfv,
    gen_textures: GLGenTextures,
    bind_texture: GLBindTexture,
    tex_image_2d: GLTexImage2D,
    generate_mipmap: GLGenerateMipmap,
    enable: GLEnable,
}

pub struct GL {
    pub boolean: BooleanConstants,
    pub buffer_bit: BufferBitConstants,
    pub primitive: PrimitiveConstants,
    pub data_type: DataTypeConstants,
    pub shader: ShaderConstants,
    pub buffer: BufferConstants,
    pub texture: TextureConstants,
    pub format: FormatConstants,
    functions: Functions,
}

impl GL {
    pub fn load_with<F>(loader: F) -> Self
    where
        F: Fn(&str) -> *const std::ffi::c_void,
    {
        unsafe {
            let constants = Constants::new();
            Self {
                boolean: constants.boolean,
                buffer_bit: constants.buffer_bit,
                primitive: constants.primitive,
                data_type: constants.data_type,
                shader: constants.shader,
                buffer: constants.buffer,
                texture: constants.texture,
                format: constants.format,
                functions: Functions {
                    clear: std::mem::transmute(loader("glClear")),
                    clear_color: std::mem::transmute(loader("glClearColor")),
                    viewport: std::mem::transmute(loader("glViewport")),
                    create_shader: std::mem::transmute(loader("glCreateShader")),
                    delete_shader: std::mem::transmute(loader("glDeleteShader")),
                    shader_source: std::mem::transmute(loader("glShaderSource")),
                    compile_shader: std::mem::transmute(loader("glCompileShader")),
                    create_program: std::mem::transmute(loader("glCreateProgram")),
                    attach_shader: std::mem::transmute(loader("glAttachShader")),
                    link_program: std::mem::transmute(loader("glLinkProgram")),
                    use_program: std::mem::transmute(loader("glUseProgram")),
                    gen_buffers: std::mem::transmute(loader("glGenBuffers")),
                    delete_buffers: std::mem::transmute(loader("glDeleteBuffers")),
                    bind_buffer: std::mem::transmute(loader("glBindBuffer")),
                    buffer_data: std::mem::transmute(loader("glBufferData")),
                    gen_vertex_arrays: std::mem::transmute(loader("glGenVertexArrays")),
                    delete_vertex_arrays: std::mem::transmute(loader("glDeleteVertexArrays")),
                    bind_vertex_array: std::mem::transmute(loader("glBindVertexArray")),
                    vertex_attrib_pointer: std::mem::transmute(loader("glVertexAttribPointer")),
                    enable_vertex_attrib_array: std::mem::transmute(loader(
                        "glEnableVertexAttribArray",
                    )),
                    draw_arrays: std::mem::transmute(loader("glDrawArrays")),
                    draw_elements: std::mem::transmute(loader("glDrawElements")),
                    get_uniform_location: std::mem::transmute(loader("glGetUniformLocation")),
                    uniform_4f: std::mem::transmute(loader("glUniform4f")),
                    uniform_mat4fv: std::mem::transmute(loader("glUniformMatrix4fv")),
                    tex_parameteri: std::mem::transmute(loader("glTexParameteri")),
                    tex_parameterfv: std::mem::transmute(loader("glTexParameterfv")),
                    gen_textures: std::mem::transmute(loader("glGenTextures")),
                    bind_texture: std::mem::transmute(loader("glBindTexture")),
                    tex_image_2d: std::mem::transmute(loader("glTexImage2D")),
                    generate_mipmap: std::mem::transmute(loader("glGenerateMipmap")),
                    enable: std::mem::transmute(loader("glEnable"))
                },
            }
        }
    }

    pub fn clear(&self, mask: GLbitfield) {
        unsafe { (self.functions.clear)(mask) }
    }

    pub fn clear_color(&self, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat) {
        unsafe { (self.functions.clear_color)(r, g, b, a) }
    }

    pub fn viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        unsafe { (self.functions.viewport)(x, y, width, height) }
    }

    pub fn create_shader(&self, shader_type: GLenum) -> GLuint {
        unsafe { (self.functions.create_shader)(shader_type) }
    }

    pub fn delete_shader(&self, shader: GLuint) {
        unsafe { (self.functions.delete_shader)(shader) }
    }

    pub fn shader_source(
        &self,
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint,
    ) {
        unsafe { (self.functions.shader_source)(shader, count, string, length) }
    }

    pub fn compile_shader(&self, shader: GLuint) {
        unsafe { (self.functions.compile_shader)(shader) }
    }

    pub fn create_program(&self) -> GLuint {
        unsafe { (self.functions.create_program)() }
    }

    pub fn attach_shader(&self, program: GLuint, shader: GLuint) {
        unsafe { (self.functions.attach_shader)(program, shader) }
    }

    pub fn link_program(&self, program: GLuint) {
        unsafe { (self.functions.link_program)(program) }
    }

    pub fn use_program(&self, program: GLuint) {
        unsafe { (self.functions.use_program)(program) }
    }

    pub fn gen_buffers(&self, n: GLsizei, buffers: *mut GLuint) {
        unsafe { (self.functions.gen_buffers)(n, buffers) }
    }

    pub fn delete_buffers(&self, n: GLsizei, buffers: *const GLuint) {
        unsafe { (self.functions.delete_buffers)(n, buffers) }
    }

    pub fn bind_buffer(&self, target: GLenum, buffer: GLuint) {
        unsafe { (self.functions.bind_buffer)(target, buffer) }
    }

    pub fn buffer_data(
        &self,
        target: GLenum,
        size: GLsizeiptr,
        data: *const ffi::c_void,
        usage: GLenum,
    ) {
        unsafe { (self.functions.buffer_data)(target, size, data, usage) }
    }

    pub fn gen_vertex_arrays(&self, n: GLsizei, arrays: *mut GLuint) {
        unsafe { (self.functions.gen_vertex_arrays)(n, arrays) }
    }

    pub fn delete_vertex_arrays(&self, n: GLsizei, arrays: *const GLuint) {
        unsafe { (self.functions.delete_vertex_arrays)(n, arrays) }
    }

    pub fn bind_vertex_array(&self, array: GLuint) {
        unsafe { (self.functions.bind_vertex_array)(array) }
    }

    pub fn vertex_attrib_pointer(
        &self,
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const ffi::c_void,
    ) {
        unsafe {
            (self.functions.vertex_attrib_pointer)(index, size, type_, normalized, stride, pointer)
        }
    }

    pub fn enable_vertex_attrib_array(&self, index: GLuint) {
        unsafe { (self.functions.enable_vertex_attrib_array)(index) }
    }

    pub fn draw_arrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
        unsafe { (self.functions.draw_arrays)(mode, first, count) }
    }

    pub fn draw_elements(
        &self,
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const ffi::c_void,
    ) {
        unsafe { (self.functions.draw_elements)(mode, count, type_, indices) }
    }

    pub fn get_uniform_location(&self, program: GLuint, name: *const GLchar) -> GLint {
        unsafe { (self.functions.get_uniform_location)(program, name) }
    }

    pub fn uniform_4f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
        unsafe { (self.functions.uniform_4f)(location, v0, v1, v2, v3) }
    }

    pub fn uniform_mat4fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        unsafe { (self.functions.uniform_mat4fv)(location, count, transpose, value) }
    }

    pub fn tex_parameteri(&self, target: GLenum, pname: GLenum, param: GLint) {
        unsafe { (self.functions.tex_parameteri)(target, pname, param) }
    }

    pub fn tex_parameterfv(&self, target: GLenum, pname: GLenum, params: *const GLfloat) {
        unsafe { (self.functions.tex_parameterfv)(target, pname, params) }
    }

    pub fn gen_textures(&self, n: GLsizei, textures: *mut GLuint) {
        unsafe { (self.functions.gen_textures)(n, textures) }
    }

    pub fn bind_texture(&self, target: GLenum, texture: GLuint) {
        unsafe { (self.functions.bind_texture)(target, texture) }
    }

    pub fn tex_image_2d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        data: *const ffi::c_void,
    ) {
        unsafe {
            (self.functions.tex_image_2d)(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                format,
                type_,
                data,
            )
        }
    }

    pub fn generate_mipmap(&self, target: GLenum) {
        unsafe { (self.functions.generate_mipmap)(target) }
    }

    pub fn enable(&self, cap: GLenum) {
        unsafe { (self.functions.enable)(cap) }
    }
}
