use crate::sys;
use crate::sys::SDL_JoystickPowerLevel;

use crate::clear_error;
use crate::common::{validate_int, IntegerOrSdlError};
use crate::get_error;
use crate::JoystickSubsystem;
use libc::c_char;
use std::ffi::{CStr, CString, NulError};
use std::fmt::{Display, Error, Formatter};

impl JoystickSubsystem {
    /// Retrieve the total number of attached joysticks *and* controllers identified by SDL.
    #[doc(alias = "SDL_NumJoysticks")]
    pub fn num_joysticks(&self) -> Result<u32, String> {
        let result = unsafe { sys::SDL_NumJoysticks() };

        if result >= 0 {
            Ok(result as u32)
        } else {
            Err(get_error())
        }
    }

    /// Attempt to open the joystick at index `joystick_index` and return it.
    #[doc(alias = "SDL_JoystickOpen")]
    pub fn open(&self, joystick_index: u32) -> Result<Joystick, IntegerOrSdlError> {
        use crate::common::IntegerOrSdlError::*;
        let joystick_index = validate_int(joystick_index, "joystick_index")?;

        let joystick = unsafe { sys::SDL_JoystickOpen(joystick_index) };

        if joystick.is_null() {
            Err(SdlError(get_error()))
        } else {
            Ok(Joystick {
                subsystem: self.clone(),
                raw: joystick,
            })
        }
    }

    /// Return the name of the joystick at index `joystick_index`.
    #[doc(alias = "SDL_JoystickNameForIndex")]
    pub fn name_for_index(&self, joystick_index: u32) -> Result<String, IntegerOrSdlError> {
        use crate::common::IntegerOrSdlError::*;
        let joystick_index = validate_int(joystick_index, "joystick_index")?;

        let c_str = unsafe { sys::SDL_JoystickNameForIndex(joystick_index) };

        if c_str.is_null() {
            Err(SdlError(get_error()))
        } else {
            Ok(unsafe {
                CStr::from_ptr(c_str as *const _)
                    .to_str()
                    .unwrap()
                    .to_string()
            })
        }
    }

    /// Get the GUID for the joystick at index `joystick_index`
    #[doc(alias = "SDL_JoystickGetDeviceGUID")]
    pub fn device_guid(&self, joystick_index: u32) -> Result<Guid, IntegerOrSdlError> {
        use crate::common::IntegerOrSdlError::*;
        let joystick_index = validate_int(joystick_index, "joystick_index")?;

        let raw = unsafe { sys::SDL_JoystickGetDeviceGUID(joystick_index) };

        let guid = Guid { raw };

        if guid.is_zero() {
            Err(SdlError(get_error()))
        } else {
            Ok(guid)
        }
    }

    /// If state is `true` joystick events are processed, otherwise
    /// they're ignored.
    #[doc(alias = "SDL_JoystickEventState")]
    pub fn set_event_state(&self, state: bool) {
        unsafe { sys::SDL_JoystickEventState(state as i32) };
    }

    /// Return `true` if joystick events are processed.
    #[doc(alias = "SDL_JoystickEventState")]
    pub fn event_state(&self) -> bool {
        unsafe { sys::SDL_JoystickEventState(sys::SDL_QUERY as i32) == sys::SDL_ENABLE as i32 }
    }

    /// Force joystick update when not using the event loop
    #[inline]
    #[doc(alias = "SDL_JoystickUpdate")]
    pub fn update(&self) {
        unsafe { sys::SDL_JoystickUpdate() };
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(i32)]
pub enum PowerLevel {
    Unknown = SDL_JoystickPowerLevel::SDL_JOYSTICK_POWER_UNKNOWN as i32,
    Empty = SDL_JoystickPowerLevel::SDL_JOYSTICK_POWER_EMPTY as i32,
    Low = SDL_JoystickPowerLevel::SDL_JOYSTICK_POWER_LOW as i32,
    Medium = SDL_JoystickPowerLevel::SDL_JOYSTICK_POWER_MEDIUM as i32,
    Full = SDL_JoystickPowerLevel::SDL_JOYSTICK_POWER_FULL as i32,
    Wired = SDL_JoystickPowerLevel::SDL_JOYSTICK_POWER_WIRED as i32,
}

impl PowerLevel {
    pub fn from_ll(raw: SDL_JoystickPowerLevel) -> PowerLevel {
        match raw {
            SDL_JoystickPowerLevel::SDL_JOYSTICK_POWER_UNKNOWN => PowerLevel::Unknown,
            SDL_JoystickPowerLevel::SDL_JOYSTICK_POWER_EMPTY => PowerLevel::Empty,
            SDL_JoystickPowerLevel::SDL_JOYSTICK_POWER_LOW => PowerLevel::Low,
            SDL_JoystickPowerLevel::SDL_JOYSTICK_POWER_MEDIUM => PowerLevel::Medium,
            SDL_JoystickPowerLevel::SDL_JOYSTICK_POWER_FULL => PowerLevel::Full,
            SDL_JoystickPowerLevel::SDL_JOYSTICK_POWER_WIRED => PowerLevel::Wired,
            _ => panic!("Unexpected power level: {:?}", raw),
        }
    }

