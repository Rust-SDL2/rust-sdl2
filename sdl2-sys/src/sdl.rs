use libc::{c_int, c_uint, c_char, uint32_t};

// Setup linking for all targets.
#[cfg(target_os="macos")]
mod mac {
    #[cfg(mac_framework)]
    #[link(kind="framework", name="SDL2")]
    extern {}

    #[cfg(not(mac_framework))]
    #[link(name="SDL2")]
    extern {}
}

#[cfg(any(target_os="windows", target_os="linux", target_os="freebsd"))]
mod others {
    #[link(name="SDL2")]
    extern {}
}

pub type SDL_errorcode = c_uint;
pub const SDL_ENOMEM: SDL_errorcode = 0;
pub const SDL_EFREAD: SDL_errorcode = 1;
pub const SDL_EFWRITE: SDL_errorcode = 2;
pub const SDL_EFSEEK: SDL_errorcode = 3;
pub const SDL_UNSUPPORTED: SDL_errorcode = 4;
pub const SDL_LASTERROR: SDL_errorcode = 5;

pub type SDL_InitFlag = uint32_t;
pub const SDL_INIT_TIMER: SDL_InitFlag = 0x00000001;
pub const SDL_INIT_AUDIO: SDL_InitFlag = 0x00000010;
pub const SDL_INIT_VIDEO: SDL_InitFlag = 0x00000020;
pub const SDL_INIT_JOYSTICK: SDL_InitFlag = 0x00000200;
pub const SDL_INIT_HAPTIC: SDL_InitFlag = 0x00001000;
pub const SDL_INIT_GAMECONTROLLER: SDL_InitFlag = 0x00002000;
pub const SDL_INIT_EVENTS: SDL_InitFlag = 0x00004000;
pub const SDL_INIT_NOPARACHUTE: SDL_InitFlag = 0x00100000;
pub const SDL_INIT_EVERYTHING: SDL_InitFlag = 0x0000FFFF;

//SDL_error.h
extern "C" {
    pub fn SDL_ClearError();
    pub fn SDL_Error(code: SDL_errorcode) -> c_int;
    pub fn SDL_SetError(fmt: *const c_char) -> c_int;
    pub fn SDL_GetError() -> *const c_char;

    //SDL.h
    pub fn SDL_Init(flags: uint32_t) -> c_int;
    pub fn SDL_InitSubSystem(flags: SDL_InitFlag) -> c_int;
    pub fn SDL_QuitSubSystem(flags: SDL_InitFlag);
    pub fn SDL_WasInit(flags: SDL_InitFlag) -> SDL_InitFlag;
    pub fn SDL_Quit();

    //SDL_timer.h
    pub fn SDL_GetTicks() -> uint32_t;
}
