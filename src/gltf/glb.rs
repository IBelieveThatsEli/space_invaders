use std::fs;

const GLB_MAGIC: u32 = 0x46546C67;
const JSON_CHUNK: u32 = 0x4E4F534A;
const BIN_CHUNK: u32 = 0x004E4942;

pub struct Glb {
    pub json: String,
    pub bin: Vec<u8>,
}

impl Glb {
    pub fn load(path: &str) -> Result<Self, String> {
        let bytes = fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;
        Self::parse(&bytes)
    }

    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if data.len() < 12 {
            return Err("File too small".into());
        }

        let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let version = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        let length = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);

        if magic != GLB_MAGIC {
            return Err("Invalid GLB magic".into());
        }
        if version != 2 {
            return Err("Unsupported GLB version".into());
        }
        if length as usize != data.len() {
            return Err("Invalid GLB length".into());
        }

        let mut offset = 12;

        // JSON chunk
        let (json, new_offset) = Self::read_chunk(data, offset, JSON_CHUNK)?;
        offset = new_offset;

        // BIN chunk
        let (bin, _) = Self::read_chunk(data, offset, BIN_CHUNK)?;

        let json_str = String::from_utf8(json).map_err(|_| "Invalid UTF-8 in JSON")?;

        Ok(Self {
            json: json_str,
            bin,
        })
    }

    fn read_chunk(
        data: &[u8],
        offset: usize,
        expected_type: u32,
    ) -> Result<(Vec<u8>, usize), String> {
        if offset + 8 > data.len() {
            return Err("Incomplete chunk header".into());
        }

        let len = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]) as usize;
        let chunk_type = u32::from_le_bytes([
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);

        if chunk_type != expected_type {
            return Err("Unexpected chunk type".into());
        }

        let start = offset + 8;
        let end = start + len;
        if end > data.len() {
            return Err("Incomplete chunk data".into());
        }

        Ok((data[start..end].to_vec(), end))
    }
}
