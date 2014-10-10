use std::ptr;

use get_error;
use SdlResult;
use surface;
use video;

#[allow(non_camel_case_types)]
pub mod ll {
    use libc::{c_int, c_uint, c_void, uint8_t, uint32_t};
    use surface::ll::SDL_Surface;
    use video::ll::SDL_Window;

    pub type SDL_bool = c_int;
    pub type SDL_Cursor = c_void;

    pub type SDL_SystemCursor = c_uint;
    pub const SDL_SYSTEM_CURSOR_ARROW: SDL_SystemCursor = 0;
    pub const SDL_SYSTEM_CURSOR_IBEAM: SDL_SystemCursor = 1;
    pub const SDL_SYSTEM_CURSOR_WAIT: SDL_SystemCursor = 2;
    pub const SDL_SYSTEM_CURSOR_CROSSHAIR: SDL_SystemCursor = 3;
    pub const SDL_SYSTEM_CURSOR_WAITARROW: SDL_SystemCursor = 4;
    pub const SDL_SYSTEM_CURSOR_SIZENWSE: SDL_SystemCursor = 5;
    pub const SDL_SYSTEM_CURSOR_SIZENESW: SDL_SystemCursor = 6;
    pub const SDL_SYSTEM_CURSOR_SIZEWE: SDL_SystemCursor = 7;
    pub const SDL_SYSTEM_CURSOR_SIZENS: SDL_SystemCursor = 8;
    pub const SDL_SYSTEM_CURSOR_SIZEALL: SDL_SystemCursor = 9;
    pub const SDL_SYSTEM_CURSOR_NO: SDL_SystemCursor = 10;
    pub const SDL_SYSTEM_CURSOR_HAND: SDL_SystemCursor = 11;
    pub const SDL_NUM_SYSTEM_CURSORS: SDL_SystemCursor = 12;

    pub type SDL_MouseState = c_int;
    pub const SDL_DISABLE: SDL_MouseState = 0;
    pub const SDL_ENABLE: SDL_MouseState = 1;
    pub const SDL_QUERY: SDL_MouseState = -1;

    extern "C" {
        pub fn SDL_GetMouseFocus() -> *const SDL_Window;
        pub fn SDL_GetMouseState(x: *const c_int, y: *const c_int) -> uint32_t;
        pub fn SDL_GetRelativeMouseState(x: *const c_int, y: *const c_int) -> uint32_t;
        pub fn SDL_WarpMouseInWindow(window: *const SDL_Window, x: c_int, y: c_int);
        pub fn SDL_SetRelativeMouseMode(enabled: SDL_bool) -> c_int;
        pub fn SDL_GetRelativeMouseMode() -> SDL_bool;
        pub fn SDL_CreateCursor(data: *const uint8_t, mask: *const uint8_t, w: c_int,
                                      h: c_int, hot_x: c_int, hot_y: c_int) ->
                  *const SDL_Cursor;
        pub fn SDL_CreateColorCursor(surface: *const SDL_Surface, hot_x: c_int,
                                           hot_y: c_int) -> *const SDL_Cursor;
        pub fn SDL_CreateSystemCursor(id: SDL_SystemCursor) -> *const SDL_Cursor;
        pub fn SDL_SetCursor(cursor: *const SDL_Cursor);
        pub fn SDL_GetCursor() -> *const SDL_Cursor;
        pub fn SDL_GetDefaultCursor() -> *const SDL_Cursor;
        pub fn SDL_FreeCursor(cursor: *const SDL_Cursor);
        pub fn SDL_ShowCursor(toggle: SDL_MouseState) -> SDL_MouseState;
    }
}

#[deriving(PartialEq)]
#[repr(u32)]
pub enum SystemCursor {
    ArrowCursor = ll::SDL_SYSTEM_CURSOR_ARROW,
    IBeamCursor = ll::SDL_SYSTEM_CURSOR_IBEAM,
    WaitCursor = ll::SDL_SYSTEM_CURSOR_WAIT,
    CrosshairCursor = ll::SDL_SYSTEM_CURSOR_CROSSHAIR,
    WaitArrowCursor = ll::SDL_SYSTEM_CURSOR_WAITARROW,
    SizeNWSECursor = ll::SDL_SYSTEM_CURSOR_SIZENWSE,
    SizeNESWCursor = ll::SDL_SYSTEM_CURSOR_SIZENESW,
    SizeWECursor = ll::SDL_SYSTEM_CURSOR_SIZEWE,
    SizeNSCursor = ll::SDL_SYSTEM_CURSOR_SIZENS,
    SizeAllCursor = ll::SDL_SYSTEM_CURSOR_SIZEALL,
    NoCursor = ll::SDL_SYSTEM_CURSOR_NO,
    HandCursor = ll::SDL_SYSTEM_CURSOR_HAND,
}

