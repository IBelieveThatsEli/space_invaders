use crate::{
    input::types::{action::Action, key::Key, keymods::KeyMods, mousebutton::MouseButton},
    window::x11::{
        ffi::{constants::*, functions::*, types},
        window::X11Window,
    },
};
use std::ffi;

#[derive(PartialEq)]
pub enum Event {
    Close,
    Resize(u32, u32),
    Iconified(bool),
    Focused(bool),
    Maximized(bool),
    Key(Key, i32, Action, KeyMods),
    MouseButton(MouseButton, Action),
    MouseScroll(f32, f32),
    CursorPos(i32, i32),
    CursorEnter(bool),
}

pub fn translate_events(window: &mut X11Window, event: &mut types::Event) -> Option<Event> {
    unsafe {
        match event.type_ {
            CLIENT_MESSAGE => {
                let client = event.client;

                if client.data[0] == window.wm_delete as ffi::c_long {
                    window.should_close = true;
                    return Some(Event::Close);
                }
                None
            }
            CONFIGURE_NOTIFY => {
                let config = event.config;

                let width = config.width as u32;
                let height = config.height as u32;

                window.width = width;
                window.height = height;

                return Some(Event::Resize(width, height));
            }
            UNMAP_NOTIFY => {
                return Some(Event::Iconified(true));
            }
            MAP_NOTIFY => {
                return Some(Event::Iconified(false));
            }
            FOCUS_IN => {
                return Some(Event::Focused(true));
            }
            FOCUS_OUT => {
                return Some(Event::Focused(false));
            }
            PROPERTY_NOTIFY => {
                if event.property.atom == window.wm_state {
                    let mut actual_type: types::Atom = 0;
                    let mut actual_format: ffi::c_int = 0;
                    let mut nitems: ffi::c_ulong = 0;
                    let mut bytes_after: ffi::c_ulong = 0;
                    let mut atoms: *mut types::Atom = std::ptr::null_mut();

                    get_window_property(
                        window.display,
                        window.native,
                        window.wm_state,
                        0,
                        1024,
                        0,
                        XA_ATOM,
                        &mut actual_type,
                        &mut actual_format,
                        &mut nitems,
                        &mut bytes_after,
                        &mut atoms as *mut *mut types::Atom as *mut *mut ffi::c_uchar,
                    );

                    if !atoms.is_null() {
                        let slice = std::slice::from_raw_parts(atoms, nitems as usize);

                        let mut is_maximized: ffi::c_int = 0;
                        for &atom in slice {
                            if atom == window.max_vert || atom == window.max_horz {
                                is_maximized = 1;
                            }
                        }

                        free(atoms as *mut _);
                        return Some(Event::Maximized(is_maximized == 1));
                    }
                }
                None
            }
            KEY_PRESS | KEY_RELEASE => {
                let mut keysym: types::KeySym = 0;
                lookup_string(
                    &mut event.key as *mut _,
                    std::ptr::null_mut(),
                    0,
                    &mut keysym,
                    std::ptr::null_mut(),
                );

                let key = Key::from_x11_keysym(keysym);
                let action = if event.type_ == KEY_PRESS {
                    Action::Press
                } else {
                    Action::Release
                };
                let mods = KeyMods::x11_mods_to_key_mods(event.key.state);

                return Some(Event::Key(key, event.key.keycode as i32, action, mods));
            }
            BUTTON_PRESS | BUTTON_RELEASE => {
                let button = MouseButton::x11_mousebtn_to_mousebtn(event.button.button);
                let action = if event.type_ == BUTTON_PRESS {
                    Action::Press
                } else {
                    Action::Release
                };

                if button == MouseButton::ScrollUp {
                    return Some(Event::MouseScroll(0.0, 1.0));
                }
                if button == MouseButton::ScrollDown {
                    return Some(Event::MouseScroll(0.0, -1.0));
                }

                return Some(Event::MouseButton(button, action));
            }
            MOTION_NOTIFY => {
                let mut root_ret: types::Window = 0;
                let mut child_ret: types::Window = 0;
                let mut root_x: ffi::c_int = 0;
                let mut root_y: ffi::c_int = 0;
                let mut win_x: ffi::c_int = 0;
                let mut win_y: ffi::c_int = 0;
                let mut mask_ret: ffi::c_uint = 0;

                if query_pointer(
                    window.display,
                    window.native,
                    &mut root_ret,
                    &mut child_ret,
                    &mut root_x,
                    &mut root_y,
                    &mut win_x,
                    &mut win_y,
                    &mut mask_ret,
                ) == 1
                {
                    return Some(Event::CursorPos(win_x, win_y));
                }
                None
            }
            ENTER_NOTIFY | LEAVE_NOTIFY => {
                return Some(if event.type_ == ENTER_NOTIFY {
                    Event::CursorEnter(true)
                } else {
                    Event::CursorEnter(false)
                });
            }
            _ => None,
        }
    }
}
impl Key {
    pub fn from_x11_keysym(keysym: u64) -> Self {
        match keysym {
            0xFF1B => Key::Escape,
            0xFF0D => Key::Enter,
            0xFF09 => Key::Tab,
            0xFF08 => Key::Backspace,
            0xFF63 => Key::Insert,
            0xFFFF => Key::Delete,
            0xFF51 => Key::Left,
            0xFF52 => Key::Up,
            0xFF53 => Key::Right,
            0xFF54 => Key::Down,
            0xFF55 => Key::PageUp,
            0xFF56 => Key::PageDown,
            0xFF50 => Key::Home,
            0xFF57 => Key::End,
            0xFFE1 => Key::LeftShift,
            0xFFE2 => Key::RightShift,
            0xFFE3 => Key::LeftControl,
            0xFFE4 => Key::RightControl,
            0xFFE9 => Key::LeftAlt,
            0xFFEA => Key::RightAlt,
            0xFFEB => Key::LeftSuper,
            0xFFEC => Key::RightSuper,
            0xFF67 => Key::Menu,
            0xFFE5 => Key::CapsLock,
            0xFF14 => Key::ScrollLock,
            0xFF7F => Key::NumLock,
            0xFF61 => Key::PrintScreen,
            0xFF13 => Key::Pause,
            0xFFBE => Key::F1,
            0xFFBF => Key::F2,
            0xFFC0 => Key::F3,
            0xFFC1 => Key::F4,
            0xFFC2 => Key::F5,
            0xFFC3 => Key::F6,
            0xFFC4 => Key::F7,
            0xFFC5 => Key::F8,
            0xFFC6 => Key::F9,
            0xFFC7 => Key::F10,
            0xFFC8 => Key::F11,
            0xFFC9 => Key::F12,
            0xFFCA => Key::F13,
            0xFFCB => Key::F14,
            0xFFCC => Key::F15,
            0xFFCD => Key::F16,
            0xFFCE => Key::F17,
            0xFFCF => Key::F18,
            0xFFD0 => Key::F19,
            0xFFD1 => Key::F20,
            0xFFD2 => Key::F21,
            0xFFD3 => Key::F22,
            0xFFD4 => Key::F23,
            0xFFD5 => Key::F24,
            0xFFD6 => Key::F25,
            0xFFB0 => Key::Kp0,
            0xFFB1 => Key::Kp1,
            0xFFB2 => Key::Kp2,
            0xFFB3 => Key::Kp3,
            0xFFB4 => Key::Kp4,
            0xFFB5 => Key::Kp5,
            0xFFB6 => Key::Kp6,
            0xFFB7 => Key::Kp7,
            0xFFB8 => Key::Kp8,
            0xFFB9 => Key::Kp9,
            0xFFAE => Key::KpDecimal,
            0xFFAF => Key::KpDivide,
            0xFFAA => Key::KpMultiply,
            0xFFAD => Key::KpSubtract,
            0xFFAB => Key::KpAdd,
            0xFF8D => Key::KpEnter,
            0xFFBD => Key::KpEqual,
            0x0020 => Key::Space,
            0x0027 => Key::Apostrophe,
            0x002C => Key::Comma,
            0x002D => Key::Minus,
            0x002E => Key::Period,
            0x002F => Key::Slash,
            0x0030 => Key::Key0,
            0x0031 => Key::Key1,
            0x0032 => Key::Key2,
            0x0033 => Key::Key3,
            0x0034 => Key::Key4,
            0x0035 => Key::Key5,
            0x0036 => Key::Key6,
            0x0037 => Key::Key7,
            0x0038 => Key::Key8,
            0x0039 => Key::Key9,
            0x003B => Key::Semicolon,
            0x003D => Key::Equal,
            0x005B => Key::LeftBracket,
            0x005C => Key::Backslash,
            0x005D => Key::RightBracket,
            0x0060 => Key::GraveAccent,
            0x0061 => Key::A,
            0x0062 => Key::B,
            0x0063 => Key::C,
            0x0064 => Key::D,
            0x0065 => Key::E,
            0x0066 => Key::F,
            0x0067 => Key::G,
            0x0068 => Key::H,
            0x0069 => Key::I,
            0x006A => Key::J,
            0x006B => Key::K,
            0x006C => Key::L,
            0x006D => Key::M,
            0x006E => Key::N,
            0x006F => Key::O,
            0x0070 => Key::P,
            0x0071 => Key::Q,
            0x0072 => Key::R,
            0x0073 => Key::S,
            0x0074 => Key::T,
            0x0075 => Key::U,
            0x0076 => Key::V,
            0x0077 => Key::W,
            0x0078 => Key::X,
            0x0079 => Key::Y,
            0x007A => Key::Z,
            0x0041 => Key::A,
            0x0042 => Key::B,
            0x0043 => Key::C,
            0x0044 => Key::D,
            0x0045 => Key::E,
            0x0046 => Key::F,
            0x0047 => Key::G,
            0x0048 => Key::H,
            0x0049 => Key::I,
            0x004A => Key::J,
            0x004B => Key::K,
            0x004C => Key::L,
            0x004D => Key::M,
            0x004E => Key::N,
            0x004F => Key::O,
            0x0050 => Key::P,
            0x0051 => Key::Q,
            0x0052 => Key::R,
            0x0053 => Key::S,
            0x0054 => Key::T,
            0x0055 => Key::U,
            0x0056 => Key::V,
            0x0057 => Key::W,
            0x0058 => Key::X,
            0x0059 => Key::Y,
            0x005A => Key::Z,
            0x00A1 => Key::World1,
            0x00A2 => Key::World2,
            _ => Key::Unknown,
        }
    }
}
impl KeyMods {
    pub fn x11_mods_to_key_mods(state: ffi::c_uint) -> Self {
        let mut mods = KeyMods::empty();

        if state & SHIFT_MASK != 0 {
            mods |= KeyMods::SHIFT;
        }
        if state & CONTROL_MASK != 0 {
            mods |= KeyMods::CONTROL;
        }
        if state & MOD1_MASK != 0 {
            mods |= KeyMods::ALT;
        }
        if state & MOD4_MASK != 0 {
            mods |= KeyMods::SUPER;
        }
        if state & LOCK_MASK != 0 {
            mods |= KeyMods::CAPS_LOCK;
        }
        if state & MOD2_MASK != 0 {
            mods |= KeyMods::NUM_LOCK;
        }

        mods
    }
}
impl MouseButton {
    pub fn x11_mousebtn_to_mousebtn(btn: u32) -> Self {
        match btn {
            1 => Self::Left,
            2 => Self::Middle,
            3 => Self::Right,
            4 => Self::ScrollUp,
            5 => Self::ScrollDown,
            _ => Self::Unknown,
        }
    }
}
