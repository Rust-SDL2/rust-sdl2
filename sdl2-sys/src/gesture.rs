use libc::{c_int, int64_t};
use rwops::SDL_RWops;
use touch::SDL_TouchID;

pub type SDL_GestureID = int64_t;

extern "C" {
    pub fn SDL_RecordGesture(touchId: SDL_TouchID) -> c_int;
    pub fn SDL_SaveAllDollarTemplates(src: *const SDL_RWops) -> c_int;
    pub fn SDL_SaveDollarTemplate(gestureId: SDL_GestureID, src: *const SDL_RWops) -> c_int;
    pub fn SDL_LoadDollarTemplates(touchId: SDL_TouchID, src: *const SDL_RWops) -> c_int;
}
