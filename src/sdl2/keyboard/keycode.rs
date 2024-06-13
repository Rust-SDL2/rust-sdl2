#![allow(unreachable_patterns)]

use libc::c_char;
use std::ffi::{CStr, CString};
use std::mem::transmute;

use crate::sys;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Keycode(i32);

#[allow(non_upper_case_globals)]
impl Keycode {
    pub const BACKSPACE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_BACKSPACE as i32);
    pub const TAB: Keycode = Keycode(sys::SDL_KeyCode::SDLK_TAB as i32);
    pub const RETURN: Keycode = Keycode(sys::SDL_KeyCode::SDLK_RETURN as i32);
    pub const ESCAPE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_ESCAPE as i32);
    pub const SPACE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_SPACE as i32);
    pub const EXCLAIM: Keycode = Keycode(sys::SDL_KeyCode::SDLK_EXCLAIM as i32);
    pub const QUOTEDBL: Keycode = Keycode(sys::SDL_KeyCode::SDLK_QUOTEDBL as i32);
    pub const HASH: Keycode = Keycode(sys::SDL_KeyCode::SDLK_HASH as i32);
    pub const DOLLAR: Keycode = Keycode(sys::SDL_KeyCode::SDLK_DOLLAR as i32);
    pub const PERCENT: Keycode = Keycode(sys::SDL_KeyCode::SDLK_PERCENT as i32);
    pub const AMPERSAND: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AMPERSAND as i32);
    pub const QUOTE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_QUOTE as i32);
    pub const LEFTPAREN: Keycode = Keycode(sys::SDL_KeyCode::SDLK_LEFTPAREN as i32);
    pub const RIGHTPAREN: Keycode = Keycode(sys::SDL_KeyCode::SDLK_RIGHTPAREN as i32);
    pub const ASTERISK: Keycode = Keycode(sys::SDL_KeyCode::SDLK_ASTERISK as i32);
    pub const PLUS: Keycode = Keycode(sys::SDL_KeyCode::SDLK_PLUS as i32);
    pub const COMMA: Keycode = Keycode(sys::SDL_KeyCode::SDLK_COMMA as i32);
    pub const MINUS: Keycode = Keycode(sys::SDL_KeyCode::SDLK_MINUS as i32);
    pub const PERIOD: Keycode = Keycode(sys::SDL_KeyCode::SDLK_PERIOD as i32);
    pub const SLASH: Keycode = Keycode(sys::SDL_KeyCode::SDLK_SLASH as i32);
    pub const NUM_0: Keycode = Keycode(sys::SDL_KeyCode::SDLK_0 as i32);
    pub const NUM_1: Keycode = Keycode(sys::SDL_KeyCode::SDLK_1 as i32);
    pub const NUM_2: Keycode = Keycode(sys::SDL_KeyCode::SDLK_2 as i32);
    pub const NUM_3: Keycode = Keycode(sys::SDL_KeyCode::SDLK_3 as i32);
    pub const NUM_4: Keycode = Keycode(sys::SDL_KeyCode::SDLK_4 as i32);
    pub const NUM_5: Keycode = Keycode(sys::SDL_KeyCode::SDLK_5 as i32);
    pub const NUM_6: Keycode = Keycode(sys::SDL_KeyCode::SDLK_6 as i32);
    pub const NUM_7: Keycode = Keycode(sys::SDL_KeyCode::SDLK_7 as i32);
    pub const NUM_8: Keycode = Keycode(sys::SDL_KeyCode::SDLK_8 as i32);
    pub const NUM_9: Keycode = Keycode(sys::SDL_KeyCode::SDLK_9 as i32);
    pub const COLON: Keycode = Keycode(sys::SDL_KeyCode::SDLK_COLON as i32);
    pub const SEMICOLON: Keycode = Keycode(sys::SDL_KeyCode::SDLK_SEMICOLON as i32);
    pub const LESS: Keycode = Keycode(sys::SDL_KeyCode::SDLK_LESS as i32);
    pub const EQUALS: Keycode = Keycode(sys::SDL_KeyCode::SDLK_EQUALS as i32);
    pub const GREATER: Keycode = Keycode(sys::SDL_KeyCode::SDLK_GREATER as i32);
    pub const QUESTION: Keycode = Keycode(sys::SDL_KeyCode::SDLK_QUESTION as i32);
    pub const AT: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AT as i32);
    pub const LEFTBRACKET: Keycode = Keycode(sys::SDL_KeyCode::SDLK_LEFTBRACKET as i32);
    pub const BACKSLASH: Keycode = Keycode(sys::SDL_KeyCode::SDLK_BACKSLASH as i32);
    pub const RIGHTBRACKET: Keycode = Keycode(sys::SDL_KeyCode::SDLK_RIGHTBRACKET as i32);
    pub const CARET: Keycode = Keycode(sys::SDL_KeyCode::SDLK_CARET as i32);
    pub const UNDERSCORE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_UNDERSCORE as i32);
    pub const BACKQUOTE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_BACKQUOTE as i32);
    pub const A: Keycode = Keycode(sys::SDL_KeyCode::SDLK_a as i32);
    pub const B: Keycode = Keycode(sys::SDL_KeyCode::SDLK_b as i32);
    pub const C: Keycode = Keycode(sys::SDL_KeyCode::SDLK_c as i32);
    pub const D: Keycode = Keycode(sys::SDL_KeyCode::SDLK_d as i32);
    pub const E: Keycode = Keycode(sys::SDL_KeyCode::SDLK_e as i32);
    pub const F: Keycode = Keycode(sys::SDL_KeyCode::SDLK_f as i32);
    pub const G: Keycode = Keycode(sys::SDL_KeyCode::SDLK_g as i32);
    pub const H: Keycode = Keycode(sys::SDL_KeyCode::SDLK_h as i32);
    pub const I: Keycode = Keycode(sys::SDL_KeyCode::SDLK_i as i32);
    pub const J: Keycode = Keycode(sys::SDL_KeyCode::SDLK_j as i32);
    pub const K: Keycode = Keycode(sys::SDL_KeyCode::SDLK_k as i32);
    pub const L: Keycode = Keycode(sys::SDL_KeyCode::SDLK_l as i32);
    pub const M: Keycode = Keycode(sys::SDL_KeyCode::SDLK_m as i32);
    pub const N: Keycode = Keycode(sys::SDL_KeyCode::SDLK_n as i32);
    pub const O: Keycode = Keycode(sys::SDL_KeyCode::SDLK_o as i32);
    pub const P: Keycode = Keycode(sys::SDL_KeyCode::SDLK_p as i32);
    pub const Q: Keycode = Keycode(sys::SDL_KeyCode::SDLK_q as i32);
    pub const R: Keycode = Keycode(sys::SDL_KeyCode::SDLK_r as i32);
    pub const S: Keycode = Keycode(sys::SDL_KeyCode::SDLK_s as i32);
    pub const T: Keycode = Keycode(sys::SDL_KeyCode::SDLK_t as i32);
    pub const U: Keycode = Keycode(sys::SDL_KeyCode::SDLK_u as i32);
    pub const V: Keycode = Keycode(sys::SDL_KeyCode::SDLK_v as i32);
    pub const W: Keycode = Keycode(sys::SDL_KeyCode::SDLK_w as i32);
    pub const X: Keycode = Keycode(sys::SDL_KeyCode::SDLK_x as i32);
    pub const Y: Keycode = Keycode(sys::SDL_KeyCode::SDLK_y as i32);
    pub const Z: Keycode = Keycode(sys::SDL_KeyCode::SDLK_z as i32);
    pub const DELETE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_DELETE as i32);
    pub const CAPSLOCK: Keycode = Keycode(sys::SDL_KeyCode::SDLK_CAPSLOCK as i32);
    pub const F1: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F1 as i32);
    pub const F2: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F2 as i32);
    pub const F3: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F3 as i32);
    pub const F4: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F4 as i32);
    pub const F5: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F5 as i32);
    pub const F6: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F6 as i32);
    pub const F7: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F7 as i32);
    pub const F8: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F8 as i32);
    pub const F9: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F9 as i32);
    pub const F10: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F10 as i32);
    pub const F11: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F11 as i32);
    pub const F12: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F12 as i32);
    pub const PRINTSCREEN: Keycode = Keycode(sys::SDL_KeyCode::SDLK_PRINTSCREEN as i32);
    pub const SCROLLLOCK: Keycode = Keycode(sys::SDL_KeyCode::SDLK_SCROLLLOCK as i32);
    pub const PAUSE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_PAUSE as i32);
    pub const INSERT: Keycode = Keycode(sys::SDL_KeyCode::SDLK_INSERT as i32);
    pub const HOME: Keycode = Keycode(sys::SDL_KeyCode::SDLK_HOME as i32);
    pub const PAGEUP: Keycode = Keycode(sys::SDL_KeyCode::SDLK_PAGEUP as i32);
    pub const END: Keycode = Keycode(sys::SDL_KeyCode::SDLK_END as i32);
    pub const PAGEDOWN: Keycode = Keycode(sys::SDL_KeyCode::SDLK_PAGEDOWN as i32);
    pub const RIGHT: Keycode = Keycode(sys::SDL_KeyCode::SDLK_RIGHT as i32);
    pub const LEFT: Keycode = Keycode(sys::SDL_KeyCode::SDLK_LEFT as i32);
    pub const DOWN: Keycode = Keycode(sys::SDL_KeyCode::SDLK_DOWN as i32);
    pub const UP: Keycode = Keycode(sys::SDL_KeyCode::SDLK_UP as i32);
    pub const NUMLOCKCLEAR: Keycode = Keycode(sys::SDL_KeyCode::SDLK_NUMLOCKCLEAR as i32);
    pub const KP_DIVIDE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_DIVIDE as i32);
    pub const KP_MULTIPLY: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_MULTIPLY as i32);
    pub const KP_MINUS: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_MINUS as i32);
    pub const KP_PLUS: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_PLUS as i32);
    pub const KP_ENTER: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_ENTER as i32);
    pub const KP_1: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_1 as i32);
    pub const KP_2: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_2 as i32);
    pub const KP_3: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_3 as i32);
    pub const KP_4: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_4 as i32);
    pub const KP_5: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_5 as i32);
    pub const KP_6: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_6 as i32);
    pub const KP_7: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_7 as i32);
    pub const KP_8: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_8 as i32);
    pub const KP_9: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_9 as i32);
    pub const KP_0: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_0 as i32);
    pub const KP_PERIOD: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_PERIOD as i32);
    pub const APPLICATION: Keycode = Keycode(sys::SDL_KeyCode::SDLK_APPLICATION as i32);
    pub const POWER: Keycode = Keycode(sys::SDL_KeyCode::SDLK_POWER as i32);
    pub const KP_EQUALS: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_EQUALS as i32);
    pub const F13: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F13 as i32);
    pub const F14: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F14 as i32);
    pub const F15: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F15 as i32);
    pub const F16: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F16 as i32);
    pub const F17: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F17 as i32);
    pub const F18: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F18 as i32);
    pub const F19: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F19 as i32);
    pub const F20: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F20 as i32);
    pub const F21: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F21 as i32);
    pub const F22: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F22 as i32);
    pub const F23: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F23 as i32);
    pub const F24: Keycode = Keycode(sys::SDL_KeyCode::SDLK_F24 as i32);
    pub const EXECUTE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_EXECUTE as i32);
    pub const HELP: Keycode = Keycode(sys::SDL_KeyCode::SDLK_HELP as i32);
    pub const MENU: Keycode = Keycode(sys::SDL_KeyCode::SDLK_MENU as i32);
    pub const SELECT: Keycode = Keycode(sys::SDL_KeyCode::SDLK_SELECT as i32);
    pub const STOP: Keycode = Keycode(sys::SDL_KeyCode::SDLK_STOP as i32);
    pub const AGAIN: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AGAIN as i32);
    pub const UNDO: Keycode = Keycode(sys::SDL_KeyCode::SDLK_UNDO as i32);
    pub const CUT: Keycode = Keycode(sys::SDL_KeyCode::SDLK_CUT as i32);
    pub const COPY: Keycode = Keycode(sys::SDL_KeyCode::SDLK_COPY as i32);
    pub const PASTE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_PASTE as i32);
    pub const FIND: Keycode = Keycode(sys::SDL_KeyCode::SDLK_FIND as i32);
    pub const MUTE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_MUTE as i32);
    pub const VOLUMEUP: Keycode = Keycode(sys::SDL_KeyCode::SDLK_VOLUMEUP as i32);
    pub const VOLUMEDOWN: Keycode = Keycode(sys::SDL_KeyCode::SDLK_VOLUMEDOWN as i32);
    pub const KP_COMMA: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_COMMA as i32);
    pub const KP_EQUALSAS400: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_EQUALSAS400 as i32);
    pub const ALTERASE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_ALTERASE as i32);
    pub const SYSREQ: Keycode = Keycode(sys::SDL_KeyCode::SDLK_SYSREQ as i32);
    pub const CANCEL: Keycode = Keycode(sys::SDL_KeyCode::SDLK_CANCEL as i32);
    pub const CLEAR: Keycode = Keycode(sys::SDL_KeyCode::SDLK_CLEAR as i32);
    pub const PRIOR: Keycode = Keycode(sys::SDL_KeyCode::SDLK_PRIOR as i32);
    pub const RETURN2: Keycode = Keycode(sys::SDL_KeyCode::SDLK_RETURN2 as i32);
    pub const SEPARATOR: Keycode = Keycode(sys::SDL_KeyCode::SDLK_SEPARATOR as i32);
    pub const OUT: Keycode = Keycode(sys::SDL_KeyCode::SDLK_OUT as i32);
    pub const OPER: Keycode = Keycode(sys::SDL_KeyCode::SDLK_OPER as i32);
    pub const CLEARAGAIN: Keycode = Keycode(sys::SDL_KeyCode::SDLK_CLEARAGAIN as i32);
    pub const CRSEL: Keycode = Keycode(sys::SDL_KeyCode::SDLK_CRSEL as i32);
    pub const EXSEL: Keycode = Keycode(sys::SDL_KeyCode::SDLK_EXSEL as i32);
    pub const KP_00: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_00 as i32);
    pub const KP_000: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_000 as i32);
    pub const THOUSANDSSEPARATOR: Keycode =
        Keycode(sys::SDL_KeyCode::SDLK_THOUSANDSSEPARATOR as i32);
    pub const DECIMALSEPARATOR: Keycode = Keycode(sys::SDL_KeyCode::SDLK_DECIMALSEPARATOR as i32);
    pub const CURRENCYUNIT: Keycode = Keycode(sys::SDL_KeyCode::SDLK_CURRENCYUNIT as i32);
    pub const CURRENCYSUBUNIT: Keycode = Keycode(sys::SDL_KeyCode::SDLK_CURRENCYSUBUNIT as i32);
    pub const KP_LEFTPAREN: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_LEFTPAREN as i32);
    pub const KP_RIGHTPAREN: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_RIGHTPAREN as i32);
    pub const KP_LEFTBRACE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_LEFTBRACE as i32);
    pub const KP_RIGHTBRACE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_RIGHTBRACE as i32);
    pub const KP_TAB: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_TAB as i32);
    pub const KP_BACKSPACE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_BACKSPACE as i32);
    pub const KP_A: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_A as i32);
    pub const KP_B: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_B as i32);
    pub const KP_C: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_C as i32);
    pub const KP_D: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_D as i32);
    pub const KP_E: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_E as i32);
    pub const KP_F: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_F as i32);
    pub const KP_XOR: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_XOR as i32);
    pub const KP_POWER: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_POWER as i32);
    pub const KP_PERCENT: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_PERCENT as i32);
    pub const KP_LESS: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_LESS as i32);
    pub const KP_GREATER: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_GREATER as i32);
    pub const KP_AMPERSAND: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_AMPERSAND as i32);
    pub const KP_DBLAMPERSAND: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_DBLAMPERSAND as i32);
    pub const KP_VERTICALBAR: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_VERTICALBAR as i32);
    pub const KP_DBLVERTICALBAR: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_DBLVERTICALBAR as i32);
    pub const KP_COLON: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_COLON as i32);
    pub const KP_HASH: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_HASH as i32);
    pub const KP_SPACE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_SPACE as i32);
    pub const KP_AT: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_AT as i32);
    pub const KP_EXCLAM: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_EXCLAM as i32);
    pub const KP_MEMSTORE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_MEMSTORE as i32);
    pub const KP_MEMRECALL: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_MEMRECALL as i32);
    pub const KP_MEMCLEAR: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_MEMCLEAR as i32);
    pub const KP_MEMADD: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_MEMADD as i32);
    pub const KP_MEMSUBTRACT: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_MEMSUBTRACT as i32);
    pub const KP_MEMMULTIPLY: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_MEMMULTIPLY as i32);
    pub const KP_MEMDIVIDE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_MEMDIVIDE as i32);
    pub const KP_PLUSMINUS: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_PLUSMINUS as i32);
    pub const KP_CLEAR: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_CLEAR as i32);
    pub const KP_CLEARENTRY: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_CLEARENTRY as i32);
    pub const KP_BINARY: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_BINARY as i32);
    pub const KP_OCTAL: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_OCTAL as i32);
    pub const KP_DECIMAL: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_DECIMAL as i32);
    pub const KP_HEXADECIMAL: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_HEXADECIMAL as i32);
    pub const LCTRL: Keycode = Keycode(sys::SDL_KeyCode::SDLK_LCTRL as i32);
    pub const LSHIFT: Keycode = Keycode(sys::SDL_KeyCode::SDLK_LSHIFT as i32);
    pub const LALT: Keycode = Keycode(sys::SDL_KeyCode::SDLK_LALT as i32);
    pub const LGUI: Keycode = Keycode(sys::SDL_KeyCode::SDLK_LGUI as i32);
    pub const RCTRL: Keycode = Keycode(sys::SDL_KeyCode::SDLK_RCTRL as i32);
    pub const RSHIFT: Keycode = Keycode(sys::SDL_KeyCode::SDLK_RSHIFT as i32);
    pub const RALT: Keycode = Keycode(sys::SDL_KeyCode::SDLK_RALT as i32);
    pub const RGUI: Keycode = Keycode(sys::SDL_KeyCode::SDLK_RGUI as i32);
    pub const MODE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_MODE as i32);
    pub const AUDIONEXT: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AUDIONEXT as i32);
    pub const AUDIOPREV: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AUDIOPREV as i32);
    pub const AUDIOSTOP: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AUDIOSTOP as i32);
    pub const AUDIOPLAY: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AUDIOPLAY as i32);
    pub const AUDIOMUTE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AUDIOMUTE as i32);
    pub const MEDIASELECT: Keycode = Keycode(sys::SDL_KeyCode::SDLK_MEDIASELECT as i32);
    pub const WWW: Keycode = Keycode(sys::SDL_KeyCode::SDLK_WWW as i32);
    pub const MAIL: Keycode = Keycode(sys::SDL_KeyCode::SDLK_MAIL as i32);
    pub const CALCULATOR: Keycode = Keycode(sys::SDL_KeyCode::SDLK_CALCULATOR as i32);
    pub const COMPUTER: Keycode = Keycode(sys::SDL_KeyCode::SDLK_COMPUTER as i32);
    pub const AC_SEARCH: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AC_SEARCH as i32);
    pub const AC_HOME: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AC_HOME as i32);
    pub const AC_BACK: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AC_BACK as i32);
    pub const AC_FORWARD: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AC_FORWARD as i32);
    pub const AC_STOP: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AC_STOP as i32);
    pub const AC_REFRESH: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AC_REFRESH as i32);
    pub const AC_BOOKMARKS: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AC_BOOKMARKS as i32);
    pub const BRIGHTNESSDOWN: Keycode = Keycode(sys::SDL_KeyCode::SDLK_BRIGHTNESSDOWN as i32);
    pub const BRIGHTNESSUP: Keycode = Keycode(sys::SDL_KeyCode::SDLK_BRIGHTNESSUP as i32);
    pub const DISPLAYSWITCH: Keycode = Keycode(sys::SDL_KeyCode::SDLK_DISPLAYSWITCH as i32);
    pub const KBDILLUMTOGGLE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KBDILLUMTOGGLE as i32);
    pub const KBDILLUMDOWN: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KBDILLUMDOWN as i32);
    pub const KBDILLUMUP: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KBDILLUMUP as i32);
    pub const EJECT: Keycode = Keycode(sys::SDL_KeyCode::SDLK_EJECT as i32);
    pub const SLEEP: Keycode = Keycode(sys::SDL_KeyCode::SDLK_SLEEP as i32);

    // The constants below are here only for backwards-compability,
    // because Keycode used to be an enum in 0.36 and below.
    pub const Backspace: Keycode = Keycode::BACKSPACE;
    pub const Tab: Keycode = Keycode::TAB;
    pub const Return: Keycode = Keycode::RETURN;
    pub const Escape: Keycode = Keycode::ESCAPE;
    pub const Space: Keycode = Keycode::SPACE;
    pub const Exclaim: Keycode = Keycode::EXCLAIM;
    pub const Quotedbl: Keycode = Keycode::QUOTEDBL;
    pub const Hash: Keycode = Keycode::HASH;
    pub const Dollar: Keycode = Keycode::DOLLAR;
    pub const Percent: Keycode = Keycode::PERCENT;
    pub const Ampersand: Keycode = Keycode::AMPERSAND;
    pub const Quote: Keycode = Keycode::QUOTE;
    pub const LeftParen: Keycode = Keycode::LEFTPAREN;
    pub const RightParen: Keycode = Keycode::RIGHTPAREN;
    pub const Asterisk: Keycode = Keycode::ASTERISK;
    pub const Plus: Keycode = Keycode::PLUS;
    pub const Comma: Keycode = Keycode::COMMA;
    pub const Minus: Keycode = Keycode::MINUS;
    pub const Period: Keycode = Keycode::PERIOD;
    pub const Slash: Keycode = Keycode::SLASH;
    pub const Num0: Keycode = Keycode::NUM_0;
    pub const Num1: Keycode = Keycode::NUM_1;
    pub const Num2: Keycode = Keycode::NUM_2;
    pub const Num3: Keycode = Keycode::NUM_3;
    pub const Num4: Keycode = Keycode::NUM_4;
    pub const Num5: Keycode = Keycode::NUM_5;
    pub const Num6: Keycode = Keycode::NUM_6;
    pub const Num7: Keycode = Keycode::NUM_7;
    pub const Num8: Keycode = Keycode::NUM_8;
    pub const Num9: Keycode = Keycode::NUM_9;
    pub const Colon: Keycode = Keycode::COLON;
    pub const Semicolon: Keycode = Keycode::SEMICOLON;
    pub const Less: Keycode = Keycode::LESS;
    pub const Equals: Keycode = Keycode::EQUALS;
    pub const Greater: Keycode = Keycode::GREATER;
    pub const Question: Keycode = Keycode::QUESTION;
    pub const At: Keycode = Keycode::AT;
    pub const LeftBracket: Keycode = Keycode::LEFTBRACKET;
    pub const Backslash: Keycode = Keycode::BACKSLASH;
    pub const RightBracket: Keycode = Keycode::RIGHTBRACKET;
    pub const Caret: Keycode = Keycode::CARET;
    pub const Underscore: Keycode = Keycode::UNDERSCORE;
    pub const Backquote: Keycode = Keycode::BACKQUOTE;
    pub const Delete: Keycode = Keycode::DELETE;
    pub const CapsLock: Keycode = Keycode::CAPSLOCK;
    pub const PrintScreen: Keycode = Keycode::PRINTSCREEN;
    pub const ScrollLock: Keycode = Keycode::SCROLLLOCK;
    pub const Pause: Keycode = Keycode::PAUSE;
    pub const Insert: Keycode = Keycode::INSERT;
    pub const Home: Keycode = Keycode::HOME;
    pub const PageUp: Keycode = Keycode::PAGEUP;
    pub const End: Keycode = Keycode::END;
    pub const PageDown: Keycode = Keycode::PAGEDOWN;
    pub const Right: Keycode = Keycode::RIGHT;
    pub const Left: Keycode = Keycode::LEFT;
    pub const Down: Keycode = Keycode::DOWN;
    pub const Up: Keycode = Keycode::UP;
    pub const NumLockClear: Keycode = Keycode::NUMLOCKCLEAR;
    pub const KpDivide: Keycode = Keycode::KP_DIVIDE;
    pub const KpMultiply: Keycode = Keycode::KP_MULTIPLY;
    pub const KpMinus: Keycode = Keycode::KP_MINUS;
    pub const KpPlus: Keycode = Keycode::KP_PLUS;
    pub const KpEnter: Keycode = Keycode::KP_ENTER;
    pub const Kp1: Keycode = Keycode::KP_1;
    pub const Kp2: Keycode = Keycode::KP_2;
    pub const Kp3: Keycode = Keycode::KP_3;
    pub const Kp4: Keycode = Keycode::KP_4;
    pub const Kp5: Keycode = Keycode::KP_5;
    pub const Kp6: Keycode = Keycode::KP_6;
    pub const Kp7: Keycode = Keycode::KP_7;
    pub const Kp8: Keycode = Keycode::KP_8;
    pub const Kp9: Keycode = Keycode::KP_9;
    pub const Kp0: Keycode = Keycode::KP_0;
    pub const KpPeriod: Keycode = Keycode::KP_PERIOD;
    pub const Application: Keycode = Keycode::APPLICATION;
    pub const Power: Keycode = Keycode::POWER;
    pub const KpEquals: Keycode = Keycode::KP_EQUALS;
    pub const Execute: Keycode = Keycode::EXECUTE;
    pub const Help: Keycode = Keycode::HELP;
    pub const Menu: Keycode = Keycode::MENU;
    pub const Select: Keycode = Keycode::SELECT;
    pub const Stop: Keycode = Keycode::STOP;
    pub const Again: Keycode = Keycode::AGAIN;
    pub const Undo: Keycode = Keycode::UNDO;
    pub const Cut: Keycode = Keycode::CUT;
    pub const Copy: Keycode = Keycode::COPY;
    pub const Paste: Keycode = Keycode::PASTE;
    pub const Find: Keycode = Keycode::FIND;
    pub const Mute: Keycode = Keycode::MUTE;
    pub const VolumeUp: Keycode = Keycode::VOLUMEUP;
    pub const VolumeDown: Keycode = Keycode::VOLUMEDOWN;
    pub const KpComma: Keycode = Keycode::KP_COMMA;
    pub const KpEqualsAS400: Keycode = Keycode::KP_EQUALSAS400;
    pub const AltErase: Keycode = Keycode::ALTERASE;
    pub const Sysreq: Keycode = Keycode::SYSREQ;
    pub const Cancel: Keycode = Keycode::CANCEL;
    pub const Clear: Keycode = Keycode::CLEAR;
    pub const Prior: Keycode = Keycode::PRIOR;
    pub const Return2: Keycode = Keycode::RETURN2;
    pub const Separator: Keycode = Keycode::SEPARATOR;
    pub const Out: Keycode = Keycode::OUT;
    pub const Oper: Keycode = Keycode::OPER;
    pub const ClearAgain: Keycode = Keycode::CLEARAGAIN;
    pub const CrSel: Keycode = Keycode::CRSEL;
    pub const ExSel: Keycode = Keycode::EXSEL;
    pub const Kp00: Keycode = Keycode::KP_00;
    pub const Kp000: Keycode = Keycode::KP_000;
    pub const ThousandsSeparator: Keycode = Keycode::THOUSANDSSEPARATOR;
    pub const DecimalSeparator: Keycode = Keycode::DECIMALSEPARATOR;
    pub const CurrencyUnit: Keycode = Keycode::CURRENCYUNIT;
    pub const CurrencySubUnit: Keycode = Keycode::CURRENCYSUBUNIT;
    pub const KpLeftParen: Keycode = Keycode::KP_LEFTPAREN;
    pub const KpRightParen: Keycode = Keycode::KP_RIGHTPAREN;
    pub const KpLeftBrace: Keycode = Keycode::KP_LEFTBRACE;
    pub const KpRightBrace: Keycode = Keycode::KP_RIGHTBRACE;
    pub const KpTab: Keycode = Keycode::KP_TAB;
    pub const KpBackspace: Keycode = Keycode::KP_BACKSPACE;
    pub const KpA: Keycode = Keycode::KP_A;
    pub const KpB: Keycode = Keycode::KP_B;
    pub const KpC: Keycode = Keycode::KP_C;
    pub const KpD: Keycode = Keycode::KP_D;
    pub const KpE: Keycode = Keycode::KP_E;
    pub const KpF: Keycode = Keycode::KP_F;
    pub const KpXor: Keycode = Keycode::KP_XOR;
    pub const KpPower: Keycode = Keycode::KP_POWER;
    pub const KpPercent: Keycode = Keycode::KP_PERCENT;
    pub const KpLess: Keycode = Keycode::KP_LESS;
    pub const KpGreater: Keycode = Keycode::KP_GREATER;
    pub const KpAmpersand: Keycode = Keycode::KP_AMPERSAND;
    pub const KpDblAmpersand: Keycode = Keycode::KP_DBLAMPERSAND;
    pub const KpVerticalBar: Keycode = Keycode::KP_VERTICALBAR;
    pub const KpDblVerticalBar: Keycode = Keycode::KP_DBLVERTICALBAR;
    pub const KpColon: Keycode = Keycode::KP_COLON;
    pub const KpHash: Keycode = Keycode::KP_HASH;
    pub const KpSpace: Keycode = Keycode::KP_SPACE;
    pub const KpAt: Keycode = Keycode::KP_AT;
    pub const KpExclam: Keycode = Keycode::KP_EXCLAM;
    pub const KpMemStore: Keycode = Keycode::KP_MEMSTORE;
    pub const KpMemRecall: Keycode = Keycode::KP_MEMRECALL;
    pub const KpMemClear: Keycode = Keycode::KP_MEMCLEAR;
    pub const KpMemAdd: Keycode = Keycode::KP_MEMADD;
    pub const KpMemSubtract: Keycode = Keycode::KP_MEMSUBTRACT;
    pub const KpMemMultiply: Keycode = Keycode::KP_MEMMULTIPLY;
    pub const KpMemDivide: Keycode = Keycode::KP_MEMDIVIDE;
    pub const KpPlusMinus: Keycode = Keycode::KP_PLUSMINUS;
    pub const KpClear: Keycode = Keycode::KP_CLEAR;
    pub const KpClearEntry: Keycode = Keycode::KP_CLEARENTRY;
    pub const KpBinary: Keycode = Keycode::KP_BINARY;
    pub const KpOctal: Keycode = Keycode::KP_OCTAL;
    pub const KpDecimal: Keycode = Keycode::KP_DECIMAL;
    pub const KpHexadecimal: Keycode = Keycode::KP_HEXADECIMAL;
    pub const LCtrl: Keycode = Keycode::LCTRL;
    pub const LShift: Keycode = Keycode::LSHIFT;
    pub const LAlt: Keycode = Keycode::LALT;
    pub const LGui: Keycode = Keycode::LGUI;
    pub const RCtrl: Keycode = Keycode::RCTRL;
    pub const RShift: Keycode = Keycode::RSHIFT;
    pub const RAlt: Keycode = Keycode::RALT;
    pub const RGui: Keycode = Keycode::RGUI;
    pub const Mode: Keycode = Keycode::MODE;
    pub const AudioNext: Keycode = Keycode::AUDIONEXT;
    pub const AudioPrev: Keycode = Keycode::AUDIOPREV;
    pub const AudioStop: Keycode = Keycode::AUDIOSTOP;
    pub const AudioPlay: Keycode = Keycode::AUDIOPLAY;
    pub const AudioMute: Keycode = Keycode::AUDIOMUTE;
    pub const MediaSelect: Keycode = Keycode::MEDIASELECT;
    pub const Www: Keycode = Keycode::WWW;
    pub const Mail: Keycode = Keycode::MAIL;
    pub const Calculator: Keycode = Keycode::CALCULATOR;
    pub const Computer: Keycode = Keycode::COMPUTER;
    pub const AcSearch: Keycode = Keycode::AC_SEARCH;
    pub const AcHome: Keycode = Keycode::AC_HOME;
    pub const AcBack: Keycode = Keycode::AC_BACK;
    pub const AcForward: Keycode = Keycode::AC_FORWARD;
    pub const AcStop: Keycode = Keycode::AC_STOP;
    pub const AcRefresh: Keycode = Keycode::AC_REFRESH;
    pub const AcBookmarks: Keycode = Keycode::AC_BOOKMARKS;
    pub const BrightnessDown: Keycode = Keycode::BRIGHTNESSDOWN;
    pub const BrightnessUp: Keycode = Keycode::BRIGHTNESSUP;
    pub const DisplaySwitch: Keycode = Keycode::DISPLAYSWITCH;
    pub const KbdIllumToggle: Keycode = Keycode::KBDILLUMTOGGLE;
    pub const KbdIllumDown: Keycode = Keycode::KBDILLUMDOWN;
    pub const KbdIllumUp: Keycode = Keycode::KBDILLUMUP;
    pub const Eject: Keycode = Keycode::EJECT;
    pub const Sleep: Keycode = Keycode::SLEEP;
}

