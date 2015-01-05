use std::ptr;
use std::c_str::ToCStr;

use video::Window;
use get_error;
use SdlResult;

pub use sys::messagebox as ll;

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
