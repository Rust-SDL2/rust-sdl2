//! Audio Functions

use std::ptr;
use std::mem;
use std::c_str::CString;
use std::c_vec::CVec;
use libc;
use libc::{c_int, size_t, c_void};
use libc::{uint8_t};
use rustrt::task::Task;

use get_error;
use rwops::RWops;
use SdlResult;


#[allow(non_camel_case_types)]
pub mod ll {
    use libc::{c_int, c_uint, c_void, uint8_t, uint32_t};
    use libc::{uint16_t, c_double, c_char};
    use rwops::ll::SDL_RWops;

    // assume LSB
    pub type SDL_AudioFormat = uint16_t;
    pub const AUDIO_U8 : SDL_AudioFormat =         0x0008;
    pub const AUDIO_S8 : SDL_AudioFormat =         0x8008;
    pub const AUDIO_U16LSB : SDL_AudioFormat =     0x0010;
    pub const AUDIO_S16LSB : SDL_AudioFormat =     0x8010;
    pub const AUDIO_U16MSB : SDL_AudioFormat =     0x1010;
    pub const AUDIO_S16MSB : SDL_AudioFormat =     0x9010;
    pub const AUDIO_U16 : SDL_AudioFormat =        AUDIO_U16LSB;
    pub const AUDIO_S16 : SDL_AudioFormat =        AUDIO_S16LSB;
    pub const AUDIO_S32LSB : SDL_AudioFormat =     0x8020;
    pub const AUDIO_S32MSB : SDL_AudioFormat =     0x9020;
    pub const AUDIO_S32 : SDL_AudioFormat =        AUDIO_S32LSB;
    pub const AUDIO_F32LSB : SDL_AudioFormat =     0x8120;
    pub const AUDIO_F32MSB : SDL_AudioFormat =     0x9120;
    pub const AUDIO_F32 : SDL_AudioFormat =        AUDIO_F32LSB;
    pub const AUDIO_U16SYS : SDL_AudioFormat =     AUDIO_U16LSB;
    pub const AUDIO_S16SYS : SDL_AudioFormat =     AUDIO_S16LSB;
    pub const AUDIO_S32SYS : SDL_AudioFormat =     AUDIO_S32LSB;
    pub const AUDIO_F32SYS : SDL_AudioFormat =     AUDIO_F32LSB;

