use crate::rwops::RWops;
use libc::c_char;
use std::error;
use std::ffi::{CStr, CString, NulError};
use std::fmt;
use std::io;
use std::path::Path;

#[cfg(feature = "hidapi")]
use crate::sensor::SensorType;
#[cfg(feature = "hidapi")]
use std::convert::TryInto;

use crate::common::{validate_int, IntegerOrSdlError};
use crate::get_error;
use crate::joystick;
use crate::GameControllerSubsystem;
use std::mem::transmute;

use crate::sys;

#[derive(Debug, Clone)]
pub enum AddMappingError {
    InvalidMapping(NulError),
    InvalidFilePath(String),
    ReadError(String),
    SdlError(String),
}

impl fmt::Display for AddMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::AddMappingError::*;

        match *self {
            InvalidMapping(ref e) => write!(f, "Null error: {}", e),
            InvalidFilePath(ref value) => write!(f, "Invalid file path ({})", value),
            ReadError(ref e) => write!(f, "Read error: {}", e),
            SdlError(ref e) => write!(f, "SDL error: {}", e),
        }
    }
}

impl error::Error for AddMappingError {
    fn description(&self) -> &str {
        use self::AddMappingError::*;

        match *self {
            InvalidMapping(_) => "invalid mapping",
            InvalidFilePath(_) => "invalid file path",
            ReadError(_) => "read error",
            SdlError(ref e) => e,
        }
    }
}

impl GameControllerSubsystem {
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

    /// Return true if the joystick at index `joystick_index` is a game controller.
    #[inline]
    #[doc(alias = "SDL_IsGameController")]
    pub fn is_game_controller(&self, joystick_index: u32) -> bool {
        match validate_int(joystick_index, "joystick_index") {
            Ok(joystick_index) => unsafe {
                sys::SDL_IsGameController(joystick_index) != sys::SDL_bool::SDL_FALSE
            },
            Err(_) => false,
        }
    }

    /// Attempt to open the controller at index `joystick_index` and return it.
    /// Controller IDs are the same as joystick IDs and the maximum number can
    /// be retrieved using the `SDL_NumJoysticks` function.
    #[doc(alias = "SDL_GameControllerOpen")]
    pub fn open(&self, joystick_index: u32) -> Result<GameController, IntegerOrSdlError> {
        use crate::common::IntegerOrSdlError::*;
        let joystick_index = validate_int(joystick_index, "joystick_index")?;
        let controller = unsafe { sys::SDL_GameControllerOpen(joystick_index) };

        if controller.is_null() {
            Err(SdlError(get_error()))
        } else {
            Ok(GameController {
                subsystem: self.clone(),
                raw: controller,
            })
        }
    }

    /// Return the name of the controller at index `joystick_index`.
    #[doc(alias = "SDL_GameControllerNameForIndex")]
    pub fn name_for_index(&self, joystick_index: u32) -> Result<String, IntegerOrSdlError> {
        use crate::common::IntegerOrSdlError::*;
        let joystick_index = validate_int(joystick_index, "joystick_index")?;
        let c_str = unsafe { sys::SDL_GameControllerNameForIndex(joystick_index) };

        if c_str.is_null() {
            Err(SdlError(get_error()))
        } else {
            Ok(unsafe {
                CStr::from_ptr(c_str as *const _)
                    .to_str()
                    .unwrap()
                    .to_owned()
            })
        }
    }

    /// If state is `true` controller events are processed, otherwise
    /// they're ignored.
    #[doc(alias = "SDL_GameControllerEventState")]
    pub fn set_event_state(&self, state: bool) {
        unsafe { sys::SDL_GameControllerEventState(state as i32) };
    }

    /// Return `true` if controller events are processed.
    #[doc(alias = "SDL_GameControllerEventState")]
    pub fn event_state(&self) -> bool {
        unsafe {
            sys::SDL_GameControllerEventState(sys::SDL_QUERY as i32) == sys::SDL_ENABLE as i32
        }
    }

    /// Add a new controller input mapping from a mapping string.
    #[doc(alias = "SDL_GameControllerAddMapping")]
    pub fn add_mapping(&self, mapping: &str) -> Result<MappingStatus, AddMappingError> {
        use self::AddMappingError::*;
        let mapping = match CString::new(mapping) {
            Ok(s) => s,
            Err(err) => return Err(InvalidMapping(err)),
        };

        let result =
            unsafe { sys::SDL_GameControllerAddMapping(mapping.as_ptr() as *const c_char) };

        match result {
            1 => Ok(MappingStatus::Added),
            0 => Ok(MappingStatus::Updated),
            _ => Err(SdlError(get_error())),
        }
    }

