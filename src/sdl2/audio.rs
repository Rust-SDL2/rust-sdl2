//! Audio Functions
use std::ffi::{c_str_to_bytes, CString};
use std::num::FromPrimitive;
use libc::{c_int, c_void, uint8_t};
use std::ops::{Deref, DerefMut};

use get_error;
use rwops::RWops;
use SdlResult;

use sys::audio as ll;

pub type AudioFormat = ll::SDL_AudioFormat;

pub const AUDIOU8     : AudioFormat = ll::AUDIO_U8;
pub const AUDIOS8     : AudioFormat = ll::AUDIO_S8;
pub const AUDIOU16LSB : AudioFormat = ll::AUDIO_U16LSB;
pub const AUDIOS16LSB : AudioFormat = ll::AUDIO_S16LSB;
pub const AUDIOU16MSB : AudioFormat = ll::AUDIO_U16MSB;
pub const AUDIOS16MSB : AudioFormat = ll::AUDIO_S16MSB;
pub const AUDIOU16    : AudioFormat = ll::AUDIO_U16;
pub const AUDIOS16    : AudioFormat = ll::AUDIO_S16;
pub const AUDIOS32LSB : AudioFormat = ll::AUDIO_S32LSB;
pub const AUDIOS32MSB : AudioFormat = ll::AUDIO_S32MSB;
pub const AUDIOS32    : AudioFormat = ll::AUDIO_S32;
pub const AUDIOF32LSB : AudioFormat = ll::AUDIO_F32LSB;
pub const AUDIOF32MSB : AudioFormat = ll::AUDIO_F32MSB;
pub const AUDIOF32    : AudioFormat = ll::AUDIO_F32;
pub const AUDIOU16SYS : AudioFormat = ll::AUDIO_U16SYS;
pub const AUDIOS16SYS : AudioFormat = ll::AUDIO_S16SYS;
pub const AUDIOS32SYS : AudioFormat = ll::AUDIO_S32SYS;
pub const AUDIOF32SYS : AudioFormat = ll::AUDIO_F32SYS;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Hash, Debug, FromPrimitive)]
pub enum AudioStatus {
    Stopped = ll::SDL_AUDIO_STOPPED as isize,
    Playing = ll::SDL_AUDIO_PLAYING as isize,
    Paused  = ll::SDL_AUDIO_PAUSED  as isize,
}

pub fn get_num_audio_drivers() -> i32 {
    unsafe { ll::SDL_GetNumAudioDrivers() as i32 }
}

pub fn get_audio_driver(index: i32) -> String {
    unsafe {
        let driver = ll::SDL_GetAudioDriver(index as c_int);
        String::from_utf8_lossy(c_str_to_bytes(&driver)).to_string()
    }
}

pub fn get_num_audio_devices(iscapture: i32) -> i32 {
    unsafe { ll::SDL_GetNumAudioDevices(iscapture as c_int) as i32 }
}

pub fn get_audio_device_name(index: i32, iscapture: i32) -> String {
    unsafe {
        let dev_name = ll::SDL_GetAudioDeviceName(index as c_int, iscapture as c_int);
        String::from_utf8_lossy(c_str_to_bytes(&dev_name)).to_string()
    }
}

pub fn audio_init(name: &str) -> SdlResult<()> {
    let buf = CString::from_slice(name.as_bytes()).as_ptr();
    let ret = unsafe { ll::SDL_AudioInit(buf) };

    if ret == 0 {
        Ok(())
    } else {
        Err(get_error())
    }
}

pub fn audio_quit() {
    unsafe { ll::SDL_AudioQuit() }
}

