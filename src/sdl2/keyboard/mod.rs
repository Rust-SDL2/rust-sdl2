use num::{ToPrimitive, FromPrimitive};
use std::ptr;

use EventPump;
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

pub struct KeyboardState<'a> {
    keyboard_state: &'a [u8]
}

impl<'a> KeyboardState<'a> {
    pub fn new(_e: &'a EventPump) -> KeyboardState<'a> {
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
    /// fn is_a_pressed(e: &sdl2::EventPump) -> bool {
    ///     e.keyboard_state().is_scancode_pressed(Scancode::A)
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
    /// fn pressed_scancode_set(e: &sdl2::EventPump) -> HashSet<Scancode> {
    ///     e.keyboard_state().pressed_scancodes().collect()
    /// }
    ///
    /// fn pressed_keycode_set(e: &sdl2::EventPump) -> HashSet<Keycode> {
    ///     e.keyboard_state().pressed_scancodes()
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

impl ::Sdl {
    #[inline]
    pub fn keyboard(&self) -> KeyboardUtil {
        KeyboardUtil {
            _sdldrop: self.sdldrop()
        }
    }
}

impl ::VideoSubsystem {
    #[inline]
    pub fn text_input(&self) -> TextInputUtil {
        TextInputUtil {
            _subsystem: self.clone()
        }
    }
}

/// Keyboard utility functions. Access with `Sdl::keyboard()`.
///
/// ```no_run
/// let sdl_context = sdl2::init().unwrap();
///
/// let focused = sdl_context.keyboard().focused_window_id().is_some();
/// ```
pub struct KeyboardUtil {
    _sdldrop: ::std::rc::Rc<::SdlDrop>
}

impl KeyboardUtil {
    /// Gets the id of the window which currently has keyboard focus.
    pub fn focused_window_id(&self) -> Option<u32> {
        let raw = unsafe { ll::SDL_GetKeyboardFocus() };
        if raw == ptr::null_mut() {
            None
        } else {
            let id = unsafe { ::sys::video::SDL_GetWindowID(raw) };
            Some(id)
        }
    }

    pub fn mod_state(&self) -> Mod {
        unsafe { Mod::from_bits(ll::SDL_GetModState()).unwrap() }
    }

    pub fn set_mod_state(&self, flags: Mod) {
        unsafe { ll::SDL_SetModState(flags.bits()); }
    }
}

/// Text input utility functions. Access with `VideoSubsystem::text_input()`.
///
/// These functions require the video subsystem to be initialized and are not thread-safe.
///
/// ```no_run
/// let sdl_context = sdl2::init().unwrap();
/// let video_subsystem = sdl_context.video().unwrap();
///
/// // Start accepting text input events...
/// video_subsystem.text_input().start();
/// ```
pub struct TextInputUtil {
    _subsystem: ::VideoSubsystem
}

impl TextInputUtil {
    pub fn start(&self) {
        unsafe { ll::SDL_StartTextInput(); }
    }

    pub fn is_active(&self, ) -> bool {
        unsafe { ll::SDL_IsTextInputActive() == 1 }
    }

    pub fn stop(&self) {
        unsafe { ll::SDL_StopTextInput(); }
    }

    pub fn set_rect(&self, rect: &Rect) {
        unsafe { ll::SDL_SetTextInputRect(rect.raw()); }
    }

    pub fn has_screen_keyboard_support(&self) -> bool {
        unsafe { ll::SDL_HasScreenKeyboardSupport() == 1 }
    }

    pub fn is_screen_keyboard_shown(&self, window: &Window) -> bool {
        unsafe { ll::SDL_IsScreenKeyboardShown(window.raw()) == 1 }
    }
}
