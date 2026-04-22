use std::ffi;

use super::types::*;

#[link(name = "pulse-simple")]
#[link(name = "pulse")]
unsafe extern "C" {
    pub fn pa_simple_new(
        server: *const ffi::c_char,
        name: *const ffi::c_char,
        dir: StreamDirectionT,
        dev: *const ffi::c_char,
        stream_name: *const ffi::c_char,
        ss: *const SampleSpec,
        map: *const ffi::c_void,
        attr: *const ffi::c_void,
        error: *mut ffi::c_int,
    ) -> *mut Simple;

    pub fn pa_simple_write(
        s: *mut Simple,
        data: *const ffi::c_void,
        bytes: usize,
        error: *mut ffi::c_int,
    ) -> ffi::c_int;

    pub fn pa_simple_drain(s: *mut Simple, error: *mut ffi::c_int) -> ffi::c_int;

    pub fn pa_simple_free(s: *mut Simple);

    pub fn pa_strerror(error: ffi::c_int) -> *const ffi::c_char;
}
