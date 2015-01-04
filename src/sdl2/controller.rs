use libc::c_int;

pub use sys::controller as ll;

#[derive(Copy, Clone, PartialEq)]
#[repr(i32)]
pub enum ControllerAxis {
    Invalid      = ll::SDL_CONTROLLER_AXIS_INVALID,
    LeftX        = ll::SDL_CONTROLLER_AXIS_LEFTX,
    LeftY        = ll::SDL_CONTROLLER_AXIS_LEFTY,
    RightX       = ll::SDL_CONTROLLER_AXIS_RIGHTX,
    RightY       = ll::SDL_CONTROLLER_AXIS_RIGHTY,
    TriggerLeft  = ll::SDL_CONTROLLER_AXIS_TRIGGERLEFT,
    TriggerRight = ll::SDL_CONTROLLER_AXIS_TRIGGERRIGHT,
}

pub fn wrap_controller_axis(bitflags: u8) -> ControllerAxis {
    match bitflags as c_int {
        ll::SDL_CONTROLLER_AXIS_LEFTX        => ControllerAxis::LeftX,
        ll::SDL_CONTROLLER_AXIS_LEFTY        => ControllerAxis::LeftY,
        ll::SDL_CONTROLLER_AXIS_RIGHTX       => ControllerAxis::RightX,
        ll::SDL_CONTROLLER_AXIS_RIGHTY       => ControllerAxis::RightY,
        ll::SDL_CONTROLLER_AXIS_TRIGGERLEFT  => ControllerAxis::TriggerLeft,
        ll::SDL_CONTROLLER_AXIS_TRIGGERRIGHT => ControllerAxis::TriggerRight,
        _ => panic!("unhandled controller axis")
    }
}

#[derive(Copy, Clone, PartialEq)]
#[repr(i32)]
pub enum ControllerButton {
    Invalid       = ll::SDL_CONTROLLER_BUTTON_INVALID,
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

pub fn wrap_controller_button(bitflags: u8) -> ControllerButton {
    match bitflags as c_int {
        ll::SDL_CONTROLLER_BUTTON_A             => ControllerButton::A,
        ll::SDL_CONTROLLER_BUTTON_B             => ControllerButton::B,
        ll::SDL_CONTROLLER_BUTTON_X             => ControllerButton::X,
        ll::SDL_CONTROLLER_BUTTON_Y             => ControllerButton::Y,
        ll::SDL_CONTROLLER_BUTTON_BACK          => ControllerButton::Back,
        ll::SDL_CONTROLLER_BUTTON_GUIDE         => ControllerButton::Guide,
        ll::SDL_CONTROLLER_BUTTON_START         => ControllerButton::Start,
        ll::SDL_CONTROLLER_BUTTON_LEFTSTICK     => ControllerButton::LeftStick,
        ll::SDL_CONTROLLER_BUTTON_RIGHTSTICK    => ControllerButton::RightStick,
        ll::SDL_CONTROLLER_BUTTON_LEFTSHOULDER  => ControllerButton::LeftShoulder,
        ll::SDL_CONTROLLER_BUTTON_RIGHTSHOULDER => ControllerButton::RightShoulder,
        ll::SDL_CONTROLLER_BUTTON_DPAD_UP       => ControllerButton::DPadUp,
        ll::SDL_CONTROLLER_BUTTON_DPAD_DOWN     => ControllerButton::DPadDown,
        ll::SDL_CONTROLLER_BUTTON_DPAD_LEFT     => ControllerButton::DPadLeft,
        ll::SDL_CONTROLLER_BUTTON_DPAD_RIGHT    => ControllerButton::DPadRight,
        _ => panic!("unhandled controller button")
    }
}
