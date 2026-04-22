pub mod constant;
pub mod functions;
pub mod types;

use constant::*;
use functions::*;
use std::{ffi::CString, ptr};
use types::*;

pub struct Audio {
    s: *mut Simple,
    error: i32,
}

impl Audio {
    pub fn new(path: &str) -> Option<Self> {
        unsafe {
            let mut error: i32 = 0;

            let ss = SampleSpec {
                format: SAMPLE_S16LE,
                rate: 44100,
                channels: 2,
            };

            let name = CString::new(path).unwrap();
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

            Some(Self { s, error })
        }
    }
    pub fn play(&mut self) {
        unsafe {
            let buffer: [i16; 1024] = [0; 1024];

            pa_simple_write(
                self.s,
                buffer.as_ptr() as *const _,
                buffer.len() * std::mem::size_of::<i16>(),
                &mut self.error,
            );

            if pa_simple_drain(self.s, &mut self.error) < 0 {
                let err_str = pa_strerror(self.error);
                println!("pa_simple_drain() failed: {:?}", err_str);
            }

            pa_simple_free(self.s);
        }
    }
}
