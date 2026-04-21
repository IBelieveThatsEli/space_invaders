use crate::gl::types::*;
use std::ffi;

pub type GLClear = unsafe extern "C" fn(mask: GLbitfield);
pub type GLClearColor = unsafe extern "C" fn(r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat);
pub type GLViewport = unsafe extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub type GLCreateShader = unsafe extern "C" fn(shader_type: GLenum) -> GLuint;
pub type GLDeleteShader = unsafe extern "C" fn(shader: GLuint);
pub type GLShaderSource = unsafe extern "C" fn(
    shader: GLuint,
    count: GLsizei,
    string: *const *const GLchar,
    length: *const GLint,
);
pub type GLCompileShader = unsafe extern "C" fn(shader: GLuint);
pub type GLCreateProgram = unsafe extern "C" fn() -> GLuint;
pub type GLAttachShader = unsafe extern "C" fn(program: GLuint, shader: GLuint);
pub type GLLinkProgram = unsafe extern "C" fn(program: GLuint);
pub type GLUseProgram = unsafe extern "C" fn(program: GLuint);

pub type GLGenBuffers = unsafe extern "C" fn(n: GLsizei, buffers: *mut GLuint);
pub type GLDeleteBuffers = unsafe extern "C" fn(n: GLsizei, buffers: *const GLuint);
pub type GLBindBuffer = unsafe extern "C" fn(target: GLenum, buffer: GLuint);
pub type GLBufferData =
    unsafe extern "C" fn(target: GLenum, size: GLsizeiptr, data: *const ffi::c_void, usage: GLenum);

pub type GLGenVertexArrays = unsafe extern "C" fn(n: GLsizei, arrays: *mut GLuint);
pub type GLDeleteVertexArrays = unsafe extern "C" fn(n: GLsizei, arrays: *const GLuint);
pub type GLBindVertexArray = unsafe extern "C" fn(array: GLuint);
pub type GLVertexAttribPointer = unsafe extern "C" fn(
    index: GLuint,
    size: GLint,
    type_: GLenum,
    normalized: GLboolean,
    stride: GLsizei,
    pointer: *const ffi::c_void,
);
pub type GLEnableVertexAttribArray = unsafe extern "C" fn(index: GLuint);

pub type GLDrawArrays = unsafe extern "C" fn(mode: GLenum, first: GLint, count: GLsizei);
pub type GLDrawElements =
    unsafe extern "C" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const ffi::c_void);

pub type GLGetUniformLocation = unsafe extern "C" fn(program: GLuint, name: *const GLchar) -> GLint;
pub type GLUniform4f =
    unsafe extern "C" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
pub type GLUniformMatrix4fv = unsafe extern "C" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
);

pub type GLTexParameteri = unsafe extern "C" fn(target: GLenum, pname: GLenum, param: GLint);
pub type GLTexParameterfv =
    unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *const GLfloat);
pub type GLGenTextures = unsafe extern "C" fn(n: GLsizei, textures: *mut GLuint);
pub type GLBindTexture = unsafe extern "C" fn(target: GLenum, texture: GLuint);
pub type GLTexImage2D = unsafe extern "C" fn(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    data: *const ffi::c_void,
);
pub type GLGenerateMipmap = unsafe extern "C" fn(target: GLenum);

pub type GLEnable = unsafe extern "C" fn(cap: GLenum);
