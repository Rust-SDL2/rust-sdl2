use libc::c_int;

#[allow(non_camel_case_types)]
pub mod ll {
    use std::mem;
    use libc::{c_int, c_char, c_uchar, c_uint, c_void, int16_t, uint8_t};
    use joystick::ll::{SDL_Joystick, SDL_JoystickGUID};

    pub type SDL_bool = c_int;

    pub type SDL_GameController = c_void;

    pub type SDL_GameControllerBindType = c_uint;
    pub static SDL_CONTROLLER_BINDTYPE_NONE: SDL_GameControllerBindType = 0;
    pub static SDL_CONTROLLER_BINDTYPE_BUTTON: SDL_GameControllerBindType = 1;
    pub static SDL_CONTROLLER_BINDTYPE_AXIS: SDL_GameControllerBindType = 2;
    pub static SDL_CONTROLLER_BINDTYPE_HAT: SDL_GameControllerBindType = 3;

    #[allow(dead_code)]
    pub struct SDL_GameControllerButtonBind {
        bindType: SDL_GameControllerBindType,
        value: SDL_GameControllerButtonBindData,
    }

    #[allow(dead_code)]
    pub struct SDL_GameControllerButtonBindData {
        data: [c_uchar, ..8u],
    }

    #[allow(dead_code)]
    pub struct SDL_GameControllerButtonBindDataHat {
        hat: c_int,
        hat_mask: c_int,
    }

    impl SDL_GameControllerButtonBindData {
        pub fn button(&self) -> *const c_int {
            unsafe { mem::transmute_copy(&self) }
        }
        pub fn axis(&self) -> *const c_int {
            unsafe { mem::transmute_copy(&self) }
        }
        pub fn hat(&self) -> *const SDL_GameControllerButtonBindDataHat {
            unsafe { mem::transmute_copy(&self) }
        }
    }

    pub type SDL_GameControllerAxis = c_int;
    pub static SDL_CONTROLLER_AXIS_INVALID: SDL_GameControllerAxis = -1;
    pub static SDL_CONTROLLER_AXIS_LEFTX: SDL_GameControllerAxis = 0;
    pub static SDL_CONTROLLER_AXIS_LEFTY: SDL_GameControllerAxis = 1;
    pub static SDL_CONTROLLER_AXIS_RIGHTX: SDL_GameControllerAxis = 2;
    pub static SDL_CONTROLLER_AXIS_RIGHTY: SDL_GameControllerAxis = 3;
    pub static SDL_CONTROLLER_AXIS_TRIGGERLEFT: SDL_GameControllerAxis = 4;
    pub static SDL_CONTROLLER_AXIS_TRIGGERRIGHT: SDL_GameControllerAxis = 5;
    pub static SDL_CONTROLLER_AXIS_MAX: SDL_GameControllerAxis = 6;

    pub type SDL_GameControllerButton = c_int;
    pub static SDL_CONTROLLER_BUTTON_INVALID: SDL_GameControllerButton = -1;
    pub static SDL_CONTROLLER_BUTTON_A: SDL_GameControllerButton = 0;
    pub static SDL_CONTROLLER_BUTTON_B: SDL_GameControllerButton = 1;
    pub static SDL_CONTROLLER_BUTTON_X: SDL_GameControllerButton = 2;
    pub static SDL_CONTROLLER_BUTTON_Y: SDL_GameControllerButton = 3;
    pub static SDL_CONTROLLER_BUTTON_BACK: SDL_GameControllerButton = 4;
    pub static SDL_CONTROLLER_BUTTON_GUIDE: SDL_GameControllerButton = 5;
    pub static SDL_CONTROLLER_BUTTON_START: SDL_GameControllerButton = 6;
    pub static SDL_CONTROLLER_BUTTON_LEFTSTICK: SDL_GameControllerButton = 7;
    pub static SDL_CONTROLLER_BUTTON_RIGHTSTICK: SDL_GameControllerButton = 8;
    pub static SDL_CONTROLLER_BUTTON_LEFTSHOULDER: SDL_GameControllerButton = 9;
    pub static SDL_CONTROLLER_BUTTON_RIGHTSHOULDER: SDL_GameControllerButton = 10;
    pub static SDL_CONTROLLER_BUTTON_DPAD_UP: SDL_GameControllerButton = 11;
    pub static SDL_CONTROLLER_BUTTON_DPAD_DOWN: SDL_GameControllerButton = 12;
    pub static SDL_CONTROLLER_BUTTON_DPAD_LEFT: SDL_GameControllerButton = 13;
    pub static SDL_CONTROLLER_BUTTON_DPAD_RIGHT: SDL_GameControllerButton = 14;
    pub static SDL_CONTROLLER_BUTTON_MAX: SDL_GameControllerButton = 15;

