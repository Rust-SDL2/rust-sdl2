use sys::joystick as ll;

use SdlResult;
use get_error;
use clear_error;
use sys::event::{SDL_QUERY, SDL_ENABLE};
use std::ffi::{CString, CStr, NulError};
use std::fmt::{Display, Formatter, Error};
use libc::c_char;

/// Retreive the total number of attached joysticks *and* controllers
/// identified by SDL.
pub fn num_joysticks() -> SdlResult<i32> {
    let result = unsafe { ll::SDL_NumJoysticks() };

    if result >= 0 {
        Ok(result)
    } else {
        Err(get_error())
    }
}

/// Get the GUID for the joystick number `id`
pub fn get_device_guid(id: i32) -> SdlResult<Guid> {
    let raw = unsafe { ll::SDL_JoystickGetDeviceGUID(id) };

    let guid = Guid { raw: raw };

    if guid.is_zero() {
        Err(get_error())
    } else {
        Ok(guid)
    }
}

/// If state is `true` joystick events are processed, otherwise
/// they're ignored.
pub fn set_event_state(state: bool) {
    unsafe { ll::SDL_JoystickEventState(state as i32) };
}

/// Return `true` if joystick events are processed.
pub fn get_event_state() -> bool {
    unsafe { ll::SDL_JoystickEventState(SDL_QUERY as i32)
             == SDL_ENABLE as i32 }
}

/// Return the name of the joystick at index `id`
pub fn name_for_index(id: i32) -> SdlResult<String> {
    let name = unsafe { ll::SDL_JoystickNameForIndex(id) };

    c_str_to_string_or_err(name)
}

/// Force joystick update when not using the event loop
pub fn update() {
    unsafe { ll::SDL_JoystickUpdate() };
}

/// Wrapper around the SDL_Joystick object
pub struct Joystick {
    raw: *mut ll::SDL_Joystick,
}

impl Joystick {
    /// Attempt to open the joystick at number `id` and return it.
    pub fn open(id: i32) -> SdlResult<Joystick> {
        let joystick = unsafe { ll::SDL_JoystickOpen(id) };

        if joystick.is_null() {
            Err(get_error())
        } else {
            Ok(Joystick { raw: joystick })
        }
    }

    /// Return the name of the joystick or an empty string if no name
    /// is found.
    pub fn name(&self) -> String {
        let name = unsafe { ll::SDL_JoystickName(self.raw) };

        c_str_to_string(name)
    }

    /// Return true if the joystick has been opened and currently
    /// connected.
    pub fn get_attached(&self) -> bool {
        unsafe { ll::SDL_JoystickGetAttached(self.raw) != 0 }
    }

    pub fn get_instance_id(&self) -> SdlResult<i32> {
        let result = unsafe { ll::SDL_JoystickInstanceID(self.raw) };

        if result < 0 {
            Err(get_error())
        } else {
            Ok(result)
        }
    }

    /// Retreive the joystick's GUID
    pub fn get_guid(&self) -> SdlResult<Guid> {
        let raw = unsafe { ll::SDL_JoystickGetGUID(self.raw) };

        let guid = Guid { raw: raw };

        if guid.is_zero() {
            Err(get_error())
        } else {
            Ok(guid)
        }
    }

    /// Retreive the number of axes for this joystick
    pub fn get_num_axis(&self) -> SdlResult<i32> {
        let result = unsafe { ll::SDL_JoystickNumAxes(self.raw) };

        if result < 0 {
            Err(get_error())
        } else {
            Ok(result)
        }
    }

    /// Get the position of the given `axis`
    pub fn get_axis(&self, axis: i32) -> SdlResult<i16> {
        // This interface is a bit messed up: 0 is a valid position
        // but can also mean that an error occured. As far as I can
        // tell the only way to know if an error happened is to see if
        // get_error() returns a non-empty string.
        clear_error();

        let pos = unsafe { ll::SDL_JoystickGetAxis(self.raw, axis) };

        if pos != 0 {
            Ok(pos)
        } else {
            let err = get_error();

            if err.is_empty() {
                Ok(pos)
            } else {
                Err(err)
            }
        }
    }

    /// Retreive the number of buttons for this joystick
    pub fn get_num_buttons(&self) -> SdlResult<i32> {
        let result = unsafe { ll::SDL_JoystickNumButtons(self.raw) };

        if result < 0 {
            Err(get_error())
        } else {
            Ok(result)
        }
    }

    /// Return `Ok(true)` if `button` is pressed.
    pub fn get_button(&self, button: i32) -> SdlResult<bool> {
        // Same deal as get_axis, 0 can mean both unpressed or
        // error...
        clear_error();

        let pressed =
            unsafe { ll::SDL_JoystickGetButton(self.raw, button) };

        match pressed {
            1 => Ok(true),
            0 => {
                let err = get_error();

                if err.is_empty() {
                    // Button is not pressed
                    Ok(false)
                } else {
                    Err(err)
                }
            }
            // Should be unreachable
            _ => Err(get_error()),
        }
    }

