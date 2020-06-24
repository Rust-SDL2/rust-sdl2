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

use std::ffi::{CStr, CString};
use std::convert::TryFrom;
use libc::{c_int, c_void, c_char};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::marker::PhantomData;
use std::mem;
use std::ptr;

use crate::AudioSubsystem;
use crate::get_error;
use crate::rwops::RWops;

use crate::sys;
use crate::sys::SDL_AudioStatus;

impl AudioSubsystem {
    /// Opens a new audio device given the desired parameters and callback.
    ///
    /// If you want to modify the callback-owned data at a later point (for example to update
    /// its data buffer) you're likely to be interested in the
    /// [AudioDevice.lock method](audio/struct.AudioDevice.html#method.lock).
    #[inline]
    pub fn open_playback<'a, CB, F, D>(&self, device: D, spec: &AudioSpecDesired, get_callback: F) -> Result<AudioDevice <CB>, String>
    where CB: AudioCallback, F: FnOnce(AudioSpec) -> CB, D: Into<Option<&'a str>>,
    {
        AudioDevice::open_playback(self, device, spec, get_callback)
    }

    /// Opens a new audio device for capture (given the desired parameters and callback).
    /// Supported since SDL 2.0.5
    ///
    /// If you want to modify the callback-owned data at a later point (for example to update
    /// its data buffer) you're likely to be interested in the
    /// [AudioDevice.lock method](audio/struct.AudioDevice.html#method.lock).
    pub fn open_capture<'a, CB, F, D>(&self, device: D, spec: &AudioSpecDesired, get_callback: F) -> Result<AudioDevice <CB>, String>
        where CB: AudioCallback, F: FnOnce(AudioSpec) -> CB, D: Into<Option<&'a str>>,
    {
        AudioDevice::open_capture(self, device, spec, get_callback)
    }

    /// Opens a new audio device which uses queueing rather than older callback method.
    #[inline]
    pub fn open_queue<'a, Channel, D>(&self, device: D, spec: &AudioSpecDesired) -> Result<AudioQueue<Channel>, String>
    where Channel: AudioFormatNum, D: Into<Option<&'a str>>,
    {
        AudioQueue::open_queue(self, device, spec)
    }

    pub fn current_audio_driver(&self) -> &'static str {
        unsafe {
            let buf = sys::SDL_GetCurrentAudioDriver();
            assert!(!buf.is_null());

            CStr::from_ptr(buf as *const _).to_str().unwrap()
        }
    }

    pub fn num_audio_playback_devices(&self) -> Option<u32> {
        let result = unsafe { sys::SDL_GetNumAudioDevices(0) };
        if result < 0 {
            // SDL cannot retrieve a list of audio devices. This is not necessarily an error (see the SDL2 docs).
            None
        } else {
            Some(result as u32)
        }
    }

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

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AudioFormat {
    /// Unsigned 8-bit samples
    U8 = sys::AUDIO_U8 as i32,
    /// Signed 8-bit samples
    S8 = sys::AUDIO_S8 as i32,
    /// Unsigned 16-bit samples, little-endian
    U16LSB = sys::AUDIO_U16LSB as i32,
    /// Unsigned 16-bit samples, big-endian
    U16MSB = sys::AUDIO_U16MSB as i32,
    /// Signed 16-bit samples, little-endian
    S16LSB = sys::AUDIO_S16LSB as i32,
    /// Signed 16-bit samples, big-endian
    S16MSB = sys::AUDIO_S16MSB as i32,
    /// Signed 32-bit samples, little-endian
    S32LSB = sys::AUDIO_S32LSB as i32,
    /// Signed 32-bit samples, big-endian
    S32MSB = sys::AUDIO_S32MSB as i32,
    /// 32-bit floating point samples, little-endian
    F32LSB = sys::AUDIO_F32LSB as i32,
    /// 32-bit floating point samples, big-endian
    F32MSB = sys::AUDIO_F32MSB as i32
}

impl AudioFormat {
    fn from_ll(raw: sys::SDL_AudioFormat) -> Option<AudioFormat> {
        use self::AudioFormat::*;
        match raw as u32 {
            sys::AUDIO_U8 => Some(U8),
            sys::AUDIO_S8 => Some(S8),
            sys::AUDIO_U16LSB => Some(U16LSB),
            sys::AUDIO_U16MSB => Some(U16MSB),
            sys::AUDIO_S16LSB => Some(S16LSB),
            sys::AUDIO_S16MSB => Some(S16MSB),
            sys::AUDIO_S32LSB => Some(S32LSB),
            sys::AUDIO_S32MSB => Some(S32MSB),
            sys::AUDIO_F32LSB => Some(F32LSB),
            sys::AUDIO_F32MSB => Some(F32MSB),
            _ => None
        }
    }

