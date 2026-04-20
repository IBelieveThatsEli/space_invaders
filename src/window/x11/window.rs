use std::{ffi, ptr};
use thiserror::Error;

use super::events::*;
use super::ffi::{constants::*, functions::*, types};

#[derive(Clone, Default)]
#[allow(dead_code)]
pub struct X11Window {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub display: *mut types::Display,
    pub native: types::Window,
    pub screen: ffi::c_int,
    pub wm_delete: types::Atom,
    pub wm_state: types::Atom,
    pub max_vert: types::Atom,
    pub max_horz: types::Atom,
    pub should_close: bool,
    pub gl_context: types::GLXContext,
}

#[derive(Error, Debug)]
pub enum X11Error {
    #[error("The system cannot find a graphical server to draw the window")]
    DisplayFailed,
    #[error("Failed to get GLX framebuffer config")]
    FramebufferFailed,
    #[error("Failed to create opengl context")]
    GLContextFailed,
}

#[allow(dead_code)]
impl X11Window {
    pub fn new(width: u32, height: u32, title: &str) -> Result<Self, X11Error> {
        unsafe {
            let display = Self::init_display()?;
            let screen = default_screen(display);
            let window = Self::create_window(display, screen, width, height, title);
            let (wm_delete, wm_state, max_vert, max_horz) = Self::setup_atoms(display, window);
            let gl_context = Self::create_gl_context(display, screen, window)?;

            Ok(Self {
                width,
                height,
                title: title.into(),
                display,
                native: window,
                screen,
                wm_delete,
                wm_state,
                max_vert,
                max_horz,
                should_close: false,
                gl_context,
            })
        }
    }
    pub fn poll_events(&mut self) -> Option<Event> {
        unsafe {
            while pending(self.display) > 0 {
                let mut event: types::Event = std::mem::zeroed();
                next_event(self.display, &mut event);

                if let Some(e) = translate_events(self, &mut event) {
                    return Some(e);
                }
            }
            None
        }
    }
    pub fn swap_buffers(&mut self) {
        unsafe {
            gl_swap_buffers(self.display, self.native);
        }
    }
    pub fn get_proc_address(&self, symbol: &str) -> *const ffi::c_void {
        let c_str = ffi::CString::new(symbol).unwrap();
        unsafe { gl_get_proc_address(c_str.as_ptr()) as *const ffi::c_void }
    }

    fn init_display() -> Result<*mut types::Display, X11Error> {
        unsafe {
            let display = open_display(ptr::null());

            if display.is_null() {
                return Err(X11Error::DisplayFailed);
            }

            Ok(display)
        }
    }
    fn create_window(
        display: *mut types::Display,
        screen: i32,
        width: u32,
        height: u32,
        title: &str,
    ) -> types::Window {
        unsafe {
            let window = create_simple_window(
                display,
                root_window(display, screen),
                0,
                0,
                width,
                height,
                1,
                black_pixel(display, screen),
                white_pixel(display, screen),
            );

            select_input(display, window, COMMON_MASKS);
            map_window(display, window);

            sync(display, 1);

            let c_string = ffi::CString::new(title).expect("CString::new Failed");
            let ptr = c_string.as_ptr();
            store_name(display, window, ptr);

            window
        }
    }
    fn setup_atoms(
        display: *mut types::Display,
        window: types::Window,
    ) -> (types::Atom, types::Atom, types::Atom, types::Atom) {
        let wm_delete = Self::atom(display, "WM_DELETE_WINDOW");
        unsafe { set_wm_protocols(display, window, &mut (wm_delete as _), 1) };

        let wm_state = Self::atom(display, "_NET_WM_STATE");
        let max_vert = Self::atom(display, "_NET_WM_STATE_MAXIMIZED_VERT");
        let max_horz = Self::atom(display, "_NET_WM_STATE_MAXIMIZED_HORZ");

        (wm_delete, wm_state, max_vert, max_horz)
    }
    fn atom(display: *mut types::Display, name: &str) -> types::Atom {
        unsafe {
            let c = ffi::CString::new(name).unwrap();
            intern_atom(display, c.as_ptr(), 1)
        }
    }
    fn create_gl_context(
        display: *mut types::Display,
        screen: i32,
        window: types::Window,
    ) -> Result<types::GLXContext, X11Error> {
        unsafe {
            let major = 3;
            let minor = 3;

            let fb_attribs = [
                GLX_X_RENDERABLE,
                1,
                GLX_DRAWABLE_TYPE,
                GLX_WINDOW_BIT,
                GLX_RENDER_TYPE,
                GLX_RGBA_BIT,
                GLX_X_VISUAL_TYPE,
                GLX_TRUE_COLOR,
                GLX_RED_SIZE,
                8,
                GLX_GREEN_SIZE,
                8,
                GLX_BLUE_SIZE,
                8,
                GLX_ALPHA_SIZE,
                8,
                GLX_DEPTH_SIZE,
                24,
                GLX_STENCIL_SIZE,
                8,
                GLX_DOUBLEBUFFER,
                1,
                0, // NULL terminator
            ];

            let mut fb_count: ffi::c_int = 0;
            let fb_configs =
                gl_choose_fb_config(display, screen, fb_attribs.as_ptr(), &mut fb_count);

            if fb_configs.is_null() || fb_count == 0 {
                return Err(X11Error::FramebufferFailed);
            }

            let fb_config = *fb_configs;

            let mut context_attribs = vec![
                GLX_CONTEXT_MAJOR_VERSION_ARB,
                major,
                GLX_CONTEXT_MINOR_VERSION_ARB,
                minor,
            ];

            context_attribs.push(GLX_CONTEXT_PROFILE_MASK_ARB);
            context_attribs.push(GLX_CONTEXT_CORE_PROFILE_BIT_ARB);

            context_attribs.push(0);

            let context = gl_create_context_attribs_arb(
                display,
                fb_config,
                std::ptr::null_mut(),
                1,
                context_attribs.as_ptr(),
            );

            if context.is_null() {
                return Err(X11Error::GLContextFailed);
            }

            gl_make_current(display, window, context);

            free(fb_configs as *mut _);

            Ok(context)
        }
    }
}
impl Drop for X11Window {
    fn drop(&mut self) {
        unsafe {
            gl_make_current(self.display, 0, std::ptr::null_mut());
            gl_destroy_context(self.display, self.gl_context);

            close_display(self.display);
        }
    }
}
