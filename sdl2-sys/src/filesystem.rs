use libc::{c_char};

extern "C" {
    pub fn SDL_GetBasePath() -> *const c_char;
    pub fn SDL_GetPrefPath(arg: *const c_char, app: *const c_char) -> *const c_char;
}
