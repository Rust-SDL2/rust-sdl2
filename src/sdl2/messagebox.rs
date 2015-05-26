use std::ffi::CString;
use std::ptr;

use video::Window;
use get_error;
use SdlResult;
use util::CStringExt;

use sys::messagebox as ll;

bitflags! {
    flags MessageBoxFlag: u32 {
        const MESSAGEBOX_ERROR = ll::SDL_MESSAGEBOX_ERROR,
        const MESSAGEBOX_WARNING = ll::SDL_MESSAGEBOX_WARNING,
        const MESSAGEBOX_INFORMATION = ll::SDL_MESSAGEBOX_INFORMATION
    }
}

pub fn show_simple_message_box(flags: MessageBoxFlag, title: &str, message: &str, window: Option<&Window>) -> SdlResult<()> {
    let result = unsafe {
        let title = CString::new(title).remove_nul();
        let message = CString::new(message).remove_nul();
        ll::SDL_ShowSimpleMessageBox(flags.bits(),
                                     title.as_ptr(),
                                     message.as_ptr(),
                                     window.map_or(ptr::null_mut(), |win| win.raw()))
    } == 0;

    if result {
        Ok(())
    } else {
        Err(get_error())
    }
}
