use libc::c_char;
use std::error;
use std::ffi::{CString, CStr, NulError};
use std::fmt;
use std::path::Path;
use rwops::RWops;

use GameControllerSubsystem;
use get_error;
use joystick;
use common::{validate_int, IntegerOrSdlError};

use sys::controller as ll;
use sys::event::{SDL_QUERY, SDL_ENABLE};

#[derive(Debug)]
pub enum AddMappingError {
    InvalidMapping(NulError),
    InvalidFilePath(String),
    SdlError(String),
}

impl fmt::Display for AddMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::AddMappingError::*;

        match *self {
            InvalidMapping(ref e) => write!(f, "Null error: {}", e),
            InvalidFilePath(ref value) => write!(f, "Invalid file path ({})", value),
            SdlError(ref e) => write!(f, "SDL error: {}", e)
        }
    }
}

impl error::Error for AddMappingError {
    fn description(&self) -> &str {
        use self::AddMappingError::*;

        match *self {
            InvalidMapping(_) => "invalid mapping",
            InvalidFilePath(_) => "invalid file path",
            SdlError(ref e) => e,
        }
    }
}

impl GameControllerSubsystem {
    /// Retreive the total number of attached joysticks *and* controllers identified by SDL.
    pub fn num_joysticks(&self) -> Result<u32, String> {
        let result = unsafe { ::sys::joystick::SDL_NumJoysticks() };

        if result >= 0 {
            Ok(result as u32)
        } else {
            Err(get_error())
        }
    }

    /// Return true if the joystick at index `joystick_index` is a game controller.
    #[inline]
    pub fn is_game_controller(&self, joystick_index: u32) -> bool {
        match validate_int(joystick_index, "joystick_index") {
            Ok(joystick_index) => unsafe { ll::SDL_IsGameController(joystick_index) != 0 },
            Err(_) => false
        }
    }

    /// Attempt to open the controller ad index `joystick_index` and return it.
    /// Controller IDs are the same as joystick IDs and the maximum number can
    /// be retreived using the `SDL_NumJoysticks` function.
    pub fn open(&self, joystick_index: u32) -> Result<GameController, IntegerOrSdlError> {
        use common::IntegerOrSdlError::*;
        let joystick_index = try!(validate_int(joystick_index, "joystick_index"));
        let controller = unsafe { ll::SDL_GameControllerOpen(joystick_index) };

        if controller.is_null() {
            Err(SdlError(get_error()))
        } else {
            Ok(GameController {
                subsystem: self.clone(),
                raw: controller
            })
        }
    }

    /// Return the name of the controller at index `joystick_index`.
    pub fn name_for_index(&self, joystick_index: u32) -> Result<String, IntegerOrSdlError> {
        use common::IntegerOrSdlError::*;
        let joystick_index = try!(validate_int(joystick_index, "joystick_index"));
        let c_str = unsafe { ll::SDL_GameControllerNameForIndex(joystick_index) };

        if c_str.is_null() {
            Err(SdlError(get_error()))
        } else {
            Ok(unsafe {
                CStr::from_ptr(c_str as *const _).to_str().unwrap().to_owned()
            })
        }
    }

    /// If state is `true` controller events are processed, otherwise
    /// they're ignored.
    pub fn set_event_state(&self, state: bool) {
        unsafe { ll::SDL_GameControllerEventState(state as i32) };
    }

    /// Return `true` if controller events are processed.
    pub fn event_state(&self) -> bool {
        unsafe { ll::SDL_GameControllerEventState(SDL_QUERY as i32)
                 == SDL_ENABLE as i32 }
    }

    /// Add a new mapping from a mapping string
    pub fn add_mapping(&self, mapping: &str) 
            -> Result<MappingStatus, AddMappingError> {
        use self::AddMappingError::*;
        let mapping = match CString::new(mapping) {
            Ok(s) => s,
            Err(err) => return Err(InvalidMapping(err)),
        };

        let result = unsafe { ll::SDL_GameControllerAddMapping(mapping.as_ptr() as *const c_char) };

        match result {
            1 => Ok(MappingStatus::Added),
            0 => Ok(MappingStatus::Updated),
            _ => Err(SdlError(get_error())),
        }
    }

