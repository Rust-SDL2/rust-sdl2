use std::cast;
use std::ptr;
use std::mem;
use std::c_str::CString;
use std::c_vec::CVec;
use libc;
use libc::{c_int, size_t, c_void};
use libc::{uint8_t, uint16_t, uint32_t};
use std::raw::Slice;

use get_error;
use rwops::RWops;


#[allow(non_camel_case_types)]
pub mod ll {
    use libc::{c_int, c_uint, c_void, uint8_t, uint32_t};
    use libc::{uint16_t, c_double, c_char};
    use rwops::ll::SDL_RWops;

    // assume LSB
    pub type SDL_AudioFormat = uint16_t;
    pub static AUDIO_U8 : SDL_AudioFormat =         0x0008;
    pub static AUDIO_S8 : SDL_AudioFormat =         0x8008;
    pub static AUDIO_U16LSB : SDL_AudioFormat =     0x0010;
    pub static AUDIO_S16LSB : SDL_AudioFormat =     0x8010;
    pub static AUDIO_U16MSB : SDL_AudioFormat =     0x1010;
    pub static AUDIO_S16MSB : SDL_AudioFormat =     0x9010;
    pub static AUDIO_U16 : SDL_AudioFormat =        AUDIO_U16LSB;
    pub static AUDIO_S16 : SDL_AudioFormat =        AUDIO_S16LSB;
    pub static AUDIO_S32LSB : SDL_AudioFormat =     0x8020;
    pub static AUDIO_S32MSB : SDL_AudioFormat =     0x9020;
    pub static AUDIO_S32 : SDL_AudioFormat =        AUDIO_S32LSB;
    pub static AUDIO_F32LSB : SDL_AudioFormat =     0x8120;
    pub static AUDIO_F32MSB : SDL_AudioFormat =     0x9120;
    pub static AUDIO_F32 : SDL_AudioFormat =        AUDIO_F32LSB;
    pub static AUDIO_U16SYS : SDL_AudioFormat =     AUDIO_U16LSB;
    pub static AUDIO_S16SYS : SDL_AudioFormat =     AUDIO_S16LSB;
    pub static AUDIO_S32SYS : SDL_AudioFormat =     AUDIO_S32LSB;
    pub static AUDIO_F32SYS : SDL_AudioFormat =     AUDIO_F32LSB;