    extern "C" {
        pub fn SDL_GameControllerAddMapping(mappingString: *const c_char) -> c_int;
        pub fn SDL_GameControllerMappingForGUID(guid: SDL_JoystickGUID) ->
                  *const c_char;
        pub fn SDL_GameControllerMapping(gamecontroller: *const SDL_GameController)
                  -> *const c_char;
        pub fn SDL_IsGameController(joystick_index: c_int) -> SDL_bool;
        pub fn SDL_GameControllerNameForIndex(joystick_index: c_int) ->
                  *const c_char;
        pub fn SDL_GameControllerOpen(joystick_index: c_int) ->
                  *const SDL_GameController;
        pub fn SDL_GameControllerName(gamecontroller: *const SDL_GameController) ->
                  *const c_char;
        pub fn SDL_GameControllerGetAttached(gamecontroller:
                                                       *const SDL_GameController) ->
                  SDL_bool;
        pub fn SDL_GameControllerGetJoystick(gamecontroller:
                                                       *const SDL_GameController) ->
                  *const SDL_Joystick;
        pub fn SDL_GameControllerEventState(state: c_int) -> c_int;
        pub fn SDL_GameControllerUpdate();
        pub fn SDL_GameControllerGetAxisFromString(pchString: *const c_char) ->
                  SDL_GameControllerAxis;
        pub fn SDL_GameControllerGetStringForAxis(axis:
                                                            SDL_GameControllerAxis)
                  -> *const c_char;
        pub fn SDL_GameControllerGetBindForAxis(gamecontroller:
                                                          *const SDL_GameController,
                                                      axis: SDL_GameControllerAxis)
                  -> SDL_GameControllerButtonBind;
        pub fn SDL_GameControllerGetAxis(gamecontroller: *const SDL_GameController,
                                               axis: SDL_GameControllerAxis) ->
                  int16_t;
        pub fn SDL_GameControllerGetButtonFromString(pchString: *const c_char) ->
                  SDL_GameControllerButton;
        pub fn SDL_GameControllerGetStringForButton(button:
                                                          SDL_GameControllerButton)
                  -> *const c_char;
        pub fn SDL_GameControllerGetBindForButton(gamecontroller:
                                                            *const SDL_GameController,
                                                        button:
                                                          SDL_GameControllerButton)
                  -> SDL_GameControllerButtonBind;
        pub fn SDL_GameControllerGetButton(gamecontroller:
                                                     *const SDL_GameController,
                                                 button: SDL_GameControllerButton)
                  -> uint8_t;
        pub fn SDL_GameControllerClose(gamecontroller: *const SDL_GameController);
    }
}

#[deriving(PartialEq)]
#[repr(i32)]
pub enum ControllerAxis {
    InvalidAxis      = ll::SDL_CONTROLLER_AXIS_INVALID,
    LeftXAxis        = ll::SDL_CONTROLLER_AXIS_LEFTX,
    LeftYAxis        = ll::SDL_CONTROLLER_AXIS_LEFTY,
    RightXAxis       = ll::SDL_CONTROLLER_AXIS_RIGHTX,
    RightYAxis       = ll::SDL_CONTROLLER_AXIS_RIGHTY,
    TriggerLeftAxis  = ll::SDL_CONTROLLER_AXIS_TRIGGERLEFT,
    TriggerRightAxis = ll::SDL_CONTROLLER_AXIS_TRIGGERRIGHT,
}