    /// Load mappings from a file
    pub fn load_mappings<P: AsRef<Path>>(&self, path: P)
            -> Result<i32, AddMappingError> {
        use self::AddMappingError::*;

        let file = match RWops::from_file(path, "r") {
            Ok(f) => f,
            Err(s) => return Err(InvalidFilePath(s))
        };

        let result = unsafe { ll::SDL_GameControllerAddMappingsFromRW(file.raw(), 0) };

        match result {
            -1 => Err(SdlError(get_error())),
            _ => Ok(result)
        }
    }


    pub fn mapping_for_guid(&self, guid: joystick::Guid) -> Result<String, String> {
        let c_str = unsafe { ll::SDL_GameControllerMappingForGUID(guid.raw()) };

        c_str_to_string_or_err(c_str)
    }

    #[inline]
    /// Force controller update when not using the event loop
    pub fn update(&self) {
        unsafe { ll::SDL_GameControllerUpdate() };
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(i32)]
pub enum Axis {
    LeftX        = ll::SDL_CONTROLLER_AXIS_LEFTX,
    LeftY        = ll::SDL_CONTROLLER_AXIS_LEFTY,
    RightX       = ll::SDL_CONTROLLER_AXIS_RIGHTX,
    RightY       = ll::SDL_CONTROLLER_AXIS_RIGHTY,
    TriggerLeft  = ll::SDL_CONTROLLER_AXIS_TRIGGERLEFT,
    TriggerRight = ll::SDL_CONTROLLER_AXIS_TRIGGERRIGHT,
}

impl Axis {
    /// Return the Axis from a string description in the same format
    /// used by the game controller mapping strings.
    pub fn from_string(axis: &str) -> Option<Axis> {
        let id = match CString::new(axis) {
            Ok(axis) => unsafe { ll::SDL_GameControllerGetAxisFromString(axis.as_ptr() as *const c_char) },
            // string contains a nul byte - it won't match anything.
            Err(_) => ll::SDL_CONTROLLER_AXIS_INVALID
        };

        Axis::from_ll(id)
    }

    /// Return a string for a given axis in the same format using by
    /// the game controller mapping strings
    pub fn string(self) -> String {
        let axis = self as ll::SDL_GameControllerAxis;

        let string = unsafe { ll::SDL_GameControllerGetStringForAxis(axis) };

        c_str_to_string(string)
    }

    pub fn from_ll(bitflags: ll::SDL_GameControllerAxis) -> Option<Axis> {
        Some(match bitflags {
            ll::SDL_CONTROLLER_AXIS_INVALID      => return None,
            ll::SDL_CONTROLLER_AXIS_LEFTX        => Axis::LeftX,
            ll::SDL_CONTROLLER_AXIS_LEFTY        => Axis::LeftY,
            ll::SDL_CONTROLLER_AXIS_RIGHTX       => Axis::RightX,
            ll::SDL_CONTROLLER_AXIS_RIGHTY       => Axis::RightY,
            ll::SDL_CONTROLLER_AXIS_TRIGGERLEFT  => Axis::TriggerLeft,
            ll::SDL_CONTROLLER_AXIS_TRIGGERRIGHT => Axis::TriggerRight,
            _ => panic!("unhandled controller axis")
        })
    }

