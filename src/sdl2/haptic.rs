//! Haptic Functions
#[allow(unused)]
use sys::haptic as ll;
use sys::joystick as sys_joystick;

use HapticSubsystem;
use common::IntegerOrSdlError;
use get_error;

impl HapticSubsystem {
    /// Attempt to open the joystick at number `id` and return it.
    pub fn open_from_joystick_id(&self, joystick_index: i32) -> Result<Haptic, IntegerOrSdlError> {
        use common::IntegerOrSdlError::*;

        let haptic = unsafe {
            let joystick = sys_joystick::SDL_JoystickOpen(joystick_index);
            ll::SDL_HapticOpenFromJoystick(joystick)
        };

        if haptic.is_null() {
            Err(SdlError(get_error()))
        } else {
            unsafe { ll::SDL_HapticRumbleInit(haptic) };
            Ok(Haptic {
                subsystem: self.clone(),
                raw: haptic,
            })
        }
    }
}

#[allow(unused)]
pub struct Haptic {
    subsystem: HapticSubsystem,
    raw: *mut ll::SDL_Haptic,
}


impl Haptic{
  pub fn play(&self, strenght: f32, duration: u32){
   unsafe{ ll::SDL_HapticRumblePlay(self.raw, strenght, duration)};
  }

  pub fn stop(&self){
    unsafe { ll::SDL_HapticRumbleStop(self.raw) };
  }
}


impl Drop for Haptic{
  fn drop(&mut self){
    unsafe{ll::SDL_HapticClose(self.raw)}
  }
}
