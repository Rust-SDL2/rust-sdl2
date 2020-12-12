//! Audio Functions
//!
//! # Example
//! ```no_run
//! use sdl2::audio::{AudioCallback, AudioSpecDesired};
//! use std::time::Duration;
//!
//! struct SquareWave {
//!     phase_inc: f32,
//!     phase: f32,
//!     volume: f32
//! }
//!
//! impl AudioCallback for SquareWave {
//!     type Channel = f32;
//!
//!     fn callback(&mut self, out: &mut [f32]) {
//!         // Generate a square wave
//!         for x in out.iter_mut() {
//!             *x = if self.phase <= 0.5 {
//!                 self.volume
//!             } else {
//!                 -self.volume
//!             };
//!             self.phase = (self.phase + self.phase_inc) % 1.0;
//!         }
//!     }
//! }
//!
//! let sdl_context = sdl2::init().unwrap();
//! let audio_subsystem = sdl_context.audio().unwrap();
//!
//! let desired_spec = AudioSpecDesired {
//!     freq: Some(44100),
//!     channels: Some(1),  // mono
//!     samples: None       // default sample size
//! };
//!
//! let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
//!     // initialize the audio callback
//!     SquareWave {
//!         phase_inc: 440.0 / spec.freq as f32,
//!         phase: 0.0,
//!         volume: 0.25
//!     }
//! }).unwrap();
//!
//! // Start playback
//! device.resume();
//!
//! // Play for 2 seconds
//! std::thread::sleep(Duration::from_millis(2000));
//! ```

use libc::c_int;
use std::ffi::CStr;

use crate::get_error;
use crate::AudioSubsystem;

use crate::sys;

mod cvt;
pub use self::cvt::AudioCVT;
mod device;
pub use self::device::AudioDevice;
mod driver;
pub use self::driver::*;
mod format;
pub use self::format::*;
mod queue;
pub use self::queue::AudioQueue;
mod spec;
pub use self::spec::*;
mod status;
pub use self::status::AudioStatus;

impl AudioSubsystem {
    /// Opens a new audio device given the desired parameters and callback.
    ///
    /// If you want to modify the callback-owned data at a later point (for example to update
    /// its data buffer) you're likely to be interested in the
    /// [AudioDevice.lock method](audio/struct.AudioDevice.html#method.lock).
    #[inline]
    pub fn open_playback<'a, CB, F, D>(
        &self,
        device: D,
        spec: &AudioSpecDesired,
        get_callback: F,
    ) -> Result<AudioDevice<CB>, String>
    where
        CB: AudioCallback,
        F: FnOnce(AudioSpec) -> CB,
        D: Into<Option<&'a str>>,
    {
        AudioDevice::open_playback(self, device, spec, get_callback)
    }

    /// Opens a new audio device for capture (given the desired parameters and callback).
    /// Supported since SDL 2.0.5
    ///
    /// If you want to modify the callback-owned data at a later point (for example to update
    /// its data buffer) you're likely to be interested in the
    /// [AudioDevice.lock method](audio/struct.AudioDevice.html#method.lock).
    pub fn open_capture<'a, CB, F, D>(
        &self,
        device: D,
        spec: &AudioSpecDesired,
        get_callback: F,
    ) -> Result<AudioDevice<CB>, String>
    where
        CB: AudioCallback,
        F: FnOnce(AudioSpec) -> CB,
        D: Into<Option<&'a str>>,
    {
        AudioDevice::open_capture(self, device, spec, get_callback)
    }

    /// Opens a new audio device which uses queueing rather than older callback method.
    #[inline]
    pub fn open_queue<'a, Channel, D>(
        &self,
        device: D,
        spec: &AudioSpecDesired,
    ) -> Result<AudioQueue<Channel>, String>
    where
        Channel: AudioFormatNum,
        D: Into<Option<&'a str>>,
    {
        AudioQueue::open_queue(self, device, spec)
    }

    #[doc(alias = "SDL_GetCurrentAudioDriver")]
    pub fn current_audio_driver(&self) -> &'static str {
        unsafe {
            let buf = sys::SDL_GetCurrentAudioDriver();
            assert!(!buf.is_null());

            CStr::from_ptr(buf as *const _).to_str().unwrap()
        }
    }

    #[doc(alias = "SDL_GetNumAudioDevices")]
    pub fn num_audio_playback_devices(&self) -> Option<u32> {
        let result = unsafe { sys::SDL_GetNumAudioDevices(0) };
        if result < 0 {
            // SDL cannot retrieve a list of audio devices. This is not necessarily an error (see the SDL2 docs).
            None
        } else {
            Some(result as u32)
        }
    }

    #[doc(alias = "SDL_GetAudioDeviceName")]
    pub fn audio_playback_device_name(&self, index: u32) -> Result<String, String> {
        unsafe {
            let dev_name = sys::SDL_GetAudioDeviceName(index as c_int, 0);
            if dev_name.is_null() {
                Err(get_error())
            } else {
                let cstr = CStr::from_ptr(dev_name as *const _);
                Ok(cstr.to_str().unwrap().to_owned())
            }
        }
    }
}

pub trait AudioCallback: Send
where
    Self::Channel: AudioFormatNum + 'static,
{
    type Channel;

    fn callback(&mut self, _: &mut [Self::Channel]);
}

#[cfg(test)]
mod test {
    use super::{AudioCVT, AudioFormat};

    #[test]
    fn test_audio_cvt() {
        use std::iter::repeat;

        // 0,1,2,3, ...
        let buffer: Vec<u8> = (0..255).collect();

        // 0,0,1,1,2,2,3,3, ...
        let new_buffer_expected: Vec<u8> = (0..255).flat_map(|v| repeat(v).take(2)).collect();

        let cvt = AudioCVT::new(AudioFormat::U8, 1, 44100, AudioFormat::U8, 2, 44100).unwrap();
        assert!(cvt.is_conversion_needed());

        // since we're going from mono to stereo, our capacity must be at least twice the original (255) vec size
        assert!(
            cvt.capacity(255) >= 255 * 2,
            "capacity must be able to hold the converted audio sample"
        );

        let new_buffer = cvt.convert(buffer);
        assert_eq!(
            new_buffer.len(),
            new_buffer_expected.len(),
            "capacity must be exactly equal to twice the original vec size"
        );

        // // this has been commented, see https://discourse.libsdl.org/t/change-of-behavior-in-audiocvt-sdl-convertaudio-from-2-0-5-to-2-0-6/24682
        // // to maybe re-enable it someday
        // assert_eq!(new_buffer, new_buffer_expected);
    }
}
