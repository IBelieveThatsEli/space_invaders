// use super::glb::Glb;
// use crate::json::Json;

// #[derive(Debug)]
// pub struct GltfModel {
//     pub meshes: Vec<GltfMesh>,
//     pub buffer_data: Vec<u8>,
// }

// #[derive(Debug)]
// pub struct GltfMesh {
//     pub primitives: Vec<Primitive>,
// }

// #[derive(Debug)]
// pub struct Primitive {
//     pub positions: Vec<f32>,
//     pub normals: Option<Vec<f32>>,
//     pub tex_coords: Option<Vec<f32>>,
//     pub indices: Option<Vec<u32>>,
// }

// impl GltfModel {
//     pub fn load(path: &str) -> Result<Self, String> {
//         let glb = Glb::load(path)?;
//         let json = Json::parse(&glb.json)?;
//         Self::from_json(json, glb.bin)
//     }

//     fn from_json(json: Json, buffer: Vec<u8>) -> Result<Self, String> {
//         let root = json.as_object().ok_or("Root must be object")?;

//         let buffer_views = parse_buffer_views(root.get("bufferViews"))?;
//         let accessors = parse_accessors(root.get("accessors"))?;
//         let meshes = parse_meshes(root.get("meshes"), &accessors, &buffer_views, &buffer)?;

//         Ok(Self {
//             meshes,
//             buffer_data: buffer,
//         })
//     }
// }

// #[derive(Clone)]
// struct BufferView {
//     offset: usize,
//     length: usize,
// }

// struct Accessor {
//     buffer_view: usize,
//     count: usize,
//     component_type: u32,
//     data_type: DataType,
// }

// #[derive(Clone, Copy)]
// enum DataType {
//     Scalar,
//     Vec2,
//     Vec3,
//     Vec4,
// }

// impl DataType {
//     fn from_str(s: &str) -> Option<Self> {
//         match s {
//             "SCALAR" => Some(Self::Scalar),
//             "VEC2" => Some(Self::Vec2),
//             "VEC3" => Some(Self::Vec3),
//             "VEC4" => Some(Self::Vec4),
//             _ => None,
//         }
//     }

//     fn components(&self) -> usize {
//         match self {
//             Self::Scalar => 1,
//             Self::Vec2 => 2,
//             Self::Vec3 => 3,
//             Self::Vec4 => 4,
//         }
//     }
// }

// fn parse_buffer_views(json: Option<&Json>) -> Result<Vec<BufferView>, String> {
//     let arr = match json.and_then(|j| j.as_array()) {
//         Some(a) => a,
//         None => return Ok(Vec::new()),
//     };

//     arr.iter()
//         .map(|v| {
//             let obj = v.as_object().ok_or("BufferView must be object")?;
//             Ok(BufferView {
//                 offset: obj
//                     .get("byteOffset")
//                     .and_then(|j| j.as_usize())
//                     .unwrap_or(0),
//                 length: obj
//                     .get("byteLength")
//                     .and_then(|j| j.as_usize())
//                     .ok_or("Missing byteLength")?,
//             })
//         })
//         .collect()
// }

// fn parse_accessors(json: Option<&Json>) -> Result<Vec<Accessor>, String> {
//     let arr = match json.and_then(|j| j.as_array()) {
//         Some(a) => a,
//         None => return Ok(Vec::new()),
//     };

//     arr.iter()
//         .map(|v| {
//             let obj = v.as_object().ok_or("Accessor must be object")?;
//             Ok(Accessor {
//                 buffer_view: obj
//                     .get("bufferView")
//                     .and_then(|j| j.as_usize())
//                     .ok_or("Missing bufferView")?,
//                 count: obj
//                     .get("count")
//                     .and_then(|j| j.as_usize())
//                     .ok_or("Missing count")?,
//                 component_type: obj
//                     .get("componentType")
//                     .and_then(|j| j.as_usize())
//                     .ok_or("Missing componentType")? as u32,
//                 data_type: obj
//                     .get("type")
//                     .and_then(|j| j.as_str())
//                     .and_then(DataType::from_str)
//                     .ok_or("Missing or invalid type")?,
//             })
//         })
//         .collect()
// }

