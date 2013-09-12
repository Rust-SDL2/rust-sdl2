
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
        scancode: SDL_Scancode,
        sym: SDL_Keycode,
        _mod: uint16_t,
        unused: uint32_t,
    }

    externfn!(fn SDL_GetKeyboardFocus() -> *SDL_Window)
    externfn!(fn SDL_GetKeyboardState(numkeys: *c_int) -> *uint8_t)
    externfn!(fn SDL_GetModState() -> SDL_Keymod)
    externfn!(fn SDL_SetModState(modstate: SDL_Keymod))
    externfn!(fn SDL_GetKeyFromScancode(scancode: SDL_Scancode) -> SDL_Keycode)
    externfn!(fn SDL_GetScancodeFromKey(key: SDL_Keycode) -> SDL_Scancode)
    externfn!(fn SDL_GetScancodeName(scancode: SDL_Scancode) -> *c_schar)
    externfn!(fn SDL_GetScancodeFromName(name: *c_schar) -> SDL_Scancode)
    externfn!(fn SDL_GetKeyName(key: SDL_Keycode) -> *c_schar)
    externfn!(fn SDL_GetKeyFromName(name: *c_schar) -> SDL_Keycode)
    externfn!(fn SDL_StartTextInput())
    externfn!(fn SDL_IsTextInputActive() -> SDL_bool)
    externfn!(fn SDL_StopTextInput())
    externfn!(fn SDL_SetTextInputRect(rect: *SDL_Rect))
    externfn!(fn SDL_HasScreenKeyboardSupport() -> SDL_bool)
    externfn!(fn SDL_IsScreenKeyboardShown(window: *SDL_Window) -> SDL_bool)
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
