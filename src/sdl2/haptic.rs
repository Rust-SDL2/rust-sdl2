//! Haptic Functions
use crate::sys;

use crate::common::{validate_int, IntegerOrSdlError};
use crate::get_error;
use crate::HapticSubsystem;

impl HapticSubsystem {
    /// Attempt to open the joystick at index `joystick_index` and return its haptic device.
    #[doc(alias = "SDL_JoystickOpen")]
    pub fn open_from_joystick_id(&self, joystick_index: u32) -> Result<Haptic, IntegerOrSdlError> {
        use crate::common::IntegerOrSdlError::*;
        let joystick_index = validate_int(joystick_index, "joystick_index")?;

        let haptic = unsafe {
            let joystick = sys::SDL_JoystickOpen(joystick_index);
            sys::SDL_HapticOpenFromJoystick(joystick)
        };

        if haptic.is_null() {
            Err(SdlError(get_error()))
        } else {
            unsafe { sys::SDL_HapticRumbleInit(haptic) };
            Ok(Haptic {
                subsystem: self.clone(),
                raw: haptic,
            })
        }
    }
}

/// Wrapper around the `SDL_Haptic` object
pub struct Haptic {
    subsystem: HapticSubsystem,
    raw: *mut sys::SDL_Haptic,
}

impl Haptic {
    #[inline]
    #[doc(alias = "SDL_HapticRumblePlay")]
    pub fn subsystem(&self) -> &HapticSubsystem {
        &self.subsystem
    }

    /// Run a simple rumble effect on the haptic device.
    pub fn rumble_play(&mut self, strength: f32, duration: u32) {
        unsafe { sys::SDL_HapticRumblePlay(self.raw, strength, duration) };
    }

    /// Stop the simple rumble on the haptic device.
    #[doc(alias = "SDL_HapticRumbleStop")]
    pub fn rumble_stop(&mut self) {
        unsafe { sys::SDL_HapticRumbleStop(self.raw) };
    }
}

impl Drop for Haptic {
    #[doc(alias = "SDL_HapticClose")]
    fn drop(&mut self) {
        unsafe { sys::SDL_HapticClose(self.raw) }
    }
}
