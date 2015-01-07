use std::ptr;

use get_error;
use SdlResult;
use surface;
use video;

pub use sys::mouse as ll;

#[derive(Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum SystemCursor {
    Arrow = ll::SDL_SYSTEM_CURSOR_ARROW,
    IBeam = ll::SDL_SYSTEM_CURSOR_IBEAM,
    Wait = ll::SDL_SYSTEM_CURSOR_WAIT,
    Crosshair = ll::SDL_SYSTEM_CURSOR_CROSSHAIR,
    WaitArrow = ll::SDL_SYSTEM_CURSOR_WAITARROW,
    SizeNWSE = ll::SDL_SYSTEM_CURSOR_SIZENWSE,
    SizeNESW = ll::SDL_SYSTEM_CURSOR_SIZENESW,
    SizeWE = ll::SDL_SYSTEM_CURSOR_SIZEWE,
    SizeNS = ll::SDL_SYSTEM_CURSOR_SIZENS,
    SizeAll = ll::SDL_SYSTEM_CURSOR_SIZEALL,
    No = ll::SDL_SYSTEM_CURSOR_NO,
    Hand = ll::SDL_SYSTEM_CURSOR_HAND,
}

#[derive(PartialEq)] #[allow(raw_pointer_derive)]
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

#[derive(Copy, Clone, PartialEq)]
pub enum Mouse {
    Left,
    Middle,
    Right,
    X1,
    X2,
    Unknown(u8)
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
        1 => Mouse::Left,
        2 => Mouse::Middle,
        3 => Mouse::Right,
        4 => Mouse::X1,
        5 => Mouse::X2,
        _ => Mouse::Unknown(bitflags)
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
