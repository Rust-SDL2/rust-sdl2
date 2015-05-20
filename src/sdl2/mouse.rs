use std::ptr;

use get_error;
use SdlResult;
use surface;
use video;

use sys::mouse as ll;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
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

pub struct Cursor {
    raw: *mut ll::SDL_Cursor,
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
    pub fn new(data: &[u8], mask: &[u8], width: i32, height: i32, hot_x: i32, hot_y: i32) -> SdlResult<Cursor> {
        unsafe {
            let raw = ll::SDL_CreateCursor(data.as_ptr(),
                                           mask.as_ptr(),
                                           width as i32, height as i32,
                                           hot_x as i32, hot_y as i32);

            if raw == ptr::null_mut() {
                Err(get_error())
            } else {
                Ok(Cursor{ raw: raw, owned: true })
            }
        }
    }

    // TODO: figure out how to pass Surface in here correctly
    pub fn from_surface(surface: &surface::Surface, hot_x: i32, hot_y: i32) -> SdlResult<Cursor> {
        unsafe {
            let raw = ll::SDL_CreateColorCursor(surface.raw(), hot_x, hot_y);

            if raw == ptr::null_mut() {
                Err(get_error())
            } else {
                Ok(Cursor{ raw: raw, owned: true })
            }
        }
    }

    pub fn from_system(cursor: SystemCursor) -> SdlResult<Cursor> {
        unsafe {
            let raw = ll::SDL_CreateSystemCursor(cursor as u32);

            if raw == ptr::null_mut() {
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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Mouse {
    Left,
    Middle,
    Right,
    X1,
    X2,
    Unknown(u8)
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MouseState {
    flags: u32
}

impl MouseState {
    /// Tests if a mouse button was pressed.
    pub fn button(&self, button: Mouse) -> bool {
        match button {
            Mouse::Left => self.left(),
            Mouse::Middle => self.middle(),
            Mouse::Right => self.right(),
            Mouse::X1 => self.x1(),
            Mouse::X2 => self.x2(),
            Mouse::Unknown(x) => {
                assert!(x <= 32);
                let mask = 1 << ((x as u32) - 1);
                (self.flags & mask) != 0
            }
        }
    }

    /// Tests if the left mouse button was pressed.
    pub fn left(&self) -> bool { (self.flags & ll::SDL_BUTTON_LMASK) != 0 }

    /// Tests if the middle mouse button was pressed.
    pub fn middle(&self) -> bool { (self.flags & ll::SDL_BUTTON_MMASK) != 0 }

    /// Tests if the right mouse button was pressed.
    pub fn right(&self) -> bool { (self.flags & ll::SDL_BUTTON_RMASK) != 0 }

    /// Tests if the X1 mouse button was pressed.
    pub fn x1(&self) -> bool { (self.flags & ll::SDL_BUTTON_X1MASK) != 0 }

    /// Tests if the X2 mouse button was pressed.
    pub fn x2(&self) -> bool { (self.flags & ll::SDL_BUTTON_X2MASK) != 0 }

    pub fn from_flags(flags: u32) -> MouseState {
        MouseState { flags: flags }
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
    if raw == ptr::null_mut() {
        None
    } else {
        unsafe { Some(video::Window::from_ll(raw, false)) }
    }
}

pub fn get_mouse_state() -> (MouseState, i32, i32) {
    let mut x = 0;
    let mut y = 0;
    unsafe {
        let raw = ll::SDL_GetMouseState(&mut x, &mut y);
        return (MouseState::from_flags(raw), x as i32, y as i32);
    }
}

pub fn get_relative_mouse_state() -> (MouseState, i32, i32) {
    let mut x = 0;
    let mut y = 0;
    unsafe {
        let raw = ll::SDL_GetRelativeMouseState(&mut x, &mut y);
        return (MouseState::from_flags(raw), x as i32, y as i32);
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

    if raw == ptr::null_mut() {
        None
    } else {
        Some(Cursor { raw: raw, owned: false })
    }
}

pub fn get_default_cursor() -> Option<Cursor> {
    let raw = unsafe { ll::SDL_GetDefaultCursor() };

    if raw == ptr::null_mut() {
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
