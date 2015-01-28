use libc::{c_int, c_uint, c_void, uint8_t, uint32_t};
use surface::SDL_Surface;
use video::SDL_Window;

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

pub const SDL_BUTTON_LEFT: u8 = 1;
pub const SDL_BUTTON_MIDDLE: u8 = 2;
pub const SDL_BUTTON_RIGHT: u8 = 3;
pub const SDL_BUTTON_X1: u8 = 4;
pub const SDL_BUTTON_X2: u8 = 5;
pub const SDL_BUTTON_LMASK: u32  = 0x01;
pub const SDL_BUTTON_MMASK: u32  = 0x02;
pub const SDL_BUTTON_RMASK: u32  = 0x04;
pub const SDL_BUTTON_X1MASK: u32 = 0x08;
pub const SDL_BUTTON_X2MASK: u32 = 0x10;


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