    fn to_ll(self) -> sys::SDL_AudioFormat {
        self as sys::SDL_AudioFormat
    }
}

#[cfg(target_endian = "little")]
impl AudioFormat {
    /// Unsigned 16-bit samples, native endian
    #[inline] pub const fn u16_sys() -> AudioFormat { AudioFormat::U16LSB }
    /// Signed 16-bit samples, native endian
    #[inline] pub const fn s16_sys() -> AudioFormat { AudioFormat::S16LSB }
    /// Signed 32-bit samples, native endian
    #[inline] pub const fn s32_sys() -> AudioFormat { AudioFormat::S32LSB }
    /// 32-bit floating point samples, native endian
    #[inline] pub const fn f32_sys() -> AudioFormat { AudioFormat::F32LSB }
}

#[cfg(target_endian = "big")]
impl AudioFormat {
    /// Unsigned 16-bit samples, native endian
    #[inline] pub const fn u16_sys() -> AudioFormat { AudioFormat::U16MSB }
    /// Signed 16-bit samples, native endian
    #[inline] pub const fn s16_sys() -> AudioFormat { AudioFormat::S16MSB }
    /// Signed 32-bit samples, native endian
    #[inline] pub const fn s32_sys() -> AudioFormat { AudioFormat::S32MSB }
    /// 32-bit floating point samples, native endian
    #[inline] pub const fn f32_sys() -> AudioFormat { AudioFormat::F32MSB }
}

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AudioStatus {
    Stopped = SDL_AudioStatus::SDL_AUDIO_STOPPED as i32,
    Playing = SDL_AudioStatus::SDL_AUDIO_PLAYING as i32,
    Paused  = SDL_AudioStatus::SDL_AUDIO_PAUSED  as i32,
}

impl TryFrom<u32> for AudioStatus {
    type Error = ();

    fn try_from(n: u32) -> Result<Self, Self::Error> {
        use self::AudioStatus::*;
        use crate::sys::SDL_AudioStatus::*;

        Ok(match unsafe { mem::transmute(n) } {
            SDL_AUDIO_STOPPED => Stopped,
            SDL_AUDIO_PLAYING => Playing,
            SDL_AUDIO_PAUSED  => Paused,
        })
    }
}

#[derive(Copy, Clone)]
pub struct DriverIterator {
    length: i32,
    index: i32
}

impl Iterator for DriverIterator {
    type Item = &'static str;

    #[inline]
    fn next(&mut self) -> Option<&'static str> {
        if self.index >= self.length {
            None
        } else {
            unsafe {
                let buf = sys::SDL_GetAudioDriver(self.index);
                assert!(!buf.is_null());
                self.index += 1;

                Some(CStr::from_ptr(buf as *const _).to_str().unwrap())
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let l = self.length as usize;
        (l, Some(l))
    }
}

impl ExactSizeIterator for DriverIterator { }

/// Gets an iterator of all audio drivers compiled into the SDL2 library.
#[inline]
pub fn drivers() -> DriverIterator {
    // This function is thread-safe and doesn't require the audio subsystem to be initialized.
    // The list of drivers are read-only and statically compiled into SDL2, varying by platform.

    // SDL_GetNumAudioDrivers can never return a negative value.
    DriverIterator {
        length: unsafe { sys::SDL_GetNumAudioDrivers() },
        index: 0
    }
}

pub struct AudioSpecWAV {
    pub freq: i32,
    pub format: AudioFormat,
    pub channels: u8,
    audio_buf: *mut u8,
    audio_len: u32
}

impl AudioSpecWAV {
    /// Loads a WAVE from the file path.
    pub fn load_wav<P: AsRef<Path>>(path: P) -> Result<AudioSpecWAV, String> {
        let mut file = RWops::from_file(path, "rb")?;
        AudioSpecWAV::load_wav_rw(&mut file)
    }

