use libc::{c_int};

pub type SDL_bool = c_int;

// SDL_cpuinfo.h
extern "C" {
    pub fn SDL_GetCPUCount() -> c_int;
    pub fn SDL_GetCPUCacheLineSize() -> c_int;
    pub fn SDL_HasRDTSC() -> SDL_bool;
    pub fn SDL_HasAltiVec() -> SDL_bool;
    pub fn SDL_HasMMX() -> SDL_bool;
    pub fn SDL_Has3DNow() -> SDL_bool;
    pub fn SDL_HasSSE() -> SDL_bool;
    pub fn SDL_HasSSE2() -> SDL_bool;
    pub fn SDL_HasSSE3() -> SDL_bool;
    pub fn SDL_HasSSE41() -> SDL_bool;
    pub fn SDL_HasSSE42() -> SDL_bool;
    pub fn SDL_HasAVX() -> SDL_bool;
    pub fn SDL_GetSystemRAM() -> c_int;
}