// fn parse_meshes(
//     json: Option<&Json>,
//     accessors: &[Accessor],
//     buffer_views: &[BufferView],
//     buffer: &[u8],
// ) -> Result<Vec<GltfMesh>, String> {
//     let arr = match json.and_then(|j| j.as_array()) {
//         Some(a) => a,
//         None => return Ok(Vec::new()),
//     };

//     arr.iter()
//         .map(|mesh| {
//             let obj = mesh.as_object().ok_or("Mesh must be object")?;
//             let prims = obj
//                 .get("primitives")
//                 .and_then(|p| p.as_array())
//                 .ok_or("Missing primitives")?;

//             let primitives = prims
//                 .iter()
//                 .filter_map(|p| parse_primitive(p, accessors, buffer_views, buffer).ok())
//                 .collect();

//             Ok(GltfMesh { primitives })
//         })
//         .collect()
// }

// fn parse_primitive(
//     json: &Json,
//     accessors: &[Accessor],
//     buffer_views: &[BufferView],
//     buffer: &[u8],
// ) -> Result<Primitive, String> {
//     let obj = json.as_object().ok_or("Primitive must be object")?;
//     let attrs = obj
//         .get("attributes")
//         .and_then(|a| a.as_object())
//         .ok_or("Missing attributes")?;

//     let pos_idx = attrs
//         .get("POSITION")
//         .and_then(|p| p.as_usize())
//         .ok_or("Missing POSITION")?;
//     let positions = read_vec3_accessor(pos_idx, accessors, buffer_views, buffer)?;

//     let normals = attrs
//         .get("NORMAL")
//         .and_then(|n| n.as_usize())
//         .map(|idx| read_vec3_accessor(idx, accessors, buffer_views, buffer))
//         .transpose()?;

//     let tex_coords = attrs
//         .get("TEXCOORD_0")
//         .and_then(|t| t.as_usize())
//         .map(|idx| read_vec2_accessor(idx, accessors, buffer_views, buffer))
//         .transpose()?;

//     let indices = obj
//         .get("indices")
//         .and_then(|i| i.as_usize())
//         .map(|idx| read_indices_accessor(idx, accessors, buffer_views, buffer))
//         .transpose()?;

//     Ok(Primitive {
//         positions,
//         normals,
//         tex_coords,
//         indices,
//     })
// }

// fn read_vec3_accessor(
//     idx: usize,
//     accessors: &[Accessor],
//     buffer_views: &[BufferView],
//     buffer: &[u8],
// ) -> Result<Vec<f32>, String> {
//     let acc = accessors.get(idx).ok_or("Invalid accessor index")?;
//     let view = buffer_views
//         .get(acc.buffer_view)
//         .ok_or("Invalid buffer view")?;

//     if acc.component_type != 5126 {
//         return Err("Must be FLOAT".into());
//     }
//     if acc.data_type.components() != 3 {
//         return Err("Must be VEC3".into());
//     }

//     let data = &buffer[view.offset..view.offset + view.length];
//     let floats: Vec<f32> = data
//         .chunks_exact(4)
//         .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
//         .collect();

//     if floats.len() != acc.count * 3 {
//         return Err("Data size mismatch".into());
//     }

//     Ok(floats)
// }

// fn read_vec2_accessor(
//     idx: usize,
//     accessors: &[Accessor],
//     buffer_views: &[BufferView],
//     buffer: &[u8],
// ) -> Result<Vec<f32>, String> {
//     let acc = accessors.get(idx).ok_or("Invalid accessor index")?;
//     let view = buffer_views
//         .get(acc.buffer_view)
//         .ok_or("Invalid buffer view")?;

//     if acc.component_type != 5126 {
//         return Err("Must be FLOAT".into());
//     }
//     if acc.data_type.components() != 2 {
//         return Err("Must be VEC2".into());
//     }

//     let data = &buffer[view.offset..view.offset + view.length];
//     let floats: Vec<f32> = data
//         .chunks_exact(4)
//         .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
//         .collect();

//     if floats.len() != acc.count * 2 {
//         return Err("Data size mismatch".into());
//     }

//     Ok(floats)
// }

