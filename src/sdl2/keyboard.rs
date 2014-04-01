use collections::hashmap::HashMap;
use std::num::FromPrimitive;
use std::ptr;
use std::str;
use std::slice;
use std::vec::Vec;

use keycode::KeyCode;
use rect::Rect;
use scancode::ScanCode;
use video::Window;

#[allow(non_camel_case_types)]
pub mod ll {
    use std::libc::{c_int, c_schar, c_uint, int32_t, uint8_t, uint16_t,
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
        pub fn SDL_GetKeyboardFocus() -> *SDL_Window;
        pub fn SDL_GetKeyboardState(numkeys: *c_int) -> *uint8_t;
        pub fn SDL_GetModState() -> SDL_Keymod;
        pub fn SDL_SetModState(modstate: SDL_Keymod);
        pub fn SDL_GetKeyFromScancode(scancode: SDL_Scancode) -> SDL_Keycode;
        pub fn SDL_GetScancodeFromKey(key: SDL_Keycode) -> SDL_Scancode;
        pub fn SDL_GetScancodeName(scancode: SDL_Scancode) -> *c_schar;
        pub fn SDL_GetScancodeFromName(name: *c_schar) -> SDL_Scancode;
        pub fn SDL_GetKeyName(key: SDL_Keycode) -> *c_schar;
        pub fn SDL_GetKeyFromName(name: *c_schar) -> SDL_Keycode;
        pub fn SDL_StartTextInput();
        pub fn SDL_IsTextInputActive() -> SDL_bool;
        pub fn SDL_StopTextInput();
        pub fn SDL_SetTextInputRect(rect: *SDL_Rect);
        pub fn SDL_HasScreenKeyboardSupport() -> SDL_bool;
        pub fn SDL_IsScreenKeyboardShown(window: *SDL_Window) -> SDL_bool;
    }
}

#[deriving(Eq)]
pub enum Mod {
     NoMod = 0x0000,
     LShiftMod = 0x0001,
     RShiftMod = 0x0002,
     LCtrlMod = 0x0040,
     RCtrlMod = 0x0080,
     LAltMod = 0x0100,
     RAltMod = 0x0200,
     LGuiMod = 0x0400,
     RGuiMod = 0x0800,
     NumMod = 0x1000,
     CapsMod = 0x2000,
     ModeMod = 0x4000,
     ReservedMod = 0x8000
}

pub fn wrap_mod_state(bitflags: ll::SDL_Keymod) -> Vec<Mod> {
    let flags = [NoMod,
        LShiftMod,
        RShiftMod,
        LCtrlMod,
        RCtrlMod,
        LAltMod,
        RAltMod,
        LGuiMod,
        RGuiMod,
        NumMod,
        CapsMod,
        ModeMod,
        ReservedMod];

    flags.iter().filter_map(|&flag| {
        if bitflags & (flag as ll::SDL_Keymod) != 0 { Some(flag) }
        else { None }
    }).collect()
}

pub fn get_keyboard_focus() -> Option<~Window> {
    let raw = unsafe { ll::SDL_GetKeyboardFocus() };
    if raw == ptr::null() {
        None
    } else {
        Some(~Window{ raw: raw, owned: false })
    }
}

pub fn get_keyboard_state() -> ~HashMap<ScanCode, bool> {
    let mut state: ~HashMap<ScanCode, bool> = ~HashMap::new();
    let count = 0;

    let raw = unsafe { slice::raw::from_buf_raw(ll::SDL_GetKeyboardState(&count),
                                              count as uint) };

    let mut current = 0;
    while current < raw.len() {
        state.insert(FromPrimitive::from_int(current as int).unwrap(),
                     raw[current] == 1);
        current += 1;
    }

    return state;
}

pub fn get_mod_state() -> Vec<Mod> {
    unsafe { wrap_mod_state(ll::SDL_GetModState()) }
}

pub fn set_mod_state(flags: &[Mod]) {
    let mut state = 0;
    for flag in flags.iter() {
        state |= *flag as ll::SDL_Keymod;
    }

    unsafe { ll::SDL_SetModState(state); }
}

pub fn get_key_from_scancode(scancode: ScanCode) -> KeyCode {
    unsafe {
        FromPrimitive::from_int(ll::SDL_GetKeyFromScancode(scancode.code()
                                                            as u32) as int).unwrap()
    }
}

pub fn get_scancode_from_key(key: KeyCode) -> ScanCode {
    unsafe {
        FromPrimitive::from_int(ll::SDL_GetScancodeFromKey(key.code())
                                 as int).unwrap()
    }
}

pub fn get_scancode_name(scancode: ScanCode) -> ~str {
    unsafe {
        str::raw::from_c_str(ll::SDL_GetScancodeName(scancode.code() as u32))
    }
}

pub fn get_scancode_from_name(name: &str) -> ScanCode {
    unsafe {
        name.with_c_str(|name| {
            FromPrimitive::from_int(ll::SDL_GetScancodeFromName(name) as int).unwrap()
        })
    }
}

pub fn get_key_name(key: KeyCode) -> ~str {
    unsafe {
        str::raw::from_c_str(ll::SDL_GetKeyName(key.code()))
    }
}

pub fn get_key_from_name(name: &str) -> KeyCode {
    unsafe {
        name.with_c_str(|name| {
            FromPrimitive::from_int(ll::SDL_GetKeyFromName(name) as int).unwrap()
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
    unsafe { ll::SDL_IsScreenKeyboardShown(window.raw) == 1 }
}