    /// Load controller input mappings from a file.
    pub fn load_mappings<P: AsRef<Path>>(&self, path: P) -> Result<i32, AddMappingError> {
        use self::AddMappingError::*;

        let rw = RWops::from_file(path, "r").map_err(InvalidFilePath)?;
        self.load_mappings_from_rw(rw)
    }

    /// Load controller input mappings from a [`Read`](std::io::Read) object.
    pub fn load_mappings_from_read<R: io::Read>(
        &self,
        read: &mut R,
    ) -> Result<i32, AddMappingError> {
        use self::AddMappingError::*;

        let mut buffer = Vec::with_capacity(1024);
        let rw = RWops::from_read(read, &mut buffer).map_err(ReadError)?;
        self.load_mappings_from_rw(rw)
    }

    /// Load controller input mappings from an SDL [`RWops`] object.
    #[doc(alias = "SDL_GameControllerAddMappingsFromRW")]
    pub fn load_mappings_from_rw<'a>(&self, rw: RWops<'a>) -> Result<i32, AddMappingError> {
        use self::AddMappingError::*;

        let result = unsafe { sys::SDL_GameControllerAddMappingsFromRW(rw.raw(), 0) };
        match result {
            -1 => Err(SdlError(get_error())),
            _ => Ok(result),
        }
    }

    #[doc(alias = "SDL_GameControllerMappingForGUID")]
    pub fn mapping_for_guid(&self, guid: joystick::Guid) -> Result<String, String> {
        let c_str = unsafe { sys::SDL_GameControllerMappingForGUID(guid.raw()) };

        c_str_to_string_or_err(c_str)
    }

    #[inline]
    /// Force controller update when not using the event loop
    #[doc(alias = "SDL_GameControllerUpdate")]
    pub fn update(&self) {
        unsafe { sys::SDL_GameControllerUpdate() };
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(i32)]
pub enum Axis {
    LeftX = sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_LEFTX as i32,
    LeftY = sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_LEFTY as i32,
    RightX = sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_RIGHTX as i32,
    RightY = sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_RIGHTY as i32,
    TriggerLeft = sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_TRIGGERLEFT as i32,
    TriggerRight = sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_TRIGGERRIGHT as i32,
}

impl Axis {
    /// Return the Axis from a string description in the same format
    /// used by the game controller mapping strings.
    #[doc(alias = "SDL_GameControllerGetAxisFromString")]
    pub fn from_string(axis: &str) -> Option<Axis> {
        let id = match CString::new(axis) {
            Ok(axis) => unsafe {
                sys::SDL_GameControllerGetAxisFromString(axis.as_ptr() as *const c_char)
            },
            // string contains a nul byte - it won't match anything.
            Err(_) => sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_INVALID,
        };

        Axis::from_ll(id)
    }

    /// Return a string for a given axis in the same format using by
    /// the game controller mapping strings
    #[doc(alias = "SDL_GameControllerGetStringForAxis")]
    pub fn string(self) -> String {
        let axis: sys::SDL_GameControllerAxis;
        unsafe {
            axis = transmute(self);
        }

        let string = unsafe { sys::SDL_GameControllerGetStringForAxis(axis) };

        c_str_to_string(string)
    }

    pub fn from_ll(bitflags: sys::SDL_GameControllerAxis) -> Option<Axis> {
        Some(match bitflags {
            sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_INVALID => return None,
            sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_LEFTX => Axis::LeftX,
            sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_LEFTY => Axis::LeftY,
            sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_RIGHTX => Axis::RightX,
            sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_RIGHTY => Axis::RightY,
            sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_TRIGGERLEFT => Axis::TriggerLeft,
            sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_TRIGGERRIGHT => Axis::TriggerRight,
            _ => return None,
        })
    }

