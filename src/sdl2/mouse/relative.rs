use crate::EventPump;

use crate::sys;

use super::{MouseButton, MouseButtonIterator, PressedMouseButtonIterator};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RelativeMouseState {
    mouse_state: u32,
    x: i32,
    y: i32,
}

impl RelativeMouseState {
    #[doc(alias = "SDL_GetRelativeMouseState")]
    pub fn new(_e: &EventPump) -> RelativeMouseState {
        let mut x = 0;
        let mut y = 0;
        let mouse_state = unsafe {
            // This call is the only difference between MouseState
            sys::SDL_GetRelativeMouseState(&mut x, &mut y)
        };

        RelativeMouseState {
            mouse_state,
            x: x as i32,
            y: y as i32,
        }
    }

    pub fn from_sdl_state(state: u32) -> RelativeMouseState {
        RelativeMouseState {
            mouse_state: state,
            x: 0,
            y: 0,
        }
    }
    pub fn to_sdl_state(&self) -> u32 {
        self.mouse_state
    }

    fn button_mask(&self, button: u32) -> u32 {
        1 << (button - 1)
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
    pub fn left(&self) -> bool {
        (self.mouse_state & self.button_mask(sys::SDL_BUTTON_LEFT)) != 0
    }

    /// Tests if the middle mouse button was pressed.
    pub fn middle(&self) -> bool {
        (self.mouse_state & self.button_mask(sys::SDL_BUTTON_MIDDLE)) != 0
    }

    /// Tests if the right mouse button was pressed.
    pub fn right(&self) -> bool {
        (self.mouse_state & self.button_mask(sys::SDL_BUTTON_RIGHT)) != 0
    }

    /// Tests if the X1 mouse button was pressed.
    pub fn x1(&self) -> bool {
        (self.mouse_state & self.button_mask(sys::SDL_BUTTON_X1)) != 0
    }

    /// Tests if the X2 mouse button was pressed.
    pub fn x2(&self) -> bool {
        (self.mouse_state & self.button_mask(sys::SDL_BUTTON_X2)) != 0
    }

    /// Returns the x coordinate of the state
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Returns the y coordinate of the state
    pub fn y(&self) -> i32 {
        self.y
    }

    /// Returns true if the mouse button is pressed.
    ///
    /// # Example
    /// ```no_run
    /// use sdl2::mouse::MouseButton;
    ///
    /// fn is_left_pressed(e: &sdl2::EventPump) -> bool {
    ///     e.mouse_state().is_mouse_button_pressed(MouseButton::Left)
    /// }
    /// ```
    pub fn is_mouse_button_pressed(&self, mouse_button: MouseButton) -> bool {
        let mask = 1 << ((mouse_button as u32) - 1);
        self.mouse_state & mask != 0
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
            cur_button: 1,
            mouse_state: &self.mouse_state,
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
            iter: self.mouse_buttons(),
        }
    }
}
