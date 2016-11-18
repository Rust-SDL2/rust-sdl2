use num::{ToPrimitive, FromPrimitive};
use std::ptr;

use get_error;
use surface::SurfaceRef;
use video;
//
use EventPump;

use sys::mouse as ll;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(u32)]
pub enum SystemCursor {
    Arrow = ll::SDL_SYSTEM_CURSOR_ARROW,
    IBeam = ll::SDL_SYSTEM_CURSOR_IBEAM,
    Wait = ll::SDL_SYSTEM_CURSOR_WAIT,
    Crosshair = ll::SDL_SYSTEM_CURSOR_CROSSHAIR,
    WaitArrow = ll::SDL_SYSTEM_CURSOR_WAITARROW,
    SizeNWSE = ll::SDL_SYSTEM_CURSOR_SIZENWSE,
    SizeNESW = ll::SDL_SYSTEM_CURSOR_SIZENESW,
    SizeWE = ll::SDL_SYSTEM_CURSOR_SIZEWE,
    SizeNS = ll::SDL_SYSTEM_CURSOR_SIZENS,
    SizeAll = ll::SDL_SYSTEM_CURSOR_SIZEALL,
    No = ll::SDL_SYSTEM_CURSOR_NO,
    Hand = ll::SDL_SYSTEM_CURSOR_HAND,
}

pub struct Cursor {
    raw: *mut ll::SDL_Cursor
}

impl Drop for Cursor {
    #[inline]
    fn drop(&mut self) {
        unsafe { ll::SDL_FreeCursor(self.raw) };
    }
}

impl Cursor {
    pub fn new(data: &[u8], mask: &[u8], width: i32, height: i32, hot_x: i32, hot_y: i32) -> Result<Cursor, String> {
        unsafe {
            let raw = ll::SDL_CreateCursor(data.as_ptr(),
                                           mask.as_ptr(),
                                           width as i32, height as i32,
                                           hot_x as i32, hot_y as i32);

            if raw == ptr::null_mut() {
                Err(get_error())
            } else {
                Ok(Cursor{ raw: raw })
            }
        }
    }

    // TODO: figure out how to pass Surface in here correctly
    pub fn from_surface<S: AsRef<SurfaceRef>>(surface: S, hot_x: i32, hot_y: i32) -> Result<Cursor, String> {
        unsafe {
            let raw = ll::SDL_CreateColorCursor(surface.as_ref().raw(), hot_x, hot_y);

            if raw == ptr::null_mut() {
                Err(get_error())
            } else {
                Ok(Cursor{ raw: raw })
            }
        }
    }

    pub fn from_system(cursor: SystemCursor) -> Result<Cursor, String> {
        unsafe {
            let raw = ll::SDL_CreateSystemCursor(cursor as u32);

            if raw == ptr::null_mut() {
                Err(get_error())
            } else {
                Ok(Cursor{ raw: raw })
            }
        }
    }