    pub fn to_ll(self) -> SDL_JoystickPowerLevel {
        match self {
            PowerLevel::Unknown => SDL_JoystickPowerLevel::SDL_JOYSTICK_POWER_UNKNOWN,
            PowerLevel::Empty => SDL_JoystickPowerLevel::SDL_JOYSTICK_POWER_EMPTY,
            PowerLevel::Low => SDL_JoystickPowerLevel::SDL_JOYSTICK_POWER_LOW,
            PowerLevel::Medium => SDL_JoystickPowerLevel::SDL_JOYSTICK_POWER_MEDIUM,
            PowerLevel::Full => SDL_JoystickPowerLevel::SDL_JOYSTICK_POWER_FULL,
            PowerLevel::Wired => SDL_JoystickPowerLevel::SDL_JOYSTICK_POWER_WIRED,
        }
    }
}

/// Wrapper around the `SDL_Joystick` object
pub struct Joystick {
    subsystem: JoystickSubsystem,
    raw: *mut sys::SDL_Joystick,
}

impl Joystick {
    #[inline]
    pub const fn subsystem(&self) -> &JoystickSubsystem {
        &self.subsystem
    }

    /// Return the name of the joystick or an empty string if no name
    /// is found.
    #[doc(alias = "SDL_JoystickName")]
    pub fn name(&self) -> String {
        let name = unsafe { sys::SDL_JoystickName(self.raw) };

        c_str_to_string(name)
    }

    /// Return true if the joystick has been opened and currently
    /// connected.
    #[doc(alias = "SDL_JoystickGetAttached")]
    pub fn attached(&self) -> bool {
        unsafe { sys::SDL_JoystickGetAttached(self.raw) != sys::SDL_bool::SDL_FALSE }
    }

    #[doc(alias = "SDL_JoystickInstanceID")]
    pub fn instance_id(&self) -> u32 {
        let result = unsafe { sys::SDL_JoystickInstanceID(self.raw) };

        if result < 0 {
            // Should only fail if the joystick is NULL.
            panic!("{}", get_error())
        } else {
            result as u32
        }
    }

    /// Retrieve the joystick's GUID
    #[doc(alias = "SDL_JoystickGetGUID")]
    pub fn guid(&self) -> Guid {
        let raw = unsafe { sys::SDL_JoystickGetGUID(self.raw) };

        let guid = Guid { raw };

        if guid.is_zero() {
            // Should only fail if the joystick is NULL.
            panic!("{}", get_error())
        } else {
            guid
        }
    }

    /// Retrieve the battery level of this joystick
    #[doc(alias = "SDL_JoystickCurrentPowerLevel")]
    pub fn power_level(&self) -> Result<PowerLevel, IntegerOrSdlError> {
        use crate::common::IntegerOrSdlError::*;
        clear_error();

        let result = unsafe { sys::SDL_JoystickCurrentPowerLevel(self.raw) };

        let state = PowerLevel::from_ll(result);

        if result != SDL_JoystickPowerLevel::SDL_JOYSTICK_POWER_UNKNOWN {
            Ok(state)
        } else {
            let err = get_error();

            if err.is_empty() {
                Ok(state)
            } else {
                Err(SdlError(err))
            }
        }
    }