    pub type SDL_AudioCallback =
        ::std::option::Option<extern "C" fn
                                  (arg1: *const c_void, arg2: *const uint8_t,
                                   arg3: c_int)>;
    #[allow(missing_copy_implementations)]
    #[repr(C)]
    pub struct SDL_AudioSpec {
        pub freq: c_int,
        pub format: SDL_AudioFormat,
        pub channels: uint8_t,
        pub silence: uint8_t,
        pub samples: uint16_t,
        pub padding: uint16_t,
        pub size: uint32_t,
        pub callback: SDL_AudioCallback,
        pub userdata: *const c_void,
    }
    pub type SDL_AudioFilter =
        ::std::option::Option<extern "C" fn
                                  (arg1: *const SDL_AudioCVT,
                                   arg2: SDL_AudioFormat)>;
    #[allow(dead_code, missing_copy_implementations)]
    #[repr(C)]
    pub struct SDL_AudioCVT {
        pub needed: c_int,
        pub src_format: SDL_AudioFormat,
        pub dst_format: SDL_AudioFormat,
        pub rate_incr: c_double,
        pub buf: *mut uint8_t,
        pub len: c_int,
        pub len_cvt: c_int,
        pub len_mult: c_int,
        pub len_ratio: c_double,
        filters: [SDL_AudioFilter, ..10u],
        filter_index: c_int,
    }
    pub type SDL_AudioDeviceID = uint32_t;
    pub type SDL_AudioStatus = c_uint;
    pub const SDL_AUDIO_STOPPED: c_uint = 0;
    pub const SDL_AUDIO_PLAYING: c_uint = 1;
    pub const SDL_AUDIO_PAUSED: c_uint = 2;
    extern "C" {
        pub fn SDL_GetNumAudioDrivers() -> c_int;
        pub fn SDL_GetAudioDriver(index: c_int) -> *const c_char;
        pub fn SDL_AudioInit(driver_name: *const c_char) -> c_int;
        pub fn SDL_AudioQuit();
        pub fn SDL_GetCurrentAudioDriver() -> *const c_char;
        pub fn SDL_OpenAudio(desired: *const SDL_AudioSpec,
                             obtained: *const SDL_AudioSpec) -> c_int;
        pub fn SDL_GetNumAudioDevices(iscapture: c_int) -> c_int;
        pub fn SDL_GetAudioDeviceName(index: c_int, iscapture: c_int) -> *const c_char;
        pub fn SDL_OpenAudioDevice(device: *const c_char, iscapture: c_int,
                                   desired: *const SDL_AudioSpec,
                                   obtained: *mut SDL_AudioSpec,
                                   allowed_changes: c_int) -> SDL_AudioDeviceID;
        pub fn SDL_GetAudioStatus() -> SDL_AudioStatus;
        pub fn SDL_GetAudioDeviceStatus(dev: SDL_AudioDeviceID) ->
            SDL_AudioStatus;
        pub fn SDL_PauseAudio(pause_on: c_int);
        pub fn SDL_PauseAudioDevice(dev: SDL_AudioDeviceID, pause_on: c_int);
        pub fn SDL_LoadWAV_RW(src: *const SDL_RWops, freesrc: c_int,
                              spec: *mut SDL_AudioSpec,
                              audio_buf: *mut *mut uint8_t, audio_len: *mut uint32_t) -> *mut SDL_AudioSpec;
        pub fn SDL_FreeWAV(audio_buf: *mut uint8_t);
        pub fn SDL_BuildAudioCVT(cvt: *mut SDL_AudioCVT,
                                 src_format: SDL_AudioFormat, src_channels: uint8_t,
                                 src_rate: c_int, dst_format: SDL_AudioFormat,
                                 dst_channels: uint8_t, dst_rate: c_int) -> c_int;
        pub fn SDL_ConvertAudio(cvt: *mut SDL_AudioCVT) -> c_int;
        pub fn SDL_MixAudio(dst: *const uint8_t, src: *const uint8_t, len: uint32_t,
                            volume: c_int);
        pub fn SDL_MixAudioFormat(dst: *const uint8_t, src: *const uint8_t,
                                  format: SDL_AudioFormat, len: uint32_t,
                                  volume: c_int);
        pub fn SDL_LockAudio();
        pub fn SDL_LockAudioDevice(dev: SDL_AudioDeviceID);
        pub fn SDL_UnlockAudio();
        pub fn SDL_UnlockAudioDevice(dev: SDL_AudioDeviceID);
        pub fn SDL_CloseAudio();
        pub fn SDL_CloseAudioDevice(dev: SDL_AudioDeviceID);
    }

}

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
#[deriving(Copy, Clone, PartialEq, Hash, Show, FromPrimitive)]
pub enum AudioStatus {
    Stopped = ll::SDL_AUDIO_STOPPED as int,
    Playing = ll::SDL_AUDIO_PLAYING as int,
    Paused  = ll::SDL_AUDIO_PAUSED  as int,
}

pub fn get_num_audio_drivers() -> int {
    unsafe { ll::SDL_GetNumAudioDrivers() as int }
}

pub fn get_audio_driver(index: int) -> String {
    unsafe {
        let buf = ll::SDL_GetAudioDriver(index as c_int);
        CString::new(buf, false).as_str().unwrap().into_string()
    }
}

pub fn get_num_audio_devices(iscapture: int) -> int {
    unsafe { ll::SDL_GetNumAudioDevices(iscapture as c_int) as int }
}

pub fn get_audio_device_name(index: int, iscapture: int) -> String {
    unsafe {
        let buf = ll::SDL_GetAudioDeviceName(index as c_int, iscapture as c_int);
        CString::new(buf, false).as_str().unwrap().into_string()
    }
}

pub fn audio_init(name: &str) -> SdlResult<()> {
    let ret = name.with_c_str(|buf| {
            unsafe { ll::SDL_AudioInit(buf) }
        });
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
        let buf = ll::SDL_GetCurrentAudioDriver();
        CString::new(buf, false).as_str().unwrap().into_string()
    }
}

#[deriving(Copy, Clone, Show)]
pub struct AudioSpecWAV {
    pub freq: i32,
    // TODO: Showing format should be prettier
    pub format: AudioFormat,
    pub channels: u8
}

impl AudioSpecWAV {
    /// Loads a WAVE from the file path. Uses `SDL_LoadWAV_RW`.
    pub fn load_wav(path: &Path) -> SdlResult<(AudioSpecWAV, CVec<u8>)> {
        let ops = try!(RWops::from_file(path, "rb"));
        AudioSpecWAV::load_wav_rw(&ops)
    }