    /// Loads a WAVE from the data source.
    pub fn load_wav_rw(src: &mut RWops) -> Result<AudioSpecWAV, String> {
        use std::mem::MaybeUninit;
        use std::ptr::null_mut;

        let mut desired = MaybeUninit::uninit();
        let mut audio_buf: *mut u8 = null_mut();
        let mut audio_len: u32 = 0;
        unsafe {
            let ret = sys::SDL_LoadWAV_RW(src.raw(), 0, desired.as_mut_ptr(), &mut audio_buf, &mut audio_len);
            if ret.is_null() {
                Err(get_error())
            } else {
                let desired = desired.assume_init();
                Ok(AudioSpecWAV {
                    freq: desired.freq,
                    format: AudioFormat::from_ll(desired.format).unwrap(),
                    channels: desired.channels,
                    audio_buf,
                    audio_len
                })
            }
        }
    }

    pub fn buffer(&self) -> &[u8] {
        use std::slice::from_raw_parts;
        unsafe {
            let ptr = self.audio_buf as *const u8;
            let len = self.audio_len as usize;
            from_raw_parts(ptr, len)
        }
    }
}

impl Drop for AudioSpecWAV {
    fn drop(&mut self) {
        unsafe { sys::SDL_FreeWAV(self.audio_buf); }
    }
}

pub trait AudioCallback: Send
where Self::Channel: AudioFormatNum + 'static
{
    type Channel;

    fn callback(&mut self, _: &mut [Self::Channel]);
}

/// A phantom type for retrieving the `SDL_AudioFormat` of a given generic type.
/// All format types are returned as native-endian.
pub trait AudioFormatNum {
    fn audio_format() -> AudioFormat;

    /// The appropriately typed silence value for the audio format used.
    ///
    /// # Examples
    ///
    /// ```
    /// // The AudioFormatNum trait has to be imported for the Channel::SILENCE part to work.
    /// use sdl2::audio::{AudioCallback, AudioFormatNum};
    ///
    /// struct Silence;
    ///
    /// impl AudioCallback for Silence {
    ///     type Channel = u16;
    ///
    ///     fn callback(&mut self, out: &mut [u16]) {
    ///         for dst in out.iter_mut() {
    ///             *dst = Self::Channel::SILENCE;
    ///         }
    ///     }
    /// }
    /// ```
    const SILENCE: Self;
}

/// `AUDIO_S8`
impl AudioFormatNum for i8 {
    fn audio_format() -> AudioFormat { AudioFormat::S8 }
    const SILENCE: i8 = 0;
}
/// `AUDIO_U8`
impl AudioFormatNum for u8 {
    fn audio_format() -> AudioFormat { AudioFormat::U8 }
    const SILENCE: u8 = 0x80;
}
/// `AUDIO_S16`
impl AudioFormatNum for i16 {
    fn audio_format() -> AudioFormat { AudioFormat::s16_sys() }
    const SILENCE: i16 = 0;
}
/// `AUDIO_U16`
impl AudioFormatNum for u16 {
    fn audio_format() -> AudioFormat { AudioFormat::u16_sys() }
    const SILENCE: u16 = 0x8000;
}
/// `AUDIO_S32`
impl AudioFormatNum for i32 {
    fn audio_format() -> AudioFormat { AudioFormat::s32_sys() }
    const SILENCE: i32 = 0;
}
/// `AUDIO_F32`
impl AudioFormatNum for f32 {
    fn audio_format() -> AudioFormat { AudioFormat::f32_sys() }
    const SILENCE: f32 = 0.0;
}

extern "C" fn audio_callback_marshall<CB: AudioCallback>
(userdata: *mut c_void, stream: *mut u8, len: c_int) {
    use std::slice::from_raw_parts_mut;
    use std::mem::size_of;
    unsafe {
        let cb_userdata: &mut Option<CB> = &mut *(userdata as *mut _);
        let buf: &mut [CB::Channel] = from_raw_parts_mut(
            stream as *mut CB::Channel,
            len as usize / size_of::<CB::Channel>()
        );

        if let Some(cb) = cb_userdata {
            cb.callback(buf);
        }
    }
}

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
    fn convert_to_ll<CB, F, C, S>(freq: F, channels: C, samples: S, userdata: *mut Option<CB>) -> sys::SDL_AudioSpec
    where
        CB: AudioCallback,
        F: Into<Option<i32>>,
        C: Into<Option<u8>>,
        S: Into<Option<u16>>,
    {

        let freq = freq.into();
        let channels = channels.into();
        let samples = samples.into();

        if let Some(freq) = freq { assert!(freq > 0); }
        if let Some(channels) = channels { assert!(channels > 0); }
        if let Some(samples) = samples { assert!(samples > 0); }

        // A value of 0 means "fallback" or "default".

        sys::SDL_AudioSpec {
            freq: freq.unwrap_or(0),
            format: <CB::Channel as AudioFormatNum>::audio_format().to_ll(),
            channels: channels.unwrap_or(0),
            silence: 0,
            samples: samples.unwrap_or(0),
            padding: 0,
            size: 0,
            callback: Some(audio_callback_marshall::<CB>
                    as extern "C" fn
                        (arg1: *mut c_void,
                            arg2: *mut u8,
                            arg3: c_int)),
            userdata: userdata as *mut _,
        }
    }

