use std::ffi::{CString, CStr};
use libc::c_void;
use libc::c_char;
use crate::{Error, get_error_as_error};

use crate::sys;

/// Clipboard utility functions. Access with `VideoSubsystem::clipboard()`.
///
/// These functions require the video subsystem to be initialized.
///
/// ```no_run
/// let sdl_context = sdl2::init().unwrap();
/// let video_subsystem = sdl_context.video().unwrap();
///
/// video_subsystem.clipboard().set_clipboard_text("Hello World!").unwrap();
/// ```
pub struct ClipboardUtil {
    _subsystem: crate::VideoSubsystem
}

impl crate::VideoSubsystem {
    #[inline]
    pub fn clipboard(&self) -> ClipboardUtil {
        ClipboardUtil {
            _subsystem: self.clone()
        }
    }
}

impl ClipboardUtil {
    pub fn set_clipboard_text(&self, text: &str) -> Result<(), Error> {
        unsafe {
            let text = CString::new(text).unwrap();
            let result = sys::SDL_SetClipboardText(text.as_ptr() as *const c_char);

            if result != 0 {
                Err(get_error_as_error())
            } else {
                Ok(())
            }
        }
    }

    pub fn clipboard_text(&self) -> Result<String, Error> {
        unsafe {
            let buf = sys::SDL_GetClipboardText();

            if buf.is_null() {
                Err(get_error_as_error())
            } else {
                let s = CStr::from_ptr(buf as *const _).to_str().unwrap().to_owned();
                sys::SDL_free(buf as *mut c_void);
                Ok(s)
            }
        }
    }

    pub fn has_clipboard_text(&self) -> bool {
        unsafe { sys::SDL_HasClipboardText() == sys::SDL_bool::SDL_TRUE }
    }
}
