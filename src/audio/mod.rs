pub mod constant;
pub mod functions;
pub mod types;

use constant::*;
use functions::*;
use std::{ffi::CString, fs::File, io::Read, ptr};
use types::*;

pub struct Audio {
    s: *mut Simple,
    error: i32,
    samples: Vec<i16>,
    playing: bool,
}

impl Audio {
    pub fn new(path: &str) -> Option<Self> {
        unsafe {
            let mut error: i32 = 0;

            // Read and decode MP3 file
            let mut file = File::open(path).ok()?;
            let mut data = Vec::new();
            file.read_to_end(&mut data).ok()?;

            let mut decoder = minimp3::Decoder::new(&data[..]);
            let mut samples = Vec::new();

            loop {
                match decoder.next_frame() {
                    Ok(frame) => {
                        samples.extend_from_slice(&frame.data);
                    }
                    Err(minimp3::Error::Eof) => break,
                    Err(_) => return None,
                }
            }

            let ss = SampleSpec {
                format: SAMPLE_S16LE,
                rate: 44100,
                channels: 2,
            };

            let name = CString::new("Space Invaders").unwrap();
            let stream = CString::new("Music").unwrap();

            let s = pa_simple_new(
                ptr::null(),
                name.as_ptr(),
                STREAM_PLAYBACK,
                ptr::null(),
                stream.as_ptr(),
                &ss,
                ptr::null(),
                ptr::null(),
                &mut error,
            );

            if s.is_null() {
                let err_str = pa_strerror(error);
                println!("pa_simple_new() failed: {:?}", err_str);
                return None;
            }

            Some(Self {
                s,
                error,
                samples,
                playing: false,
            })
        }
    }

    pub fn play(&mut self) {
        if self.playing {
            return;
        }

        self.playing = true;

        unsafe {
            pa_simple_write(
                self.s,
                self.samples.as_ptr() as *const _,
                self.samples.len() * std::mem::size_of::<i16>(),
                &mut self.error,
            );

            if pa_simple_drain(self.s, &mut self.error) < 0 {
                let err_str = pa_strerror(self.error);
                println!("pa_simple_drain() failed: {:?}", err_str);
            }
        }
    }
}

impl Drop for Audio {
    fn drop(&mut self) {
        unsafe {
            if !self.s.is_null() {
                pa_simple_free(self.s);
            }
        }
    }
}
