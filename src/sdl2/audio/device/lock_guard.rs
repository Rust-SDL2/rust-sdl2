use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use super::{AudioCallback, AudioDevice};

/// Similar to `std::sync::MutexGuard`, but for use with `AudioDevice::lock()`.
pub struct AudioDeviceLockGuard<'a, CB>
where
    CB: AudioCallback,
    CB: 'a,
{
    pub(super) device: &'a mut AudioDevice<CB>,
    pub(super) _nosend: PhantomData<*mut ()>,
}

impl<'a, CB: AudioCallback> Deref for AudioDeviceLockGuard<'a, CB> {
    type Target = CB;
    #[doc(alias = "SDL_UnlockAudioDevice")]
    fn deref(&self) -> &CB {
        (*self.device.userdata).as_ref().expect("Missing callback")
    }
}

impl<'a, CB: AudioCallback> DerefMut for AudioDeviceLockGuard<'a, CB> {
    fn deref_mut(&mut self) -> &mut CB {
        (*self.device.userdata).as_mut().expect("Missing callback")
    }
}

impl<'a, CB: AudioCallback> Drop for AudioDeviceLockGuard<'a, CB> {
    fn drop(&mut self) {
        unsafe { sys::SDL_UnlockAudioDevice(self.device.device_id.id()) }
    }
}
