use std::ffi::{CString, NulError};
use std::ptr;
use libc::c_char;

use video::WindowRef;
use get_error;

use sys::messagebox as ll;

bitflags! {
    flags MessageBoxFlag: u32 {
        const MESSAGEBOX_ERROR = ll::SDL_MESSAGEBOX_ERROR,
        const MESSAGEBOX_WARNING = ll::SDL_MESSAGEBOX_WARNING,
        const MESSAGEBOX_INFORMATION = ll::SDL_MESSAGEBOX_INFORMATION
    }
}

pub enum ShowMessageError {
    InvalidTitle(NulError),
    InvalidMessage(NulError),
    SdlError(String),
}

pub fn show_simple_message_box(flags: MessageBoxFlag, title: &str, 
        message: &str, window: Option<&WindowRef>) 
        -> Result<(), ShowMessageError> {
    use self::ShowMessageError::*;
    let result = unsafe {
        let title = match CString::new(title) {
            Ok(s) => s,
            Err(err) => return Err(InvalidTitle(err)),
        };
        let message = match CString::new(message) {
            Ok(s) => s,
            Err(err) => return Err(InvalidMessage(err)),
        };
        ll::SDL_ShowSimpleMessageBox(
            flags.bits(),
            title.as_ptr() as *const c_char,
            message.as_ptr() as *const c_char,
            window.map_or(ptr::null_mut(), |win| win.raw())
        )
    } == 0;

    if result {
        Ok(())
    } else {
        Err(SdlError(get_error()))
    }
}
