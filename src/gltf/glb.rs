use std::fs::File;
use std::io::Read;

pub struct Glb {
    pub json: String,
    pub bin: Vec<u8>,
}

pub fn load_glb(path: &str) -> Result<Glb, String> {
    let mut file = File::open(path).map_err(|e| e.to_string())?;

    let mut bytes = Vec::new();

    file.read_to_end(&mut bytes).map_err(|e| e.to_string())?;

    if bytes.len() < 12 {
        return Err("Invalid GLB".into());
    }

    let magic = u32::from_le_bytes(bytes[0..4].try_into().unwrap());

    let version = u32::from_le_bytes(bytes[4..8].try_into().unwrap());

    let length = u32::from_le_bytes(bytes[8..12].try_into().unwrap());

    if magic != 0x46546C67 {
        return Err("Invalid GLB magic".into());
    }

    if version != 2 {
        return Err("Unsupported GLB version".into());
    }

    if length as usize != bytes.len() {
        return Err("Invalid GLB length".into());
    }

    let mut offset = 12;

    // JSON chunk
    let json_length = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;

    offset += 4;

    let json_type = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());

    offset += 4;

    if json_type != 0x4E4F534A {
        return Err("Missing JSON chunk".into());
    }

    let json = String::from_utf8(bytes[offset..offset + json_length].to_vec())
        .map_err(|e| e.to_string())?;

    offset += json_length;

    // BIN chunk
    let bin_length = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;

    offset += 4;

    let bin_type = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());

    offset += 4;

    if bin_type != 0x004E4942 {
        return Err("Missing BIN chunk".into());
    }

    let bin = bytes[offset..offset + bin_length].to_vec();

    Ok(Glb { json, bin })
}
