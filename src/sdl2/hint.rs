use std::ffi::{CString, CStr};
use sys::hint as ll;
use std::ptr;

pub enum Hint {
    Default,
    Normal,
    Override
}

pub fn set(name: &str, value: &str) -> bool{
    let name = CString::new(name).unwrap();
    let value = CString::new(value).unwrap();
    unsafe {
        ll::SDL_SetHint(name.as_ptr(), value.as_ptr()) == 1
    }
}

pub fn get(name: &str) -> Option<String> {
    use std::str;

    let name = CString::new(name).unwrap();

    unsafe {
        let res = ll::SDL_GetHint(name.as_ptr());

        if res == ptr::null_mut() {
            None
        } else {
            Some(str::from_utf8(CStr::from_ptr(res).to_bytes()).unwrap().to_owned())
        }
    }
}

pub fn set_with_priority(name: &str, value: &str, priority: Hint) -> bool {
    let name = CString::new(name).unwrap();
    let value = CString::new(value).unwrap();

    let priority_val = match priority {
        Hint::Normal => ll::SDL_HintPriority::SDL_HINT_NORMAL,
        Hint::Override => ll::SDL_HintPriority::SDL_HINT_OVERRIDE,
        Hint::Default => ll::SDL_HintPriority::SDL_HINT_DEFAULT
    };

    unsafe {
        ll::SDL_SetHintWithPriority(name.as_ptr(), value.as_ptr(), priority_val) == 1
    }
}