    /// Retrieve the number of axes for this joystick
    #[doc(alias = "SDL_JoystickNumAxes")]
    pub fn num_axes(&self) -> u32 {
        let result = unsafe { sys::SDL_JoystickNumAxes(self.raw) };

        if result < 0 {
            // Should only fail if the joystick is NULL.
            panic!("{}", get_error())
        } else {
            result as u32
        }
    }

    /// Gets the position of the given `axis`.
    ///
    /// The function will fail if the joystick doesn't have the provided axis.
    #[doc(alias = "SDL_JoystickGetAxis")]
    pub fn axis(&self, axis: u32) -> Result<i16, IntegerOrSdlError> {
        use crate::common::IntegerOrSdlError::*;
        // This interface is a bit messed up: 0 is a valid position
        // but can also mean that an error occured. As far as I can
        // tell the only way to know if an error happened is to see if
        // get_error() returns a non-empty string.
        clear_error();

        let axis = validate_int(axis, "axis")?;
        let pos = unsafe { sys::SDL_JoystickGetAxis(self.raw, axis) };

        if pos != 0 {
            Ok(pos)
        } else {
            let err = get_error();

            if err.is_empty() {
                Ok(pos)
            } else {
                Err(SdlError(err))
            }
        }
    }

    /// Retrieve the number of buttons for this joystick
    #[doc(alias = "SDL_JoystickNumButtons")]
    pub fn num_buttons(&self) -> u32 {
        let result = unsafe { sys::SDL_JoystickNumButtons(self.raw) };

        if result < 0 {
            // Should only fail if the joystick is NULL.
            panic!("{}", get_error())
        } else {
            result as u32
        }
    }

    /// Return `Ok(true)` if `button` is pressed.
    ///
    /// The function will fail if the joystick doesn't have the provided button.
    #[doc(alias = "SDL_JoystickGetButton")]
    pub fn button(&self, button: u32) -> Result<bool, IntegerOrSdlError> {
        use crate::common::IntegerOrSdlError::*;
        // Same deal as axis, 0 can mean both unpressed or
        // error...
        clear_error();

        let button = validate_int(button, "button")?;
        let pressed = unsafe { sys::SDL_JoystickGetButton(self.raw, button) };

        match pressed {
            1 => Ok(true),
            0 => {
                let err = get_error();

                if err.is_empty() {
                    // Button is not pressed
                    Ok(false)
                } else {
                    Err(SdlError(err))
                }
            }
            // Should be unreachable
            _ => unreachable!(),
        }
    }

    /// Retrieve the number of balls for this joystick
    #[doc(alias = "SDL_JoystickNumBalls")]
    pub fn num_balls(&self) -> u32 {
        let result = unsafe { sys::SDL_JoystickNumBalls(self.raw) };

        if result < 0 {
            // Should only fail if the joystick is NULL.
            panic!("{}", get_error())
        } else {
            result as u32
        }
    }

    /// Return a pair `(dx, dy)` containing the difference in axis
    /// position since the last poll
    #[doc(alias = "SDL_JoystickGetBall")]
    pub fn ball(&self, ball: u32) -> Result<(i32, i32), IntegerOrSdlError> {
        use crate::common::IntegerOrSdlError::*;
        let mut dx = 0;
        let mut dy = 0;

        let ball = validate_int(ball, "ball")?;
        let result = unsafe { sys::SDL_JoystickGetBall(self.raw, ball, &mut dx, &mut dy) };

        if result == 0 {
            Ok((dx, dy))
        } else {
            Err(SdlError(get_error()))
        }
    }

    /// Retrieve the number of balls for this joystick
    #[doc(alias = "SDL_JoystickNumHats")]
    pub fn num_hats(&self) -> u32 {
        let result = unsafe { sys::SDL_JoystickNumHats(self.raw) };

        if result < 0 {
            // Should only fail if the joystick is NULL.
            panic!("{}", get_error())
        } else {
            result as u32
        }
    }

