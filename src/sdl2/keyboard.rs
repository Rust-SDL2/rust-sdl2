use std::collections::HashMap;
use std::num::FromPrimitive;
use std::ptr;
use std::string;
use std::vec;

use keycode::{KeyCode, UnknownKey};
use rect::Rect;
use scancode::{ScanCode, UnknownScanCode};
use video::Window;

#[allow(non_camel_case_types)]
pub mod ll {
    use libc::{c_int, c_char, c_uint, int32_t, uint8_t, uint16_t,
                    uint32_t};
    use rect::Rect;
    use video::ll::SDL_Window;

    pub type SDL_bool = c_int;
    pub type SDL_Rect = Rect;
    pub type SDL_Keycode = int32_t;
    pub type SDL_Keymod = c_uint;
    pub type SDL_Scancode = c_uint;

    // SDL_keyboard.h
    pub struct SDL_Keysym {
        pub scancode: SDL_Scancode,
        pub sym: SDL_Keycode,
        pub _mod: uint16_t,
        pub unused: uint32_t,
    }

    extern "C" {
        pub fn SDL_GetKeyboardFocus() -> *const SDL_Window;
        pub fn SDL_GetKeyboardState(numkeys: *const c_int) -> *const uint8_t;
        pub fn SDL_GetModState() -> SDL_Keymod;
        pub fn SDL_SetModState(modstate: SDL_Keymod);
        pub fn SDL_GetKeyFromScancode(scancode: SDL_Scancode) -> SDL_Keycode;
        pub fn SDL_GetScancodeFromKey(key: SDL_Keycode) -> SDL_Scancode;
        pub fn SDL_GetScancodeName(scancode: SDL_Scancode) -> *const c_char;
        pub fn SDL_GetScancodeFromName(name: *const c_char) -> SDL_Scancode;
        pub fn SDL_GetKeyName(key: SDL_Keycode) -> *const c_char;
        pub fn SDL_GetKeyFromName(name: *const c_char) -> SDL_Keycode;
        pub fn SDL_StartTextInput();
        pub fn SDL_IsTextInputActive() -> SDL_bool;
        pub fn SDL_StopTextInput();
        pub fn SDL_SetTextInputRect(rect: *const SDL_Rect);
        pub fn SDL_HasScreenKeyboardSupport() -> SDL_bool;
        pub fn SDL_IsScreenKeyboardShown(window: *const SDL_Window) -> SDL_bool;
    }
}

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

    let raw = unsafe { vec::raw::from_buf(ll::SDL_GetKeyboardState(&count),
                                          count as uint) };

    let mut current = 0;
    while current < raw.len() {
        state.insert(FromPrimitive::from_int(current as int)
                        .unwrap_or(UnknownScanCode),
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
            .unwrap_or(UnknownKey)
    }
}

pub fn get_scancode_from_key(key: KeyCode) -> ScanCode {
    unsafe {
        FromPrimitive::from_int(ll::SDL_GetScancodeFromKey(key as i32) as int)
            .unwrap_or(UnknownScanCode)
    }
}

pub fn get_scancode_name(scancode: ScanCode) -> String {
    unsafe {
        let scancode_name = ll::SDL_GetScancodeName(scancode as u32);
        string::raw::from_buf(scancode_name as *const u8)
    }
}

pub fn get_scancode_from_name(name: &str) -> ScanCode {
    unsafe {
        name.with_c_str(|name| {
            FromPrimitive::from_int(ll::SDL_GetScancodeFromName(name) as int)
                .unwrap_or(UnknownScanCode)
        })
    }
}

pub fn get_key_name(key: KeyCode) -> String {
    unsafe {
        let key_name = ll::SDL_GetKeyName(key as i32);
        string::raw::from_buf(key_name as *const u8)
    }
}

pub fn get_key_from_name(name: &str) -> KeyCode {
    unsafe {
        name.with_c_str(|name| {
            FromPrimitive::from_int(ll::SDL_GetKeyFromName(name) as int)
                .unwrap_or(UnknownKey)
        })
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
