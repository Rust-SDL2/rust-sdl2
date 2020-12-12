use std::ptr;

use libc::{c_int, c_void};

use super::super::{AudioCallback, AudioFormatNum};

#[derive(Clone)]
pub struct AudioSpecDesired {
    /// DSP frequency (samples per second). Set to None for the device's fallback frequency.
    pub freq: Option<i32>,
    /// Number of separate audio channels. Set to None for the device's fallback number of channels.
    pub channels: Option<u8>,
    /// Audio buffer size in samples (power of 2). Set to None for the device's fallback sample size.
    pub samples: Option<u16>,
}

impl AudioSpecDesired {
    pub(in crate::audio) fn convert_to_ll<CB, F, C, S>(
        freq: F,
        channels: C,
        samples: S,
        userdata: *mut Option<CB>,
    ) -> sys::SDL_AudioSpec
    where
        CB: AudioCallback,
        F: Into<Option<i32>>,
        C: Into<Option<u8>>,
        S: Into<Option<u16>>,
    {
        let freq = freq.into();
        let channels = channels.into();
        let samples = samples.into();

        if let Some(freq) = freq {
            assert!(freq > 0);
        }
        if let Some(channels) = channels {
            assert!(channels > 0);
        }
        if let Some(samples) = samples {
            assert!(samples > 0);
        }

        // A value of 0 means "fallback" or "default".

        sys::SDL_AudioSpec {
            freq: freq.unwrap_or(0),
            format: <CB::Channel as AudioFormatNum>::audio_format().to_ll(),
            channels: channels.unwrap_or(0),
            silence: 0,
            samples: samples.unwrap_or(0),
            padding: 0,
            size: 0,
            callback: Some(
                audio_callback_marshall::<CB>
                    as extern "C" fn(arg1: *mut c_void, arg2: *mut u8, arg3: c_int),
            ),
            userdata: userdata as *mut _,
        }
    }

    pub(in crate::audio) fn convert_queue_to_ll<Channel, F, C, S>(
        freq: F,
        channels: C,
        samples: S,
    ) -> sys::SDL_AudioSpec
    where
        Channel: AudioFormatNum,
        F: Into<Option<i32>>,
        C: Into<Option<u8>>,
        S: Into<Option<u16>>,
    {
        let freq = freq.into();
        let channels = channels.into();
        let samples = samples.into();

        if let Some(freq) = freq {
            assert!(freq > 0);
        }
        if let Some(channels) = channels {
            assert!(channels > 0);
        }
        if let Some(samples) = samples {
            assert!(samples > 0);
        }

        // A value of 0 means "fallback" or "default".

        sys::SDL_AudioSpec {
            freq: freq.unwrap_or(0),
            format: <Channel as AudioFormatNum>::audio_format().to_ll(),
            channels: channels.unwrap_or(0),
            silence: 0,
            samples: samples.unwrap_or(0),
            padding: 0,
            size: 0,
            callback: None,
            userdata: ptr::null_mut(),
        }
    }
}

extern "C" fn audio_callback_marshall<CB: AudioCallback>(
    userdata: *mut c_void,
    stream: *mut u8,
    len: c_int,
) {
    use std::mem::size_of;
    use std::slice::from_raw_parts_mut;
    unsafe {
        let cb_userdata: &mut Option<CB> = &mut *(userdata as *mut _);
        let buf: &mut [CB::Channel] = from_raw_parts_mut(
            stream as *mut CB::Channel,
            len as usize / size_of::<CB::Channel>(),
        );

        if let Some(cb) = cb_userdata {
            cb.callback(buf);
        }
    }
}
