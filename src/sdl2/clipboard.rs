use std::str;
use SdlResult;
use get_error;

#[allow(non_camel_case_types)]
pub mod ll {
    use libc::{c_int, c_char};

    pub type SDL_bool = c_int;

    extern "C" {
        pub fn SDL_SetClipboardText(text: *c_char) -> c_int;
        pub fn SDL_GetClipboardText() -> *c_char;
        pub fn SDL_HasClipboardText() -> SDL_bool;
    }
}

pub fn set_clipboard_text(text: &String) -> SdlResult<()> {
    unsafe {
        let result = text.with_c_str(|buff| {
            ll::SDL_SetClipboardText(buff)
        });

        if result == 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }
}

pub fn get_clipboard_text() -> SdlResult<String> {
    let result = unsafe {
        let cstr = ll::SDL_GetClipboardText();
        str::raw::from_c_str(cstr)
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

