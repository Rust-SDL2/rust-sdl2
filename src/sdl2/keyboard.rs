use std::collections::HashMap;
use std::ffi::{c_str_to_bytes, CString};
use std::num::FromPrimitive;
use std::ptr;

use keycode::KeyCode;
use rect::Rect;
use scancode::ScanCode;
use video::Window;

pub use sys::keyboard as ll;

bitflags! {
    flags Mod: u32 {
        const NOMOD = 0x0000,
        const LSHIFTMOD = 0x0001,
        const RSHIFTMOD = 0x0002,
        const LCTRLMOD = 0x0040,
        const RCTRLMOD = 0x0080,
        const LALTMOD = 0x0100,
        const RALTMOD = 0x0200,
        const LGUIMOD = 0x0400,
        const RGUIMOD = 0x0800,
        const NUMMOD = 0x1000,
        const CAPSMOD = 0x2000,
        const MODEMOD = 0x4000,
        const RESERVEDMOD = 0x8000
    }
}

pub fn get_keyboard_focus() -> Option<Window> {
    let raw = unsafe { ll::SDL_GetKeyboardFocus() };
    if raw == ptr::null() {
        None
    } else {
        unsafe { Some(Window::from_ll(raw, false)) }
    }
}

pub fn get_keyboard_state() -> HashMap<ScanCode, bool> {
    let mut state: HashMap<ScanCode, bool> = HashMap::new();
    let count = 0;

    let raw = unsafe { Vec::from_raw_buf(ll::SDL_GetKeyboardState(&count),
                                          count as uint) };

    let mut current = 0;
    while current < raw.len() {
        state.insert(FromPrimitive::from_int(current as int)
                        .unwrap_or(ScanCode::Unknown),
                     raw[current] == 1);
        current += 1;
    }

    return state;
}

pub fn get_mod_state() -> Mod {
    unsafe { Mod::from_bits(ll::SDL_GetModState()).unwrap() }
}

pub fn set_mod_state(flags: Mod) {
    unsafe { ll::SDL_SetModState(flags.bits()); }
}

pub fn get_key_from_scancode(scancode: ScanCode) -> KeyCode {
    unsafe {
        FromPrimitive::from_int(ll::SDL_GetKeyFromScancode(scancode as u32) as int)
            .unwrap_or(KeyCode::Unknown)
    }
}

pub fn get_scancode_from_key(key: KeyCode) -> ScanCode {
    unsafe {
        FromPrimitive::from_int(ll::SDL_GetScancodeFromKey(key as i32) as int)
            .unwrap_or(ScanCode::Unknown)
    }
}

pub fn get_scancode_name(scancode: ScanCode) -> String {
    unsafe {
        let scancode_name = ll::SDL_GetScancodeName(scancode as u32);
        String::from_utf8_lossy(c_str_to_bytes(&scancode_name)).to_string()
    }
}

pub fn get_scancode_from_name(name: &str) -> ScanCode {
    unsafe {
        let name = CString::from_slice(name.as_bytes()).as_ptr();
        FromPrimitive::from_int(ll::SDL_GetScancodeFromName(name) as int)
            .unwrap_or(ScanCode::Unknown)
    }
}

pub fn get_key_name(key: KeyCode) -> String {
    unsafe {
        let key_name = ll::SDL_GetKeyName(key as i32);
        String::from_utf8_lossy(c_str_to_bytes(&key_name)).to_string()
    }
}

pub fn get_key_from_name(name: &str) -> KeyCode {
    unsafe {
        let name = CString::from_slice(name.as_bytes()).as_ptr();
        FromPrimitive::from_int(ll::SDL_GetKeyFromName(name) as int)
            .unwrap_or(KeyCode::Unknown)
    }
}

pub fn start_text_input() {
    unsafe { ll::SDL_StartTextInput(); }
}

pub fn is_text_input_active() -> bool {
    unsafe { ll::SDL_IsTextInputActive() == 1 }
}

pub fn stop_text_input() {
    unsafe { ll::SDL_StopTextInput(); }
}

pub fn set_text_input_rect(rect: &Rect) {
    unsafe { ll::SDL_SetTextInputRect(rect); }
}

pub fn has_screen_keyboard_support() -> bool {
    unsafe { ll::SDL_HasScreenKeyboardSupport() == 1 }
}

pub fn is_screen_keyboard_shown(window: &Window) -> bool {
    unsafe { ll::SDL_IsScreenKeyboardShown(window.raw()) == 1 }
}
