use std::ffi;

pub const COMMON_MASKS: ffi::c_long = KEY_PRESS_MASK
    | KEY_RELEASE_MASK
    | BUTTON_PRESS_MASK
    | BUTTON_RELEASE_MASK
    | ENTER_WINDOW_MASK
    | LEAVE_WINDOW_MASK
    | POINTER_MOTION_MASK
    | EXPOSURE_MASK
    | STRUCTURE_NOTIFY_MASK
    | FOCUS_CHANGE_MASK
    | PROPERTY_CHANGE_MASK;

pub const KEY_PRESS_MASK: ffi::c_long = 1 << 0;
pub const KEY_RELEASE_MASK: ffi::c_long = 1 << 1;
pub const BUTTON_PRESS_MASK: ffi::c_long = 1 << 2;
pub const BUTTON_RELEASE_MASK: ffi::c_long = 1 << 3;
pub const ENTER_WINDOW_MASK: ffi::c_long = 1 << 4;
pub const LEAVE_WINDOW_MASK: ffi::c_long = 1 << 5;
pub const POINTER_MOTION_MASK: ffi::c_long = 1 << 6;
pub const EXPOSURE_MASK: ffi::c_long = 1 << 15;
pub const STRUCTURE_NOTIFY_MASK: ffi::c_long = 1 << 17;
pub const FOCUS_CHANGE_MASK: ffi::c_long = 1 << 21;
pub const PROPERTY_CHANGE_MASK: ffi::c_long = 1 << 22;

pub const KEY_PRESS: ffi::c_int = 2;
pub const KEY_RELEASE: ffi::c_int = 3;
pub const BUTTON_PRESS: ffi::c_int = 4;
pub const BUTTON_RELEASE: ffi::c_int = 5;
pub const MOTION_NOTIFY: ffi::c_int = 6;
pub const ENTER_NOTIFY: ffi::c_int = 7;
pub const LEAVE_NOTIFY: ffi::c_int = 8;
pub const FOCUS_IN: ffi::c_int = 9;
pub const FOCUS_OUT: ffi::c_int = 10;
pub const UNMAP_NOTIFY: ffi::c_int = 18;
pub const MAP_NOTIFY: ffi::c_int = 19;
pub const CONFIGURE_NOTIFY: ffi::c_int = 22;
pub const PROPERTY_NOTIFY: ffi::c_int = 28;
pub const CLIENT_MESSAGE: ffi::c_int = 33;

pub const XA_ATOM: ffi::c_ulong = 4;

pub const SHIFT_MASK: ffi::c_uint = 1 << 0;
pub const LOCK_MASK: ffi::c_uint = 1 << 1; // Caps Lock                                             
pub const CONTROL_MASK: ffi::c_uint = 1 << 2;
pub const MOD1_MASK: ffi::c_uint = 1 << 3; // Alt                                                   
pub const MOD2_MASK: ffi::c_uint = 1 << 4; // Num Lock                                              
pub const MOD4_MASK: ffi::c_uint = 1 << 6; // Super/Windows key 

// pub const GLX_RGBA: ffi::c_int = 4;
pub const GLX_DOUBLEBUFFER: ffi::c_int = 5;
pub const GLX_RED_SIZE: ffi::c_int = 8;
pub const GLX_GREEN_SIZE: ffi::c_int = 9;
pub const GLX_BLUE_SIZE: ffi::c_int = 10;
pub const GLX_DEPTH_SIZE: ffi::c_int = 12;
pub const GLX_CONTEXT_MAJOR_VERSION_ARB: ffi::c_int = 0x2091;
pub const GLX_CONTEXT_MINOR_VERSION_ARB: ffi::c_int = 0x2092;
pub const GLX_CONTEXT_PROFILE_MASK_ARB: ffi::c_int = 0x9126;
pub const GLX_CONTEXT_CORE_PROFILE_BIT_ARB: ffi::c_int = 0x00000001;
pub const GLX_X_RENDERABLE: ffi::c_int = 0x8012;
pub const GLX_DRAWABLE_TYPE: ffi::c_int = 0x8010;
pub const GLX_RENDER_TYPE: ffi::c_int = 0x8011;
pub const GLX_X_VISUAL_TYPE: ffi::c_int = 0x22;
pub const GLX_TRUE_COLOR: ffi::c_int = 0x8002;
pub const GLX_WINDOW_BIT: ffi::c_int = 0x00000001;
pub const GLX_RGBA_BIT: ffi::c_int = 0x00000001;
pub const GLX_ALPHA_SIZE: ffi::c_int = 11;
pub const GLX_STENCIL_SIZE: ffi::c_int = 13;
// pub const GLX_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: ffi::c_int = 0x00000002;