pub fn wrap_controller_axis(bitflags: u8) -> ControllerAxis {
    match bitflags as c_int {
        ll::SDL_CONTROLLER_AXIS_LEFTX        => LeftXAxis,
        ll::SDL_CONTROLLER_AXIS_LEFTY        => LeftYAxis,
        ll::SDL_CONTROLLER_AXIS_RIGHTX       => RightXAxis,
        ll::SDL_CONTROLLER_AXIS_RIGHTY       => RightYAxis,
        ll::SDL_CONTROLLER_AXIS_TRIGGERLEFT  => TriggerLeftAxis,
        ll::SDL_CONTROLLER_AXIS_TRIGGERRIGHT => TriggerRightAxis,
        _ => fail!("unhandled controller axis")
    }
}

#[deriving(PartialEq)]
#[repr(i32)]
pub enum ControllerButton {
    InvalidButton       = ll::SDL_CONTROLLER_BUTTON_INVALID,
    AButton             = ll::SDL_CONTROLLER_BUTTON_A,
    BButton             = ll::SDL_CONTROLLER_BUTTON_B,
    XButton             = ll::SDL_CONTROLLER_BUTTON_X,
    YButton             = ll::SDL_CONTROLLER_BUTTON_Y,
    BackButton          = ll::SDL_CONTROLLER_BUTTON_BACK,
    GuideButton         = ll::SDL_CONTROLLER_BUTTON_GUIDE,
    StartButton         = ll::SDL_CONTROLLER_BUTTON_START,
    LeftStickButton     = ll::SDL_CONTROLLER_BUTTON_LEFTSTICK,
    RightStickButton    = ll::SDL_CONTROLLER_BUTTON_RIGHTSTICK,
    LeftShoulderButton  = ll::SDL_CONTROLLER_BUTTON_LEFTSHOULDER,
    RightShoulderButton = ll::SDL_CONTROLLER_BUTTON_RIGHTSHOULDER,
    DPadUpButton        = ll::SDL_CONTROLLER_BUTTON_DPAD_UP,
    DPadDownButton      = ll::SDL_CONTROLLER_BUTTON_DPAD_DOWN,
    DPadLeftButton      = ll::SDL_CONTROLLER_BUTTON_DPAD_LEFT,
    DPadRightButton     = ll::SDL_CONTROLLER_BUTTON_DPAD_RIGHT,
}

pub fn wrap_controller_button(bitflags: u8) -> ControllerButton {
    match bitflags as c_int {
        ll::SDL_CONTROLLER_BUTTON_A             => AButton,
        ll::SDL_CONTROLLER_BUTTON_B             => BButton,
        ll::SDL_CONTROLLER_BUTTON_X             => XButton,
        ll::SDL_CONTROLLER_BUTTON_Y             => YButton,
        ll::SDL_CONTROLLER_BUTTON_BACK          => BackButton,
        ll::SDL_CONTROLLER_BUTTON_GUIDE         => GuideButton,
        ll::SDL_CONTROLLER_BUTTON_START         => StartButton,
        ll::SDL_CONTROLLER_BUTTON_LEFTSTICK     => LeftStickButton,
        ll::SDL_CONTROLLER_BUTTON_RIGHTSTICK    => RightStickButton,
        ll::SDL_CONTROLLER_BUTTON_LEFTSHOULDER  => LeftShoulderButton,
        ll::SDL_CONTROLLER_BUTTON_RIGHTSHOULDER => RightShoulderButton,
        ll::SDL_CONTROLLER_BUTTON_DPAD_UP       => DPadUpButton,
        ll::SDL_CONTROLLER_BUTTON_DPAD_DOWN     => DPadDownButton,
        ll::SDL_CONTROLLER_BUTTON_DPAD_LEFT     => DPadLeftButton,
        ll::SDL_CONTROLLER_BUTTON_DPAD_RIGHT    => DPadRightButton,
        _ => fail!("unhandled controller button")
    }
}
