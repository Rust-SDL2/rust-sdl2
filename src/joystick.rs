
pub mod ll {
    use std::libc::{c_int, c_schar, c_void, int32_t, int16_t, int8_t, uint8_t};

    pub type SDL_bool = c_int;

    pub type SDL_Joystick = c_void;

    pub struct SDL_JoystickGUID {
        data: [uint8_t, ..16u],
    }

    externfn!(fn SDL_NumJoysticks() -> c_int)
    externfn!(fn SDL_JoystickNameForIndex(device_index: c_int) -> *c_schar)
    externfn!(fn SDL_JoystickOpen(device_index: c_int) -> *SDL_Joystick)
    externfn!(fn SDL_JoystickName(joystick: *SDL_Joystick) -> *c_schar)
    externfn!(fn SDL_JoystickGetDeviceGUID(device_index: c_int) ->
              SDL_JoystickGUID)
    externfn!(fn SDL_JoystickGetGUID(joystick: *SDL_Joystick) ->
              SDL_JoystickGUID)
    externfn!(fn SDL_JoystickGetGUIDString(guid: SDL_JoystickGUID,
                                           pszGUID: *c_schar, cbGUID: c_int))
    externfn!(fn SDL_JoystickGetGUIDFromString(pchGUID: *c_schar) ->
              SDL_JoystickGUID)
    externfn!(fn SDL_JoystickGetAttached(joystick: *SDL_Joystick) -> SDL_bool)
    externfn!(fn SDL_JoystickInstanceID(joystick: *SDL_Joystick) -> int32_t)
    externfn!(fn SDL_JoystickNumAxes(joystick: *SDL_Joystick) -> c_int)
    externfn!(fn SDL_JoystickNumBalls(joystick: *SDL_Joystick) -> c_int)
    externfn!(fn SDL_JoystickNumHats(joystick: *SDL_Joystick) -> c_int)
    externfn!(fn SDL_JoystickNumButtons(joystick: *SDL_Joystick) -> c_int)
    externfn!(fn SDL_JoystickUpdate())
    externfn!(fn SDL_JoystickEventState(state: c_int) -> c_int)
    externfn!(fn SDL_JoystickGetAxis(joystick: *SDL_Joystick, axis: c_int) ->
              int16_t)
    externfn!(fn SDL_JoystickGetHat(joystick: *SDL_Joystick, hat: c_int) ->
              int8_t)
    externfn!(fn SDL_JoystickGetBall(joystick: *SDL_Joystick, ball: c_int,
                                     dx: *c_int, dy: *c_int) -> c_int)
    externfn!(fn SDL_JoystickGetButton(joystick: *SDL_Joystick, button: c_int)
              -> uint8_t)
    externfn!(fn SDL_JoystickClose(joystick: *SDL_Joystick))
}

#[deriving(Eq)]
pub enum HatState {
    CenteredHatState,
    UpHatState,
    RightHatState,
    DownHatState,
    LeftHatState
}

pub fn wrap_hat_state(bitflags: u8) -> ~[HatState] {
    let flags = [CenteredHatState,
        UpHatState,
        RightHatState,
        DownHatState,
        LeftHatState];

    do flags.iter().filter_map |&flag| {
        if bitflags & (flag as u8) != 0 { Some(flag) }
        else { None }
    }.collect()
}
