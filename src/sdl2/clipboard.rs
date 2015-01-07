use std::ffi::{c_str_to_bytes, CString};
use SdlResult;
use get_error;

pub use sys::clipboard as ll;

pub fn set_clipboard_text(text: &String) -> SdlResult<()> {
    unsafe {
        let buff = CString::from_slice(text.as_slice().as_bytes());
        let result = ll::SDL_SetClipboardText(buff.as_ptr());

        if result == 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }
}

pub fn get_clipboard_text() -> SdlResult<String> {
    let result = unsafe {
        let buf = ll::SDL_GetClipboardText();
        String::from_utf8_lossy(c_str_to_bytes(&buf)).to_string()
    };

    if result.len() == 0 {
        Err(get_error())
    } else {
        Ok(result)
    }
}

pub fn has_clipboard_text() -> bool {
    unsafe { ll::SDL_HasClipboardText() == 1 }
}

