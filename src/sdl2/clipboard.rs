use std::ffi::{CString, CStr};
use libc::c_char;
use SdlResult;
use get_error;

use sys::clipboard as ll;

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
    _subsystem: ::VideoSubsystem
}

impl ::VideoSubsystem {
    #[inline]
    pub fn clipboard(&self) -> ClipboardUtil {
        ClipboardUtil {
            _subsystem: self.clone()
        }
    }
}

impl ClipboardUtil {
    pub fn set_clipboard_text(&self, text: &str) -> SdlResult<()> {
        unsafe {
            let text = CString::new(text).unwrap();
            let result = ll::SDL_SetClipboardText(text.as_ptr() as *const c_char);

            if result == 0 {
                Err(get_error())
            } else {
                Ok(())
            }
        }
    }

    pub fn clipboard_text(&self) -> SdlResult<String> {
        unsafe {
            let buf = ll::SDL_GetClipboardText();

            if buf.is_null() {
                Err(get_error())
            } else {
                Ok(String::from_utf8_lossy(CStr::from_ptr(buf as *const i8).to_bytes()).into_owned())
            }
        }
    }

    pub fn has_clipboard_text(&self) -> bool {
        unsafe { ll::SDL_HasClipboardText() == 1 }
    }
}
