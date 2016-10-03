use libc::{c_int, c_char};
use sdl::SDL_bool;

extern "C" {
    pub fn SDL_SetClipboardText(text: *const c_char) -> c_int;
    pub fn SDL_GetClipboardText() -> *const c_char;
    pub fn SDL_HasClipboardText() -> SDL_bool;
}
