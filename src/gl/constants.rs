use super::types::*;

pub struct Constants {
    pub boolean: BooleanConstants,
    pub buffer_bit: BufferBitConstants,
    pub primitive: PrimitiveConstants,
    pub data_type: DataTypeConstants,
    pub shader: ShaderConstants,
    pub buffer: BufferConstants,
    pub texture: TextureConstants,
    pub format: FormatConstants,
}

impl Constants {
    pub const fn new() -> Self {
        Self {
            boolean: BooleanConstants { false_: 0 },
            buffer_bit: BufferBitConstants {
                color: 0x00004000,
                depth: 0x00000100,
            },
            primitive: PrimitiveConstants { triangles: 0x0004 },
            data_type: DataTypeConstants {
                float: 0x1406,
                unsigned_int: 0x1405,
                unsigned_byte: 0x1401,
            },
            shader: ShaderConstants {
                vertex: 0x8B31,
                fragment: 0x8B30,
            },
            buffer: BufferConstants {
                array: 0x8892,
                element_array: 0x8893,
                static_draw: 0x88E4,
                depth_test: 0x0B71,
            },
            texture: TextureConstants {
                texture_2d: 0x0DE1,

                wrap_s: 0x2802,
                wrap_t: 0x2803,
                repeat: 0x2901,

                min_filter: 0x2801,
                mag_filter: 0x2800,
                linear: 0x2601,
                linear_mipmap_linear: 0x2703,
            },
            format: FormatConstants {
                rgb: 0x1907,
                rgba: 0x1908,
            },
        }
    }
}

pub struct BooleanConstants {
    pub false_: GLboolean,
}
pub struct BufferBitConstants {
    pub color: GLbitfield,
    pub depth: GLenum,
}
pub struct PrimitiveConstants {
    pub triangles: GLenum,
}

pub struct DataTypeConstants {
    pub float: GLenum,
    pub unsigned_int: GLenum,
    pub unsigned_byte: GLenum,
}

pub struct ShaderConstants {
    pub vertex: GLenum,
    pub fragment: GLenum,
}

pub struct BufferConstants {
    //targets
    pub array: GLenum,
    pub element_array: GLenum,

    //usage
    pub static_draw: GLenum,

    pub depth_test: GLenum,
}

pub struct TextureConstants {
    //targets
    pub texture_2d: GLenum,

    //wrapping
    pub wrap_s: GLenum,
    pub wrap_t: GLenum,
    pub repeat: GLenum,

    //filtering
    pub min_filter: GLenum,
    pub mag_filter: GLenum,
    pub linear: GLenum,
    pub linear_mipmap_linear: GLenum,
}

pub struct FormatConstants {
    pub rgb: GLenum,
    pub rgba: GLenum,
}