    pub fn to_ll(&self) -> ll::SDL_GameControllerAxis {
        match *self {
            Axis::LeftX => ll::SDL_CONTROLLER_AXIS_LEFTX,
            Axis::LeftY => ll::SDL_CONTROLLER_AXIS_LEFTY,
            Axis::RightX => ll::SDL_CONTROLLER_AXIS_RIGHTX,
            Axis::RightY => ll::SDL_CONTROLLER_AXIS_RIGHTY,
            Axis::TriggerLeft => ll::SDL_CONTROLLER_AXIS_TRIGGERLEFT,
            Axis::TriggerRight => ll::SDL_CONTROLLER_AXIS_TRIGGERRIGHT,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(i32)]
pub enum Button {
    A             = ll::SDL_CONTROLLER_BUTTON_A,
    B             = ll::SDL_CONTROLLER_BUTTON_B,
    X             = ll::SDL_CONTROLLER_BUTTON_X,
    Y             = ll::SDL_CONTROLLER_BUTTON_Y,
    Back          = ll::SDL_CONTROLLER_BUTTON_BACK,
    Guide         = ll::SDL_CONTROLLER_BUTTON_GUIDE,
    Start         = ll::SDL_CONTROLLER_BUTTON_START,
    LeftStick     = ll::SDL_CONTROLLER_BUTTON_LEFTSTICK,
    RightStick    = ll::SDL_CONTROLLER_BUTTON_RIGHTSTICK,
    LeftShoulder  = ll::SDL_CONTROLLER_BUTTON_LEFTSHOULDER,
    RightShoulder = ll::SDL_CONTROLLER_BUTTON_RIGHTSHOULDER,
    DPadUp        = ll::SDL_CONTROLLER_BUTTON_DPAD_UP,
    DPadDown      = ll::SDL_CONTROLLER_BUTTON_DPAD_DOWN,
    DPadLeft      = ll::SDL_CONTROLLER_BUTTON_DPAD_LEFT,
    DPadRight     = ll::SDL_CONTROLLER_BUTTON_DPAD_RIGHT,
}

impl Button {
    /// Return the Button from a string description in the same format
    /// used by the game controller mapping strings.
    pub fn from_string(button: &str) -> Option<Button> {
        let id = match CString::new(button) {
            Ok(button) => unsafe { ll::SDL_GameControllerGetButtonFromString(button.as_ptr() as *const c_char) },
            // string contains a nul byte - it won't match anything.
            Err(_) => ll::SDL_CONTROLLER_BUTTON_INVALID
        };

        Button::from_ll(id)
    }

    /// Return a string for a given button in the same format using by
    /// the game controller mapping strings
    pub fn string(self) -> String {
        let button = self as ll::SDL_GameControllerButton;

        let string = unsafe { ll::SDL_GameControllerGetStringForButton(button) };

        c_str_to_string(string)
    }

    pub fn from_ll(bitflags: ll::SDL_GameControllerButton) -> Option<Button> {
        Some(match bitflags {
            ll::SDL_CONTROLLER_BUTTON_INVALID       => return None,
            ll::SDL_CONTROLLER_BUTTON_A             => Button::A,
            ll::SDL_CONTROLLER_BUTTON_B             => Button::B,
            ll::SDL_CONTROLLER_BUTTON_X             => Button::X,
            ll::SDL_CONTROLLER_BUTTON_Y             => Button::Y,
            ll::SDL_CONTROLLER_BUTTON_BACK          => Button::Back,
            ll::SDL_CONTROLLER_BUTTON_GUIDE         => Button::Guide,
            ll::SDL_CONTROLLER_BUTTON_START         => Button::Start,
            ll::SDL_CONTROLLER_BUTTON_LEFTSTICK     => Button::LeftStick,
            ll::SDL_CONTROLLER_BUTTON_RIGHTSTICK    => Button::RightStick,
            ll::SDL_CONTROLLER_BUTTON_LEFTSHOULDER  => Button::LeftShoulder,
            ll::SDL_CONTROLLER_BUTTON_RIGHTSHOULDER => Button::RightShoulder,
            ll::SDL_CONTROLLER_BUTTON_DPAD_UP       => Button::DPadUp,
            ll::SDL_CONTROLLER_BUTTON_DPAD_DOWN     => Button::DPadDown,
            ll::SDL_CONTROLLER_BUTTON_DPAD_LEFT     => Button::DPadLeft,
            ll::SDL_CONTROLLER_BUTTON_DPAD_RIGHT    => Button::DPadRight,
            _ => panic!("unhandled controller button")
        })
    }

    pub fn to_ll(&self) -> ll::SDL_GameControllerButton {
        match *self {
            Button::A             => ll::SDL_CONTROLLER_BUTTON_A,
            Button::B             => ll::SDL_CONTROLLER_BUTTON_B,
            Button::X             => ll::SDL_CONTROLLER_BUTTON_X,
            Button::Y             => ll::SDL_CONTROLLER_BUTTON_Y,
            Button::Back          => ll::SDL_CONTROLLER_BUTTON_BACK,
            Button::Guide         => ll::SDL_CONTROLLER_BUTTON_GUIDE,
            Button::Start         => ll::SDL_CONTROLLER_BUTTON_START,
            Button::LeftStick     => ll::SDL_CONTROLLER_BUTTON_LEFTSTICK,
            Button::RightStick    => ll::SDL_CONTROLLER_BUTTON_RIGHTSTICK,
            Button::LeftShoulder  => ll::SDL_CONTROLLER_BUTTON_LEFTSHOULDER,
            Button::RightShoulder => ll::SDL_CONTROLLER_BUTTON_RIGHTSHOULDER,
            Button::DPadUp        => ll::SDL_CONTROLLER_BUTTON_DPAD_UP,
            Button::DPadDown      => ll::SDL_CONTROLLER_BUTTON_DPAD_DOWN,
            Button::DPadLeft      => ll::SDL_CONTROLLER_BUTTON_DPAD_LEFT,
            Button::DPadRight     => ll::SDL_CONTROLLER_BUTTON_DPAD_RIGHT,
        }
    }
}

/// Possible return values for `add_mapping`
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MappingStatus {
    Added   = 1,
    Updated = 0,
}

/// Wrapper around the SDL_GameController object
pub struct GameController {
    subsystem: GameControllerSubsystem,
    raw: *mut ll::SDL_GameController
}

impl GameController {
    #[inline]
    pub fn subsystem(&self) -> &GameControllerSubsystem { &self.subsystem }

    /// Return the name of the controller or an empty string if no
    /// name is found.
    pub fn name(&self) -> String {
        let name = unsafe { ll::SDL_GameControllerName(self.raw) };

        c_str_to_string(name)
    }

    /// Return a String describing the controller's button and axis
    /// mappings
    pub fn mapping(&self) -> String {
        let mapping = unsafe { ll::SDL_GameControllerMapping(self.raw) };

        c_str_to_string(mapping)
    }

    /// Return true if the controller has been opened and currently
    /// connected.
    pub fn attached(&self) -> bool {
        unsafe { ll::SDL_GameControllerGetAttached(self.raw) != 0 }
    }

    /// Return the joystick instance id of this controller
    pub fn instance_id(&self) -> i32 {
        let result = unsafe {
          let joystick = ll::SDL_GameControllerGetJoystick(self.raw);
          ::sys::joystick::SDL_JoystickInstanceID(joystick)
        };

        if result < 0 {
            // Should only fail if the joystick is NULL.
            panic!(get_error())
        } else {
            result
        }
    }

    /// Get the position of the given `axis`
    pub fn axis(&self, axis: Axis) -> i16 {
        // This interface is a bit messed up: 0 is a valid position
        // but can also mean that an error occured.
        // Fortunately, an error can only occur if the controller pointer is NULL.
        // There should be no apparent reason for this to change in the future.

        let axis = axis as ll::SDL_GameControllerAxis;

        unsafe { ll::SDL_GameControllerGetAxis(self.raw, axis) }
    }

    /// Returns `true` if `button` is pressed.
    pub fn button(&self, button: Button) -> bool {
        // This interface is a bit messed up: 0 is a valid position
        // but can also mean that an error occured.
        // Fortunately, an error can only occur if the controller pointer is NULL.
        // There should be no apparent reason for this to change in the future.

        let button = button as ll::SDL_GameControllerButton;

        unsafe { ll::SDL_GameControllerGetButton(self.raw, button) != 0 }
    }
}

impl Drop for GameController {
    fn drop(&mut self) {
        unsafe { ll::SDL_GameControllerClose(self.raw) }
    }
}

/// Convert C string `c_str` to a String. Return an empty string if
/// c_str is NULL.
fn c_str_to_string(c_str: *const c_char) -> String {
    if c_str.is_null() {
        String::new()
    } else {
        unsafe { 
            CStr::from_ptr(c_str as *const _).to_str().unwrap().to_owned()
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
            CStr::from_ptr(c_str as *const _).to_str().unwrap().to_owned()
        })
    }
}
