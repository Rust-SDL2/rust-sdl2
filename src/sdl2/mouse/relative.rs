use EventPump;

use sys::mouse as ll;

use super::{MouseButton, MouseButtonIterator, PressedMouseButtonIterator};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RelativeMouseState {
    mouse_state: u32,
    x: u32,
    y: u32
}

impl RelativeMouseState {
    pub fn new(_e: &EventPump) -> RelativeMouseState {
        let mut x = 0;
        let mut y = 0;
        let mouse_state = unsafe {
            // This call is the only difference between MouseState
            ll::SDL_GetRelativeMouseState(&mut x, &mut y)
        };

        RelativeMouseState {
            mouse_state: mouse_state,
            x: x as u32,
            y: y as u32
        }
    }

    pub fn from_sdl_state(state: u32) -> RelativeMouseState {
        RelativeMouseState { mouse_state : state, x: 0, y: 0 }
    }
    pub fn to_sdl_state(&self) -> u32 {
        self.mouse_state
    }

    /// Returns true if the left mouse button is pressed.
    ///
    /// # Example
    /// ```no_run
    /// use sdl2::mouse::MouseButton;
    ///
    /// fn is_a_pressed(e: &sdl2::EventPump) -> bool {
    ///     e.mouse_state().left()
    /// }
    /// ```
    pub fn left(&self) -> bool { (self.mouse_state & ll::SDL_BUTTON_LMASK) != 0 }

    /// Tests if the middle mouse button was pressed.
    pub fn middle(&self) -> bool { (self.mouse_state & ll::SDL_BUTTON_MMASK) != 0 }

    /// Tests if the right mouse button was pressed.
    pub fn right(&self) -> bool { (self.mouse_state & ll::SDL_BUTTON_RMASK) != 0 }

    /// Tests if the X1 mouse button was pressed.
    pub fn x1(&self) -> bool { (self.mouse_state & ll::SDL_BUTTON_X1MASK) != 0 }

    /// Tests if the X2 mouse button was pressed.
    pub fn x2(&self) -> bool { (self.mouse_state & ll::SDL_BUTTON_X2MASK) != 0 }

    /// Returns true if the mouse button is pressed.
    ///
    /// # Example
    /// ```no_run
    /// use sdl2::mouse::MouseButton;
    ///
    /// fn is_a_pressed(e: &sdl2::EventPump) -> bool {
    ///     e.mouse_state().is_mouse_button_pressed(MouseButton::Left)
    /// }
    /// ```
    pub fn is_mouse_button_pressed(&self, mouse_button: MouseButton) -> bool {
        self.mouse_state<<((mouse_button as u32)-1) != 0
    }

    /// Returns an iterator all mouse buttons with a boolean indicating if the scancode is pressed.
    ///
    /// # Example
    /// ```no_run
    /// use sdl2::mouse::MouseButton;
    /// use std::collections::HashMap;
    ///
    /// fn mouse_button_set(e: &sdl2::EventPump) -> HashMap<MouseButton, bool> {
    ///     e.mouse_state().mouse_buttons().collect()
    /// }
    ///
    /// fn find_first_pressed(e: &sdl2::EventPump) -> bool {
    ///     for (key,value) in mouse_button_set(e) {
    ///         return value != false
    ///     }
    ///     false
    /// }
    ///
    /// ```
    pub fn mouse_buttons(&self) -> MouseButtonIterator {
        MouseButtonIterator {
            index: 0,
            mouse_state: &self.mouse_state
        }
    }

    /// Returns an iterator of pressed mouse buttons.
    ///
    /// # Example
    /// ```no_run
    /// use sdl2::mouse::MouseButton;
    /// use std::collections::HashSet;
    ///
    /// fn pressed_mouse_button_set(e: &sdl2::EventPump) -> HashSet<MouseButton> {
    ///     e.mouse_state().pressed_mouse_buttons().collect()
    /// }
    ///
    /// fn newly_pressed(old: &HashSet<MouseButton>, new: &HashSet<MouseButton>) -> HashSet<MouseButton> {
    ///     new - old
    ///     // sugar for: new.difference(old).collect()
    /// }
    /// ```
    pub fn pressed_mouse_buttons(&self) -> PressedMouseButtonIterator {
        PressedMouseButtonIterator {
            iter: self.mouse_buttons()
        }
    }
}
