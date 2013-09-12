
pub mod ll {
    use std::libc::{c_int, c_uint, c_void, uint8_t, uint32_t};
    use surface::ll::SDL_Surface;
    use video::ll::SDL_Window;

    pub type SDL_bool = c_int;
    pub type SDL_Cursor = c_void;

    pub type SDL_SystemCursor = c_uint;
    pub static SDL_SYSTEM_CURSOR_ARROW: SDL_SystemCursor = 0;
    pub static SDL_SYSTEM_CURSOR_IBEAM: SDL_SystemCursor = 1;
    pub static SDL_SYSTEM_CURSOR_WAIT: SDL_SystemCursor = 2;
    pub static SDL_SYSTEM_CURSOR_CROSSHAIR: SDL_SystemCursor = 3;
    pub static SDL_SYSTEM_CURSOR_WAITARROW: SDL_SystemCursor = 4;
    pub static SDL_SYSTEM_CURSOR_SIZENWSE: SDL_SystemCursor = 5;
    pub static SDL_SYSTEM_CURSOR_SIZENESW: SDL_SystemCursor = 6;
    pub static SDL_SYSTEM_CURSOR_SIZEWE: SDL_SystemCursor = 7;
    pub static SDL_SYSTEM_CURSOR_SIZENS: SDL_SystemCursor = 8;
    pub static SDL_SYSTEM_CURSOR_SIZEALL: SDL_SystemCursor = 9;
    pub static SDL_SYSTEM_CURSOR_NO: SDL_SystemCursor = 10;
    pub static SDL_SYSTEM_CURSOR_HAND: SDL_SystemCursor = 11;
    pub static SDL_NUM_SYSTEM_CURSORS: SDL_SystemCursor = 12;

    pub type SDL_MouseState = c_int;
    pub static SDL_DISABLE: SDL_MouseState = 0;
    pub static SDL_ENABLE: SDL_MouseState = 1;
    pub static SDL_QUERY: SDL_MouseState = -1;

    externfn!(fn SDL_GetMouseFocus() -> *SDL_Window)
    externfn!(fn SDL_GetMouseState(x: *c_int, y: *c_int) -> uint32_t)
    externfn!(fn SDL_GetRelativeMouseState(x: *c_int, y: *c_int) -> uint32_t)
    externfn!(fn SDL_WarpMouseInWindow(window: *SDL_Window, x: c_int, y: c_int))
    externfn!(fn SDL_SetRelativeMouseMode(enabled: SDL_bool) -> c_int)
    externfn!(fn SDL_GetRelativeMouseMode() -> SDL_bool)
    externfn!(fn SDL_CreateCursor(data: *uint8_t, mask: *uint8_t, w: c_int,
                                  h: c_int, hot_x: c_int, hot_y: c_int) ->
              *SDL_Cursor)
    externfn!(fn SDL_CreateColorCursor(surface: *SDL_Surface, hot_x: c_int,
                                       hot_y: c_int) -> *SDL_Cursor)
    externfn!(fn SDL_CreateSystemCursor(id: SDL_SystemCursor) -> *SDL_Cursor)
    externfn!(fn SDL_SetCursor(cursor: *SDL_Cursor))
    externfn!(fn SDL_GetCursor() -> *SDL_Cursor)
    externfn!(fn SDL_GetDefaultCursor() -> *SDL_Cursor)
    externfn!(fn SDL_FreeCursor(cursor: *SDL_Cursor))
    externfn!(fn SDL_ShowCursor(toggle: SDL_MouseState) -> SDL_MouseState)
}

#[deriving(Eq)]
pub enum Mouse {
    LeftMouse,
    MiddleMouse,
    RightMouse,
    X1Mouse,
    X2Mouse,
}

#[deriving(Eq)]
pub enum MouseState {
    LeftMouseState = 1,
    MiddleMouseState,
    RightMouseState,
    X1MouseState,
    X2MouseState,
}