    pub fn set(&self) {
        unsafe { ll::SDL_SetCursor(self.raw); }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MouseWheelDirection {
    Normal,
    Flipped,
    Unknown(u32),
}

// 0 and 1 are not fixed values in the SDL source code.  This value is defined as an enum which is then cast to a Uint32.
// The enum in C is defined as such:

/**
 * \brief Scroll direction types for the Scroll event
 */
//typedef enum
//{
//    SDL_MOUSEWHEEL_NORMAL,    /**< The scroll direction is normal */
//    SDL_MOUSEWHEEL_FLIPPED    /**< The scroll direction is flipped / natural */
//} SDL_MouseWheelDirection;

// Since no value is given in the enum definition these values are auto assigned by the C compiler starting at 0.
// Normally I would prefer to use the enum rather than hard code what it is implied to represent however
// the mouse wheel direction value could be described equally as well by a bool, so I don't think changes
// to this enum in the C source code are going to be a problem.

impl MouseWheelDirection {
    #[inline]
    pub fn from_ll(direction: u32) -> MouseWheelDirection {
        match direction {
            0 => MouseWheelDirection::Normal,
            1 => MouseWheelDirection::Flipped,
            _ => MouseWheelDirection::Unknown(direction),
        }
    }
    #[inline]
    pub fn to_ll(&self) -> u32 {
        match *self {
            MouseWheelDirection::Normal => 0,
            MouseWheelDirection::Flipped => 1,
            MouseWheelDirection::Unknown(direction) => direction,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Mousebutton {
    Left   = ll::SDL_BUTTON_LEFT as isize,
    Middle = ll::SDL_BUTTON_MIDDLE as isize,
    Right  = ll::SDL_BUTTON_RIGHT as isize,
    X1     = ll::SDL_BUTTON_X1 as isize,
    X2     = ll::SDL_BUTTON_X2 as isize,
}

impl ToPrimitive for Mousebutton {
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        Some(*self as i64)
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        Some(*self as u64)
    }

    #[inline]
    fn to_isize(&self) -> Option<isize> {
        Some(*self as isize)
    }
}

impl FromPrimitive for Mousebutton {
    #[inline]
    fn from_i64(n: i64) -> Option<Mousebutton> { Mousebutton::from_ll(n as u8) }
    #[inline]
    fn from_u64(n: u64) -> Option<Mousebutton> { Mousebutton::from_ll(n as u8) }
}

impl Mousebutton {
    #[inline]
    pub fn from_ll(button: u8) -> Option<Mousebutton> {
        Some(match button {
            ll::SDL_BUTTON_LEFT   => Mousebutton::Left,
            ll::SDL_BUTTON_MIDDLE => Mousebutton::Middle,
            ll::SDL_BUTTON_RIGHT  => Mousebutton::Right,
            ll::SDL_BUTTON_X1     => Mousebutton::X1,
            ll::SDL_BUTTON_X2     => Mousebutton::X2,
            _ => return None,
        })
    }
    #[inline]
    pub fn to_ll(&self) -> Option<u8> {
        Some(*self as u8)
    }

}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MouseState {
    mouse_state: u32,
}

impl MouseState {
    pub fn new(_e: &EventPump) -> MouseState {
        let mouse_state: u32 = unsafe {
            let mut x = 0;
            let mut y = 0;
            ll::SDL_GetMouseState(&mut x, &mut y)
        };

        MouseState {
            mouse_state: mouse_state
        }
    }

    pub fn from_sdl_state(state: u32) -> MouseState {
        MouseState { mouse_state : state }
    }
    pub fn to_sdl_state(&self) -> u32 {
        self.mouse_state
    }

    /// Returns true if the left mouse button is pressed.
    ///
    /// # Example
    /// ```no_run
    /// use sdl2::mouse::Mousebutton;
    ///
    /// fn is_a_pressed(e: &sdl2::EventPump) -> bool {
    ///     e.mouse_state().left()
    /// }
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
    /// use sdl2::mouse::Mousebutton;
    ///
    /// fn is_a_pressed(e: &sdl2::EventPump) -> bool {
    ///     e.mouse_state().is_mousebutton_pressed(Mousebutton::Left)
    /// }
    /// ```
    pub fn is_mousebutton_pressed(&self, mousebutton: Mousebutton) -> bool {
        self.mouse_state<<((mousebutton as u32)-1) != 0
    }

    /// Returns an iterator all scancodes with a boolean indicating if the scancode is pressed.
    pub fn mousebuttons(&self) -> MousebuttonIterator {
        MousebuttonIterator {
            index: 0,
            mouse_state: &self.mouse_state
        }
    }

    /// Returns an iterator of pressed mouse buttons.
    ///
    /// # Example
    /// ```no_run
    /// use sdl2::mouse::Mousebutton;
    /// use std::collections::HashSet;
    ///
    /// fn pressed_mousebutton_set(e: &sdl2::EventPump) -> HashSet<Mousebutton> {
    ///     e.mouse_state().pressed_mousebuttons().collect()
    /// }
    ///
    /// fn newly_pressed(old: &HashSet<Mousebutton>, new: &HashSet<Mousebutton>) -> HashSet<Mousebutton> {
    ///     new - old
    ///     // sugar for: new.difference(old).collect()
    /// }
    /// ```
    pub fn pressed_mousebuttons(&self) -> PressedMousebuttonIterator {
        PressedMousebuttonIterator {
            iter: self.mousebuttons()
        }
    }
}

pub struct MousebuttonIterator<'a> {
    index: usize,
    mouse_state: &'a u32
}

impl<'a> Iterator for MousebuttonIterator<'a> {
    type Item = (Mousebutton, bool);

    fn next(&mut self) -> Option<(Mousebutton, bool)> {
        if self.index < Mousebutton::X2 as usize {
            let index = self.index;
            self.index += 1;

            if let Some(mousebutton) = FromPrimitive::from_usize(index) {
                let pressed = self.mouse_state&(Mousebutton::Middle as u32) != 0;

                Some((mousebutton, pressed))
            } else {
                self.next()
            }
        } else {
            None
        }
    }
}

pub struct PressedMousebuttonIterator<'a> {
    iter: MousebuttonIterator<'a>
}

impl<'a> Iterator for PressedMousebuttonIterator<'a> {
    type Item = Mousebutton;

    fn next(&mut self) -> Option<Mousebutton> {
        while let Some((mousebutton, pressed)) = self.iter.next() {
            if pressed { return Some(mousebutton) }
        }
        None
    }
}

impl ::Sdl {
    #[inline]
    pub fn mouse(&self) -> MouseUtil {
        MouseUtil {
            _sdldrop: self.sdldrop()
        }
    }
}

/// Mouse utility functions. Access with `Sdl::mouse()`.
///
/// ```no_run
/// let sdl_context = sdl2::init().unwrap();
///
/// // Hide the cursor
/// sdl_context.mouse().show_cursor(false);
/// ```
pub struct MouseUtil {
    _sdldrop: ::std::rc::Rc<::SdlDrop>
}

impl MouseUtil {
    /// Gets the id of the window which currently has mouse focus.
    pub fn focused_window_id(&self) -> Option<u32> {
        let raw = unsafe { ll::SDL_GetMouseFocus() };
        if raw == ptr::null_mut() {
            None
        } else {
            let id = unsafe { ::sys::video::SDL_GetWindowID(raw) };
            Some(id)
        }
    }

    pub fn mouse_state(&self) -> (MouseState, i32, i32) {
        let mut x = 0;
        let mut y = 0;
        unsafe {
            let raw = ll::SDL_GetMouseState(&mut x, &mut y);
            return (MouseState::from_sdl_state(raw), x as i32, y as i32);
        }
    }

    pub fn relative_mouse_state(&self) -> (MouseState, i32, i32) {
        let mut x = 0;
        let mut y = 0;
        unsafe {
            let raw = ll::SDL_GetRelativeMouseState(&mut x, &mut y);
            return (MouseState::from_sdl_state(raw), x as i32, y as i32);
        }
    }

    pub fn warp_mouse_in_window(&self, window: &video::WindowRef, x: i32, y: i32) {
        unsafe { ll::SDL_WarpMouseInWindow(window.raw(), x, y); }
    }

    pub fn set_relative_mouse_mode(&self, on: bool) {
        unsafe { ll::SDL_SetRelativeMouseMode(on as i32); }
    }

    pub fn relative_mouse_mode(&self) -> bool {
        unsafe { ll::SDL_GetRelativeMouseMode() == 1 }
    }

    pub fn is_cursor_showing(&self) -> bool {
        unsafe { ll::SDL_ShowCursor(::sys::SDL_QUERY) == 1 }
    }

    pub fn show_cursor(&self, show: bool) {
        unsafe { ll::SDL_ShowCursor(show as i32); }
    }
}
