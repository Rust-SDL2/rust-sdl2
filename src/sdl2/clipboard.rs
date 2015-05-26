use std::ffi::{CString, CStr};
use SdlResult;
use get_error;

use sys::clipboard as ll;

pub fn set_clipboard_text(text: &String) -> SdlResult<()> {
    unsafe {
        let text = CString::new(text.clone()).unwrap();
        let result = ll::SDL_SetClipboardText(text.as_ptr());

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
        String::from_utf8_lossy(CStr::from_ptr(buf).to_bytes()).into_owned()
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