    fn convert_queue_to_ll<Channel, F, C, S>(freq: F, channels: C, samples: S) -> sys::SDL_AudioSpec
    where
        Channel: AudioFormatNum,
        F: Into<Option<i32>>,
        C: Into<Option<u8>>,
        S: Into<Option<u16>>
    {
        let freq = freq.into();
        let channels = channels.into();
        let samples = samples.into();

        if let Some(freq) = freq { assert!(freq > 0); }
        if let Some(channels) = channels { assert!(channels > 0); }
        if let Some(samples) = samples { assert!(samples > 0); }

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

#[allow(missing_copy_implementations)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AudioSpec {
    pub freq: i32,
    pub format: AudioFormat,
    pub channels: u8,
    /// The silence value calculated by SDL2. Note that it's inconvenient to use if your channel
    /// type is not u8 and [incorrect in case of u16](https://bugzilla.libsdl.org/show_bug.cgi?id=4805).
    /// You're likely to find [the `AudioFormatNum.SILENCE` associated constant](
    /// trait.AudioFormatNum.html#associatedconstant.SILENCE) more useful.
    pub silence: u8,
    pub samples: u16,
    pub size: u32
}

impl AudioSpec {
    fn convert_from_ll(spec: sys::SDL_AudioSpec) -> AudioSpec {
        AudioSpec {
            freq: spec.freq,
            format: AudioFormat::from_ll(spec.format).unwrap(),
            channels: spec.channels,
            silence: spec.silence,
            samples: spec.samples,
            size: spec.size
        }
    }
}

enum AudioDeviceID {
    PlaybackDevice(sys::SDL_AudioDeviceID)
}

impl AudioDeviceID {
    fn id(&self) -> sys::SDL_AudioDeviceID {
        match *self {
            AudioDeviceID::PlaybackDevice(id)  => id
        }
    }
}

impl Drop for AudioDeviceID {
    fn drop(&mut self) {
        //! Shut down audio processing and close the audio device.
        unsafe { sys::SDL_CloseAudioDevice(self.id()) }
    }
}

/// Wraps `SDL_AudioDeviceID` and owns the callback data used by the audio device.
pub struct AudioQueue<Channel: AudioFormatNum> {
    subsystem: AudioSubsystem,
    device_id: AudioDeviceID,
    phantom: PhantomData<Channel>,
    spec: AudioSpec,
}

impl<'a, Channel: AudioFormatNum> AudioQueue<Channel> {
    /// Opens a new audio device given the desired parameters and callback.
    pub fn open_queue<D: Into<Option<&'a str>>>(a: &AudioSubsystem, device: D, spec: &AudioSpecDesired) -> Result<AudioQueue<Channel>, String> {
        use std::mem::MaybeUninit;

        let desired = AudioSpecDesired::convert_queue_to_ll::<Channel, Option<i32>, Option<u8>, Option<u16>>(spec.freq, spec.channels, spec.samples);

