use video::Window;
use get_error;
use SdlResult;

#[allow(non_camel_case_types)]
pub mod ll {
    use libc::{c_int, c_char, uint32_t};
    use video::ll::SDL_Window;

    pub enum SDL_MessageBoxFlags {
        SDL_MESSAGEBOX_ERROR        = 0x00000010,
        SDL_MESSAGEBOX_WARNING      = 0x00000020,
        SDL_MESSAGEBOX_INFORMATION  = 0x00000040
    }

    extern "C" {
        pub fn SDL_ShowSimpleMessageBox(flags: uint32_t, title: *const c_char, message: *const c_char, window: *const SDL_Window) -> c_int;
    }
}

bitflags! {
    flags MessageBoxFlag: u32 {
        const MESSAGEBOX_ERROR = ll::SDL_MESSAGEBOX_ERROR as u32,
        const MESSAGEBOX_WARNING = ll::SDL_MESSAGEBOX_WARNING as u32,
        const MESSAGEBOX_INFORMATION = ll::SDL_MESSAGEBOX_INFORMATION as u32
    }
}

pub fn show_simple_message_box(flags: MessageBoxFlag, title: &str, message: &str, window: Window) -> SdlResult<()> {
    let result = unsafe {
        title.with_c_str(|title_cstr| {
        message.with_c_str(|message_cstr| {
            ll::SDL_ShowSimpleMessageBox(flags.bits(), title_cstr, message_cstr, window.raw())
        })
        })
    } == 0;

    if result {
        Ok(())
    } else {
        Err(get_error())
    }
}
