use std::ptr;
use std::mem;
use std::c_str::CString;
use std::c_vec::CVec;
use libc;
use libc::{c_int, size_t, c_void};
use libc::{uint8_t};

use get_error;
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
    #[allow(dead_code)]
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
                                   obtained: *const SDL_AudioSpec,
                                   allowed_changes: c_int) -> SDL_AudioDeviceID;
        pub fn SDL_GetAudioStatus() -> SDL_AudioStatus;
        pub fn SDL_GetAudioDeviceStatus(dev: SDL_AudioDeviceID) ->
            SDL_AudioStatus;
        pub fn SDL_PauseAudio(pause_on: c_int);
        pub fn SDL_PauseAudioDevice(dev: SDL_AudioDeviceID, pause_on: c_int);
        pub fn SDL_LoadWAV_RW(src: *const SDL_RWops, freesrc: c_int,
                              spec: *const SDL_AudioSpec,
                              audio_buf: *const *const uint8_t, audio_len: *const uint32_t) -> *const SDL_AudioSpec;
        pub fn SDL_FreeWAV(audio_buf: *const uint8_t);
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

#[repr(C)]
#[deriving(Clone, PartialEq, Hash, Show, FromPrimitive)]
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

pub trait AudioCallback<T> {
    fn callback(&mut self, &mut [T]);
}

/// A phantom type for retreiving the SDL_AudioFormat of a given generic type.
/// All format types are returned as native-endian.
///
/// Example: `assert_eq!(AudioFormat::<f32>::get_audio_format(), ll::AUDIO_F32);``
pub trait AudioFormat<T> { fn get_audio_format() -> ll::SDL_AudioFormat; }
/// AUDIO_S8
impl AudioFormat<i8> for i8 { fn get_audio_format() -> ll::SDL_AudioFormat { ll::AUDIO_S8 } }
/// AUDIO_U8
impl AudioFormat<u8> for u8 { fn get_audio_format() -> ll::SDL_AudioFormat { ll::AUDIO_U8 } }
/// AUDIO_S16
impl AudioFormat<i16> for i16 { fn get_audio_format() -> ll::SDL_AudioFormat { ll::AUDIO_S16SYS } }
/// AUDIO_U16
impl AudioFormat<u16> for u16 { fn get_audio_format() -> ll::SDL_AudioFormat { ll::AUDIO_U16SYS } }
/// AUDIO_S32
impl AudioFormat<i32> for i32 { fn get_audio_format() -> ll::SDL_AudioFormat { ll::AUDIO_S32SYS } }
/// AUDIO_F32
impl AudioFormat<f32> for f32 { fn get_audio_format() -> ll::SDL_AudioFormat { ll::AUDIO_F32SYS } }

extern "C" fn audio_callback_marshall<T: AudioFormat<T>, CB: AudioCallback<T>>
(userdata: *const c_void, stream: *const uint8_t, len: c_int) {
    use std::raw::Slice;
    use std::mem::{size_of, transmute};
    unsafe {
        let audio_callback: &mut CB = transmute(userdata);
        let mut buf: &mut [T] = transmute(Slice {
            data: stream,
            len: len as uint / size_of::<T>()
        });
        audio_callback.callback(buf);
    }
}

pub struct AudioSpecDesired<T: AudioFormat<T>, CB: AudioCallback<T>> {
    pub freq: i32,
    pub channels: u8,
    pub callback: Box<CB>
}

impl<T: AudioFormat<T>, CB: AudioCallback<T>> AudioSpecDesired<T, CB> {
    fn convert_to_ll(self) -> ll::SDL_AudioSpec {
        use std::mem::transmute;
        unsafe {
            ll::SDL_AudioSpec {
                freq: self.freq,
                format: AudioFormat::<T>::get_audio_format(),
                channels: self.channels,
                silence: 0,
                samples: 0,
                padding: 0,
                size: 0,
                callback: Some(audio_callback_marshall::<T, CB>),
                userdata: transmute(self.callback)
            }
        }
    }

    pub fn open_audio_device(self, device: Option<&str>, iscapture: bool) -> SdlResult<AudioDevice<T, CB>> {
        use std::mem::uninitialized;
        use std::mem::transmute;
        use std::ptr::null;
        use std::c_str::CString;
        use libc::{c_char};
        let desired = self.convert_to_ll();

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
            let device_id = ll::SDL_OpenAudioDevice(device_cstr_ptr, iscapture_flag, transmute(&desired), transmute(&mut obtained), 0);
            match device_id {
                0 => {
                    // uninitialize the callback data to avoid memory leaks
                    let _: Box<CB> = transmute(desired.userdata);
                    Err(get_error())
                },
                id => {
                    let device_id = match iscapture {
                        true => AudioDeviceID::RecordingDevice(id),
                        false => AudioDeviceID::PlaybackDevice(id)
                    };
                    Ok(AudioDevice {
                        device_id: device_id,
                        spec: AudioSpec::convert_from_ll(obtained)
                    })
                }
            }
        }
    }
}

pub struct AudioSpec<T: AudioFormat<T>, CB: AudioCallback<T>> {
    pub freq: i32,
    pub channels: u8,
    pub silence: u8,
    pub samples: u16,
    pub size: u32,
    pub callback: Box<CB>,
}

impl<T: AudioFormat<T>, CB: AudioCallback<T>> AudioSpec<T, CB> {
    fn convert_from_ll(spec: ll::SDL_AudioSpec) -> AudioSpec<T, CB> {
        use std::mem::transmute;
        unsafe {
            AudioSpec {
                freq: spec.freq,
                channels: spec.channels,
                silence: spec.silence,
                samples: spec.samples,
                size: spec.size,
                callback: transmute(spec.userdata)
            }
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

pub struct AudioDevice<T: AudioFormat<T>, CB: AudioCallback<T>> {
    device_id: AudioDeviceID,
    /// Every audio device corresponds to an SDL_AudioSpec.
    /// Store it in `AudioDevice` to keep it and its callback data alive.
    spec: AudioSpec<T, CB>
}

impl<T: AudioFormat<T>, CB: AudioCallback<T>> AudioDevice<T, CB> {
    pub fn get_status(&self) -> AudioStatus {
        unsafe {
            let status = ll::SDL_GetAudioDeviceStatus(self.device_id.id());
            FromPrimitive::from_int(status as int).unwrap()
        }
    }

    pub fn pause(&self) {
        unsafe { ll::SDL_PauseAudioDevice(self.device_id.id(), 1) }
    }

    pub fn resume(&self) {
        unsafe { ll::SDL_PauseAudioDevice(self.device_id.id(), 0) }
    }

    pub fn close_and_get_spec(self) -> AudioSpec<T, CB> {
        drop(self.device_id);
        self.spec
    }
}

#[deriving(PartialEq)] #[allow(raw_pointer_deriving)]
pub struct AudioCVT {
    raw: *mut ll::SDL_AudioCVT,
    owned: bool,
}

impl_raw_accessors!(AudioCVT, *mut ll::SDL_AudioCVT)
impl_owned_accessors!(AudioCVT, owned)

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
            let p = (*self.raw).buf as *mut c_void; // send to proc()
            if ret == 0 {
                Ok( CVec::new_with_dtor((*self.raw).buf as *mut u8, (*self.raw).len_cvt as uint,
                                        proc() { libc::free(p) })
                    )
            } else {
                Err(get_error())
            }
        }
    }
}
