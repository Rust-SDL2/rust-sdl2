use sys::joystick as ll;

use SdlResult;
use get_error;

bitflags! {
    flags HatState: u8 {
        const CENTEREDHATSTATE = 0,
        const UPHATSTATE = 0x01,
        const RIGHTHATSTATE = 0x02,
        const DOWNHATSTATE = 0x04,
        const LEFTHATSTATE = 0x08,
        const RIGHTUPHATSTATE = 0x02 | 0x01,   // RightHatState | UpHatState
        const RIGHTDOWNHATSTATE = 0x02 | 0x04, // RightHatState | DownHatState,
        const LEFTUPHATSTATE = 0x08 | 0x01,    // LeftHatState | UpHatState,
        const LEFTDOWNHATSTATE = 0x08 | 0x04   // LeftHatState | DownHatState
    }
}

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
