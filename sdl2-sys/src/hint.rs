use libc::c_char;
use sdl::SDL_bool;

#[derive(Copy, Clone)]
#[repr(C)]
pub enum SDL_HintPriority {
    SDL_HINT_DEFAULT = 0,
    SDL_HINT_NORMAL = 1,
    SDL_HINT_OVERRIDE = 2
}

extern "C" {
    pub fn SDL_SetHint(name: *const c_char, value: *const c_char) -> SDL_bool;
    pub fn SDL_GetHint(name: *const c_char) -> *const c_char;
    pub fn SDL_SetHintWithPriority(name: *const c_char, value: *const c_char, priority: SDL_HintPriority) -> SDL_bool;
}
