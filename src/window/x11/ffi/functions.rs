use std::ffi;

use super::types::*;

unsafe extern "C" {
    // #[link_name = "glXCreateContext"]
    // pub fn gl_create_context(
    //     display: *mut Display,
    //     visual_info: *mut VisualInfo,
    //     share_list: GLXContext,
    //     direct: ffi::c_int,
    // ) -> GLXContext;

    #[link_name = "glXMakeCurrent"]
    pub fn gl_make_current(
        display: *mut Display,
        drawable: ffi::c_ulong,
        context: GLXContext,
    ) -> ffi::c_int;

    #[link_name = "glXDestroyContext"]
    pub fn gl_destroy_context(display: *mut Display, context: GLXContext);

    #[link_name = "glXCreateContextAttribsARB"]
    pub fn gl_create_context_attribs_arb(
        display: *mut Display,
        config: GLXFBConfig,
        share_context: GLXContext,
        direct: ffi::c_int,
        attrib_list: *const ffi::c_int,
    ) -> GLXContext;

    #[link_name = "glXChooseFBConfig"]
    pub fn gl_choose_fb_config(
        display: *mut Display,
        screen: ffi::c_int,
        attrib_list: *const ffi::c_int,
        nelements: *mut ffi::c_int,
    ) -> *mut GLXFBConfig;

    #[link_name = "glXGetProcAddress"]
    pub fn gl_get_proc_address(proc_name: *const ffi::c_char) -> *mut ffi::c_void;

    // #[link_name = "glXGetProcAddressARB"]
    // pub fn gl_get_proc_address_arb(proc_name: *const ffi::c_char) -> *mut ffi::c_void;

    #[link_name = "glXSwapBuffers"]
    pub fn gl_swap_buffers(display: *mut Display, drawable: ffi::c_ulong);

    #[link_name = "XOpenDisplay"]
    pub fn open_display(display_name: *const ffi::c_char) -> *mut Display;

    #[link_name = "XCloseDisplay"]
    pub fn close_display(display: *mut Display) -> ffi::c_int;

    #[link_name = "XDefaultScreen"]
    pub fn default_screen(display: *mut Display) -> ffi::c_int;

    #[link_name = "XCreateSimpleWindow"]
    pub fn create_simple_window(
        display: *mut Display,
        parent: Window,
        x: ffi::c_int,
        y: ffi::c_int,
        width: ffi::c_uint,
        height: ffi::c_uint,
        border_width: ffi::c_uint,
        border: ffi::c_ulong,
        background: ffi::c_ulong,
    ) -> Window;

    #[link_name = "XRootWindow"]
    pub fn root_window(display: *mut Display, screen: ffi::c_int) -> Window;

    #[link_name = "XStoreName"]
    pub fn store_name(
        display: *mut Display,
        window: Window,
        window_name: *const ffi::c_char,
    ) -> ffi::c_int;

    #[link_name = "XMapWindow"]
    pub fn map_window(display: *mut Display, window: Window) -> ffi::c_int;

    #[link_name = "XSync"]
    pub fn sync(display: *mut Display, discard: ffi::c_int) -> ffi::c_int;

    #[link_name = "XSelectInput"]
    pub fn select_input(
        display: *mut Display,
        window: Window,
        event_mask: ffi::c_long,
    ) -> ffi::c_int;

    #[link_name = "XPending"]
    pub fn pending(display: *mut Display) -> ffi::c_int;

    #[link_name = "XNextEvent"]
    pub fn next_event(display: *mut Display, event_out: *mut Event) -> ffi::c_int;

    #[link_name = "XInternAtom"]
    pub fn intern_atom(
        display: *mut Display,
        atom_name: *const ffi::c_char,
        only_if_exists: ffi::c_int,
    ) -> Atom;

    #[link_name = "XSetWMProtocols"]
    pub fn set_wm_protocols(
        display: *mut Display,
        window: Window,
        protocols: *mut ffi::c_ulong,
        count: ffi::c_int,
    ) -> ffi::c_int;

    #[link_name = "XGetWindowProperty"]
    pub fn get_window_property(
        display: *mut Display,
        w: Window,
        property: Atom,
        long_offset: ffi::c_long,
        long_length: ffi::c_long,
        delete: ffi::c_int,
        req_type: Atom,
        actual_type_return: *mut Atom,
        actual_format_return: *mut ffi::c_int,
        nitems_return: *mut ffi::c_ulong,
        bytes_after_return: *mut ffi::c_ulong,
        prop_return: *mut *mut ffi::c_uchar,
    ) -> ffi::c_int;

    #[link_name = "XBlackPixel"]
    pub fn black_pixel(display: *mut Display, screen_number: ffi::c_int) -> ffi::c_ulong;

    #[link_name = "XWhitePixel"]
    pub fn white_pixel(display: *mut Display, screen_number: ffi::c_int) -> ffi::c_ulong;

    #[link_name = "XFree"]
    pub fn free(data: *mut ffi::c_void) -> ffi::c_int;

    #[link_name = "XLookupString"]
    pub fn lookup_string(
        event_struct: *mut KeyEvent,
        buffer_return: *mut ffi::c_char,
        bytes_buffer: ffi::c_int,
        keysym_return: *mut KeySym,
        status_in_out: *mut ffi::c_void,
    ) -> ffi::c_int;

    #[link_name = "XQueryPointer"]
    pub fn query_pointer(
        display: *mut Display,
        w: Window,
        root_return: *mut Window,
        child_return: *mut Window,
        root_x_return: *mut ffi::c_int,
        root_y_return: *mut ffi::c_int,
        win_x_return: *mut ffi::c_int,
        win_y_return: *mut ffi::c_int,
        mask_return: *mut ffi::c_uint,
    ) -> ffi::c_int;
}