    pub type SDL_AudioCallback =
        ::std::option::Option<extern "C" fn
                                  (arg1: *c_void, arg2: *uint8_t,
                                   arg3: c_int)>;
    pub struct SDL_AudioSpec {
        pub freq: c_int,
        pub format: SDL_AudioFormat,
        pub channels: uint8_t,
        pub silence: uint8_t,
        pub samples: uint16_t,
        pub padding: uint16_t,
        pub size: uint32_t,
        pub callback: SDL_AudioCallback,
        pub userdata: *c_void,
    }
    pub type SDL_AudioFilter =
        ::std::option::Option<extern "C" fn
                                  (arg1: *SDL_AudioCVT,
                                   arg2: SDL_AudioFormat)>;
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
    pub static SDL_AUDIO_STOPPED: c_uint = 0;
    pub static SDL_AUDIO_PLAYING: c_uint = 1;
    pub static SDL_AUDIO_PAUSED: c_uint = 2;
    extern "C" {
        pub fn SDL_GetNumAudioDrivers() -> c_int;
        pub fn SDL_GetAudioDriver(index: c_int) -> *c_char;
        pub fn SDL_AudioInit(driver_name: *c_char) -> c_int;
        pub fn SDL_AudioQuit();
        pub fn SDL_GetCurrentAudioDriver() -> *c_char;
        pub fn SDL_OpenAudio(desired: *SDL_AudioSpec,
                             obtained: *SDL_AudioSpec) -> c_int;
        pub fn SDL_GetNumAudioDevices(iscapture: c_int) -> c_int;
        pub fn SDL_GetAudioDeviceName(index: c_int, iscapture: c_int) -> *c_char;
        pub fn SDL_OpenAudioDevice(device: *c_char, iscapture: c_int,
                                   desired: *SDL_AudioSpec,
                                   obtained: *SDL_AudioSpec,
                                   allowed_changes: c_int) -> SDL_AudioDeviceID;
        pub fn SDL_GetAudioStatus() -> SDL_AudioStatus;
        pub fn SDL_GetAudioDeviceStatus(dev: SDL_AudioDeviceID) ->
            SDL_AudioStatus;
        pub fn SDL_PauseAudio(pause_on: c_int);
        pub fn SDL_PauseAudioDevice(dev: SDL_AudioDeviceID, pause_on: c_int);
        pub fn SDL_LoadWAV_RW(src: *SDL_RWops, freesrc: c_int,
                              spec: *SDL_AudioSpec,
                              audio_buf: **uint8_t, audio_len: *uint32_t) -> *SDL_AudioSpec;
        pub fn SDL_FreeWAV(audio_buf: *uint8_t);
        pub fn SDL_BuildAudioCVT(cvt: *mut SDL_AudioCVT,
                                 src_format: SDL_AudioFormat, src_channels: uint8_t,
                                 src_rate: c_int, dst_format: SDL_AudioFormat,
                                 dst_channels: uint8_t, dst_rate: c_int) -> c_int;
        pub fn SDL_ConvertAudio(cvt: *mut SDL_AudioCVT) -> c_int;
        pub fn SDL_MixAudio(dst: *uint8_t, src: *uint8_t, len: uint32_t,
                            volume: c_int);
        pub fn SDL_MixAudioFormat(dst: *uint8_t, src: *uint8_t,
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

pub static AudioU8     : AudioFormat = ll::AUDIO_U8;
pub static AudioS8     : AudioFormat = ll::AUDIO_S8;
pub static AudioU16LSB : AudioFormat = ll::AUDIO_U16LSB;
pub static AudioS16LSB : AudioFormat = ll::AUDIO_S16LSB;
pub static AudioU16MSB : AudioFormat = ll::AUDIO_U16MSB;
pub static AudioS16MSB : AudioFormat = ll::AUDIO_S16MSB;
pub static AudioU16    : AudioFormat = ll::AUDIO_U16;
pub static AudioS16    : AudioFormat = ll::AUDIO_S16;
pub static AudioS32LSB : AudioFormat = ll::AUDIO_S32LSB;
pub static AudioS32MSB : AudioFormat = ll::AUDIO_S32MSB;
pub static AudioS32    : AudioFormat = ll::AUDIO_S32;
pub static AudioF32LSB : AudioFormat = ll::AUDIO_F32LSB;
pub static AudioF32MSB : AudioFormat = ll::AUDIO_F32MSB;
pub static AudioF32    : AudioFormat = ll::AUDIO_F32;
pub static AudioU16SYS : AudioFormat = ll::AUDIO_U16SYS;
pub static AudioS16SYS : AudioFormat = ll::AUDIO_S16SYS;
pub static AudioS32SYS : AudioFormat = ll::AUDIO_S32SYS;
pub static AudioF32SYS : AudioFormat = ll::AUDIO_F32SYS;

#[repr(C)]
#[deriving(Clone, Eq, Hash, Show, FromPrimitive)]
pub enum AudioStatus {
    Stopped = ll::SDL_AUDIO_STOPPED as int,
    Playing = ll::SDL_AUDIO_PLAYING as int,
    Paused  = ll::SDL_AUDIO_PAUSED  as int,
}

pub fn get_num_audio_drivers() -> int {
    unsafe { ll::SDL_GetNumAudioDrivers() as int }
}

pub fn get_audio_driver(index: int) -> ~str {
    unsafe {
        let buf = ll::SDL_GetAudioDriver(index as c_int);
        CString::new(buf, false).as_str().unwrap().into_owned()
    }
}

pub fn get_num_audio_devices(iscapture: int) -> int {
    unsafe { ll::SDL_GetNumAudioDevices(iscapture as c_int) as int }
}

pub fn get_audio_device_name(index: int, iscapture: int) -> ~str {
    unsafe {
        let buf = ll::SDL_GetAudioDeviceName(index as c_int, iscapture as c_int);
        CString::new(buf, false).as_str().unwrap().into_owned()
    }
}

pub fn audio_init(name: &str) -> Result<(), ~str> {
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

pub fn get_current_audio_driver() -> ~str {
    unsafe {
        let buf = ll::SDL_GetCurrentAudioDriver();
        CString::new(buf, false).as_str().unwrap().into_owned()
    }
}

// make this same layout as in C
#[repr(C)]
pub struct AudioSpec<'a > {
    pub freq: c_int,
    pub format: AudioFormat,
    pub channels: uint8_t,
    pub silence: uint8_t,
    pub samples: uint16_t,
    pub padding: uint16_t,
    pub size: uint32_t,
    c_callback: ll::SDL_AudioCallback,
    pub callback: &'a |&mut [u8]|:'a, // same size as *c_void
}

extern "C" fn c_audio_callback(userdata: *c_void, stream: *uint8_t, len: c_int) {
    unsafe {
        let f : &mut |&mut [u8]| = cast::transmute(userdata);

        // FIXME: lifetime error in calling
        //slice::raw::mut_buf_as_slice(stream as *mut u8, len as uint, *f)
        (*f)(cast::transmute(Slice {
            data: stream,
            len: len as uint
        }))
    }
}


impl<'a> AudioSpec<'a> {
    pub fn load_wav(path: &Path) -> Result<(AudioSpec, CVec<u8>), ~str> {
        AudioSpec::load_wav_rw(&try!(RWops::from_file(path, "rb")))
    }

    pub fn load_wav_rw(src: &RWops) -> Result<(AudioSpec, CVec<u8>), ~str> {
        assert_eq!(mem::size_of::<AudioSpec>(), mem::size_of::<ll::SDL_AudioSpec>());
        let mut spec = unsafe { mem::uninit::<AudioSpec>() };
        let audio_buf = ptr::null::<u8>();
        let audio_len = 0u32;
        unsafe {
            let ret = ll::SDL_LoadWAV_RW(src.raw, 0, cast::transmute(&spec), &audio_buf, &audio_len);
            if ret.is_null() {
                Err(get_error())
            } else {
                let v = CVec::new_with_dtor(audio_buf as *mut u8, audio_len as uint, proc() {
                    ll::SDL_FreeWAV(audio_buf)
                });
                spec.c_callback = Some(c_audio_callback);
                Ok((spec, v))
            }
        }
    }
}

pub type AudioDeviceID = ll::SDL_AudioDeviceID;

// use rust's type system to make it right.
pub enum AudioDevice{
    PlaybackDevice(AudioDeviceID),
    RecordingDevice(AudioDeviceID),
}

impl AudioDevice {
    fn to_id(self) -> AudioDeviceID {
        match self {
            PlaybackDevice(id)  => id,
            RecordingDevice(id) => id
        }
    }

    pub fn open(device: Option<&str>, iscapture: int, spec: &AudioSpec) -> Result<(AudioDevice, AudioSpec), ~str> {
        //! SDL_OpenAudioDevice
        let obtained = unsafe { mem::uninit::<AudioSpec>() };
        unsafe {
            let device_c_str = match device {
                None => ptr::null(),
                Some(device) => device.to_c_str().unwrap(),
            };
            let ret = ll::SDL_OpenAudioDevice(device_c_str,
                                              iscapture as c_int,
                                              cast::transmute(spec),
                                              cast::transmute(&obtained),
                                              0);
            if ret == 0 {
                Err(get_error())
            } else {
                if iscapture == 0 { // plaback device
                    Ok((PlaybackDevice(ret as AudioDeviceID), obtained))
                } else {
                    Ok((RecordingDevice(ret as AudioDeviceID), obtained))
                }
            }
        }
    }

    pub fn get_status(self) -> AudioStatus {
        unsafe {
            let status = ll::SDL_GetAudioDeviceStatus(self.to_id());
            FromPrimitive::from_int(status as int).unwrap()
        }
    }

    pub fn pause(self) {
        unsafe { ll::SDL_PauseAudioDevice(self.to_id(), 1) }
    }

    pub fn resume(self) {
        unsafe { ll::SDL_PauseAudioDevice(self.to_id(), 0) }
    }

    pub fn lock(self) {
        unsafe { ll::SDL_LockAudioDevice(self.to_id()) }
    }

    pub fn unlock(self) {
        unsafe { ll::SDL_UnlockAudioDevice(self.to_id()) }
    }

    pub fn close(self) {
        //! Shut down audio processing and close the audio device.
        unsafe { ll::SDL_CloseAudioDevice(self.to_id()) }
    }
}

#[deriving(Eq)] #[allow(raw_pointer_deriving)]
pub struct AudioCVT {
    pub raw: *mut ll::SDL_AudioCVT,
    pub owned: bool,
}

impl Drop for AudioCVT {
    fn drop(&mut self) {
        if self.owned {
            unsafe { libc::free(self.raw as *mut c_void) }
        }
    }
}

impl AudioCVT {
    pub fn new(src_format: AudioFormat, src_channels: u8, src_rate: int,
               dst_format: AudioFormat, dst_channels: u8, dst_rate: int) -> Result<AudioCVT, ~str> {
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

    pub fn convert(&self, src: CVec<u8>) -> Result<CVec<u8>, ~str> {
        //! Convert audio data to a desired audio format.

        unsafe {
            if (*self.raw).needed != 1 {
                return Err("no convertion needed!".to_owned())
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