    /// Loads a WAVE from the data source. Uses `SDL_LoadWAV_RW`.
    pub fn load_wav_rw(src: &RWops) -> SdlResult<(AudioSpecWAV, CVec<u8>)> {
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
                let v = CVec::new_with_dtor(audio_buf as *mut u8, audio_len as uint, move || {
                    ll::SDL_FreeWAV(audio_buf)
                });

                Ok((AudioSpecWAV {
                    freq: desired.freq,
                    format: desired.format,
                    channels: desired.channels
                }, v))
            }
        }
    }
}

pub trait AudioCallback<T> {
    fn callback(&mut self, &mut [T]);
}

/// The userdata as seen by the SDL callback.
struct AudioCallbackUserdata<CB> {
    task: AudioCallbackTask,
    callback: CB
}

/// A Task is required to use libstd from SDL's audio callback.
struct AudioCallbackTask {
    /// Set to None if there was an error running a previous task.
    task: Option<Box<Task>>
}

impl Drop for AudioCallbackTask {
    /// Destroy the callback task.
    fn drop(&mut self) {
        use rustrt::local::Local;
        use std::mem::replace;

        // Swap out the task with None in order to own it, since drop() only
        // provides a reference.
        match replace(&mut self.task, None) {
            Some(task) => {
                // pop current task
                let old_task = Local::take();

                task.destroy();

                // put task back
                Local::put(old_task);
            },
            None => ()
        };
    }
}

/// A phantom type for retreiving the SDL_AudioFormat of a given generic type.
/// All format types are returned as native-endian.
///
/// Example: `assert_eq!(AudioFormatNum::<f32>::get_audio_format(), ll::AUDIO_F32);``
pub trait AudioFormatNum<T> {
    fn get_audio_format() -> ll::SDL_AudioFormat;
    fn zero() -> Self;
}
/// AUDIO_S8
impl AudioFormatNum<i8> for i8 {
    fn get_audio_format() -> ll::SDL_AudioFormat { ll::AUDIO_S8 }
    fn zero() -> i8 { 0 }
}
/// AUDIO_U8
impl AudioFormatNum<u8> for u8 {
    fn get_audio_format() -> ll::SDL_AudioFormat { ll::AUDIO_U8 }
    fn zero() -> u8 { 0 }
}
/// AUDIO_S16
impl AudioFormatNum<i16> for i16 {
    fn get_audio_format() -> ll::SDL_AudioFormat { ll::AUDIO_S16SYS }
    fn zero() -> i16 { 0 }
}
/// AUDIO_U16
impl AudioFormatNum<u16> for u16 {
    fn get_audio_format() -> ll::SDL_AudioFormat { ll::AUDIO_U16SYS }
    fn zero() -> u16 { 0 }
}
/// AUDIO_S32
impl AudioFormatNum<i32> for i32 {
    fn get_audio_format() -> ll::SDL_AudioFormat { ll::AUDIO_S32SYS }
    fn zero() -> i32 { 0 }
}
/// AUDIO_F32
impl AudioFormatNum<f32> for f32 {
    fn get_audio_format() -> ll::SDL_AudioFormat { ll::AUDIO_F32SYS }
    fn zero() -> f32 { 0.0 }
}

extern "C" fn audio_callback_marshall<T: AudioFormatNum<T>, CB: AudioCallback<T>>
(userdata: *const c_void, stream: *const uint8_t, len: c_int) {
    use std::raw::Slice;
    use std::mem::{replace, size_of, transmute};
    unsafe {
        let mut cb_userdata: &mut AudioCallbackUserdata<CB> = transmute(userdata);
        let buf: &mut [T] = transmute(Slice {
            data: stream,
            len: len as uint / size_of::<T>()
        });

        // Perform a dance to move tasks around without compiler errors
        let new_task = match replace(&mut cb_userdata.task.task, None) {
            Some(task) => {
                let n = task.run(|| {
                    cb_userdata.callback.callback(buf);
                });

                if n.is_destroyed() { None }
                else { Some(n) }
            },
            None => {
                // Last callback had an error. Fill buffer with silence.
                for x in buf.iter_mut() { *x = AudioFormatNum::zero(); }

                None
            }
        };

        replace(&mut cb_userdata.task.task, new_task);
    }
}

pub struct AudioSpecDesired<T: AudioFormatNum<T>, CB: AudioCallback<T>> {
    pub freq: i32,
    pub channels: u8,
    pub callback: CB
}

