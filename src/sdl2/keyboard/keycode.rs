#![allow(unreachable_patterns)]

use libc::c_char;
use std::ffi::{CStr, CString};
use std::mem::transmute;

use crate::sys;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Keycode(pub i32);

#[allow(non_upper_case_globals)]
impl Keycode {
    pub const Backspace: Keycode = Keycode(sys::SDL_KeyCode::SDLK_BACKSPACE as i32);
    pub const Tab: Keycode = Keycode(sys::SDL_KeyCode::SDLK_TAB as i32);
    pub const Return: Keycode = Keycode(sys::SDL_KeyCode::SDLK_RETURN as i32);
    pub const Escape: Keycode = Keycode(sys::SDL_KeyCode::SDLK_ESCAPE as i32);
    pub const Space: Keycode = Keycode(sys::SDL_KeyCode::SDLK_SPACE as i32);
    pub const Exclaim: Keycode = Keycode(sys::SDL_KeyCode::SDLK_EXCLAIM as i32);
    pub const Quotedbl: Keycode = Keycode(sys::SDL_KeyCode::SDLK_QUOTEDBL as i32);
    pub const Hash: Keycode = Keycode(sys::SDL_KeyCode::SDLK_HASH as i32);
    pub const Dollar: Keycode = Keycode(sys::SDL_KeyCode::SDLK_DOLLAR as i32);
    pub const Percent: Keycode = Keycode(sys::SDL_KeyCode::SDLK_PERCENT as i32);
    pub const Ampersand: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AMPERSAND as i32);
    pub const Quote: Keycode = Keycode(sys::SDL_KeyCode::SDLK_QUOTE as i32);
    pub const LeftParen: Keycode = Keycode(sys::SDL_KeyCode::SDLK_LEFTPAREN as i32);
    pub const RightParen: Keycode = Keycode(sys::SDL_KeyCode::SDLK_RIGHTPAREN as i32);
    pub const Asterisk: Keycode = Keycode(sys::SDL_KeyCode::SDLK_ASTERISK as i32);
    pub const Plus: Keycode = Keycode(sys::SDL_KeyCode::SDLK_PLUS as i32);
    pub const Comma: Keycode = Keycode(sys::SDL_KeyCode::SDLK_COMMA as i32);
    pub const Minus: Keycode = Keycode(sys::SDL_KeyCode::SDLK_MINUS as i32);
    pub const Period: Keycode = Keycode(sys::SDL_KeyCode::SDLK_PERIOD as i32);
    pub const Slash: Keycode = Keycode(sys::SDL_KeyCode::SDLK_SLASH as i32);
    pub const Num0: Keycode = Keycode(sys::SDL_KeyCode::SDLK_0 as i32);
    pub const Num1: Keycode = Keycode(sys::SDL_KeyCode::SDLK_1 as i32);
    pub const Num2: Keycode = Keycode(sys::SDL_KeyCode::SDLK_2 as i32);
    pub const Num3: Keycode = Keycode(sys::SDL_KeyCode::SDLK_3 as i32);
    pub const Num4: Keycode = Keycode(sys::SDL_KeyCode::SDLK_4 as i32);
    pub const Num5: Keycode = Keycode(sys::SDL_KeyCode::SDLK_5 as i32);
    pub const Num6: Keycode = Keycode(sys::SDL_KeyCode::SDLK_6 as i32);
    pub const Num7: Keycode = Keycode(sys::SDL_KeyCode::SDLK_7 as i32);
    pub const Num8: Keycode = Keycode(sys::SDL_KeyCode::SDLK_8 as i32);
    pub const Num9: Keycode = Keycode(sys::SDL_KeyCode::SDLK_9 as i32);
    pub const Colon: Keycode = Keycode(sys::SDL_KeyCode::SDLK_COLON as i32);
    pub const Semicolon: Keycode = Keycode(sys::SDL_KeyCode::SDLK_SEMICOLON as i32);
    pub const Less: Keycode = Keycode(sys::SDL_KeyCode::SDLK_LESS as i32);
    pub const Equals: Keycode = Keycode(sys::SDL_KeyCode::SDLK_EQUALS as i32);
    pub const Greater: Keycode = Keycode(sys::SDL_KeyCode::SDLK_GREATER as i32);
    pub const Question: Keycode = Keycode(sys::SDL_KeyCode::SDLK_QUESTION as i32);
    pub const At: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AT as i32);
    pub const LeftBracket: Keycode = Keycode(sys::SDL_KeyCode::SDLK_LEFTBRACKET as i32);
    pub const Backslash: Keycode = Keycode(sys::SDL_KeyCode::SDLK_BACKSLASH as i32);
    pub const RightBracket: Keycode = Keycode(sys::SDL_KeyCode::SDLK_RIGHTBRACKET as i32);
    pub const Caret: Keycode = Keycode(sys::SDL_KeyCode::SDLK_CARET as i32);
    pub const Underscore: Keycode = Keycode(sys::SDL_KeyCode::SDLK_UNDERSCORE as i32);
    pub const Backquote: Keycode = Keycode(sys::SDL_KeyCode::SDLK_BACKQUOTE as i32);
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
    pub const Delete: Keycode = Keycode(sys::SDL_KeyCode::SDLK_DELETE as i32);
    pub const CapsLock: Keycode = Keycode(sys::SDL_KeyCode::SDLK_CAPSLOCK as i32);
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
    pub const PrintScreen: Keycode = Keycode(sys::SDL_KeyCode::SDLK_PRINTSCREEN as i32);
    pub const ScrollLock: Keycode = Keycode(sys::SDL_KeyCode::SDLK_SCROLLLOCK as i32);
    pub const Pause: Keycode = Keycode(sys::SDL_KeyCode::SDLK_PAUSE as i32);
    pub const Insert: Keycode = Keycode(sys::SDL_KeyCode::SDLK_INSERT as i32);
    pub const Home: Keycode = Keycode(sys::SDL_KeyCode::SDLK_HOME as i32);
    pub const PageUp: Keycode = Keycode(sys::SDL_KeyCode::SDLK_PAGEUP as i32);
    pub const End: Keycode = Keycode(sys::SDL_KeyCode::SDLK_END as i32);
    pub const PageDown: Keycode = Keycode(sys::SDL_KeyCode::SDLK_PAGEDOWN as i32);
    pub const Right: Keycode = Keycode(sys::SDL_KeyCode::SDLK_RIGHT as i32);
    pub const Left: Keycode = Keycode(sys::SDL_KeyCode::SDLK_LEFT as i32);
    pub const Down: Keycode = Keycode(sys::SDL_KeyCode::SDLK_DOWN as i32);
    pub const Up: Keycode = Keycode(sys::SDL_KeyCode::SDLK_UP as i32);
    pub const NumLockClear: Keycode = Keycode(sys::SDL_KeyCode::SDLK_NUMLOCKCLEAR as i32);
    pub const KpDivide: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_DIVIDE as i32);
    pub const KpMultiply: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_MULTIPLY as i32);
    pub const KpMinus: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_MINUS as i32);
    pub const KpPlus: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_PLUS as i32);
    pub const KpEnter: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_ENTER as i32);
    pub const Kp1: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_1 as i32);
    pub const Kp2: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_2 as i32);
    pub const Kp3: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_3 as i32);
    pub const Kp4: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_4 as i32);
    pub const Kp5: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_5 as i32);
    pub const Kp6: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_6 as i32);
    pub const Kp7: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_7 as i32);
    pub const Kp8: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_8 as i32);
    pub const Kp9: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_9 as i32);
    pub const Kp0: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_0 as i32);
    pub const KpPeriod: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_PERIOD as i32);
    pub const Application: Keycode = Keycode(sys::SDL_KeyCode::SDLK_APPLICATION as i32);
    pub const Power: Keycode = Keycode(sys::SDL_KeyCode::SDLK_POWER as i32);
    pub const KpEquals: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_EQUALS as i32);
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
    pub const Execute: Keycode = Keycode(sys::SDL_KeyCode::SDLK_EXECUTE as i32);
    pub const Help: Keycode = Keycode(sys::SDL_KeyCode::SDLK_HELP as i32);
    pub const Menu: Keycode = Keycode(sys::SDL_KeyCode::SDLK_MENU as i32);
    pub const Select: Keycode = Keycode(sys::SDL_KeyCode::SDLK_SELECT as i32);
    pub const Stop: Keycode = Keycode(sys::SDL_KeyCode::SDLK_STOP as i32);
    pub const Again: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AGAIN as i32);
    pub const Undo: Keycode = Keycode(sys::SDL_KeyCode::SDLK_UNDO as i32);
    pub const Cut: Keycode = Keycode(sys::SDL_KeyCode::SDLK_CUT as i32);
    pub const Copy: Keycode = Keycode(sys::SDL_KeyCode::SDLK_COPY as i32);
    pub const Paste: Keycode = Keycode(sys::SDL_KeyCode::SDLK_PASTE as i32);
    pub const Find: Keycode = Keycode(sys::SDL_KeyCode::SDLK_FIND as i32);
    pub const Mute: Keycode = Keycode(sys::SDL_KeyCode::SDLK_MUTE as i32);
    pub const VolumeUp: Keycode = Keycode(sys::SDL_KeyCode::SDLK_VOLUMEUP as i32);
    pub const VolumeDown: Keycode = Keycode(sys::SDL_KeyCode::SDLK_VOLUMEDOWN as i32);
    pub const KpComma: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_COMMA as i32);
    pub const KpEqualsAS400: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_EQUALSAS400 as i32);
    pub const AltErase: Keycode = Keycode(sys::SDL_KeyCode::SDLK_ALTERASE as i32);
    pub const Sysreq: Keycode = Keycode(sys::SDL_KeyCode::SDLK_SYSREQ as i32);
    pub const Cancel: Keycode = Keycode(sys::SDL_KeyCode::SDLK_CANCEL as i32);
    pub const Clear: Keycode = Keycode(sys::SDL_KeyCode::SDLK_CLEAR as i32);
    pub const Prior: Keycode = Keycode(sys::SDL_KeyCode::SDLK_PRIOR as i32);
    pub const Return2: Keycode = Keycode(sys::SDL_KeyCode::SDLK_RETURN2 as i32);
    pub const Separator: Keycode = Keycode(sys::SDL_KeyCode::SDLK_SEPARATOR as i32);
    pub const Out: Keycode = Keycode(sys::SDL_KeyCode::SDLK_OUT as i32);
    pub const Oper: Keycode = Keycode(sys::SDL_KeyCode::SDLK_OPER as i32);
    pub const ClearAgain: Keycode = Keycode(sys::SDL_KeyCode::SDLK_CLEARAGAIN as i32);
    pub const CrSel: Keycode = Keycode(sys::SDL_KeyCode::SDLK_CRSEL as i32);
    pub const ExSel: Keycode = Keycode(sys::SDL_KeyCode::SDLK_EXSEL as i32);
    pub const Kp00: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_00 as i32);
    pub const Kp000: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_000 as i32);
    pub const ThousandsSeparator: Keycode = Keycode(sys::SDL_KeyCode::SDLK_THOUSANDSSEPARATOR as i32);
    pub const DecimalSeparator: Keycode = Keycode(sys::SDL_KeyCode::SDLK_DECIMALSEPARATOR as i32);
    pub const CurrencyUnit: Keycode = Keycode(sys::SDL_KeyCode::SDLK_CURRENCYUNIT as i32);
    pub const CurrencySubUnit: Keycode = Keycode(sys::SDL_KeyCode::SDLK_CURRENCYSUBUNIT as i32);
    pub const KpLeftParen: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_LEFTPAREN as i32);
    pub const KpRightParen: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_RIGHTPAREN as i32);
    pub const KpLeftBrace: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_LEFTBRACE as i32);
    pub const KpRightBrace: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_RIGHTBRACE as i32);
    pub const KpTab: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_TAB as i32);
    pub const KpBackspace: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_BACKSPACE as i32);
    pub const KpA: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_A as i32);
    pub const KpB: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_B as i32);
    pub const KpC: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_C as i32);
    pub const KpD: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_D as i32);
    pub const KpE: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_E as i32);
    pub const KpF: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_F as i32);
    pub const KpXor: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_XOR as i32);
    pub const KpPower: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_POWER as i32);
    pub const KpPercent: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_PERCENT as i32);
    pub const KpLess: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_LESS as i32);
    pub const KpGreater: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_GREATER as i32);
    pub const KpAmpersand: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_AMPERSAND as i32);
    pub const KpDblAmpersand: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_DBLAMPERSAND as i32);
    pub const KpVerticalBar: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_VERTICALBAR as i32);
    pub const KpDblVerticalBar: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_DBLVERTICALBAR as i32);
    pub const KpColon: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_COLON as i32);
    pub const KpHash: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_HASH as i32);
    pub const KpSpace: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_SPACE as i32);
    pub const KpAt: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_AT as i32);
    pub const KpExclam: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_EXCLAM as i32);
    pub const KpMemStore: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_MEMSTORE as i32);
    pub const KpMemRecall: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_MEMRECALL as i32);
    pub const KpMemClear: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_MEMCLEAR as i32);
    pub const KpMemAdd: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_MEMADD as i32);
    pub const KpMemSubtract: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_MEMSUBTRACT as i32);
    pub const KpMemMultiply: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_MEMMULTIPLY as i32);
    pub const KpMemDivide: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_MEMDIVIDE as i32);
    pub const KpPlusMinus: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_PLUSMINUS as i32);
    pub const KpClear: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_CLEAR as i32);
    pub const KpClearEntry: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_CLEARENTRY as i32);
    pub const KpBinary: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_BINARY as i32);
    pub const KpOctal: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_OCTAL as i32);
    pub const KpDecimal: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_DECIMAL as i32);
    pub const KpHexadecimal: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KP_HEXADECIMAL as i32);
    pub const LCtrl: Keycode = Keycode(sys::SDL_KeyCode::SDLK_LCTRL as i32);
    pub const LShift: Keycode = Keycode(sys::SDL_KeyCode::SDLK_LSHIFT as i32);
    pub const LAlt: Keycode = Keycode(sys::SDL_KeyCode::SDLK_LALT as i32);
    pub const LGui: Keycode = Keycode(sys::SDL_KeyCode::SDLK_LGUI as i32);
    pub const RCtrl: Keycode = Keycode(sys::SDL_KeyCode::SDLK_RCTRL as i32);
    pub const RShift: Keycode = Keycode(sys::SDL_KeyCode::SDLK_RSHIFT as i32);
    pub const RAlt: Keycode = Keycode(sys::SDL_KeyCode::SDLK_RALT as i32);
    pub const RGui: Keycode = Keycode(sys::SDL_KeyCode::SDLK_RGUI as i32);
    pub const Mode: Keycode = Keycode(sys::SDL_KeyCode::SDLK_MODE as i32);
    pub const AudioNext: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AUDIONEXT as i32);
    pub const AudioPrev: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AUDIOPREV as i32);
    pub const AudioStop: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AUDIOSTOP as i32);
    pub const AudioPlay: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AUDIOPLAY as i32);
    pub const AudioMute: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AUDIOMUTE as i32);
    pub const MediaSelect: Keycode = Keycode(sys::SDL_KeyCode::SDLK_MEDIASELECT as i32);
    pub const Www: Keycode = Keycode(sys::SDL_KeyCode::SDLK_WWW as i32);
    pub const Mail: Keycode = Keycode(sys::SDL_KeyCode::SDLK_MAIL as i32);
    pub const Calculator: Keycode = Keycode(sys::SDL_KeyCode::SDLK_CALCULATOR as i32);
    pub const Computer: Keycode = Keycode(sys::SDL_KeyCode::SDLK_COMPUTER as i32);
    pub const AcSearch: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AC_SEARCH as i32);
    pub const AcHome: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AC_HOME as i32);
    pub const AcBack: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AC_BACK as i32);
    pub const AcForward: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AC_FORWARD as i32);
    pub const AcStop: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AC_STOP as i32);
    pub const AcRefresh: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AC_REFRESH as i32);
    pub const AcBookmarks: Keycode = Keycode(sys::SDL_KeyCode::SDLK_AC_BOOKMARKS as i32);
    pub const BrightnessDown: Keycode = Keycode(sys::SDL_KeyCode::SDLK_BRIGHTNESSDOWN as i32);
    pub const BrightnessUp: Keycode = Keycode(sys::SDL_KeyCode::SDLK_BRIGHTNESSUP as i32);
    pub const DisplaySwitch: Keycode = Keycode(sys::SDL_KeyCode::SDLK_DISPLAYSWITCH as i32);
    pub const KbdIllumToggle: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KBDILLUMTOGGLE as i32);
    pub const KbdIllumDown: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KBDILLUMDOWN as i32);
    pub const KbdIllumUp: Keycode = Keycode(sys::SDL_KeyCode::SDLK_KBDILLUMUP as i32);
    pub const Eject: Keycode = Keycode(sys::SDL_KeyCode::SDLK_EJECT as i32);
    pub const Sleep: Keycode = Keycode(sys::SDL_KeyCode::SDLK_SLEEP as i32);
}

impl Keycode {
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
            let buf = sys::SDL_GetKeyName(self.0);
            CStr::from_ptr(buf as *const _).to_str().unwrap().to_owned()
        }
    }
}
