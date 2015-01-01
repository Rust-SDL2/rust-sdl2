use libc::{c_int, c_char};

pub type SDL_bool = c_int;

extern "C" {
    pub fn SDL_SetClipboardText(text: *const c_char) -> c_int;
    pub fn SDL_GetClipboardText() -> *const c_char;
    pub fn SDL_HasClipboardText() -> SDL_bool;
}