// fn read_indices_accessor(
//     idx: usize,
//     accessors: &[Accessor],
//     buffer_views: &[BufferView],
//     buffer: &[u8],
// ) -> Result<Vec<u32>, String> {
//     let acc = accessors.get(idx).ok_or("Invalid accessor index")?;
//     let view = buffer_views
//         .get(acc.buffer_view)
//         .ok_or("Invalid buffer view")?;

//     let data = &buffer[view.offset..view.offset + view.length];

//     let indices = match acc.component_type {
//         5121 => data.iter().map(|&b| b as u32).collect(), // UNSIGNED_BYTE
//         5123 => data
//             .chunks_exact(2)
//             .map(|c| u16::from_le_bytes([c[0], c[1]]) as u32)
//             .collect(), // UNSIGNED_SHORT
//         5125 => data
//             .chunks_exact(4)
//             .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
//             .collect(), // UNSIGNED_INT
//         _ => return Err("Unsupported index type".into()),
//     };

//     Ok(indices)
// }
use super::glb::Glb;
use crate::json::Json;

#[derive(Debug, Clone)]
pub struct Material {
    pub base_color_factor: [f32; 4],
    pub emissive_factor: [f32; 3],
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub alpha_mode: AlphaMode,
    pub double_sided: bool,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            base_color_factor: [1.0, 1.0, 1.0, 1.0],
            emissive_factor: [0.0, 0.0, 0.0],
            metallic_factor: 1.0,
            roughness_factor: 1.0,
            alpha_mode: AlphaMode::Opaque,
            double_sided: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AlphaMode {
    Opaque,
    Mask,
    Blend,
}

impl AlphaMode {
    fn from_str(value: &str) -> Self {
        match value {
            "MASK" => Self::Mask,
            "BLEND" => Self::Blend,
            _ => Self::Opaque,
        }
    }

    pub fn as_int(self) -> i32 {
        match self {
            Self::Opaque => 0,
            Self::Mask => 1,
            Self::Blend => 2,
        }
    }
}

#[derive(Debug)]
pub struct GltfModel {
    pub meshes: Vec<GltfMesh>,
    pub buffer_data: Vec<u8>,
}

#[derive(Debug)]
pub struct GltfMesh {
    pub primitives: Vec<Primitive>,
}

#[derive(Debug)]
pub struct Primitive {
    pub positions: Vec<f32>,
    pub normals: Option<Vec<f32>>,
    pub tex_coords: Option<Vec<f32>>,
    pub indices: Option<Vec<u32>>,
    pub material: Material,
}

impl GltfModel {
    pub fn load(path: &str) -> Result<Self, String> {
        let glb = Glb::load(path)?;
        let json = Json::parse(&glb.json)?;
        Self::from_json(json, glb.bin)
    }

    fn from_json(json: Json, buffer: Vec<u8>) -> Result<Self, String> {
        let root = json.as_object().ok_or("Root must be object")?;

        let buffer_views = parse_buffer_views(root.get("bufferViews"))?;
        let accessors = parse_accessors(root.get("accessors"))?;
        let materials = parse_materials(root.get("materials"))?;
        let meshes = parse_meshes(
            root.get("meshes"),
            &accessors,
            &buffer_views,
            &buffer,
            &materials,
        )?;

        Ok(Self {
            meshes,
            buffer_data: buffer,
        })
    }
}

#[derive(Clone)]
struct BufferView {
    offset: usize,
    length: usize,
}

struct Accessor {
    buffer_view: usize,
    count: usize,
    component_type: u32,
    data_type: DataType,
}

#[derive(Clone, Copy)]
enum DataType {
    Scalar,
    Vec2,
    Vec3,
    Vec4,
}

impl DataType {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "SCALAR" => Some(Self::Scalar),
            "VEC2" => Some(Self::Vec2),
            "VEC3" => Some(Self::Vec3),
            "VEC4" => Some(Self::Vec4),
            _ => None,
        }
    }

    fn components(&self) -> usize {
        match self {
            Self::Scalar => 1,
            Self::Vec2 => 2,
            Self::Vec3 => 3,
            Self::Vec4 => 4,
        }
    }
}