impl Keycode {
    pub fn into_i32(&self) -> i32 {
        return self.0;
    }

    pub fn from_i32(n: i32) -> Option<Keycode> {
        if n != 0 {
            return Some(Keycode(n));
        }
        return None;
    }
}

use std::fmt;

impl fmt::Display for Keycode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.name())
    }
}

use std::ops::Deref;

impl Deref for Keycode {
    type Target = i32;

    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl Into<i32> for Keycode {
    fn into(self) -> i32 {
        self.into_i32()
    }
}

use crate::keyboard::Scancode;

impl Keycode {
    /// Gets the virtual key from a scancode. Returns None if there is no corresponding virtual key.
    #[doc(alias = "SDL_GetKeyFromScancode")]
    pub fn from_scancode(scancode: Scancode) -> Option<Keycode> {
        const UNKNOWN: i32 = sys::SDL_KeyCode::SDLK_UNKNOWN as i32;
        unsafe {
            match sys::SDL_GetKeyFromScancode(transmute::<u32, sys::SDL_Scancode>(scancode as u32))
            {
                UNKNOWN => None,
                keycode_id => Keycode::from_i32(keycode_id as i32),
            }
        }
    }

    #[doc(alias = "SDL_GetKeyFromName")]
    pub fn from_name(name: &str) -> Option<Keycode> {
        const UNKNOWN: i32 = sys::SDL_KeyCode::SDLK_UNKNOWN as i32;
        unsafe {
            match CString::new(name) {
                Ok(name) => match sys::SDL_GetKeyFromName(name.as_ptr() as *const c_char) {
                    UNKNOWN => None,
                    keycode_id => Some(Keycode::from_i32(keycode_id as i32).unwrap()),
                },
                // string contains a nul byte - it won't match anything.
                Err(_) => None,
            }
        }
    }

    #[doc(alias = "SDL_GetKeyName")]
    pub fn name(self) -> String {
        // The name string pointer's contents _might_ change, depending on the last call to SDL_GetKeyName.
        // Knowing this, we must always return a new string.
        unsafe {
            let buf = sys::SDL_GetKeyName(self.into());
            CStr::from_ptr(buf).to_str().unwrap().to_owned()
        }
    }
}