pub fn get_current_audio_driver() -> String {
    unsafe {
        let driver = ll::SDL_GetCurrentAudioDriver();
        String::from_utf8_lossy(c_str_to_bytes(&driver)).to_string()
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
    /// Loads a WAVE from the file path. Uses `SDL_LoadWAV_RW`.
    pub fn load_wav(path: &Path) -> SdlResult<AudioSpecWAV> {
        let ops = try!(RWops::from_file(path, "rb"));
        AudioSpecWAV::load_wav_rw(&ops)
    }

    /// Loads a WAVE from the data source. Uses `SDL_LoadWAV_RW`.
    pub fn load_wav_rw(src: &RWops) -> SdlResult<AudioSpecWAV> {
        use std::mem::uninitialized;
        use std::ptr::null_mut;

        let mut desired = unsafe { uninitialized::<ll::SDL_AudioSpec>() };
        let mut audio_buf: *mut u8 = null_mut();
        let mut audio_len: u32 = 0;
        unsafe {
            let ret = ll::SDL_LoadWAV_RW(src.raw(), 0, &mut desired, &mut audio_buf, &mut audio_len);
            if ret.is_null() {
                Err(get_error())
            } else {
                Ok(AudioSpecWAV {
                    freq: desired.freq,
                    format: desired.format,
                    channels: desired.channels,
                    audio_buf: audio_buf,
                    audio_len: audio_len
                })
            }
        }
    }

    pub fn get_buffer(&self) -> &[u8] {
        use std::raw::Slice;
        use std::mem::transmute;
        unsafe {
            transmute(Slice {
                data: self.audio_buf,
                len: self.audio_len as usize
            })
        }
    }
}

impl Drop for AudioSpecWAV {
    fn drop(&mut self) {
        unsafe { ll::SDL_FreeWAV(self.audio_buf); }
    }
}

pub trait AudioCallback<T>: Send {
    fn callback(&mut self, &mut [T]);
}

/// The userdata as seen by the SDL callback.
struct AudioCallbackUserdata<CB> {
    callback: CB
}

/// A phantom type for retreiving the SDL_AudioFormat of a given generic type.
/// All format types are returned as native-endian.
///
/// Example: `assert_eq!(AudioFormatNum::<f32>::get_audio_format(), ll::AUDIO_F32);``
pub trait AudioFormatNum<T> {
    fn get_audio_format(self) -> ll::SDL_AudioFormat;
    fn zero() -> Self;
}
/// AUDIO_S8
impl AudioFormatNum<i8> for i8 {
    fn get_audio_format(self) -> ll::SDL_AudioFormat { ll::AUDIO_S8 }
    fn zero() -> i8 { 0 }
}
/// AUDIO_U8
impl AudioFormatNum<u8> for u8 {
    fn get_audio_format(self) -> ll::SDL_AudioFormat { ll::AUDIO_U8 }
    fn zero() -> u8 { 0 }
}
/// AUDIO_S16
impl AudioFormatNum<i16> for i16 {
    fn get_audio_format(self) -> ll::SDL_AudioFormat { ll::AUDIO_S16SYS }
    fn zero() -> i16 { 0 }
}
/// AUDIO_U16
impl AudioFormatNum<u16> for u16 {
    fn get_audio_format(self) -> ll::SDL_AudioFormat { ll::AUDIO_U16SYS }
    fn zero() -> u16 { 0 }
}
/// AUDIO_S32
impl AudioFormatNum<i32> for i32 {
    fn get_audio_format(self) -> ll::SDL_AudioFormat { ll::AUDIO_S32SYS }
    fn zero() -> i32 { 0 }
}
/// AUDIO_F32
impl AudioFormatNum<f32> for f32 {
    fn get_audio_format(self) -> ll::SDL_AudioFormat { ll::AUDIO_F32SYS }
    fn zero() -> f32 { 0.0 }
}

extern "C" fn audio_callback_marshall<T: AudioFormatNum<T>, CB: AudioCallback<T>>
(userdata: *const c_void, stream: *const uint8_t, len: c_int) {
    use std::raw::Slice;
    use std::mem::{size_of, transmute};
    unsafe {
        let mut cb_userdata: &mut AudioCallbackUserdata<CB> = transmute(userdata);
        let buf: &mut [T] = transmute(Slice {
            data: stream,
            len: len as usize / size_of::<T>()
        });

        cb_userdata.callback.callback(buf);
    }
}

pub struct AudioSpecDesired<T: AudioFormatNum<T>, CB: AudioCallback<T>> {
    pub freq: i32,
    pub channels: u8,
    pub samples: u16,
    pub callback: CB
}

impl<T: AudioFormatNum<T>, CB: AudioCallback<T>> AudioSpecDesired<T, CB> {
    fn convert_to_ll(freq: i32, channels: u8, samples: u16, userdata: &mut AudioCallbackUserdata<CB>) -> ll::SDL_AudioSpec {
        use std::mem::transmute;

        let format_num: T = AudioFormatNum::zero();

        unsafe {
            ll::SDL_AudioSpec {
                freq: freq,
                format: format_num.get_audio_format(),
                channels: channels,
                silence: 0,
                samples: samples,
                padding: 0,
                size: 0,
                callback: Some(audio_callback_marshall::<T, CB>
                    as extern "C" fn
                        (arg1: *const c_void,
                         arg2: *const uint8_t,
                         arg3: c_int)),
                userdata: transmute(userdata)
            }
        }
    }

    fn callback_to_userdata(callback: CB) -> Box<AudioCallbackUserdata<CB>> {
        Box::new(AudioCallbackUserdata {
            callback: callback
        })
    }

    /// Opens a new audio device given the desired parameters and callback.
    /// Uses `SDL_OpenAudioDevice`.
    pub fn open_audio_device(self, device: Option<&str>, iscapture: bool) -> SdlResult<AudioDevice<CB>> {
        use std::mem::uninitialized;
        use std::ptr::null;
        use libc::c_char;

        let mut userdata = AudioSpecDesired::callback_to_userdata(self.callback);
        let desired = AudioSpecDesired::convert_to_ll(self.freq, self.channels, self.samples, &mut *userdata);

        let mut obtained = unsafe { uninitialized::<ll::SDL_AudioSpec>() };
        unsafe {
            let device_cstr: Option<CString> = match device {
                None => None,
                Some(d) => Some(CString::from_slice(d.as_bytes()))
            };
            let device_cstr_ptr: *const c_char = match device_cstr {
                None => null(),
                Some(ref s) => s.as_ptr()
            };
            let iscapture_flag = match iscapture { false => 0, true => 1 };
            let device_id = ll::SDL_OpenAudioDevice(device_cstr_ptr, iscapture_flag, &desired, &mut obtained, 0);
            match device_id {
                0 => {
                    Err(get_error())
                },
                id => {
                    let device_id = match iscapture {
                        true => AudioDeviceID::RecordingDevice(id),
                        false => AudioDeviceID::PlaybackDevice(id)
                    };

                    let spec = AudioSpec::convert_from_ll(obtained);

                    Ok(AudioDevice {
                        device_id: device_id,
                        spec: spec,
                        userdata: userdata
                    })
                }
            }
        }
    }
}

#[allow(missing_copy_implementations)]
#[derive(Debug)]
pub struct AudioSpec {
    pub freq: i32,
    // TODO: Showing format should be prettier
    pub format: AudioFormat,
    pub channels: u8,
    pub silence: u8,
    pub samples: u16,
    pub size: u32
}

impl AudioSpec {
    fn convert_from_ll(spec: ll::SDL_AudioSpec) -> AudioSpec {
        AudioSpec {
            freq: spec.freq,
            format: spec.format,
            channels: spec.channels,
            silence: spec.silence,
            samples: spec.samples,
            size: spec.size
        }
    }
}

enum AudioDeviceID {
    PlaybackDevice(ll::SDL_AudioDeviceID),
    RecordingDevice(ll::SDL_AudioDeviceID)
}

impl AudioDeviceID {
    fn id(&self) -> ll::SDL_AudioDeviceID {
        match self {
            &AudioDeviceID::PlaybackDevice(id)  => id,
            &AudioDeviceID::RecordingDevice(id) => id
        }
    }
}

impl Drop for AudioDeviceID {
    fn drop(&mut self) {
        //! Shut down audio processing and close the audio device.
        unsafe { ll::SDL_CloseAudioDevice(self.id()) }
    }
}

/// Wraps SDL_AudioDeviceID and owns the callback data used by the audio device.
pub struct AudioDevice<CB> {
    device_id: AudioDeviceID,
    /// Every audio device corresponds to an SDL_AudioSpec.
    spec: AudioSpec,
    /// Store the callback to keep it alive for the entire duration of `AudioDevice`.
    userdata: Box<AudioCallbackUserdata<CB>>
}

impl<CB> AudioDevice<CB> {
    pub fn get_status(&self) -> AudioStatus {
        unsafe {
            let status = ll::SDL_GetAudioDeviceStatus(self.device_id.id());
            FromPrimitive::from_i32(status as i32).unwrap()
        }
    }

    /// Get the obtained AudioSpec of the audio device.
    pub fn get_spec(&self) -> &AudioSpec { &self.spec }

    /// Pauses playback of the audio device.
    pub fn pause(&self) {
        unsafe { ll::SDL_PauseAudioDevice(self.device_id.id(), 1) }
    }

    /// Starts playback of the audio device.
    pub fn resume(&self) {
        unsafe { ll::SDL_PauseAudioDevice(self.device_id.id(), 0) }
    }

    /// Locks the audio device using `SDL_LockAudioDevice`.
    ///
    /// When the returned lock guard is dropped, `SDL_UnlockAudioDevice` is
    /// called.
    /// Use this method to read and mutate callback data.
    pub fn lock<'a>(&'a mut self) -> AudioDeviceLockGuard<'a, CB> {
        unsafe { ll::SDL_LockAudioDevice(self.device_id.id()) };
        AudioDeviceLockGuard {
            device: self
        }
    }

    /// Closes the audio device and saves the callback data from being dropped.
    ///
    /// Note that simply dropping `AudioDevice` will close the audio device,
    /// but the callback data will be dropped.
    pub fn close_and_get_callback(self) -> CB {
        drop(self.device_id);
        self.userdata.callback
    }
}

