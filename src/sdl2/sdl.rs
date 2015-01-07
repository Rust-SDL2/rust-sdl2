use std::ffi::{c_str_to_bytes, CString};

use sys::sdl as ll;

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

#[derive(Copy, Clone, PartialEq)]
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
        let err = ll::SDL_GetError();
        String::from_utf8_lossy(c_str_to_bytes(&err)).to_string()
    }
}

pub fn set_error(err: &str) {
    let buf = CString::from_slice(err.as_bytes()).as_ptr();
    unsafe { ll::SDL_SetError(buf); }
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
