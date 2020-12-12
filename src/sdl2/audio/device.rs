use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr;

use libc::c_char;

use crate::get_error;

mod id;
pub(super) use self::id::AudioDeviceID;
mod lock_guard;
use self::lock_guard::AudioDeviceLockGuard;

use super::{AudioCallback, AudioSpec, AudioSpecDesired, AudioStatus, AudioSubsystem};

/// Wraps `SDL_AudioDeviceID` and owns the callback data used by the audio device.
pub struct AudioDevice<CB: AudioCallback> {
    subsystem: AudioSubsystem,
    device_id: AudioDeviceID,
    spec: AudioSpec,
    /// Store the callback to keep it alive for the entire duration of `AudioDevice`.
    userdata: Box<Option<CB>>,
}

impl<CB: AudioCallback> AudioDevice<CB> {
    /// Opens a new audio device for playback or capture (given the desired parameters and callback).
    #[doc(alias = "SDL_OpenAudioDevice")]
    fn open<'a, F, D>(
        a: &AudioSubsystem,
        device: D,
        spec: &AudioSpecDesired,
        get_callback: F,
        capture: bool,
    ) -> Result<AudioDevice<CB>, String>
    where
        F: FnOnce(AudioSpec) -> CB,
        D: Into<Option<&'a str>>,
    {
        use std::mem::MaybeUninit;

        let mut userdata: Box<Option<CB>> = Box::new(None);
        let desired =
            AudioSpecDesired::convert_to_ll(spec.freq, spec.channels, spec.samples, &mut *userdata);

        let mut obtained = MaybeUninit::uninit();
        unsafe {
            let device = match device.into() {
                Some(device) => Some(CString::new(device).unwrap()),
                None => None,
            };
            // Warning: map_or consumes its argument; `device.map_or()` would therefore consume the
            // CString and drop it, making device_ptr a dangling pointer! To avoid that we downgrade
            // device to an Option<&_> first.
            let device_ptr = device.as_ref().map_or(ptr::null(), |s| s.as_ptr());

            let iscapture_flag = if capture { 1 } else { 0 };
            let device_id = sys::SDL_OpenAudioDevice(
                device_ptr as *const c_char,
                iscapture_flag,
                &desired,
                obtained.as_mut_ptr(),
                0,
            );
            match device_id {
                0 => Err(get_error()),
                id => {
                    let obtained = obtained.assume_init();
                    let device_id = AudioDeviceID::PlaybackDevice(id);
                    let spec = AudioSpec::convert_from_ll(obtained);

                    *userdata = Some(get_callback(spec));

                    Ok(AudioDevice {
                        subsystem: a.clone(),
                        device_id,
                        userdata,
                        spec,
                    })
                }
            }
        }
    }

    /// Opens a new audio device for playback (given the desired parameters and callback).
    ///
    /// If you want to modify the callback-owned data at a later point (for example to update
    /// its data buffer) you're likely to be interested in the [lock method](#method.lock).
    pub fn open_playback<'a, F, D>(
        a: &AudioSubsystem,
        device: D,
        spec: &AudioSpecDesired,
        get_callback: F,
    ) -> Result<AudioDevice<CB>, String>
    where
        F: FnOnce(AudioSpec) -> CB,
        D: Into<Option<&'a str>>,
    {
        AudioDevice::open(a, device, spec, get_callback, false)
    }

    /// Opens a new audio device for capture (given the desired parameters and callback).
    /// Supported since SDL 2.0.5
    ///
    /// If you want to modify the callback-owned data at a later point (for example to update
    /// its data buffer) you're likely to be interested in the [lock method](#method.lock).
    pub fn open_capture<'a, F, D>(
        a: &AudioSubsystem,
        device: D,
        spec: &AudioSpecDesired,
        get_callback: F,
    ) -> Result<AudioDevice<CB>, String>
    where
        F: FnOnce(AudioSpec) -> CB,
        D: Into<Option<&'a str>>,
    {
        AudioDevice::open(a, device, spec, get_callback, true)
    }

    #[inline]
    #[doc(alias = "SDL_GetAudioDeviceStatus")]
    pub fn subsystem(&self) -> &AudioSubsystem {
        &self.subsystem
    }

    #[inline]
    pub fn spec(&self) -> &AudioSpec {
        &self.spec
    }

    pub fn status(&self) -> AudioStatus {
        use std::convert::TryFrom;
        unsafe {
            let status = sys::SDL_GetAudioDeviceStatus(self.device_id.id());
            AudioStatus::try_from(status as u32).unwrap()
        }
    }

    /// Pauses playback of the audio device.
    #[doc(alias = "SDL_PauseAudioDevice")]
    pub fn pause(&self) {
        unsafe { sys::SDL_PauseAudioDevice(self.device_id.id(), 1) }
    }

    /// Starts playback of the audio device.
    #[doc(alias = "SDL_PauseAudioDevice")]
    pub fn resume(&self) {
        unsafe { sys::SDL_PauseAudioDevice(self.device_id.id(), 0) }
    }

    /// Locks the audio device using `SDL_LockAudioDevice`.
    ///
    /// When the returned lock guard is dropped, `SDL_UnlockAudioDevice` is
    /// called.
    /// Use this method to read and mutate callback data.
    #[doc(alias = "SDL_LockAudioDevice")]
    pub fn lock(&mut self) -> AudioDeviceLockGuard<CB> {
        unsafe { sys::SDL_LockAudioDevice(self.device_id.id()) };
        AudioDeviceLockGuard {
            device: self,
            _nosend: PhantomData,
        }
    }

    /// Closes the audio device and saves the callback data from being dropped.
    ///
    /// Note that simply dropping `AudioDevice` will close the audio device,
    /// but the callback data will be dropped.
    pub fn close_and_get_callback(self) -> CB {
        drop(self.device_id);
        self.userdata.expect("Missing callback")
    }
}