#[deriving(PartialEq)] #[allow(raw_pointer_deriving)]
pub struct Cursor {
    raw: *const ll::SDL_Cursor,
    owned: bool
}

impl Drop for Cursor {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ll::SDL_FreeCursor(self.raw);
            }
        }
    }
}

impl Cursor {
    pub fn new(data: &[u8], mask: &[u8], width: int, height: int, hot_x: int, hot_y: int) -> SdlResult<Cursor> {
        unsafe {
            let raw = ll::SDL_CreateCursor(data.as_ptr(),
                                           mask.as_ptr(),
                                           width as i32, height as i32,
                                           hot_x as i32, hot_y as i32);

            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(Cursor{ raw: raw, owned: true })
            }
        }
    }

    // TODO: figure out how to pass Surface in here correctly
    pub fn from_surface(surface: &surface::Surface, hot_x: int, hot_y: int) -> SdlResult<Cursor> {
        unsafe {
            let raw = ll::SDL_CreateColorCursor(surface.raw(), hot_x as i32,
                                                hot_y as i32);

            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(Cursor{ raw: raw, owned: true })
            }
        }
    }

    pub fn from_system(cursor: SystemCursor) -> SdlResult<Cursor> {
        unsafe {
            let raw = ll::SDL_CreateSystemCursor(cursor as u32);

            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(Cursor{ raw: raw, owned: true })
            }
        }
    }

    pub fn set(&self) {
        unsafe { ll::SDL_SetCursor(self.raw); }
    }
}

#[deriving(PartialEq)]
pub enum Mouse {
    LeftMouse,
    MiddleMouse,
    RightMouse,
    X1Mouse,
    X2Mouse,
    UnknownMouse(u8)
}

bitflags! {
    flags MouseState: u32 {
        const LEFTMOUSESTATE = 0x01,
        const MIDDLEMOUSESTATE = 0x02,
        const RIGHTMOUSESTATE = 0x04,
        const X1MOUSESTATE = 0x08,
        const X2MOUSESTATE = 0x10
    }
}

pub fn wrap_mouse(bitflags: u8) -> Mouse {
    match bitflags {
        1 => LeftMouse,
        2 => MiddleMouse,
        3 => RightMouse,
        4 => X1Mouse,
        5 => X2Mouse,
        _ => UnknownMouse(bitflags)
    }
}

pub fn get_mouse_focus() -> Option<video::Window> {
    let raw = unsafe { ll::SDL_GetMouseFocus() };
    if raw == ptr::null() {
        None
    } else {
        unsafe { Some(video::Window::from_ll(raw, false)) }
    }
}

pub fn get_mouse_state() -> (MouseState, int, int) {
    let x = 0;
    let y = 0;
    unsafe {
        let raw = ll::SDL_GetMouseState(&x, &y);
        return (MouseState::from_bits(raw).unwrap(), x as int, y as int);
    }
}

pub fn get_relative_mouse_state() -> (MouseState, int, int) {
    let x = 0;
    let y = 0;
    unsafe {
        let raw = ll::SDL_GetRelativeMouseState(&x, &y);
        return (MouseState::from_bits(raw).unwrap(), x as int, y as int);
    }
}

pub fn warp_mouse_in_window(window: &video::Window, x: i32, y: i32) {
    unsafe { ll::SDL_WarpMouseInWindow(window.raw(), x, y); }
}

pub fn set_relative_mouse_mode(on: bool) {
    unsafe { ll::SDL_SetRelativeMouseMode(on as i32); }
}

pub fn get_relative_mouse_mode() -> bool {
    unsafe { ll::SDL_GetRelativeMouseMode() == 1 }
}

pub fn get_cursor() -> Option<Cursor> {
    let raw = unsafe { ll::SDL_GetCursor() };

    if raw == ptr::null() {
        None
    } else {
        Some(Cursor { raw: raw, owned: false })
    }
}

pub fn get_default_cursor() -> Option<Cursor> {
    let raw = unsafe { ll::SDL_GetDefaultCursor() };

    if raw == ptr::null() {
        None
    } else {
        Some(Cursor { raw: raw, owned: false })
    }
}

pub fn is_cursor_showing() -> bool {
    unsafe { ll::SDL_ShowCursor(ll::SDL_QUERY) == 1 }
}

pub fn show_cursor(show: bool) {
    unsafe { ll::SDL_ShowCursor(show as i32); }
}