fn parse_buffer_views(json: Option<&Json>) -> Result<Vec<BufferView>, String> {
    let arr = match json.and_then(|j| j.as_array()) {
        Some(a) => a,
        None => return Ok(Vec::new()),
    };

    arr.iter()
        .map(|v| {
            let obj = v.as_object().ok_or("BufferView must be object")?;
            Ok(BufferView {
                offset: obj
                    .get("byteOffset")
                    .and_then(|j| j.as_usize())
                    .unwrap_or(0),
                length: obj
                    .get("byteLength")
                    .and_then(|j| j.as_usize())
                    .ok_or("Missing byteLength")?,
            })
        })
        .collect()
}

fn parse_accessors(json: Option<&Json>) -> Result<Vec<Accessor>, String> {
    let arr = match json.and_then(|j| j.as_array()) {
        Some(a) => a,
        None => return Ok(Vec::new()),
    };

    arr.iter()
        .map(|v| {
            let obj = v.as_object().ok_or("Accessor must be object")?;
            Ok(Accessor {
                buffer_view: obj
                    .get("bufferView")
                    .and_then(|j| j.as_usize())
                    .ok_or("Missing bufferView")?,
                count: obj
                    .get("count")
                    .and_then(|j| j.as_usize())
                    .ok_or("Missing count")?,
                component_type: obj
                    .get("componentType")
                    .and_then(|j| j.as_usize())
                    .ok_or("Missing componentType")? as u32,
                data_type: obj
                    .get("type")
                    .and_then(|j| j.as_str())
                    .and_then(DataType::from_str)
                    .ok_or("Missing or invalid type")?,
            })
        })
        .collect()
}

fn parse_materials(json: Option<&Json>) -> Result<Vec<Material>, String> {
    let arr = match json.and_then(|j| j.as_array()) {
        Some(a) => a,
        None => return Ok(Vec::new()),
    };

    arr.iter().map(parse_material).collect()
}

fn parse_material(json: &Json) -> Result<Material, String> {
    let obj = json.as_object().ok_or("Material must be object")?;

    let pbr = obj.get("pbrMetallicRoughness").and_then(|j| j.as_object());

    let base_color_factor = pbr
        .and_then(|p| p.get("baseColorFactor"))
        .and_then(parse_vec4)
        .unwrap_or([1.0, 1.0, 1.0, 1.0]);

    let metallic_factor = pbr
        .and_then(|p| p.get("metallicFactor"))
        .and_then(|j| j.as_f32())
        .unwrap_or(1.0);

    let roughness_factor = pbr
        .and_then(|p| p.get("roughnessFactor"))
        .and_then(|j| j.as_f32())
        .unwrap_or(1.0);

    let emissive_factor = obj
        .get("emissiveFactor")
        .and_then(parse_vec3)
        .unwrap_or([0.0, 0.0, 0.0]);

    let alpha_mode = obj
        .get("alphaMode")
        .and_then(|j| j.as_str())
        .map(AlphaMode::from_str)
        .unwrap_or(AlphaMode::Opaque);

    let double_sided = obj
        .get("doubleSided")
        .and_then(|j| j.as_bool())
        .unwrap_or(false);

    Ok(Material {
        base_color_factor,
        emissive_factor,
        metallic_factor,
        roughness_factor,
        alpha_mode,
        double_sided,
    })
}

fn parse_vec3(json: &Json) -> Option<[f32; 3]> {
    let arr = json.as_array()?;
    if arr.len() < 3 {
        return None;
    }

    Some([arr[0].as_f32()?, arr[1].as_f32()?, arr[2].as_f32()?])
}

fn parse_vec4(json: &Json) -> Option<[f32; 4]> {
    let arr = json.as_array()?;
    if arr.len() < 4 {
        return None;
    }

    Some([
        arr[0].as_f32()?,
        arr[1].as_f32()?,
        arr[2].as_f32()?,
        arr[3].as_f32()?,
    ])
}

fn parse_meshes(
    json: Option<&Json>,
    accessors: &[Accessor],
    buffer_views: &[BufferView],
    buffer: &[u8],
    materials: &[Material],
) -> Result<Vec<GltfMesh>, String> {
    let arr = match json.and_then(|j| j.as_array()) {
        Some(a) => a,
        None => return Ok(Vec::new()),
    };

    arr.iter()
        .map(|mesh| {
            let obj = mesh.as_object().ok_or("Mesh must be object")?;
            let prims = obj
                .get("primitives")
                .and_then(|p| p.as_array())
                .ok_or("Missing primitives")?;

            let primitives = prims
                .iter()
                .filter_map(|p| parse_primitive(p, accessors, buffer_views, buffer, materials).ok())
                .collect();

            Ok(GltfMesh { primitives })
        })
        .collect()
}