    pub fn to_ll(self) -> sys::SDL_GameControllerAxis {
        match self {
            Axis::LeftX => sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_LEFTX,
            Axis::LeftY => sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_LEFTY,
            Axis::RightX => sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_RIGHTX,
            Axis::RightY => sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_RIGHTY,
            Axis::TriggerLeft => sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_TRIGGERLEFT,
            Axis::TriggerRight => sys::SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_TRIGGERRIGHT,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(i32)]
pub enum Button {
    A = sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_A as i32,
    B = sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_B as i32,
    X = sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_X as i32,
    Y = sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_Y as i32,
    Back = sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_BACK as i32,
    Guide = sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_GUIDE as i32,
    Start = sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_START as i32,
    LeftStick = sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_LEFTSTICK as i32,
    RightStick = sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_RIGHTSTICK as i32,
    LeftShoulder = sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_LEFTSHOULDER as i32,
    RightShoulder = sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_RIGHTSHOULDER as i32,
    DPadUp = sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_DPAD_UP as i32,
    DPadDown = sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_DPAD_DOWN as i32,
    DPadLeft = sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_DPAD_LEFT as i32,
    DPadRight = sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_DPAD_RIGHT as i32,
    Misc1 = sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_MISC1 as i32,
    Paddle1 = sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_PADDLE1 as i32,
    Paddle2 = sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_PADDLE2 as i32,
    Paddle3 = sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_PADDLE3 as i32,
    Paddle4 = sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_PADDLE4 as i32,
    Touchpad = sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_TOUCHPAD as i32,
}

impl Button {
    /// Return the Button from a string description in the same format
    /// used by the game controller mapping strings.
    #[doc(alias = "SDL_GameControllerGetButtonFromString")]
    pub fn from_string(button: &str) -> Option<Button> {
        let id = match CString::new(button) {
            Ok(button) => unsafe {
                sys::SDL_GameControllerGetButtonFromString(button.as_ptr() as *const c_char)
            },
            // string contains a nul byte - it won't match anything.
            Err(_) => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_INVALID,
        };

        Button::from_ll(id)
    }

    /// Return a string for a given button in the same format using by
    /// the game controller mapping strings
    #[doc(alias = "SDL_GameControllerGetStringForButton")]
    pub fn string(self) -> String {
        let button: sys::SDL_GameControllerButton;
        unsafe {
            button = transmute(self);
        }

        let string = unsafe { sys::SDL_GameControllerGetStringForButton(button) };

        c_str_to_string(string)
    }

    pub fn from_ll(bitflags: sys::SDL_GameControllerButton) -> Option<Button> {
        Some(match bitflags {
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_INVALID => return None,
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_A => Button::A,
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_B => Button::B,
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_X => Button::X,
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_Y => Button::Y,
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_BACK => Button::Back,
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_GUIDE => Button::Guide,
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_START => Button::Start,
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_LEFTSTICK => Button::LeftStick,
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_RIGHTSTICK => Button::RightStick,
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_LEFTSHOULDER => {
                Button::LeftShoulder
            }
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_RIGHTSHOULDER => {
                Button::RightShoulder
            }
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_DPAD_UP => Button::DPadUp,
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_DPAD_DOWN => Button::DPadDown,
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_DPAD_LEFT => Button::DPadLeft,
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_DPAD_RIGHT => Button::DPadRight,
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_MISC1 => Button::Misc1,
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_PADDLE1 => Button::Paddle1,
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_PADDLE2 => Button::Paddle2,
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_PADDLE3 => Button::Paddle3,
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_PADDLE4 => Button::Paddle4,
            sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_TOUCHPAD => Button::Touchpad,
            _ => return None,
        })
    }

    pub fn to_ll(self) -> sys::SDL_GameControllerButton {
        match self {
            Button::A => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_A,
            Button::B => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_B,
            Button::X => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_X,
            Button::Y => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_Y,
            Button::Back => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_BACK,
            Button::Guide => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_GUIDE,
            Button::Start => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_START,
            Button::LeftStick => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_LEFTSTICK,
            Button::RightStick => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_RIGHTSTICK,
            Button::LeftShoulder => {
                sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_LEFTSHOULDER
            }
            Button::RightShoulder => {
                sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_RIGHTSHOULDER
            }
            Button::DPadUp => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_DPAD_UP,
            Button::DPadDown => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_DPAD_DOWN,
            Button::DPadLeft => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_DPAD_LEFT,
            Button::DPadRight => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_DPAD_RIGHT,
            Button::Misc1 => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_MISC1,
            Button::Paddle1 => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_PADDLE1,
            Button::Paddle2 => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_PADDLE2,
            Button::Paddle3 => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_PADDLE3,
            Button::Paddle4 => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_PADDLE4,
            Button::Touchpad => sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_TOUCHPAD,
        }
    }
}

/// Possible return values for `add_mapping`
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MappingStatus {
    Added = 1,
    Updated = 0,
}

/// Wrapper around the `SDL_GameController` object
pub struct GameController {
    subsystem: GameControllerSubsystem,
    raw: *mut sys::SDL_GameController,
}

impl GameController {
    #[inline]
    pub fn subsystem(&self) -> &GameControllerSubsystem {
        &self.subsystem
    }

    /// Return the name of the controller or an empty string if no
    /// name is found.
    #[doc(alias = "SDL_GameControllerName")]
    pub fn name(&self) -> String {
        let name = unsafe { sys::SDL_GameControllerName(self.raw) };

        c_str_to_string(name)
    }

    /// Return a String describing the controller's button and axis
    /// mappings
    #[doc(alias = "SDL_GameControllerMapping")]
    pub fn mapping(&self) -> String {
        let mapping = unsafe { sys::SDL_GameControllerMapping(self.raw) };

        c_str_to_string(mapping)
    }

    /// Return true if the controller has been opened and currently
    /// connected.
    #[doc(alias = "SDL_GameControllerGetAttached")]
    pub fn attached(&self) -> bool {
        unsafe { sys::SDL_GameControllerGetAttached(self.raw) != sys::SDL_bool::SDL_FALSE }
    }

    /// Return the joystick instance id of this controller
    #[doc(alias = "SDL_GameControllerGetJoystick")]
    pub fn instance_id(&self) -> u32 {
        let result = unsafe {
            let joystick = sys::SDL_GameControllerGetJoystick(self.raw);
            sys::SDL_JoystickInstanceID(joystick)
        };

        if result < 0 {
            // Should only fail if the joystick is NULL.
            panic!("{}", get_error())
        } else {
            result as u32
        }
    }

    /// Get the position of the given `axis`
    #[doc(alias = "SDL_GameControllerGetAxis")]
    pub fn axis(&self, axis: Axis) -> i16 {
        // This interface is a bit messed up: 0 is a valid position
        // but can also mean that an error occured.
        // Fortunately, an error can only occur if the controller pointer is NULL.
        // There should be no apparent reason for this to change in the future.

        let raw_axis: sys::SDL_GameControllerAxis;
        unsafe {
            raw_axis = transmute(axis);
        }

        unsafe { sys::SDL_GameControllerGetAxis(self.raw, raw_axis) }
    }

    /// Returns `true` if `button` is pressed.
    #[doc(alias = "SDL_GameControllerGetButton")]
    pub fn button(&self, button: Button) -> bool {
        // This interface is a bit messed up: 0 is a valid position
        // but can also mean that an error occured.
        // Fortunately, an error can only occur if the controller pointer is NULL.
        // There should be no apparent reason for this to change in the future.

        let raw_button: sys::SDL_GameControllerButton;
        unsafe {
            raw_button = transmute(button);
        }

        unsafe { sys::SDL_GameControllerGetButton(self.raw, raw_button) != 0 }
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
    #[doc(alias = "SDL_GameControllerRumble")]
    pub fn set_rumble(
        &mut self,
        low_frequency_rumble: u16,
        high_frequency_rumble: u16,
        duration_ms: u32,
    ) -> Result<(), IntegerOrSdlError> {
        let result = unsafe {
            sys::SDL_GameControllerRumble(
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

    /// Start a rumble effect in the game controller's triggers.
    #[doc(alias = "SDL_GameControllerRumbleTriggers")]
    pub fn set_rumble_triggers(
        &mut self,
        left_rumble: u16,
        right_rumble: u16,
        duration_ms: u32,
    ) -> Result<(), IntegerOrSdlError> {
        let result = unsafe {
            sys::SDL_GameControllerRumbleTriggers(self.raw, left_rumble, right_rumble, duration_ms)
        };

        if result != 0 {
            Err(IntegerOrSdlError::SdlError(get_error()))
        } else {
            Ok(())
        }
    }

    /// Query whether a game controller has an LED.
    #[doc(alias = "SDL_GameControllerHasLED")]
    pub fn has_led(&self) -> bool {
        let result = unsafe { sys::SDL_GameControllerHasLED(self.raw) };

        match result {
            sys::SDL_bool::SDL_FALSE => false,
            sys::SDL_bool::SDL_TRUE => true,
        }
    }

    /// Query whether a game controller has rumble support.
    #[doc(alias = "SDL_GameControllerHasRumble")]
    pub fn has_rumble(&self) -> bool {
        let result = unsafe { sys::SDL_GameControllerHasRumble(self.raw) };

        match result {
            sys::SDL_bool::SDL_FALSE => false,
            sys::SDL_bool::SDL_TRUE => true,
        }
    }

    /// Query whether a game controller has rumble support on triggers.
    #[doc(alias = "SDL_GameControllerHasRumbleTriggers")]
    pub fn has_rumble_triggers(&self) -> bool {
        let result = unsafe { sys::SDL_GameControllerHasRumbleTriggers(self.raw) };

        match result {
            sys::SDL_bool::SDL_FALSE => false,
            sys::SDL_bool::SDL_TRUE => true,
        }
    }

    /// Update a game controller's LED color.
    #[doc(alias = "SDL_GameControllerSetLED")]
    pub fn set_led(&mut self, red: u8, green: u8, blue: u8) -> Result<(), IntegerOrSdlError> {
        let result = unsafe { sys::SDL_GameControllerSetLED(self.raw, red, green, blue) };

        if result != 0 {
            Err(IntegerOrSdlError::SdlError(get_error()))
        } else {
            Ok(())
        }
    }

    /// Send a controller specific effect packet.
    #[doc(alias = "SDL_GameControllerSendEffect")]
    pub fn send_effect(&mut self, data: &[u8]) -> Result<(), String> {
        let result = unsafe {
            sys::SDL_GameControllerSendEffect(
                self.raw,
                data.as_ptr() as *const libc::c_void,
                data.len() as i32,
            )
        };

        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }
}

#[cfg(feature = "hidapi")]
impl GameController {
    #[doc(alias = "SDL_GameControllerHasSensor")]
    pub fn has_sensor(&self, sensor_type: crate::sensor::SensorType) -> bool {
        let result = unsafe { sys::SDL_GameControllerHasSensor(self.raw, sensor_type.into()) };

        match result {
            sys::SDL_bool::SDL_FALSE => false,
            sys::SDL_bool::SDL_TRUE => true,
        }
    }

    #[doc(alias = "SDL_GameControllerIsSensorEnabled")]
    pub fn sensor_enabled(&self, sensor_type: crate::sensor::SensorType) -> bool {
        let result =
            unsafe { sys::SDL_GameControllerIsSensorEnabled(self.raw, sensor_type.into()) };

        match result {
            sys::SDL_bool::SDL_FALSE => false,
            sys::SDL_bool::SDL_TRUE => true,
        }
    }

    #[doc(alias = "SDL_GameControllerSetSensorEnabled")]
    pub fn sensor_set_enabled(
        &self,
        sensor_type: crate::sensor::SensorType,
        enabled: bool,
    ) -> Result<(), IntegerOrSdlError> {
        let result = unsafe {
            sys::SDL_GameControllerSetSensorEnabled(
                self.raw,
                sensor_type.into(),
                if enabled {
                    sys::SDL_bool::SDL_TRUE
                } else {
                    sys::SDL_bool::SDL_FALSE
                },
            )
        };

        if result != 0 {
            Err(IntegerOrSdlError::SdlError(get_error()))
        } else {
            Ok(())
        }
    }

    /// Get the data rate (number of events per second) of a game controller sensor.
    #[doc(alias = "SDL_GameControllerGetSensorDataRate")]
    pub fn sensor_get_data_rate(&self, sensor_type: SensorType) -> f32 {
        unsafe { sys::SDL_GameControllerGetSensorDataRate(self.raw, sensor_type.into()) }
    }

    /// Get data from a sensor.
    ///
    /// The number of data points depends on the sensor. Both Gyroscope and
    /// Accelerometer return 3 values, one for each axis.
    #[doc(alias = "SDL_GameControllerGetSensorData")]
    pub fn sensor_get_data(
        &self,
        sensor_type: SensorType,
        data: &mut [f32],
    ) -> Result<(), IntegerOrSdlError> {
        let result = unsafe {
            sys::SDL_GameControllerGetSensorData(
                self.raw,
                sensor_type.into(),
                data.as_mut_ptr(),
                data.len().try_into().unwrap(),
            )
        };

        if result != 0 {
            Err(IntegerOrSdlError::SdlError(get_error()))
        } else {
            Ok(())
        }
    }
}

impl Drop for GameController {
    #[doc(alias = "SDL_GameControllerClose")]
    fn drop(&mut self) {
        unsafe { sys::SDL_GameControllerClose(self.raw) }
    }
}

/// Convert C string `c_str` to a String. Return an empty string if
/// `c_str` is NULL.
fn c_str_to_string(c_str: *const c_char) -> String {
    if c_str.is_null() {
        String::new()
    } else {
        unsafe {
            CStr::from_ptr(c_str as *const _)
                .to_str()
                .unwrap()
                .to_owned()
        }
    }
}

/// Convert C string `c_str` to a String. Return an SDL error if
/// `c_str` is NULL.
fn c_str_to_string_or_err(c_str: *const c_char) -> Result<String, String> {
    if c_str.is_null() {
        Err(get_error())
    } else {
        Ok(unsafe {
            CStr::from_ptr(c_str as *const _)
                .to_str()
                .unwrap()
                .to_owned()
        })
    }
}