    /// Return the position of `hat` for this joystick
    #[doc(alias = "SDL_JoystickGetHat")]
    pub fn hat(&self, hat: u32) -> Result<HatState, IntegerOrSdlError> {
        use crate::common::IntegerOrSdlError::*;
        // Guess what? This function as well uses 0 to report an error
        // but 0 is also a valid value (HatState::Centered). So we
        // have to use the same hack as `axis`...
        clear_error();

        let hat = validate_int(hat, "hat")?;
        let result = unsafe { sys::SDL_JoystickGetHat(self.raw, hat) };

        let state = HatState::from_raw(result as u8);

        if result != 0 {
            Ok(state)
        } else {
            let err = get_error();

            if err.is_empty() {
                Ok(state)
            } else {
                Err(SdlError(err))
            }
        }
    }

    /// Set the rumble motors to their specified intensities, if supported.
    /// Automatically resets back to zero after `duration_ms` milliseconds have passed.
    ///
    /// # Notes
    ///
    /// The value range for the intensities is 0 to 0xFFFF.
    ///
    /// Do *not* use `std::u32::MAX` or similar for `duration_ms` if you want
    /// the rumble effect to keep playing for a long time, as this results in
    /// the effect ending immediately after starting due to an overflow.
    /// Use some smaller, "huge enough" number instead.
    #[doc(alias = "SDL_JoystickRumble")]
    pub fn set_rumble(
        &mut self,
        low_frequency_rumble: u16,
        high_frequency_rumble: u16,
        duration_ms: u32,
    ) -> Result<(), IntegerOrSdlError> {
        let result = unsafe {
            sys::SDL_JoystickRumble(
                self.raw,
                low_frequency_rumble,
                high_frequency_rumble,
                duration_ms,
            )
        };

        if result != 0 {
            Err(IntegerOrSdlError::SdlError(get_error()))
        } else {
            Ok(())
        }
    }

    /// Start a rumble effect in the joystick's triggers.
    #[doc(alias = "SDL_JoystickRumbleTriggers")]
    pub fn set_rumble_triggers(
        &mut self,
        left_rumble: u16,
        right_rumble: u16,
        duration_ms: u32,
    ) -> Result<(), IntegerOrSdlError> {
        let result = unsafe {
            sys::SDL_JoystickRumbleTriggers(self.raw, left_rumble, right_rumble, duration_ms)
        };

        if result != 0 {
            Err(IntegerOrSdlError::SdlError(get_error()))
        } else {
            Ok(())
        }
    }

    /// Query whether a joystick has an LED.
    #[doc(alias = "SDL_JoystickHasLED")]
    pub fn has_led(&self) -> bool {
        let result = unsafe { sys::SDL_JoystickHasLED(self.raw) };

        match result {
            sys::SDL_bool::SDL_FALSE => false,
            sys::SDL_bool::SDL_TRUE => true,
        }
    }

    /// Query whether a joystick has rumble support.
    #[doc(alias = "SDL_JoystickHasRumble")]
    pub fn has_rumble(&self) -> bool {
        let result = unsafe { sys::SDL_JoystickHasRumble(self.raw) };

        match result {
            sys::SDL_bool::SDL_FALSE => false,
            sys::SDL_bool::SDL_TRUE => true,
        }
    }

    /// Query whether a joystick has rumble support on triggers.
    #[doc(alias = "SDL_JoystickHasRumbleTriggers")]
    pub fn has_rumble_triggers(&self) -> bool {
        let result = unsafe { sys::SDL_JoystickHasRumbleTriggers(self.raw) };

        match result {
            sys::SDL_bool::SDL_FALSE => false,
            sys::SDL_bool::SDL_TRUE => true,
        }
    }

    /// Update a joystick's LED color.
    #[doc(alias = "SDL_JoystickSetLED")]
    pub fn set_led(&mut self, red: u8, green: u8, blue: u8) -> Result<(), IntegerOrSdlError> {
        let result = unsafe { sys::SDL_JoystickSetLED(self.raw, red, green, blue) };

        if result != 0 {
            Err(IntegerOrSdlError::SdlError(get_error()))
        } else {
            Ok(())
        }
    }