fn parse_primitive(
    json: &Json,
    accessors: &[Accessor],
    buffer_views: &[BufferView],
    buffer: &[u8],
    materials: &[Material],
) -> Result<Primitive, String> {
    let obj = json.as_object().ok_or("Primitive must be object")?;
    let attrs = obj
        .get("attributes")
        .and_then(|a| a.as_object())
        .ok_or("Missing attributes")?;

    let pos_idx = attrs
        .get("POSITION")
        .and_then(|p| p.as_usize())
        .ok_or("Missing POSITION")?;
    let positions = read_vec3_accessor(pos_idx, accessors, buffer_views, buffer)?;

    let normals = attrs
        .get("NORMAL")
        .and_then(|n| n.as_usize())
        .map(|idx| read_vec3_accessor(idx, accessors, buffer_views, buffer))
        .transpose()?;

    let tex_coords = attrs
        .get("TEXCOORD_0")
        .and_then(|t| t.as_usize())
        .map(|idx| read_vec2_accessor(idx, accessors, buffer_views, buffer))
        .transpose()?;

    let indices = obj
        .get("indices")
        .and_then(|i| i.as_usize())
        .map(|idx| read_indices_accessor(idx, accessors, buffer_views, buffer))
        .transpose()?;

    let material = obj
        .get("material")
        .and_then(|m| m.as_usize())
        .and_then(|idx| materials.get(idx).cloned())
        .unwrap_or_default();

    Ok(Primitive {
        positions,
        normals,
        tex_coords,
        indices,
        material,
    })
}

fn read_vec3_accessor(
    idx: usize,
    accessors: &[Accessor],
    buffer_views: &[BufferView],
    buffer: &[u8],
) -> Result<Vec<f32>, String> {
    let acc = accessors.get(idx).ok_or("Invalid accessor index")?;
    let view = buffer_views
        .get(acc.buffer_view)
        .ok_or("Invalid buffer view")?;

    if acc.component_type != 5126 {
        return Err("Must be FLOAT".into());
    }
    if acc.data_type.components() != 3 {
        return Err("Must be VEC3".into());
    }

    let data = &buffer[view.offset..view.offset + view.length];
    let floats: Vec<f32> = data
        .chunks_exact(4)
        .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect();

    if floats.len() != acc.count * 3 {
        return Err("Data size mismatch".into());
    }

    Ok(floats)
}

fn read_vec2_accessor(
    idx: usize,
    accessors: &[Accessor],
    buffer_views: &[BufferView],
    buffer: &[u8],
) -> Result<Vec<f32>, String> {
    let acc = accessors.get(idx).ok_or("Invalid accessor index")?;
    let view = buffer_views
        .get(acc.buffer_view)
        .ok_or("Invalid buffer view")?;

    if acc.component_type != 5126 {
        return Err("Must be FLOAT".into());
    }
    if acc.data_type.components() != 2 {
        return Err("Must be VEC2".into());
    }

    let data = &buffer[view.offset..view.offset + view.length];
    let floats: Vec<f32> = data
        .chunks_exact(4)
        .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect();

    if floats.len() != acc.count * 2 {
        return Err("Data size mismatch".into());
    }

    Ok(floats)
}

fn read_indices_accessor(
    idx: usize,
    accessors: &[Accessor],
    buffer_views: &[BufferView],
    buffer: &[u8],
) -> Result<Vec<u32>, String> {
    let acc = accessors.get(idx).ok_or("Invalid accessor index")?;
    let view = buffer_views
        .get(acc.buffer_view)
        .ok_or("Invalid buffer view")?;

    let data = &buffer[view.offset..view.offset + view.length];

    let indices = match acc.component_type {
        5121 => data.iter().map(|&b| b as u32).collect(),
        5123 => data
            .chunks_exact(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]) as u32)
            .collect(),
        5125 => data
            .chunks_exact(4)
            .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect(),
        _ => return Err("Unsupported index type".into()),
    };

    Ok(indices)
}
