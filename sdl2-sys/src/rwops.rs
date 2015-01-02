use libc::{c_uchar, uint32_t, c_char, FILE, c_void};
use libc::{c_int, int64_t, size_t};

#[allow(dead_code)]
#[repr(C)]
struct SDL_RWops_Anon {
    data: [c_uchar; 24],
}

pub type SDL_bool = c_int;

pub static RW_SEEK_SET: c_int = 0;
pub static RW_SEEK_CUR: c_int = 1;
pub static RW_SEEK_END: c_int = 2;

#[allow(dead_code)]
#[repr(C)]
pub struct SDL_RWops {
    pub size:  extern "C" fn(context: *const SDL_RWops) -> int64_t,
    pub seek:  extern "C" fn(context: *const SDL_RWops, offset: int64_t, whence: c_int) -> int64_t,
    pub read:  extern "C" fn(context: *const SDL_RWops, ptr: *const c_void,
                             size: size_t, maxnum: size_t) -> size_t,
    pub write: extern "C" fn(context: *const SDL_RWops, ptr: *const c_void,
                             size: size_t, maxnum: size_t) -> size_t,
    pub close: extern "C" fn(context: *const SDL_RWops) -> c_int,
    pub _type: uint32_t,
    hidden: SDL_RWops_Anon
}

extern "C" {
    pub fn SDL_RWFromFile(file: *const c_char, mode: *const c_char) -> *const SDL_RWops;
    pub fn SDL_RWFromFP(fp: *const FILE, autoclose: SDL_bool) -> *const SDL_RWops;
    pub fn SDL_RWFromMem(mem: *const c_void, size: c_int) -> *const SDL_RWops;
    pub fn SDL_RWFromConstMem(mem: *const c_void, size: c_int) -> *const SDL_RWops;

    pub fn SDL_AllocRW() -> *const SDL_RWops;
    pub fn SDL_FreeRW(area: *const SDL_RWops);
}