/// Similar to `std::sync::MutexGuard`, but for use with `AudioDevice::lock()`.
pub struct AudioDeviceLockGuard<'a, CB: 'a> {
    device: &'a mut AudioDevice<CB>
}

impl<'a, CB: 'a> !Send for AudioDeviceLockGuard<'a, CB> {}

impl<'a, CB: 'a> Deref for AudioDeviceLockGuard<'a, CB> {
    type Target = CB;
    fn deref(&self) -> &CB { &self.device.userdata.callback }
}

impl<'a, CB: 'a> DerefMut for AudioDeviceLockGuard<'a, CB> {
    fn deref_mut(&mut self) -> &mut CB { &mut self.device.userdata.callback }
}

#[unsafe_destructor]
impl<'a, CB> Drop for AudioDeviceLockGuard<'a, CB> {
    fn drop(&mut self) {
        unsafe { ll::SDL_UnlockAudioDevice(self.device.device_id.id()) }
    }
}

#[derive(Copy)]
pub struct AudioCVT {
    raw: ll::SDL_AudioCVT
}

impl AudioCVT {
    pub fn new(src_format: ll::SDL_AudioFormat, src_channels: u8, src_rate: i32,
               dst_format: ll::SDL_AudioFormat, dst_channels: u8, dst_rate: i32) -> SdlResult<AudioCVT>
    {
        use std::mem;
        unsafe {
            let mut raw: ll::SDL_AudioCVT = mem::uninitialized();
            let ret = ll::SDL_BuildAudioCVT(&mut raw,
                                            src_format, src_channels, src_rate as c_int,
                                            dst_format, dst_channels, dst_rate as c_int);
            if ret == 1 || ret == 0 {
                Ok(AudioCVT { raw: raw })
            } else {
                Err(get_error())
            }
        }
    }