        let mut obtained = MaybeUninit::uninit();
        unsafe {
            let device = match device.into() {
                Some(device) => Some(CString::new(device).unwrap()),
                None => None
            };
            // Warning: map_or consumes its argument; `device.map_or()` would therefore consume the
            // CString and drop it, making device_ptr a dangling pointer! To avoid that we downgrade
            // device to an Option<&_> first.
            let device_ptr = device.as_ref().map_or(ptr::null(), |s| s.as_ptr());

            let iscapture_flag = 0;
            let device_id = sys::SDL_OpenAudioDevice(
                device_ptr as *const c_char, iscapture_flag, &desired,
                obtained.as_mut_ptr(), 0
            );
            match device_id {
                0 => {
                    Err(get_error())
                },
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
    pub fn subsystem(&self) -> &AudioSubsystem { &self.subsystem }

    #[inline]
    pub fn spec(&self) -> &AudioSpec { &self.spec }

    pub fn status(&self) -> AudioStatus {
        unsafe {
            let status = sys::SDL_GetAudioDeviceStatus(self.device_id.id());
            AudioStatus::try_from(status as u32).unwrap()
        }
    }

    /// Pauses playback of the audio device.
    pub fn pause(&self) {
        unsafe { sys::SDL_PauseAudioDevice(self.device_id.id(), 1) }
    }

    /// Starts playback of the audio device.
    pub fn resume(&self) {
        unsafe { sys::SDL_PauseAudioDevice(self.device_id.id(), 0) }
    }

    /// Adds data to the audio queue.
    pub fn queue(&self, data: &[Channel]) -> bool {
        let result = unsafe {sys::SDL_QueueAudio(self.device_id.id(), data.as_ptr() as *const c_void, (data.len() * mem::size_of::<Channel>()) as u32)};
        result == 0
    }

    pub fn size(&self) -> u32 {
        unsafe {sys::SDL_GetQueuedAudioSize(self.device_id.id())}
    }

    /// Clears all data from the current audio queue.
    pub fn clear(&self) {
        unsafe {sys::SDL_ClearQueuedAudio(self.device_id.id());}
    }
}

/// Wraps `SDL_AudioDeviceID` and owns the callback data used by the audio device.
pub struct AudioDevice<CB: AudioCallback> {
    subsystem: AudioSubsystem,
    device_id: AudioDeviceID,
    spec: AudioSpec,
    /// Store the callback to keep it alive for the entire duration of `AudioDevice`.
    userdata: Box<Option<CB>>
}

impl<CB: AudioCallback> AudioDevice<CB> {
    /// Opens a new audio device for playback or capture (given the desired parameters and callback).
    fn open<'a, F, D>(a: &AudioSubsystem, device: D, spec: &AudioSpecDesired, get_callback: F, capture: bool) -> Result<AudioDevice <CB>, String>
    where
        F: FnOnce(AudioSpec) -> CB,
        D: Into<Option<&'a str>>,
    {
        use std::mem::MaybeUninit;

        let mut userdata: Box<Option<CB>> = Box::new(None);
        let desired = AudioSpecDesired::convert_to_ll(spec.freq, spec.channels, spec.samples, &mut *userdata);

        let mut obtained = MaybeUninit::uninit();
        unsafe {
            let device = match device.into() {
                Some(device) => Some(CString::new(device).unwrap()),
                None => None
            };
            // Warning: map_or consumes its argument; `device.map_or()` would therefore consume the
            // CString and drop it, making device_ptr a dangling pointer! To avoid that we downgrade
            // device to an Option<&_> first.
            let device_ptr = device.as_ref().map_or(ptr::null(), |s| s.as_ptr());

            let iscapture_flag = if capture { 1 } else { 0 };
            let device_id = sys::SDL_OpenAudioDevice(
                device_ptr as *const c_char, iscapture_flag, &desired,
                obtained.as_mut_ptr(), 0
            );
            match device_id {
                0 => {
                    Err(get_error())
                },
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
    pub fn open_playback<'a, F, D>(a: &AudioSubsystem, device: D, spec: &AudioSpecDesired, get_callback: F) -> Result<AudioDevice <CB>, String>
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
    pub fn open_capture<'a, F, D>(a: &AudioSubsystem, device: D, spec: &AudioSpecDesired, get_callback: F) -> Result<AudioDevice <CB>, String>
        where
            F: FnOnce(AudioSpec) -> CB,
            D: Into<Option<&'a str>>,
    {
        AudioDevice::open(a, device, spec, get_callback, true)
    }

    #[inline]
    pub fn subsystem(&self) -> &AudioSubsystem { &self.subsystem }

    #[inline]
    pub fn spec(&self) -> &AudioSpec { &self.spec }

    pub fn status(&self) -> AudioStatus {
        unsafe {
            let status = sys::SDL_GetAudioDeviceStatus(self.device_id.id());
            AudioStatus::try_from(status as u32).unwrap()
        }
    }

    /// Pauses playback of the audio device.
    pub fn pause(&self) {
        unsafe { sys::SDL_PauseAudioDevice(self.device_id.id(), 1) }
    }

    /// Starts playback of the audio device.
    pub fn resume(&self) {
        unsafe { sys::SDL_PauseAudioDevice(self.device_id.id(), 0) }
    }

    /// Locks the audio device using `SDL_LockAudioDevice`.
    ///
    /// When the returned lock guard is dropped, `SDL_UnlockAudioDevice` is
    /// called.
    /// Use this method to read and mutate callback data.
    pub fn lock(&mut self) -> AudioDeviceLockGuard<CB> {
        unsafe { sys::SDL_LockAudioDevice(self.device_id.id()) };
        AudioDeviceLockGuard {
            device:  self,
            _nosend: PhantomData
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

/// Similar to `std::sync::MutexGuard`, but for use with `AudioDevice::lock()`.
pub struct AudioDeviceLockGuard<'a, CB> where CB: AudioCallback, CB: 'a {
    device: &'a mut AudioDevice<CB>,
    _nosend: PhantomData<*mut ()>
}

impl<'a, CB: AudioCallback> Deref for AudioDeviceLockGuard<'a, CB> {
    type Target = CB;
    fn deref(&self) -> &CB { (*self.device.userdata).as_ref().expect("Missing callback") }
}

impl<'a, CB: AudioCallback> DerefMut for AudioDeviceLockGuard<'a, CB> {
    fn deref_mut(&mut self) -> &mut CB { (*self.device.userdata).as_mut().expect("Missing callback") }
}

impl<'a, CB: AudioCallback> Drop for AudioDeviceLockGuard<'a, CB> {
    fn drop(&mut self) {
        unsafe { sys::SDL_UnlockAudioDevice(self.device.device_id.id()) }
    }
}

#[derive(Copy, Clone)]
pub struct AudioCVT {
    raw: sys::SDL_AudioCVT
}

impl AudioCVT {
    pub fn new(src_format: AudioFormat, src_channels: u8, src_rate: i32,
               dst_format: AudioFormat, dst_channels: u8, dst_rate: i32) -> Result<AudioCVT, String>
    {
        use std::mem::MaybeUninit;

        let mut raw: MaybeUninit<sys::SDL_AudioCVT> = mem::MaybeUninit::uninit();

        unsafe {
            let ret = sys::SDL_BuildAudioCVT(raw.as_mut_ptr(),
                                            src_format.to_ll(), src_channels, src_rate as c_int,
                                            dst_format.to_ll(), dst_channels, dst_rate as c_int);
            if ret == 1 || ret == 0 {
                let raw = raw.assume_init();
                Ok(AudioCVT { raw })
            } else {
                Err(get_error())
            }
        }
    }

    pub fn convert(&self, mut src: Vec<u8>) -> Vec<u8> {
        //! Convert audio data to a desired audio format.
        //!
        //! The `src` vector is adjusted to the capacity necessary to perform
        //! the conversion in place; then it is passed to the SDL library.
        //!
        //! Certain conversions may cause buffer overflows. See AngryLawyer/rust-sdl2 issue #270.
        unsafe {
            if self.raw.needed != 0 {
                let mut raw = self.raw;

                // calculate the size of the dst buffer
                use std::convert::TryInto;
                raw.len = src.len().try_into().expect("Buffer length overflow");
                let dst_size = self.capacity(src.len());
                let needed = dst_size - src.len();
                src.reserve_exact(needed);

                // perform the conversion in place
                raw.buf = src.as_mut_ptr();
                let ret = sys::SDL_ConvertAudio(&mut raw);
                // There's no reason for SDL_ConvertAudio to fail.
                // The only time it can fail is if buf is NULL, which it never is.
                if ret != 0 { panic!(get_error()) }

                // return original buffer back to caller
                debug_assert!(raw.len_cvt > 0);
                debug_assert!(raw.len_cvt as usize <= src.capacity());

                src.set_len(raw.len_cvt as usize);
                src
            } else {
                // The buffer remains unmodified
                src
            }
        }
    }

    /// Checks if any conversion is needed. i.e. if the buffer that goes
    /// into `convert()` is unchanged from the result.
    pub fn is_conversion_needed(&self) -> bool { self.raw.needed != 0 }

    /// Gets the buffer capacity that can contain both the original and
    /// converted data.
    pub fn capacity(&self, src_len: usize) -> usize {
        src_len.checked_mul(self.raw.len_mult as usize).expect("Integer overflow")
    }
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
        assert!(cvt.capacity(255) >= 255*2, "capacity must be able to hold the converted audio sample");

        let new_buffer = cvt.convert(buffer);
        assert_eq!(new_buffer.len(), new_buffer_expected.len(), "capacity must be exactly equal to twice the original vec size");

        // // this has been commented, see https://discourse.libsdl.org/t/change-of-behavior-in-audiocvt-sdl-convertaudio-from-2-0-5-to-2-0-6/24682
        // // to maybe re-enable it someday
        // assert_eq!(new_buffer, new_buffer_expected);
    }
}
