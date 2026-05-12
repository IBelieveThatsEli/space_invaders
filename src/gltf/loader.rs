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

#[derive(Debug, Clone)]
pub struct NodeTransform {
    pub translation: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

impl Default for NodeTransform {
    fn default() -> Self {
        Self {
            translation: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
        }
    }
}

#[derive(Debug, Clone)]
pub enum AnimationTargetPath {
    Translation,
    Rotation,
    Scale,
}

#[derive(Debug, Clone)]
pub enum Interpolation {
    Step,
    Linear,
}

impl Interpolation {
    fn from_str(value: &str) -> Self {
        match value {
            "STEP" => Self::Step,
            _ => Self::Linear,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AnimationOutput {
    Vec3(Vec<[f32; 3]>),
    Vec4(Vec<[f32; 4]>),
}

#[derive(Debug, Clone)]
pub struct AnimationSampler {
    pub input_times: Vec<f32>,
    pub output: AnimationOutput,
    pub interpolation: Interpolation,
}

#[derive(Debug, Clone)]
pub struct AnimationChannel {
    pub sampler_index: usize,
    pub target_node: usize,
    pub target_path: AnimationTargetPath,
}

#[derive(Debug, Clone)]
pub struct AnimationClip {
    pub name: String,
    pub samplers: Vec<AnimationSampler>,
    pub channels: Vec<AnimationChannel>,
    pub duration: f32,
}

#[derive(Debug)]
pub struct GltfModel {
    pub meshes: Vec<GltfMesh>,
    pub nodes: Vec<Node>,
    pub animations: Vec<AnimationClip>,
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
    pub node_index: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub mesh: Option<usize>,
    pub children: Vec<usize>,
    pub transform: NodeTransform,
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
        let nodes = parse_nodes(root.get("nodes"))?;
        let meshes = parse_meshes(
            root.get("meshes"),
            &accessors,
            &buffer_views,
            &buffer,
            &materials,
            &nodes,
        )?;
        let animations =
            parse_animations(root.get("animations"), &accessors, &buffer_views, &buffer)?;

        Ok(Self {
            meshes,
            nodes,
            animations,
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

fn parse_nodes(json: Option<&Json>) -> Result<Vec<Node>, String> {
    let arr = match json.and_then(|j| j.as_array()) {
        Some(a) => a,
        None => return Ok(Vec::new()),
    };

    arr.iter()
        .enumerate()
        .map(|(index, node)| {
            let obj = node.as_object().ok_or("Node must be object")?;

            let name = obj
                .get("name")
                .and_then(|j| j.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("Node{}", index));

            let mesh = obj.get("mesh").and_then(|j| j.as_usize());

            let children = obj
                .get("children")
                .and_then(|j| j.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_usize()).collect())
                .unwrap_or_default();

            let mut transform = NodeTransform::default();

            if let Some(t) = obj.get("translation").and_then(parse_vec3) {
                transform.translation = t;
            }
            if let Some(r) = obj.get("rotation").and_then(parse_vec4) {
                transform.rotation = r;
            }
            if let Some(s) = obj.get("scale").and_then(parse_vec3) {
                transform.scale = s;
            }

            Ok(Node {
                name,
                mesh,
                children,
                transform,
            })
        })
        .collect()
}

fn parse_meshes(
    json: Option<&Json>,
    accessors: &[Accessor],
    buffer_views: &[BufferView],
    buffer: &[u8],
    materials: &[Material],
    nodes: &[Node],
) -> Result<Vec<GltfMesh>, String> {
    let arr = match json.and_then(|j| j.as_array()) {
        Some(a) => a,
        None => return Ok(Vec::new()),
    };

    arr.iter()
        .enumerate()
        .map(|(mesh_index, mesh)| {
            let obj = mesh.as_object().ok_or("Mesh must be object")?;
            let prims = obj
                .get("primitives")
                .and_then(|p| p.as_array())
                .ok_or("Missing primitives")?;

            let node_index = nodes.iter().position(|n| n.mesh == Some(mesh_index));

            let primitives = prims
                .iter()
                .filter_map(|p| {
                    parse_primitive(p, accessors, buffer_views, buffer, materials, node_index).ok()
                })
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
    node_index: Option<usize>,
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
        node_index,
    })
}

fn parse_animations(
    json: Option<&Json>,
    accessors: &[Accessor],
    buffer_views: &[BufferView],
    buffer: &[u8],
) -> Result<Vec<AnimationClip>, String> {
    let arr = match json.and_then(|j| j.as_array()) {
        Some(a) => a,
        None => return Ok(Vec::new()),
    };

    arr.iter()
        .enumerate()
        .map(|(index, anim)| {
            let obj = anim.as_object().ok_or("Animation must be object")?;

            let name = obj
                .get("name")
                .and_then(|j| j.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("Animation{}", index));

            let samplers_json = obj
                .get("samplers")
                .and_then(|j| j.as_array())
                .ok_or("Animation missing samplers")?;

            let samplers: Vec<AnimationSampler> = samplers_json
                .iter()
                .map(|sampler| parse_animation_sampler(sampler, accessors, buffer_views, buffer))
                .collect::<Result<_, _>>()?;

            let channels_json = obj
                .get("channels")
                .and_then(|j| j.as_array())
                .ok_or("Animation missing channels")?;

            let channels: Vec<AnimationChannel> = channels_json
                .iter()
                .map(parse_animation_channel)
                .collect::<Result<_, _>>()?;

            let duration = samplers
                .iter()
                .flat_map(|s| s.input_times.iter().copied())
                .fold(0.0_f32, f32::max);

            Ok(AnimationClip {
                name,
                samplers,
                channels,
                duration,
            })
        })
        .collect()
}

fn parse_animation_sampler(
    json: &Json,
    accessors: &[Accessor],
    buffer_views: &[BufferView],
    buffer: &[u8],
) -> Result<AnimationSampler, String> {
    let obj = json.as_object().ok_or("Animation sampler must be object")?;

    let input = obj
        .get("input")
        .and_then(|j| j.as_usize())
        .ok_or("Animation sampler missing input")?;

    let output = obj
        .get("output")
        .and_then(|j| j.as_usize())
        .ok_or("Animation sampler missing output")?;

    let interpolation = obj
        .get("interpolation")
        .and_then(|j| j.as_str())
        .map(Interpolation::from_str)
        .unwrap_or(Interpolation::Linear);

    let input_times = read_scalar_accessor(input, accessors, buffer_views, buffer)?;
    let output_accessor = accessors
        .get(output)
        .ok_or("Invalid animation output accessor")?;

    let output = match output_accessor.data_type {
        DataType::Vec3 => AnimationOutput::Vec3(read_vec3_array_accessor(
            output,
            accessors,
            buffer_views,
            buffer,
        )?),
        DataType::Vec4 => AnimationOutput::Vec4(read_vec4_array_accessor(
            output,
            accessors,
            buffer_views,
            buffer,
        )?),
        _ => return Err("Unsupported animation output type".into()),
    };

    Ok(AnimationSampler {
        input_times,
        output,
        interpolation,
    })
}

fn parse_animation_channel(json: &Json) -> Result<AnimationChannel, String> {
    let obj = json.as_object().ok_or("Animation channel must be object")?;

    let sampler_index = obj
        .get("sampler")
        .and_then(|j| j.as_usize())
        .ok_or("Animation channel missing sampler")?;

    let target = obj
        .get("target")
        .and_then(|j| j.as_object())
        .ok_or("Animation channel missing target")?;

    let target_node = target
        .get("node")
        .and_then(|j| j.as_usize())
        .ok_or("Animation channel missing target node")?;

    let target_path = match target.get("path").and_then(|j| j.as_str()) {
        Some("translation") => AnimationTargetPath::Translation,
        Some("rotation") => AnimationTargetPath::Rotation,
        Some("scale") => AnimationTargetPath::Scale,
        _ => return Err("Unsupported animation target path".into()),
    };

    Ok(AnimationChannel {
        sampler_index,
        target_node,
        target_path,
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

fn read_scalar_accessor(
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
        return Err("Scalar accessor must be FLOAT".into());
    }
    if acc.data_type.components() != 1 {
        return Err("Scalar accessor must be SCALAR".into());
    }

    let data = &buffer[view.offset..view.offset + view.length];
    let floats: Vec<f32> = data
        .chunks_exact(4)
        .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect();

    if floats.len() != acc.count {
        return Err("Data size mismatch".into());
    }

    Ok(floats)
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

fn read_vec3_array_accessor(
    idx: usize,
    accessors: &[Accessor],
    buffer_views: &[BufferView],
    buffer: &[u8],
) -> Result<Vec<[f32; 3]>, String> {
    let flat = read_vec3_accessor(idx, accessors, buffer_views, buffer)?;
    Ok(flat.chunks_exact(3).map(|c| [c[0], c[1], c[2]]).collect())
}

fn read_vec4_array_accessor(
    idx: usize,
    accessors: &[Accessor],
    buffer_views: &[BufferView],
    buffer: &[u8],
) -> Result<Vec<[f32; 4]>, String> {
    let acc = accessors.get(idx).ok_or("Invalid accessor index")?;
    let view = buffer_views
        .get(acc.buffer_view)
        .ok_or("Invalid buffer view")?;

    if acc.component_type != 5126 {
        return Err("Must be FLOAT".into());
    }
    if acc.data_type.components() != 4 {
        return Err("Must be VEC4".into());
    }

    let data = &buffer[view.offset..view.offset + view.length];
    let floats: Vec<f32> = data
        .chunks_exact(4)
        .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect();

    if floats.len() != acc.count * 4 {
        return Err("Data size mismatch".into());
    }

    Ok(floats
        .chunks_exact(4)
        .map(|c| [c[0], c[1], c[2], c[3]])
        .collect())
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
