use std::ptr;

use video::Window;
use get_error;
use SdlResult;

#[allow(non_camel_case_types)]
pub mod ll {
    use libc::{c_int, c_char, uint32_t};
    use video::ll::SDL_Window;

    pub type SDL_MessageBoxFlags = u32;
    pub const SDL_MESSAGEBOX_ERROR : SDL_MessageBoxFlags = 0x00000010;
    pub const SDL_MESSAGEBOX_WARNING : SDL_MessageBoxFlags = 0x00000020;
    pub const SDL_MESSAGEBOX_INFORMATION : SDL_MessageBoxFlags = 0x00000040;

    extern "C" {
        pub fn SDL_ShowSimpleMessageBox(flags: uint32_t, title: *const c_char, message: *const c_char, window: *const SDL_Window) -> c_int;
    }
}

bitflags! {
    flags MessageBoxFlag: u32 {
        const MESSAGEBOX_ERROR = ll::SDL_MESSAGEBOX_ERROR,
        const MESSAGEBOX_WARNING = ll::SDL_MESSAGEBOX_WARNING,
        const MESSAGEBOX_INFORMATION = ll::SDL_MESSAGEBOX_INFORMATION
    }
}

pub fn show_simple_message_box(flags: MessageBoxFlag, title: &str, message: &str, window: Option<&Window>) -> SdlResult<()> {
    let result = unsafe {
        title.with_c_str(|title_cstr| {
            message.with_c_str(|message_cstr| {
                ll::SDL_ShowSimpleMessageBox(flags.bits(), title_cstr, message_cstr, window.map_or(ptr::null(), |win| win.raw()))
            })
        })
    } == 0;

    if result {
        Ok(())
    } else {
        Err(get_error())
    }
}
