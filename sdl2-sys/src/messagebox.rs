use libc::{c_int, c_char, uint32_t};
use video::SDL_Window;

pub type SDL_MessageBoxFlags = u32;
pub const SDL_MESSAGEBOX_ERROR : SDL_MessageBoxFlags = 0x00000010;
pub const SDL_MESSAGEBOX_WARNING : SDL_MessageBoxFlags = 0x00000020;
pub const SDL_MESSAGEBOX_INFORMATION : SDL_MessageBoxFlags = 0x00000040;

extern "C" {
    pub fn SDL_ShowSimpleMessageBox(flags: uint32_t, title: *const c_char, message: *const c_char, window: *const SDL_Window) -> c_int;
}
