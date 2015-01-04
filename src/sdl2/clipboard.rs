use SdlResult;
use get_error;
use std::c_str::ToCStr;

pub use sys::clipboard as ll;

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
        let cstr = ll::SDL_GetClipboardText() as *const u8;
        String::from_raw_buf(cstr)
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