    /// Retreive the number of balls for this joystick
    pub fn get_num_balls(&self) -> SdlResult<i32> {
        let result = unsafe { ll::SDL_JoystickNumBalls(self.raw) };

        if result < 0 {
            Err(get_error())
        } else {
            Ok(result)
        }
    }

    /// Return a pair `(dx, dy)` containing the difference in axis
    /// position since the last poll
    pub fn get_ball(&self, ball: i32) -> SdlResult<(i32, i32)> {
        let mut dx = 0;
        let mut dy = 0;

        let result =
            unsafe {
                ll::SDL_JoystickGetBall(self.raw, ball, &mut dx, &mut dy)
            };

        if result == 0 {
            Ok((dx, dy))
        } else {
            Err(get_error())
        }
    }

    /// Retreive the number of balls for this joystick
    pub fn get_num_hats(&self) -> SdlResult<i32> {
        let result = unsafe { ll::SDL_JoystickNumHats(self.raw) };

        if result < 0 {
            Err(get_error())
        } else {
            Ok(result)
        }
    }

    /// Return the position of `hat` for this joystick
    pub fn get_hat(&self, hat: i32) -> SdlResult<HatState> {
        // Guess what? This function as well uses 0 to report an error
        // but 0 is also a valid value (HatState::Centered). So we
        // have to use the same hack as `get_axis`...
        clear_error();

        let result = unsafe { ll::SDL_JoystickGetHat(self.raw, hat) };

        let state = HatState::from_raw(result as u8);

        if result != 0 {
            Ok(state)
        } else {
            let err = get_error();

            if err.is_empty() {
                Ok(state)
            } else {
                Err(err)
            }
        }
    }
}

impl Drop for Joystick {
    fn drop(&mut self) {
        unsafe { ll::SDL_JoystickClose(self.raw) }
    }
}

/// Wrapper around a SDL_JoystickGUID, a globally unique identifier
/// for a joystick.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Guid {
    raw: ll::SDL_JoystickGUID,
}

impl Guid {
    /// Create a GUID from a string representation.
    pub fn from_string(guid: &str) -> Result<Guid, NulError> {
        let guid = try!(CString::new(guid));

        let raw = unsafe { ll::SDL_JoystickGetGUIDFromString(guid.as_ptr()) };

        Ok(Guid { raw: raw })
    }

    /// Return `true` if GUID is full 0s
    pub fn is_zero(&self) -> bool {
        for &i in self.raw.data.iter() {
            if i != 0 {
                return false;
            }
        }

        return true;
    }

    /// Return a String representation of GUID
    pub fn get_string(&self) -> String {
        // Doc says "buf should supply at least 33bytes". I took that
        // to mean that 33bytes should be enough in all cases, but
        // maybe I'm wrong?
        let mut buf = [0; 33];

        let len   = buf.len() as i32;
        let c_str = buf.as_mut_ptr();

        unsafe {
            ll::SDL_JoystickGetGUIDString(self.raw, c_str, len);
        }

        // The buffer should always be NUL terminated (the
        // documentation doesn't explicitely say it but I checked the
        // code)
        c_str_to_string(c_str)
    }

    /// Return a copy of the internal SDL_JoystickGUID
    pub fn raw(self) -> ll::SDL_JoystickGUID {
        self.raw
    }
}

impl Display for Guid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.get_string())
    }
}

/// This is represented in SDL2 as a bitfield but obviously not all
/// combinations make sense: 5 for instance would mean up and down at
/// the same time... To simplify things I turn it into an enum which
/// is how the SDL2 docs present it anyway (using macros).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum HatState {
    Centered  = 0,
    Up        = 0x01,
    Right     = 0x02,
    Down      = 0x04,
    Left      = 0x08,
    RightUp   = 0x02 | 0x01,
    RightDown = 0x02 | 0x04,
    LeftUp    = 0x08 | 0x01,
    Leftdown  = 0x08 | 0x04,
}

impl HatState {
    pub fn from_raw(raw: u8) -> HatState {
        match raw {
            0  => HatState::Centered,
            1  => HatState::Up,
            2  => HatState::Right,
            4  => HatState::Down,
            8  => HatState::Left,
            3  => HatState::RightUp,
            6  => HatState::RightDown,
            9  => HatState::LeftUp,
            12 => HatState::Leftdown,
            _  => panic!("Unexpected hat position: {}", raw),
        }
    }
}

/// Convert C string `c_str` to a String. Return an empty string if
/// c_str is NULL.
fn c_str_to_string(c_str: *const c_char) -> String {
    if c_str.is_null() {
        String::new()
    } else {
        let bytes = unsafe { CStr::from_ptr(c_str).to_bytes() };

        String::from_utf8_lossy(bytes).to_string()
    }
}

/// Convert C string `c_str` to a String. Return an SDL error if
/// `c_str` is NULL.
fn c_str_to_string_or_err(c_str: *const c_char) -> SdlResult<String> {
    if c_str.is_null() {
        Err(get_error())
    } else {
        let bytes = unsafe { CStr::from_ptr(c_str).to_bytes() };

        Ok(String::from_utf8_lossy(bytes).to_string())
    }
}
