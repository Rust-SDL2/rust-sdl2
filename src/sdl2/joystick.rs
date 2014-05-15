#[allow(non_camel_case_types)]
pub mod ll {
    use libc::{c_int, c_char, c_void, int32_t, int16_t, int8_t, uint8_t};

    pub type SDL_bool = c_int;

    pub type SDL_Joystick = c_void;

    pub struct SDL_JoystickGUID {
        data: [uint8_t, ..16u],
    }

    extern "C" {
        pub fn SDL_NumJoysticks() -> c_int;
        pub fn SDL_JoystickNameForIndex(device_index: c_int) -> *c_char;
        pub fn SDL_JoystickOpen(device_index: c_int) -> *SDL_Joystick;
        pub fn SDL_JoystickName(joystick: *SDL_Joystick) -> *c_char;
        pub fn SDL_JoystickGetDeviceGUID(device_index: c_int) ->
                  SDL_JoystickGUID;
        pub fn SDL_JoystickGetGUID(joystick: *SDL_Joystick) ->
                  SDL_JoystickGUID;
        pub fn SDL_JoystickGetGUIDString(guid: SDL_JoystickGUID,
                                               pszGUID: *c_char, cbGUID: c_int);
        pub fn SDL_JoystickGetGUIDFromString(pchGUID: *c_char) ->
                  SDL_JoystickGUID;
        pub fn SDL_JoystickGetAttached(joystick: *SDL_Joystick) -> SDL_bool;
        pub fn SDL_JoystickInstanceID(joystick: *SDL_Joystick) -> int32_t;
        pub fn SDL_JoystickNumAxes(joystick: *SDL_Joystick) -> c_int;
        pub fn SDL_JoystickNumBalls(joystick: *SDL_Joystick) -> c_int;
        pub fn SDL_JoystickNumHats(joystick: *SDL_Joystick) -> c_int;
        pub fn SDL_JoystickNumButtons(joystick: *SDL_Joystick) -> c_int;
        pub fn SDL_JoystickUpdate();
        pub fn SDL_JoystickEventState(state: c_int) -> c_int;
        pub fn SDL_JoystickGetAxis(joystick: *SDL_Joystick, axis: c_int) ->
                  int16_t;
        pub fn SDL_JoystickGetHat(joystick: *SDL_Joystick, hat: c_int) ->
                  int8_t;
        pub fn SDL_JoystickGetBall(joystick: *SDL_Joystick, ball: c_int,
                                         dx: *c_int, dy: *c_int) -> c_int;
        pub fn SDL_JoystickGetButton(joystick: *SDL_Joystick, button: c_int)
                  -> uint8_t;
        pub fn SDL_JoystickClose(joystick: *SDL_Joystick);
    }
}

bitflags!(flags HatState: u8 {
    static CenteredHatState = 0,
    static UpHatState = 0x01,
    static RightHatState = 0x02,
    static DownHatState = 0x04,
    static LeftHatState = 0x08,
    static RightUpHatState = 0x02 | 0x01,   // RightHatState | UpHatState
    static RightDownHatState = 0x02 | 0x04, // RightHatState | DownHatState,
    static LeftUpHatState = 0x08 | 0x01,    // LeftHatState | UpHatState,
    static LeftDownHatState = 0x08 | 0x04   // LeftHatState | DownHatState
})
