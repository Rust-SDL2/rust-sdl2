use num::{ToPrimitive, FromPrimitive};
use std::ptr;

use Sdl;
use rect::Rect;
use video::Window;

use sys::keyboard as ll;

mod keycode;
mod scancode;
pub use self::keycode::Keycode;
pub use self::scancode::Scancode;

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

pub fn get_focused_window_id() -> Option<u32> {
    let raw = unsafe { ll::SDL_GetKeyboardFocus() };
    if raw == ptr::null_mut() {
        None
    } else {
        let id = unsafe { ::sys::video::SDL_GetWindowID(raw) };
        Some(id)
    }
}

pub struct KeyboardState<'sdl> {
    keyboard_state: &'sdl [u8]
}

impl<'sdl> KeyboardState<'sdl> {
    pub fn new(_sdl: &Sdl) -> KeyboardState {
        let keyboard_state = unsafe {
            let mut count = 0;
            let state_ptr = ll::SDL_GetKeyboardState(&mut count);

            ::std::slice::from_raw_parts(state_ptr, count as usize)
        };

        KeyboardState {
            keyboard_state: keyboard_state
        }
    }

    /// Returns true if the scancode is pressed.
    ///
    /// # Example
    /// ```no_run
    /// use sdl2::keyboard::Scancode;
    ///
    /// fn is_a_pressed(sdl_context: &mut sdl2::Sdl) -> bool {
    ///     sdl_context.keyboard_state().is_scancode_pressed(Scancode::A)
    /// }
    /// ```
    pub fn is_scancode_pressed(&self, scancode: Scancode) -> bool {
        self.keyboard_state[ToPrimitive::to_isize(&scancode).unwrap() as usize] != 0
    }

    /// Returns an iterator all scancodes with a boolean indicating if the scancode is pressed.
    pub fn scancodes(&self) -> ScancodeIterator {
        ScancodeIterator {
            index: 0,
            keyboard_state: self.keyboard_state
        }
    }

    /// Returns an iterator of pressed scancodes.
    ///
    /// # Example
    /// ```no_run
    /// use sdl2::keyboard::Keycode;
    /// use sdl2::keyboard::Scancode;
    /// use std::collections::HashSet;
    ///
    /// fn pressed_scancode_set(sdl_context: &sdl2::Sdl) -> HashSet<Scancode> {
    ///     sdl_context.keyboard_state().pressed_scancodes().collect()
    /// }
    ///
    /// fn pressed_keycode_set(sdl_context: &sdl2::Sdl) -> HashSet<Keycode> {
    ///     sdl_context.keyboard_state().pressed_scancodes()
    ///         .filter_map(Keycode::from_scancode)
    ///         .collect()
    /// }
    ///
    /// fn newly_pressed(old: &HashSet<Scancode>, new: &HashSet<Scancode>) -> HashSet<Scancode> {
    ///     new - old
    ///     // sugar for: new.difference(old).collect()
    /// }
    /// ```
    pub fn pressed_scancodes(&self) -> PressedScancodeIterator {
        PressedScancodeIterator {
            iter: self.scancodes()
        }
    }
}

pub struct ScancodeIterator<'a> {
    index: usize,
    keyboard_state: &'a [u8]
}

impl<'a> Iterator for ScancodeIterator<'a> {
    type Item = (Scancode, bool);

    fn next(&mut self) -> Option<(Scancode, bool)> {
        if self.index < self.keyboard_state.len() {
            let index = self.index;
            self.index += 1;

            if let Some(scancode) = FromPrimitive::from_usize(index) {
                let pressed = self.keyboard_state[index] != 0;

                Some((scancode, pressed))
            } else {
                self.next()
            }
        } else {
            None
        }
    }
}

pub struct PressedScancodeIterator<'a> {
    iter: ScancodeIterator<'a>
}

impl<'a> Iterator for PressedScancodeIterator<'a> {
    type Item = Scancode;

    fn next(&mut self) -> Option<Scancode> {
        while let Some((scancode, pressed)) = self.iter.next() {
            if pressed { return Some(scancode) }
        }

        None
    }
}

pub fn mod_state() -> Mod {
    unsafe { Mod::from_bits(ll::SDL_GetModState()).unwrap() }
}

pub fn set_mod_state(flags: Mod) {
    unsafe { ll::SDL_SetModState(flags.bits()); }
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
    unsafe { ll::SDL_SetTextInputRect(rect.raw()); }
}

pub fn has_screen_keyboard_support() -> bool {
    unsafe { ll::SDL_HasScreenKeyboardSupport() == 1 }
}

pub fn is_screen_keyboard_shown(window: &Window) -> bool {
    unsafe { ll::SDL_IsScreenKeyboardShown(window.raw()) == 1 }
}
