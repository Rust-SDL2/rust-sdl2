use std::c_str::CString;

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

#[allow(non_camel_case_types)]
pub mod ll {
    use libc::{c_int, c_uint, c_char, uint32_t};

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
}

bitflags! {
    flags InitFlag: u32 {
        const INIT_TIMER = ll::SDL_INIT_TIMER,
        const INIT_AUDIO = ll::SDL_INIT_AUDIO,
        const INIT_VIDEO = ll::SDL_INIT_VIDEO,
        const INIT_JOYSTICK = ll::SDL_INIT_JOYSTICK,
        const INIT_HAPTIC = ll::SDL_INIT_HAPTIC,
        const INIT_GAME_CONTROLLER = ll::SDL_INIT_GAMECONTROLLER,
        const INIT_EVENTS = ll::SDL_INIT_EVENTS,
        const INIT_NOPARACHUTE = ll::SDL_INIT_NOPARACHUTE,
        const INIT_EVERYTHING = ll::SDL_INIT_EVERYTHING
    }
}

#[deriving(PartialEq)]
pub enum Error {
    NoMemError = ll::SDL_ENOMEM as int,
    ReadError = ll::SDL_EFREAD as int,
    WriteError = ll::SDL_EFWRITE as int,
    SeekError = ll::SDL_EFSEEK as int,
    UnsupportedError = ll::SDL_UNSUPPORTED as int
}

pub type SdlResult<T> = Result<T, String>;

pub fn init(flags: InitFlag) -> bool {
    unsafe {
        ll::SDL_Init(flags.bits()) == 0
    }
}

pub fn init_subsystem(flags: InitFlag) -> bool {
    unsafe {
        ll::SDL_InitSubSystem(flags.bits()) == 0
    }
}

pub fn quit_subsystem(flags: InitFlag) {
    unsafe { ll::SDL_QuitSubSystem(flags.bits()); }
}

pub fn quit() {
    unsafe { ll::SDL_Quit(); }
}

pub fn was_inited(flags: InitFlag) -> InitFlag {
    unsafe {
        let raw = ll::SDL_WasInit(flags.bits());
        flags & InitFlag::from_bits(raw).unwrap()
    }
}

pub fn get_error() -> String {
    unsafe {
        let cstr = CString::new(ll::SDL_GetError(), false);
        cstr.as_str().unwrap().into_string()
    }
}

pub fn set_error(err: &str) {
    err.with_c_str(|buf| {
        unsafe { ll::SDL_SetError(buf); }
    })
}

pub fn set_error_from_code(err: Error) {
    unsafe { ll::SDL_Error(err as ll::SDL_errorcode); }
}

pub fn clear_error() {
    unsafe { ll::SDL_ClearError(); }
}

pub fn get_ticks() -> u32 {
    unsafe { ll::SDL_GetTicks() as u32 }
}
