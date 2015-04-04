#[cfg(feature = "no_std")]
use core::prelude::*;
use libc::{c_int, c_uint, c_void, uint8_t, uint32_t};
use libc::{uint16_t, c_double, c_char};
use super::rwops::SDL_RWops;

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
    Option<extern "C" fn (arg1: *mut c_void, arg2: *mut uint8_t, arg3: c_int)>;
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
    Option<extern "C" fn (arg1: *const SDL_AudioCVT, arg2: SDL_AudioFormat)>;
#[allow(dead_code, missing_copy_implementations, raw_pointer_derive)]
#[derive(Copy, Clone)]
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
    filters: [SDL_AudioFilter; 10],
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