impl<T: AudioFormatNum<T>, CB: AudioCallback<T>> AudioSpecDesired<T, CB> {
    fn convert_to_ll(freq: i32, channels: u8, userdata: &mut AudioCallbackUserdata<CB>) -> ll::SDL_AudioSpec {
        use std::mem::transmute;

        unsafe {
            ll::SDL_AudioSpec {
                freq: freq,
                format: AudioFormatNum::<T>::get_audio_format(),
                channels: channels,
                silence: 0,
                samples: 0,
                padding: 0,
                size: 0,
                callback: Some(audio_callback_marshall::<T, CB>),
                userdata: transmute(userdata)
            }
        }
    }

    fn callback_to_userdata(callback: CB) -> Box<AudioCallbackUserdata<CB>> {
        let mut task = box Task::new(None, None);
        task.name = Some("SDL audio callback".into_cow());

        box AudioCallbackUserdata {
            task: AudioCallbackTask { task: Some(task) },
            callback: callback
        }
    }

    /// Opens a new audio device given the desired parameters and callback.
    /// Uses `SDL_OpenAudioDevice`.
    pub fn open_audio_device(self, device: Option<&str>, iscapture: bool) -> SdlResult<AudioDevice<CB>> {
        use std::mem::uninitialized;
        use std::ptr::null;
        use std::c_str::CString;
        use libc::c_char;

        let mut userdata = AudioSpecDesired::callback_to_userdata(self.callback);
        let desired = AudioSpecDesired::convert_to_ll(self.freq, self.channels, &mut *userdata);

        let mut obtained = unsafe { uninitialized::<ll::SDL_AudioSpec>() };
        unsafe {
            let device_cstr: Option<CString> = match device {
                None => None,
                Some(d) => Some(d.to_c_str())
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
#[deriving(Show)]
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
            FromPrimitive::from_int(status as int).unwrap()
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

impl<'a, CB> Deref<CB> for AudioDeviceLockGuard<'a, CB> {
    fn deref(&self) -> &CB { &self.device.userdata.callback }
}

impl<'a, CB> DerefMut<CB> for AudioDeviceLockGuard<'a, CB> {
    fn deref_mut(&mut self) -> &mut CB { &mut self.device.userdata.callback }
}

#[unsafe_destructor]
impl<'a, CB> Drop for AudioDeviceLockGuard<'a, CB> {
    fn drop(&mut self) {
        unsafe { ll::SDL_UnlockAudioDevice(self.device.device_id.id()) }
    }
}

#[deriving(PartialEq)] #[allow(raw_pointer_deriving)]
pub struct AudioCVT {
    raw: *mut ll::SDL_AudioCVT,
    owned: bool,
}

impl_raw_accessors!(AudioCVT, *mut ll::SDL_AudioCVT);
impl_owned_accessors!(AudioCVT, owned);

impl Drop for AudioCVT {
    fn drop(&mut self) {
        if self.owned {
            unsafe { libc::free(self.raw as *mut c_void) }
        }
    }
}

impl AudioCVT {
    pub fn new(src_format: ll::SDL_AudioFormat, src_channels: u8, src_rate: int,
               dst_format: ll::SDL_AudioFormat, dst_channels: u8, dst_rate: int) -> SdlResult<AudioCVT> {
        unsafe {
            let c_cvt_p = libc::malloc(mem::size_of::<ll::SDL_AudioCVT>() as size_t) as *mut ll::SDL_AudioCVT;
            let ret = ll::SDL_BuildAudioCVT(c_cvt_p,
                                            src_format, src_channels, src_rate as c_int,
                                            dst_format, dst_channels, dst_rate as c_int);
            if ret == 1 || ret == 0 {
                Ok(AudioCVT { raw: c_cvt_p, owned: true })
            } else {
                Err(get_error())
            }
        }
    }

    pub fn convert(&self, src: CVec<u8>) -> SdlResult<CVec<u8>> {
        //! Convert audio data to a desired audio format.

        unsafe {
            if (*self.raw).needed != 1 {
                return Err("no convertion needed!".into_string())
            }
            // set len
            (*self.raw).len = src.len() as c_int;
            // alloc buf
            let size = (*self.raw).len * (*self.raw).len_mult;
            (*self.raw).buf = libc::malloc(size as size_t) as *mut u8;
            // set buf
            ptr::copy_memory::<u8>((*self.raw).buf, src.as_slice().as_ptr(), src.len());
            // convert
            let ret = ll::SDL_ConvertAudio(self.raw);
            // return
            let p = (*self.raw).buf as *mut c_void; // send to move ||
            if ret == 0 {
                Ok( CVec::new_with_dtor((*self.raw).buf as *mut u8, (*self.raw).len_cvt as uint,
                                        move || { libc::free(p) })
                    )
            } else {
                Err(get_error())
            }
        }
    }
}
