use super::json::*;
use crate::json::*;

pub fn parse_mesh(json: &Json) -> Result<Vec<Mesh>, String> {
    let meshes = match obj(json).and_then(|o| get(o, "meshes")) {
        Some(Json::Array(a)) => a,
        _ => return Ok(vec![]),
    };

    let mut out = Vec::new();

    for mesh in meshes {
        let mesh_obj = match obj(mesh) {
            Some(o) => o,
            None => continue,
        };

        let primitives = match get(mesh_obj, "primitives") {
            Some(Json::Array(a)) => a,
            _ => continue,
        };

        let mut parsed_prims = Vec::new();

        for prim in primitives {
            let prim_obj = match obj(prim) {
                Some(o) => o,
                None => continue,
            };

            let attributes = match get(prim_obj, "attributes") {
                Some(Json::Object(o)) => o,
                _ => continue,
            };

            let position = match get(attributes, "POSITION") {
                Some(Json::Number(n)) => *n as usize,
                _ => return Err("Missing POSITION attribute".into()),
            };

            let indices = match get(prim_obj, "indices") {
                Some(Json::Number(n)) => Some(*n as usize),
                _ => None,
            };

            parsed_prims.push(Primitive {
                position_accessor: position,
                indices_accessor: indices,
            });
        }

        out.push(Mesh {
            primitives: parsed_prims,
        });
    }

    Ok(out)
}