    /// Send a joystick specific effect packet.
    #[doc(alias = "SDL_JoystickSendEffect")]
    pub fn send_effect(&mut self, data: &[u8]) -> Result<(), IntegerOrSdlError> {
        let result = unsafe {
            sys::SDL_JoystickSendEffect(
                self.raw,
                data.as_ptr() as *const libc::c_void,
                data.len() as i32,
            )
        };

        if result != 0 {
            Err(IntegerOrSdlError::SdlError(get_error()))
        } else {
            Ok(())
        }
    }
}

impl Drop for Joystick {
    #[doc(alias = "SDL_JoystickClose")]
    fn drop(&mut self) {
        if self.attached() {
            unsafe { sys::SDL_JoystickClose(self.raw) }
        }
    }
}

/// Wrapper around a `SDL_JoystickGUID`, a globally unique identifier
/// for a joystick.
#[derive(Copy, Clone)]
pub struct Guid {
    raw: sys::SDL_JoystickGUID,
}

impl PartialEq for Guid {
    fn eq(&self, other: &Guid) -> bool {
        self.raw.data == other.raw.data
    }
}

impl Eq for Guid {}

impl Guid {
    /// Create a GUID from a string representation.
    #[doc(alias = "SDL_JoystickGetGUIDFromString")]
    pub fn from_string(guid: &str) -> Result<Guid, NulError> {
        let guid = CString::new(guid)?;

        let raw = unsafe { sys::SDL_JoystickGetGUIDFromString(guid.as_ptr() as *const c_char) };

        Ok(Guid { raw })
    }

    /// Return `true` if GUID is full 0s
    pub fn is_zero(&self) -> bool {
        for &i in &self.raw.data {
            if i != 0 {
                return false;
            }
        }

        true
    }

    /// Return a String representation of GUID
    #[doc(alias = "SDL_JoystickGetGUIDString")]
    pub fn string(&self) -> String {
        // Doc says "buf should supply at least 33bytes". I took that
        // to mean that 33bytes should be enough in all cases, but
        // maybe I'm wrong?
        let mut buf = [0; 33];

        let len = buf.len() as i32;
        let c_str = buf.as_mut_ptr();

        unsafe {
            sys::SDL_JoystickGetGUIDString(self.raw, c_str, len);
        }

        // The buffer should always be NUL terminated (the
        // documentation doesn't explicitly say it but I checked the
        // code)
        if c_str.is_null() {
            String::new()
        } else {
            unsafe {
                CStr::from_ptr(c_str as *const _)
                    .to_str()
                    .unwrap()
                    .to_string()
            }
        }
    }

    /// Return a copy of the internal SDL_JoystickGUID
    pub fn raw(self) -> sys::SDL_JoystickGUID {
        self.raw
    }
}

impl Display for Guid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.string())
    }
}

/// This is represented in SDL2 as a bitfield but obviously not all
/// combinations make sense: 5 for instance would mean up and down at
/// the same time... To simplify things I turn it into an enum which
/// is how the SDL2 docs present it anyway (using macros).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum HatState {
    Centered = 0,
    Up = 0x01,
    Right = 0x02,
    Down = 0x04,
    Left = 0x08,
    RightUp = 0x02 | 0x01,
    RightDown = 0x02 | 0x04,
    LeftUp = 0x08 | 0x01,
    LeftDown = 0x08 | 0x04,
}

impl HatState {
    pub fn from_raw(raw: u8) -> HatState {
        match raw {
            0 => HatState::Centered,
            1 => HatState::Up,
            2 => HatState::Right,
            4 => HatState::Down,
            8 => HatState::Left,
            3 => HatState::RightUp,
            6 => HatState::RightDown,
            9 => HatState::LeftUp,
            12 => HatState::LeftDown,

            // The Xinput driver on Windows can report hat states on certain hardware that don't
            // make any sense from a gameplay perspective, and so aren't worth putting in the
            // HatState enumeration.
            _ => HatState::Centered,
        }
    }

    pub fn to_raw(self) -> u8 {
        match self {
            HatState::Centered => 0,
            HatState::Up => 1,
            HatState::Right => 2,
            HatState::Down => 4,
            HatState::Left => 8,
            HatState::RightUp => 3,
            HatState::RightDown => 6,
            HatState::LeftUp => 9,
            HatState::LeftDown => 12,
        }
    }
}

/// Convert C string `c_str` to a String. Return an empty string if
/// `c_str` is NULL.
fn c_str_to_string(c_str: *const c_char) -> String {
    if c_str.is_null() {
        String::new()
    } else {
        let bytes = unsafe { CStr::from_ptr(c_str as *const _).to_bytes() };

        String::from_utf8_lossy(bytes).to_string()
    }
}
