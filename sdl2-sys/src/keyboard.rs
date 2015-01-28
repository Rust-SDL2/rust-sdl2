use libc::{c_int, c_char, uint8_t, uint16_t,
                uint32_t};
use rect::Rect;
use video::SDL_Window;
use scancode::SDL_Scancode;
use keycode::{SDL_Keycode, SDL_Keymod};

pub type SDL_bool = c_int;
pub type SDL_Rect = Rect;

// SDL_keyboard.h
#[derive(Copy, Clone)]
pub struct SDL_Keysym {
    pub scancode: SDL_Scancode,
    pub sym: SDL_Keycode,
    pub _mod: uint16_t,
    pub unused: uint32_t,
}

extern "C" {
    pub fn SDL_GetKeyboardFocus() -> *const SDL_Window;
    pub fn SDL_GetKeyboardState(numkeys: *const c_int) -> *const uint8_t;
    pub fn SDL_GetModState() -> SDL_Keymod;
    pub fn SDL_SetModState(modstate: SDL_Keymod);
    pub fn SDL_GetKeyFromScancode(scancode: SDL_Scancode) -> SDL_Keycode;
    pub fn SDL_GetScancodeFromKey(key: SDL_Keycode) -> SDL_Scancode;
    pub fn SDL_GetScancodeName(scancode: SDL_Scancode) -> *const c_char;
    pub fn SDL_GetScancodeFromName(name: *const c_char) -> SDL_Scancode;
    pub fn SDL_GetKeyName(key: SDL_Keycode) -> *const c_char;
    pub fn SDL_GetKeyFromName(name: *const c_char) -> SDL_Keycode;
    pub fn SDL_StartTextInput();
    pub fn SDL_IsTextInputActive() -> SDL_bool;
    pub fn SDL_StopTextInput();
    pub fn SDL_SetTextInputRect(rect: *const SDL_Rect);
    pub fn SDL_HasScreenKeyboardSupport() -> SDL_bool;
    pub fn SDL_IsScreenKeyboardShown(window: *const SDL_Window) -> SDL_bool;
}
