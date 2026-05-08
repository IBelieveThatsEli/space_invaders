use crate::json::Json;

#[derive(Debug)]
pub struct Gltf {
    pub meshes: Vec<Mesh>,
    pub nodes: Vec<Node>,
    pub buffers: Vec<Buffer>,
    pub buffer_views: Vec<BufferView>,
    pub accessors: Vec<Accessor>,
}

#[derive(Debug)]
pub struct Mesh {
    pub primitives: Vec<Primitive>,
}

#[derive(Debug)]
pub struct Primitive {
    pub position_accessor: usize,
    pub indices_accessor: Option<usize>,
}

#[derive(Debug)]
pub struct Node {
    pub mesh: Option<usize>,
    pub children: Vec<usize>,
}

#[derive(Debug)]
pub struct Buffer {
    pub byte_length: usize,
    pub uri: Option<String>,
}

#[derive(Debug)]
pub struct BufferView {
    pub buffer: usize,
    pub byte_offset: usize,
    pub byte_length: usize,
}

#[derive(Debug)]
pub struct Accessor {
    pub buffer_view: usize,
    pub count: usize,
    pub component_type: u32,
    pub accessor_type: String,
}

impl Gltf {
    pub fn from_json(json: Json) -> Result<Self, String> {
        let obj = match json {
            Json::Object(o) => o,
            _ => return Err("GLTF root must be object".into()),
        };

        Ok(Self {
            meshes: parse_meshes(obj.get("meshes"))?,
            nodes: parse_nodes(obj.get("nodes"))?,
            buffers: parse_buffers(obj.get("buffers"))?,
            buffer_views: parse_buffer_views(obj.get("bufferViews"))?,
            accessors: parse_accessors(obj.get("accessors"))?,
        })
    }
}

fn parse_meshes(v: Option<&Json>) -> Result<Vec<Mesh>, String> {
    let mut out = Vec::new();

    let arr = match v {
        Some(Json::Array(a)) => a,
        _ => return Ok(out),
    };

    for mesh in arr {
        let obj = match mesh {
            Json::Object(o) => o,
            _ => continue,
        };

        let primitives = match obj.get("primitives") {
            Some(Json::Array(p)) => p,
            _ => continue,
        };

        let mut parsed = Vec::new();

        for prim in primitives {
            let p = match prim {
                Json::Object(o) => o,
                _ => continue,
            };

            let attrs = match p.get("attributes") {
                Some(Json::Object(a)) => a,
                _ => continue,
            };

            let position = match attrs.get("POSITION") {
                Some(Json::Number(n)) => *n as usize,
                _ => continue,
            };

            let indices = match p.get("indices") {
                Some(Json::Number(n)) => Some(*n as usize),
                _ => None,
            };

            parsed.push(Primitive {
                position_accessor: position,
                indices_accessor: indices,
            });
        }

        out.push(Mesh { primitives: parsed });
    }

    Ok(out)
}

fn parse_nodes(v: Option<&Json>) -> Result<Vec<Node>, String> {
    let mut out = Vec::new();

    let arr = match v {
        Some(Json::Array(a)) => a,
        _ => return Ok(out),
    };

    for node in arr {
        let obj = match node {
            Json::Object(o) => o,
            _ => continue,
        };

        let mesh = match obj.get("mesh") {
            Some(Json::Number(n)) => Some(*n as usize),
            _ => None,
        };

        let children = match obj.get("children") {
            Some(Json::Array(c)) => c
                .iter()
                .filter_map(|x| match x {
                    Json::Number(n) => Some(*n as usize),
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        };

        out.push(Node { mesh, children });
    }

    Ok(out)
}

fn parse_buffers(v: Option<&Json>) -> Result<Vec<Buffer>, String> {
    let mut out = Vec::new();

    let arr = match v {
        Some(Json::Array(a)) => a,
        _ => return Ok(out),
    };

    for buf in arr {
        let obj = match buf {
            Json::Object(o) => o,
            _ => continue,
        };

        let byte_length = match obj.get("byteLength") {
            Some(Json::Number(n)) => *n as usize,
            _ => 0,
        };

        let uri = match obj.get("uri") {
            Some(Json::String(s)) => Some(s.clone()),
            _ => None,
        };

        out.push(Buffer { byte_length, uri });
    }

    Ok(out)
}

fn parse_buffer_views(v: Option<&Json>) -> Result<Vec<BufferView>, String> {
    let mut out = Vec::new();

    let arr = match v {
        Some(Json::Array(a)) => a,
        _ => return Ok(out),
    };

    for bv in arr {
        let obj = match bv {
            Json::Object(o) => o,
            _ => continue,
        };

        let buffer = match obj.get("buffer") {
            Some(Json::Number(n)) => *n as usize,
            _ => 0,
        };

        let byte_offset = match obj.get("byteOffset") {
            Some(Json::Number(n)) => *n as usize,
            _ => 0,
        };

        let byte_length = match obj.get("byteLength") {
            Some(Json::Number(n)) => *n as usize,
            _ => 0,
        };

        out.push(BufferView {
            buffer,
            byte_offset,
            byte_length,
        });
    }

    Ok(out)
}

fn parse_accessors(v: Option<&Json>) -> Result<Vec<Accessor>, String> {
    let mut out = Vec::new();

    let arr = match v {
        Some(Json::Array(a)) => a,
        _ => return Ok(out),
    };

    for acc in arr {
        let obj = match acc {
            Json::Object(o) => o,
            _ => continue,
        };

        let buffer_view = match obj.get("bufferView") {
            Some(Json::Number(n)) => *n as usize,
            _ => 0,
        };

        let count = match obj.get("count") {
            Some(Json::Number(n)) => *n as usize,
            _ => 0,
        };

        let component_type = match obj.get("componentType") {
            Some(Json::Number(n)) => *n as u32,
            _ => 0,
        };

        let accessor_type = match obj.get("type") {
            Some(Json::String(s)) => s.clone(),
            _ => String::new(),
        };

        out.push(Accessor {
            buffer_view,
            count,
            component_type,
            accessor_type,
        });
    }

    Ok(out)
}
