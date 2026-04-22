pub type SampleFormat = i32;
pub type StreamDirectionT = u32;

pub enum Simple {}

#[repr(C)]
pub struct SampleSpec {
    pub format: SampleFormat,
    pub rate: u32,
    pub channels: u8,
}
