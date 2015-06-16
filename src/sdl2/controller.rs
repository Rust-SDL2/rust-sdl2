use libc::c_char;
use std::ffi::{CString, CStr};

use SdlResult;
use get_error;
use joystick;
use util::CStringExt;

use sys::controller as ll;
use sys::event::{SDL_QUERY, SDL_ENABLE};

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
            Ok(axis) => unsafe { ll::SDL_GameControllerGetAxisFromString(axis.as_ptr()) },
            // string contains a nul byte - it won't match anything.
            Err(_) => ll::SDL_CONTROLLER_AXIS_INVALID
        };

        Axis::from_ll(id)
    }

    /// Return a string for a given axis in the same format using by
    /// the game controller mapping strings
    pub fn get_string(self) -> String {
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
            Ok(button) => unsafe { ll::SDL_GameControllerGetButtonFromString(button.as_ptr()) },
            // string contains a nul byte - it won't match anything.
            Err(_) => ll::SDL_CONTROLLER_BUTTON_INVALID
        };

        Button::from_ll(id)
    }

    /// Return a string for a given button in the same format using by
    /// the game controller mapping strings
    pub fn get_string(self) -> String {
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
}

/// Return true if the joystick at index `id` is a game controller.
pub fn is_game_controller(id: i32) -> bool {
    unsafe { ll::SDL_IsGameController(id) != 0 }
}

/// Return the name of the controller at index `id`
pub fn name_for_index(id: i32) -> SdlResult<String> {
    let name = unsafe { ll::SDL_GameControllerNameForIndex(id) };

    c_str_to_string_or_err(name)
}

/// Force controller update when not using the event loop
pub fn update() {
    unsafe { ll::SDL_GameControllerUpdate() };
}

/// If state is `true` controller events are processed, otherwise
/// they're ignored.
pub fn set_event_state(state: bool) {
    unsafe { ll::SDL_GameControllerEventState(state as i32) };
}

/// Return `true` if controller events are processed.
pub fn get_event_state() -> bool {
    unsafe { ll::SDL_GameControllerEventState(SDL_QUERY as i32)
             == SDL_ENABLE as i32 }
}

/// Possible return values for `add_mapping`
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MappingStatus {
    Added   = 1,
    Updated = 0,
}

/// Add a new mapping from a mapping string
pub fn add_mapping(mapping: &str) -> SdlResult<MappingStatus> {
    let mapping = try!(CString::new(mapping).unwrap_or_sdlresult());

    let result = unsafe { ll::SDL_GameControllerAddMapping(mapping.as_ptr()) };

    match result {
        1 => Ok(MappingStatus::Added),
        0 => Ok(MappingStatus::Updated),
        _ => Err(get_error()),
    }
}

pub fn mapping_for_guid(guid: joystick::Guid) -> SdlResult<String> {
    let c_str = unsafe { ll::SDL_GameControllerMappingForGUID(guid.raw()) };

    c_str_to_string_or_err(c_str)
}

/// Wrapper around the SDL_GameController object
pub struct GameController {
    raw: *mut ll::SDL_GameController,
}

impl GameController {

    /// Attempt to open the controller number `id` and return
    /// it. Controller IDs are the same as joystick IDs and the
    /// maximum number can be retreived using the `SDL_NumJoysticks`
    /// function.
    pub fn open(id: i32) -> SdlResult<GameController> {
        let controller = unsafe { ll::SDL_GameControllerOpen(id) };

        if controller.is_null() {
            Err(get_error())
        } else {
            Ok(GameController { raw: controller })
        }
    }

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
    pub fn get_attached(&self) -> bool {
        unsafe { ll::SDL_GameControllerGetAttached(self.raw) != 0 }
    }

    /// Get the position of the given `axis`
    pub fn get_axis(&self, axis: Axis) -> i16 {
        // This interface is a bit messed up: 0 is a valid position
        // but can also mean that an error occured.
        // Fortunately, an error can only occur if the controller pointer is NULL.
        // There should be no apparent reason for this to change in the future.

        let axis = axis as ll::SDL_GameControllerAxis;

        unsafe { ll::SDL_GameControllerGetAxis(self.raw, axis) }
    }

    /// Returns `true` if `button` is pressed.
    pub fn get_button(&self, button: Button) -> bool {
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
