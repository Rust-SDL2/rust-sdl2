use libc::{c_int};
use video::{Window};
use get_error;
use SdlResult;

#[allow(non_camel_case_types)]
pub mod ll {
    use libc::{c_int, c_char};
    use video::ll::{SDL_Window};

    pub type SDL_MessageBoxFlags = c_int;
    pub static SDL_MESSAGEBOX_ERROR: SDL_MessageBoxFlags = 0x00000010;
    pub static SDL_MESSAGEBOX_WARNING: SDL_MessageBoxFlags = 0x00000020;
    pub static SDL_MESSAGEBOX_INFORMATION: SDL_MessageBoxFlags = 0x00000040;

    extern "C" {
        pub fn SDL_ShowSimpleMessageBox(flags: SDL_MessageBoxFlags, title: *c_char, message: *c_char, window: *SDL_Window) -> c_int;
    }
}

pub enum MessageBoxFlags {
    MessageboxError       = ll::SDL_MESSAGEBOX_ERROR as int,
    MessageboxWarning     = ll::SDL_MESSAGEBOX_WARNING as int,
    MessageboxInformation = ll::SDL_MESSAGEBOX_INFORMATION as int,
}

pub fn show_simple_message_box(flags: MessageBoxFlags, title: String, message: String, window: Window) -> SdlResult<()> {
    let ret = title.with_c_str(|bufftitle| {
        message.with_c_str(|buffmessage| {
            unsafe {
                ll::SDL_ShowSimpleMessageBox(flags as c_int, bufftitle, buffmessage, window.peek_ll())
            }
        })
    });
    if ret == -1 {
        Err(get_error())
    } else {
        Ok(())
    }
}

