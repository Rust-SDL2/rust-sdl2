use libc::{c_char, c_void};
use std::ffi::CString;
use std::marker::PhantomData;
use std::{mem, ptr};

use crate::get_error;
use crate::AudioSubsystem;

use crate::sys;

use super::device::AudioDeviceID;
use super::{AudioFormatNum, AudioSpec, AudioSpecDesired, AudioStatus};

/// Wraps `SDL_AudioDeviceID` and owns the callback data used by the audio device.
pub struct AudioQueue<Channel: AudioFormatNum> {
    subsystem: AudioSubsystem,
    device_id: AudioDeviceID,
    phantom: PhantomData<Channel>,
    spec: AudioSpec,
}

impl<'a, Channel: AudioFormatNum> AudioQueue<Channel> {
    /// Opens a new audio device given the desired parameters and callback.
    #[doc(alias = "SDL_OpenAudioDevice")]
    pub fn open_queue<D: Into<Option<&'a str>>>(
        a: &AudioSubsystem,
        device: D,
        spec: &AudioSpecDesired,
    ) -> Result<AudioQueue<Channel>, String> {
        use std::mem::MaybeUninit;

        let desired = AudioSpecDesired::convert_queue_to_ll::<
            Channel,
            Option<i32>,
            Option<u8>,
            Option<u16>,
        >(spec.freq, spec.channels, spec.samples);

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

            let iscapture_flag = 0;
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

                    Ok(AudioQueue {
                        subsystem: a.clone(),
                        device_id,
                        phantom: PhantomData::default(),
                        spec,
                    })
                }
            }
        }
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

    /// Adds data to the audio queue.
    #[doc(alias = "SDL_QueueAudio")]
    pub fn queue(&self, data: &[Channel]) -> bool {
        let result = unsafe {
            sys::SDL_QueueAudio(
                self.device_id.id(),
                data.as_ptr() as *const c_void,
                (data.len() * mem::size_of::<Channel>()) as u32,
            )
        };
        result == 0
    }

    #[doc(alias = "SDL_GetQueuedAudioSize")]
    pub fn size(&self) -> u32 {
        unsafe { sys::SDL_GetQueuedAudioSize(self.device_id.id()) }
    }

    /// Clears all data from the current audio queue.
    #[doc(alias = "SDL_ClearQueuedAudio")]
    pub fn clear(&self) {
        unsafe {
            sys::SDL_ClearQueuedAudio(self.device_id.id());
        }
    }
}