    #[unstable="Certain conversions may cause buffer overflows. See AngryLawyer/rust-sdl2 issue #270."]
    pub fn convert(&self, mut src: Vec<u8>) -> Vec<u8> {
        //! Convert audio data to a desired audio format.
        //!
        //! The `src` vector is adjusted to the capacity necessary to perform
        //! the conversion in place; then it is passed to the SDL library.
        use std::num;
        unsafe {
            if self.raw.needed != 0 {
                let mut raw = self.raw;

                // calculate the size of the dst buffer
                raw.len = num::cast(src.len()).expect("Buffer length overflow");
                let dst_size = self.get_capacity(src.len());
                let needed = dst_size - src.len();
                src.reserve_exact(needed);

                // perform the conversion in place
                raw.buf = src.as_mut_ptr();
                let ret = ll::SDL_ConvertAudio(&mut raw);
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
    pub fn get_capacity(&self, src_len: usize) -> usize {
        use std::num::Int;
        src_len.checked_mul(self.raw.len_mult as usize).expect("Integer overflow")
    }
}


#[cfg(test)]
mod test {
    use super::{AudioCVT, AUDIOU8};

    #[test]
    fn test_audio_cvt() {
        use std::iter::repeat;

        // 0,1,2,3, ...
        let buffer: Vec<u8> = range(0, 255).collect();

        // 0,0,1,1,2,2,3,3, ...
        let new_buffer_expected: Vec<u8> = range(0, 255).flat_map(|v| repeat(v).take(2)).collect();

        let cvt = AudioCVT::new(AUDIOU8, 1, 44100, AUDIOU8, 2, 44100).unwrap();
        assert!(cvt.is_conversion_needed());
        assert_eq!(cvt.get_capacity(255), 255*2);

        let new_buffer = cvt.convert(buffer);
        assert_eq!(new_buffer.len(), new_buffer_expected.len());
        assert_eq!(new_buffer, new_buffer_expected);
    }
}
