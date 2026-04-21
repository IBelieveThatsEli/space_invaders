use std::ffi;

pub type Window = ffi::c_ulong;
pub type Atom = ffi::c_ulong;
pub type Time = ffi::c_ulong;
pub type KeySym = ffi::c_ulong;
pub type GLXContext = *mut ffi::c_void;
pub type GLXFBConfig = *mut ffi::c_void;

#[repr(C)]
pub union Event {
    pub type_: ffi::c_int,
    pub client: ClientMessageEvent,
    pub config: ConfigureEvent,
    pub property: PropertyEvent,
    pub key: KeyEvent,
    pub button: ButtonEvent,
    pub pad: [u64; 24],
}

#[repr(C)]
#[derive(Default)]
pub struct Display {
    private_: [u8; 0],
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ClientMessageEvent {
    pub type_: ffi::c_int,
    pub serial: ffi::c_ulong,
    pub send_event: ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: ffi::c_int,
    pub data: [ffi::c_long; 5],
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ConfigureEvent {
    pub type_: ffi::c_int,
    pub serial: ffi::c_ulong,
    pub send_event: ffi::c_int,
    pub display: *mut Display,
    pub event: ffi::c_ulong,
    pub window: ffi::c_ulong,
    pub x: ffi::c_int,
    pub y: ffi::c_int,
    pub width: ffi::c_int,
    pub height: ffi::c_int,
    pub border_width: ffi::c_int,
    pub above: ffi::c_ulong,
    pub override_redirect: ffi::c_int,
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct PropertyEvent {
    pub type_: ffi::c_int,
    pub serial: ffi::c_ulong,
    pub send_event: ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: ffi::c_int,
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct KeyEvent {
    pub type_: ffi::c_int,
    pub serial: ffi::c_ulong,
    pub send_event: ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: ffi::c_int,
    pub y: ffi::c_int,
    pub x_root: ffi::c_int,
    pub y_root: ffi::c_int,
    pub state: ffi::c_uint,
    pub keycode: ffi::c_uint,
    pub same_screen: ffi::c_int,
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ButtonEvent {
    pub type_: ffi::c_int,
    pub serial: ffi::c_ulong,
    pub send_event: ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: ffi::c_int,
    pub y: ffi::c_int,
    pub x_root: ffi::c_int,
    pub y_root: ffi::c_int,
    pub state: ffi::c_uint,
    pub button: ffi::c_uint,
    pub same_screen: ffi::c_int,
}

#[repr(C)]
pub struct VisualInfo {
    pub visual: *mut ffi::c_void,
    pub visualid: ffi::c_ulong,
    pub screen: ffi::c_int,
    pub depth: ffi::c_int,
    pub class: ffi::c_int,
    pub red_mask: ffi::c_ulong,
    pub green_mask: ffi::c_ulong,
    pub blue_mask: ffi::c_ulong,
    pub colormap_size: ffi::c_int,
    pub bits_per_rgb: ffi::c_int,
}
