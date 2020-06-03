//!
//! A binding for the library `SDL2_mixer`
//!
//!
//! Note that you need to build with the
//! feature `mixer` for this module to be enabled,
//! like so:
//!
//! ```bash
//! $ cargo build --features "mixer"
//! ```
//!
//! If you want to use this with from inside your own
//! crate, you will need to add this in your Cargo.toml
//!
//! ```toml
//! [dependencies.sdl2]
//! version = ...
//! default-features = false
//! features = ["mixer"]
//! ```

use std::marker::PhantomData;
use std::convert::TryInto;
use std::default;
use std::fmt;
use std::ffi::{CString, CStr};
use std::str::from_utf8;
use std::borrow::ToOwned;
use std::path::Path;
use libc::c_void;
use libc::{c_int, c_double, c_uint};
use ::audio::AudioFormatNum;
use ::get_error;
use ::rwops::RWops;
use ::version::Version;
use sys;
use sys::mixer;

// This comes from SDL_audio.h
#[allow(non_camel_case_types)]
mod ll {
    pub const AUDIO_U8: u16 = 0x0008;
    pub const AUDIO_S8: u16 = 0x8008;
    pub const AUDIO_U16LSB: u16 = 0x0010;
    pub const AUDIO_S16LSB: u16 = 0x8010;
    pub const AUDIO_U16MSB: u16 = 0x1010;
    pub const AUDIO_S16MSB: u16 = 0x9010;
    pub const AUDIO_U16: u16 = AUDIO_U16LSB;
    pub const AUDIO_S16: u16 = AUDIO_S16LSB;
    pub const AUDIO_S32LSB: u16 = 0x8020;
    pub const AUDIO_S32MSB: u16 = 0x9020;
    pub const AUDIO_S32: u16 = AUDIO_S32LSB;
    pub const AUDIO_F32LSB: u16 = 0x8120;
    pub const AUDIO_F32MSB: u16 = 0x9120;
    pub const AUDIO_F32: u16 = AUDIO_F32LSB;
    pub const AUDIO_U16SYS: u16 = AUDIO_U16LSB;
    pub const AUDIO_S16SYS: u16 = AUDIO_S16LSB;
    pub const AUDIO_S32SYS: u16 = AUDIO_S32LSB;
    pub const AUDIO_F32SYS: u16 = AUDIO_F32LSB;
}

pub type AudioFormat = u16;

pub const AUDIO_U8: AudioFormat = ll::AUDIO_U8;
pub const AUDIO_S8: AudioFormat = ll::AUDIO_S8;
pub const AUDIO_U16LSB: AudioFormat = ll::AUDIO_U16LSB;
pub const AUDIO_S16LSB: AudioFormat = ll::AUDIO_S16LSB;
pub const AUDIO_U16MSB: AudioFormat = ll::AUDIO_U16MSB;
pub const AUDIO_S16MSB: AudioFormat = ll::AUDIO_S16MSB;
pub const AUDIO_U16: AudioFormat = ll::AUDIO_U16;
pub const AUDIO_S16: AudioFormat = ll::AUDIO_S16;
pub const AUDIO_S32LSB: AudioFormat = ll::AUDIO_S32LSB;
pub const AUDIO_S32MSB: AudioFormat = ll::AUDIO_S32MSB;
pub const AUDIO_S32: AudioFormat = ll::AUDIO_S32;
pub const AUDIO_F32LSB: AudioFormat = ll::AUDIO_F32LSB;
pub const AUDIO_F32MSB: AudioFormat = ll::AUDIO_F32MSB;
pub const AUDIO_F32: AudioFormat = ll::AUDIO_F32;
pub const AUDIO_U16SYS: AudioFormat = ll::AUDIO_U16SYS;
pub const AUDIO_S16SYS: AudioFormat = ll::AUDIO_S16SYS;
pub const AUDIO_S32SYS: AudioFormat = ll::AUDIO_S32SYS;
pub const AUDIO_F32SYS: AudioFormat = ll::AUDIO_F32SYS;

/// The suggested default is signed 16bit samples in host byte order.
pub const DEFAULT_FORMAT: AudioFormat = ll::AUDIO_S16SYS;
/// Default channels: Stereo.
pub const DEFAULT_CHANNELS: i32 = 2;
/// Good default sample rate in Hz (samples per second) for PC sound cards.
pub const DEFAULT_FREQUENCY: i32 = 22_050;
/// Maximum value for any volume setting.
pub const MAX_VOLUME: i32 = 128;

/// Returns the version of the dynamically linked `SDL_mixer` library
pub fn get_linked_version() -> Version {

    unsafe { Version::from_ll(*mixer::Mix_Linked_Version()) }
}

bitflags!(
    pub struct InitFlag : u32 {
        const FLAC = mixer::MIX_InitFlags_MIX_INIT_FLAC as u32;
        const MOD  = mixer::MIX_InitFlags_MIX_INIT_MOD as u32;
        const MP3  = mixer::MIX_InitFlags_MIX_INIT_MP3 as u32;
        const OGG  = mixer::MIX_InitFlags_MIX_INIT_OGG as u32;
        const MID  = mixer::MIX_InitFlags_MIX_INIT_MID as u32;
        const OPUS = mixer::MIX_InitFlags_MIX_INIT_OPUS as u32;
    }
);

impl ToString for InitFlag {
    fn to_string(&self) -> String {
        let mut string = "".to_string();
        if self.contains(InitFlag::FLAC) {
            string = string + &"INIT_FLAC ".to_string();
        }
        if self.contains(InitFlag::MOD) {
            string = string + &"INIT_MOD ".to_string();
        }
        if self.contains(InitFlag::MP3) {
            string = string + &"INIT_MP3 ".to_string();
        }
        if self.contains(InitFlag::OGG) {
            string = string + &"INIT_OGG ".to_string();
        }
        if self.contains(InitFlag::MID) {
            string = string + &"INIT_MID ".to_string();
        }
        if self.contains(InitFlag::OPUS) {
            string = string + &"INIT_OPUS ".to_string();
        }
        string
    }
}

/// Context manager for `sdl2_mixer` to manage init and quit
pub struct Sdl2MixerContext;

/// Cleans up all dynamically loaded library handles, freeing memory.
impl Drop for Sdl2MixerContext {
    fn drop(&mut self) {
        unsafe {
            mixer::Mix_Quit();
        }
    }
}

/// Loads dynamic libraries and prepares them for use.  Flags should be
/// one or more flags from `InitFlag`.
pub fn init(flags: InitFlag) -> Result<Sdl2MixerContext, String> {
    let return_flags = unsafe {
        let ret = mixer::Mix_Init(flags.bits() as c_int);
        InitFlag::from_bits_truncate(ret as u32)
    };
    // Check if all init flags were set
    if flags.intersects(return_flags) {
        Ok(Sdl2MixerContext)
    } else {
        // Flags not matching won't always set the error message text
        // according to sdl docs
        if get_error() == "" {
            let un_init_flags = return_flags ^ flags;
            let error_str = &("Could not init: ".to_string() + &un_init_flags.to_string());
            let _ = ::set_error(error_str);
        }
        Err(get_error())
    }
}


/// Open the mixer with a certain audio format.
///
/// * `chunksize`: It is recommended to choose values between 256 and 1024, depending on whether
///                you prefer latency or compatibility. Small values reduce latency but may not
///                work very well on older systems. For instance, a chunk size of 256 will give
///                you a latency of 6ms, while a chunk size of 1024 will give you a latency of 23ms
///                for a frequency of 44100kHz.
pub fn open_audio(frequency: i32,
                  format: AudioFormat,
                  channels: i32,
                  chunksize: i32)
                  -> Result<(), String> {
    let ret = unsafe {
        mixer::Mix_OpenAudio(frequency as c_int,
                           format,
                           channels as c_int,
                           chunksize as c_int)
    };
    if ret == 0 {
        Ok(())
    } else {
        Err(get_error())
    }
}

/// Shutdown and cleanup the mixer API.
pub fn close_audio() {
    unsafe { mixer::Mix_CloseAudio() }
}

/// Get the actual audio format in use by the opened audio device.
pub fn query_spec() -> Result<(i32, AudioFormat, i32), String> {
    let mut frequency: c_int = 0;
    let mut format: u16 = 0;
    let mut channels: c_int = 0;
    let ret = unsafe { mixer::Mix_QuerySpec(&mut frequency, &mut format, &mut channels) };
    if ret == 0 {
        Err(get_error())
    } else {
        Ok((frequency as i32, format as AudioFormat, channels as i32))
    }
}

// 4.2 Samples

/// Get the number of sample chunk decoders available from the `Mix_GetChunkDecoder` function.
pub fn get_chunk_decoders_number() -> i32 {
    unsafe { mixer::Mix_GetNumChunkDecoders() as i32 }
}

/// Get the name of the indexed sample chunk decoder.
pub fn get_chunk_decoder(index: i32) -> String {
    unsafe {
        let name = mixer::Mix_GetChunkDecoder(index as c_int);
        from_utf8(CStr::from_ptr(name).to_bytes()).unwrap().to_owned()
    }
}

/// The internal format for an audio chunk.
#[derive(PartialEq)]
pub struct Chunk {
    pub raw: *mut mixer::Mix_Chunk,
    pub owned: bool,
}

impl Drop for Chunk {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                // Mix_QuickLoad_* functions don't set the allocated flag, but from_raw_buffer
                // *does* take ownership of the data, so we need to deallocate the buffers here,
                // because Mix_FreeChunk won't and we'd be leaking memory otherwise.
                if (*self.raw).allocated == 0 {
                    drop(Box::from_raw((*self.raw).abuf));
                }
                mixer::Mix_FreeChunk(self.raw);
            }
        }
    }
}

impl Chunk {
    /// Load file for use as a sample.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Chunk, String> {
        let raw = unsafe { mixer::Mix_LoadWAV_RW(RWops::from_file(path, "rb")?.raw(), 0) };
        Self::from_owned_raw(raw)
    }

    /// Load chunk from a buffer containing raw audio data in the mixer format. The length of the
    /// buffer has to fit in 32-bit unsigned integer. The chunk takes ownership of the buffer.
    ///
    /// It's your responsibility to provide the audio data in the right format, as no conversion
    /// will take place when using this method.
    pub fn from_raw_buffer<T: AudioFormatNum>(buffer: Box<[T]>) -> Result<Chunk, String> {
        use std::mem::size_of;
        let len: u32 = (buffer.len() * size_of::<T>()).try_into().unwrap();
        let raw = unsafe { mixer::Mix_QuickLoad_RAW(Box::into_raw(buffer) as *mut u8, len) };
        Self::from_owned_raw(raw)
    }

    fn from_owned_raw(raw: *mut mixer::Mix_Chunk) -> Result<Chunk, String> {
        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(Chunk {
                raw: raw,
                owned: true,
            })
        }
    }

    /// Set chunk->volume to volume.
    pub fn set_volume(&mut self, volume: i32) -> i32 {
        unsafe { mixer::Mix_VolumeChunk(self.raw, volume as c_int) as i32 }
    }

    /// current volume for the chunk.
    pub fn get_volume(&self) -> i32 {
        unsafe { mixer::Mix_VolumeChunk(self.raw, -1) as i32 }
    }
}

/// Loader trait for `RWops`
pub trait LoaderRWops<'a> {
    /// Load src for use as a sample.
    fn load_wav(&self) -> Result<Chunk, String>;

    fn load_music(&'a self) -> Result<Music<'a>, String>;
}

impl<'a> LoaderRWops<'a> for RWops<'a> {
    /// Load src for use as a sample.
    fn load_wav(&self) -> Result<Chunk, String> {
        let raw = unsafe { mixer::Mix_LoadWAV_RW(self.raw(), 0) };
        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(Chunk {
                raw: raw,
                owned: true,
            })
        }
    }

    /// Load src for use as music.
    fn load_music(&self) -> Result<Music<'a>, String> {
        let raw = unsafe { mixer::Mix_LoadMUS_RW(self.raw(), 0) };
        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(Music {
                raw: raw,
                owned: true,
                _marker: PhantomData,
            })
        }
    }

}


// 4.3 Channels

/// Fader effect type enumerations
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum Fading {
    NoFading = mixer::Mix_Fading_MIX_NO_FADING as i32,
    FadingOut = mixer::Mix_Fading_MIX_FADING_OUT as i32,
    FadingIn = mixer::Mix_Fading_MIX_FADING_IN as i32,
}

/// Sound effect channel.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Channel(pub i32);

/// Set the number of channels being mixed.
pub fn allocate_channels(numchans: i32) -> i32 {
    unsafe { mixer::Mix_AllocateChannels(numchans as c_int) as i32 }
}

static mut CHANNEL_FINISHED_CALLBACK: Option<fn(Channel)> = None;

extern "C" fn c_channel_finished_callback(ch: c_int) {
    unsafe {
        match CHANNEL_FINISHED_CALLBACK {
            None => (),
            Some(cb) => cb(Channel(ch as i32)),
        }
    }
}

/// When channel playback is halted, then the specified `channel_finished` function is called.
pub fn set_channel_finished(f: fn(Channel)) {
    unsafe {
        CHANNEL_FINISHED_CALLBACK = Some(f);
        mixer::Mix_ChannelFinished(Some(c_channel_finished_callback as extern "C" fn(ch: c_int)));
    }
}

/// Unhooks the specified function set before, so no function is called when channel playback is
/// halted.
pub fn unset_channel_finished() {
    unsafe {
        mixer::Mix_ChannelFinished(None);
        CHANNEL_FINISHED_CALLBACK = None;
    }
}

impl Channel {
    /// Represent for all channels (-1)
    pub fn all() -> Channel {
        Channel(-1)
    }

    /// This is the MIX_CHANNEL_POST (-2)
    pub fn post() -> Channel {
        Channel(-2)
    }

    /// Set the volume for any allocated channel.
    pub fn set_volume(self, volume: i32) -> i32 {
        let Channel(ch) = self;
        unsafe { mixer::Mix_Volume(ch as c_int, volume as c_int) as i32 }
    }

    /// Returns the channels volume on scale of 0 to 128.
    pub fn get_volume(self) -> i32 {
        let Channel(ch) = self;
        unsafe { mixer::Mix_Volume(ch as c_int, -1) as i32 }
    }

    /// Play chunk on channel, or if channel is -1, pick the first free unreserved channel.
    pub fn play(self, chunk: &Chunk, loops: i32) -> Result<Channel, String> {
        self.play_timed(chunk, loops, -1)
    }

    pub fn play_timed(self, chunk: &Chunk, loops: i32, ticks: i32) -> Result<Channel, String> {
        let Channel(ch) = self;
        let ret = unsafe {
            mixer::Mix_PlayChannelTimed(ch as c_int, chunk.raw, loops as c_int, ticks as c_int)
        };
        if ret == -1 {
            Err(get_error())
        } else {
            Ok(Channel(ret as i32))
        }
    }

    /// Play chunk on channel, or if channel is -1, pick the first free unreserved channel.
    pub fn fade_in(self, chunk: &Chunk, loops: i32, ms: i32) -> Result<Channel, String> {
        self.fade_in_timed(chunk, loops, ms, -1)
    }

    pub fn fade_in_timed(self,
                         chunk: &Chunk,
                         loops: i32,
                         ms: i32,
                         ticks: i32)
                         -> Result<Channel, String> {
        let Channel(ch) = self;
        let ret = unsafe {
            mixer::Mix_FadeInChannelTimed(ch as c_int,
                                        chunk.raw,
                                        loops as c_int,
                                        ms as c_int,
                                        ticks as c_int)
        };
        if ret == -1 {
            Err(get_error())
        } else {
            Ok(Channel(ret as i32))
        }
    }

    /// Pause channel, or all playing channels if -1 is passed in.
    pub fn pause(self) {
        let Channel(ch) = self;
        unsafe {
            mixer::Mix_Pause(ch as c_int);
        }
    }

    /// Unpause channel, or all playing and paused channels if -1 is passed in.
    pub fn resume(self) {
        let Channel(ch) = self;
        unsafe {
            mixer::Mix_Resume(ch as c_int);
        }
    }

    /// Halt channel playback
    pub fn halt(self) {
        let Channel(ch) = self;
        unsafe {
            mixer::Mix_HaltChannel(ch as c_int);
        }
    }

    /// Halt channel playback, after ticks milliseconds.
    pub fn expire(self, ticks: i32) -> i32 {
        let Channel(ch) = self;
        unsafe { mixer::Mix_ExpireChannel(ch as c_int, ticks as c_int) as i32 }
    }

    /// Gradually fade out which channel over ms milliseconds starting from now.
    pub fn fade_out(self, ms: i32) -> i32 {
        let Channel(ch) = self;
        unsafe { mixer::Mix_FadeOutChannel(ch as c_int, ms as c_int) as i32 }
    }

    /// if channel is playing, or not.
    pub fn is_playing(self) -> bool {
        let Channel(ch) = self;
        unsafe { mixer::Mix_Playing(ch as c_int) != 0 }
    }

    ///  if channel is paused, or not.
    pub fn is_paused(self) -> bool {
        let Channel(ch) = self;
        unsafe { mixer::Mix_Paused(ch as c_int) != 0 }
    }

    /// if channel is fading in, out, or not
    pub fn get_fading(self) -> Fading {
        let Channel(ch) = self;
        let ret = unsafe { mixer::Mix_FadingChannel(ch as c_int) as c_uint };
        match ret {
            mixer::Mix_Fading_MIX_FADING_OUT    => Fading::FadingOut,
            mixer::Mix_Fading_MIX_FADING_IN     => Fading::FadingIn,
            mixer::Mix_Fading_MIX_NO_FADING | _ => Fading::NoFading
        }
    }

    /// Get the most recent sample chunk pointer played on channel.
    pub fn get_chunk(self) -> Option<Chunk> {
        let Channel(ch) = self;
        let raw = unsafe { mixer::Mix_GetChunk(ch as c_int) };
        if raw.is_null() {
            None
        } else {
            Some(Chunk {
                raw: raw,
                owned: false,
            })
        }
    }

    /// This removes all effects registered to channel.
    pub fn unregister_all_effects(self) -> Result<(), String> {
        let Channel(ch) = self;
        let ret = unsafe { mixer::Mix_UnregisterAllEffects(ch as c_int) };
        if ret == 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Sets a panning effect, where left and right is the volume of the left and right channels.
    /// They range from 0 (silence) to 255 (loud).
    pub fn set_panning(self, left: u8, right: u8) -> Result<(), String> {
        let Channel(ch) = self;
        let ret = unsafe { mixer::Mix_SetPanning(ch as c_int, left, right) };
        if ret == 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Unregisters panning effect.
    pub fn unset_panning(self) -> Result<(), String> {
        let Channel(ch) = self;
        let ret = unsafe { mixer::Mix_SetPanning(ch as c_int, 255, 255) };
        if ret == 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// This effect simulates a simple attenuation of volume due to distance.
    /// distance ranges from 0 (close/loud) to 255 (far/quiet).
    pub fn set_distance(self, distance: u8) -> Result<(), String> {
        let Channel(ch) = self;
        let ret = unsafe { mixer::Mix_SetDistance(ch as c_int, distance) };
        if ret == 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Unregisters distance effect.
    pub fn unset_distance(self) -> Result<(), String> {
        let Channel(ch) = self;
        let ret = unsafe { mixer::Mix_SetDistance(ch as c_int, 0) };
        if ret == 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// This effect emulates a simple 3D audio effect.
    /// angle ranges from 0 to 360 degrees going clockwise, where 0 is directly in front.
    /// distance ranges from 0 (close/loud) to 255 (far/quiet).
    pub fn set_position(self, angle: i16, distance: u8) -> Result<(), String> {
        let Channel(ch) = self;
        let ret = unsafe { mixer::Mix_SetPosition(ch as c_int, angle, distance) };
        if ret == 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Unregisters position effect.
    pub fn unset_position(self) -> Result<(), String> {
        let Channel(ch) = self;
        let ret = unsafe { mixer::Mix_SetPosition(ch as c_int, 0, 0) };
        if ret == 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Simple reverse stereo, swaps left and right channel sound.
    /// true for reverse, false to unregister effect.
    pub fn set_reverse_stereo(self, flip: bool) -> Result<(), String> {
        let Channel(ch) = self;
        let ret = unsafe { mixer::Mix_SetReverseStereo(ch as c_int, flip as c_int) };
        if ret == 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }
}

/// Returns how many channels are currently playing.
pub fn get_playing_channels_number() -> i32 {
    unsafe { mixer::Mix_Playing(-1) as i32 }
}

/// Returns how many channels are currently paused.
pub fn get_paused_channels_number() -> i32 {
    unsafe { mixer::Mix_Paused(-1) as i32 }
}

// 4.4 Groups

/// Reserve num channels from being used when playing samples when
/// passing in -1 as a channel number to playback functions.
pub fn reserve_channels(num: i32) -> i32 {
    unsafe { mixer::Mix_ReserveChannels(num as c_int) as i32 }
}

/// Sound effect channel grouping.
#[derive(Copy, Clone)]
pub struct Group(pub i32);

impl default::Default for Group {
    fn default() -> Group {
        Group(-1)
    }
}

impl Group {
    /// Add channels starting at from up through to to group tag,
    /// or reset it's group to the default group tag (-1).
    pub fn add_channels_range(self, from: i32, to: i32) -> i32 {
        let Group(g) = self;
        unsafe { mixer::Mix_GroupChannels(from as c_int, to as c_int, g as c_int) as i32 }
    }

    /// Add which channel to group tag, or reset it's group to the default group tag
    pub fn add_channel(self, Channel(ch): Channel) -> bool {
        let Group(g) = self;
        unsafe { mixer::Mix_GroupChannel(ch as c_int, g as c_int) == 1 }
    }

    /// Count the number of channels in group
    pub fn count(self) -> i32 {
        let Group(g) = self;
        unsafe { mixer::Mix_GroupCount(g as c_int) as i32 }
    }

    /// Find the first available (not playing) channel in group
    pub fn find_available(self) -> Option<Channel> {
        let Group(g) = self;
        let ret = unsafe { mixer::Mix_GroupAvailable(g as c_int) as i32 };
        if ret == -1 {
            None
        } else {
            Some(Channel(ret))
        }
    }

    /// Find the oldest actively playing channel in group
    pub fn find_oldest(self) -> Option<Channel> {
        let Group(g) = self;
        let ret = unsafe { mixer::Mix_GroupOldest(g as c_int) as i32 };
        if ret == -1 {
            None
        } else {
            Some(Channel(ret))
        }
    }

    /// Find the newest, most recently started, actively playing channel in group.
    pub fn find_newest(self) -> Option<Channel> {
        let Group(g) = self;
        let ret = unsafe { mixer::Mix_GroupNewer(g as c_int) as i32 };
        if ret == -1 {
            None
        } else {
            Some(Channel(ret))
        }
    }

    /// Gradually fade out channels in group over some milliseconds starting from now.
    /// Returns the number of channels set to fade out.
    pub fn fade_out(self, ms: i32) -> i32 {
        let Group(g) = self;
        unsafe { mixer::Mix_FadeOutGroup(g as c_int, ms as c_int) as i32 }
    }

    /// Halt playback on all channels in group.
    pub fn halt(self) {
        let Group(g) = self;
        unsafe {
            mixer::Mix_HaltGroup(g as c_int);
        }
    }
}

// 4.5 Music

/// Get the number of music decoders available.
pub fn get_music_decoders_number() -> i32 {
    unsafe { mixer::Mix_GetNumMusicDecoders() as i32 }
}

/// Get the name of the indexed music decoder.
pub fn get_music_decoder(index: i32) -> String {
    unsafe {
        let name = mixer::Mix_GetMusicDecoder(index as c_int);
        from_utf8(CStr::from_ptr(name).to_bytes()).unwrap().to_owned()
    }
}

/// Music type enumerations
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub enum MusicType {
    MusicNone = mixer::Mix_MusicType_MUS_NONE as i32,
    MusicCmd = mixer::Mix_MusicType_MUS_CMD as i32,
    MusicWav = mixer::Mix_MusicType_MUS_WAV as i32,
    MusicMod = mixer::Mix_MusicType_MUS_MOD as i32,
    MusicMid = mixer::Mix_MusicType_MUS_MID as i32,
    MusicOgg = mixer::Mix_MusicType_MUS_OGG as i32,
    MusicMp3 = mixer::Mix_MusicType_MUS_MP3 as i32,
    MusicMp3Mad = mixer::Mix_MusicType_MUS_MP3_MAD_UNUSED as i32,
    MusicFlac = mixer::Mix_MusicType_MUS_FLAC as i32,
    MusicModPlug = mixer::Mix_MusicType_MUS_MODPLUG_UNUSED as i32,
}

// hooks
static mut MUSIC_FINISHED_HOOK: Option<fn()> = None;

extern "C" fn c_music_finished_hook() {
    unsafe {
        match MUSIC_FINISHED_HOOK {
            None => (),
            Some(f) => f(),
        }
    }
}

/// This is an opaque data type used for Music data.
#[derive(PartialEq)]
pub struct Music<'a> {
    pub raw: *mut mixer::Mix_Music,
    pub owned: bool,
    _marker: PhantomData<&'a ()>
}

impl<'a> Drop for Music<'a> {
    fn drop(&mut self) {
        if self.owned {
            unsafe { mixer::Mix_FreeMusic(self.raw) };
        }
    }
}

impl<'a> fmt::Debug for Music<'a> {
    /// Shows the original regular expression.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Music>")
    }
}

impl<'a> Music<'a> {
    /// Load music file to use.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Music<'static>, String> {
        let raw = unsafe {
            let c_path = CString::new(path.as_ref().to_str().unwrap()).unwrap();
            mixer::Mix_LoadMUS(c_path.as_ptr())
        };
        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(Music {
                raw: raw,
                owned: true,
                _marker: PhantomData,
            })
        }
    }

    /// Load music from a static byte buffer.
    pub fn from_static_bytes(buf: &'static [u8]) -> Result<Music<'static>, String> {
        let rw = unsafe {
            sys::SDL_RWFromConstMem(buf.as_ptr() as *const c_void, buf.len() as c_int)
        };

        if rw.is_null() {
            return Err(get_error());
        }

        let raw = unsafe { mixer::Mix_LoadMUS_RW(rw, 0) };
        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(Music {
                raw: raw,
                owned: true,
                _marker: PhantomData,
            })
        }
    }

    /// The file format encoding of the music.
    pub fn get_type(&self) -> MusicType {
        let ret = unsafe { mixer::Mix_GetMusicType(self.raw) as i32 } as c_uint;
        match ret {
            mixer::Mix_MusicType_MUS_CMD      => MusicType::MusicCmd,
            mixer::Mix_MusicType_MUS_WAV      => MusicType::MusicWav,
            mixer::Mix_MusicType_MUS_MOD      => MusicType::MusicMod,
            mixer::Mix_MusicType_MUS_MID      => MusicType::MusicMid,
            mixer::Mix_MusicType_MUS_OGG      => MusicType::MusicOgg,
            mixer::Mix_MusicType_MUS_MP3      => MusicType::MusicMp3,
            mixer::Mix_MusicType_MUS_MP3_MAD_UNUSED  => MusicType::MusicMp3Mad,
            mixer::Mix_MusicType_MUS_FLAC     => MusicType::MusicFlac,
            mixer::Mix_MusicType_MUS_MODPLUG_UNUSED  => MusicType::MusicModPlug,
            mixer::Mix_MusicType_MUS_NONE | _ => MusicType::MusicNone
        }
    }

    /// Play the loaded music loop times through from start to finish. Pass -1 to loop forever.
    pub fn play(&self, loops: i32) -> Result<(), String> {
        let ret = unsafe { mixer::Mix_PlayMusic(self.raw, loops as c_int) };
        if ret == -1 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Fade in over ms milliseconds of time, the loaded music,
    /// playing it loop times through from start to finish.
    pub fn fade_in(&self, loops: i32, ms: i32) -> Result<(), String> {
        let ret = unsafe { mixer::Mix_FadeInMusic(self.raw, loops as c_int, ms as c_int) };
        if ret == -1 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Fade in over ms milliseconds of time, from position.
    pub fn fade_in_from_pos(&self, loops: i32, ms: i32, position: f64) -> Result<(), String> {
        let ret = unsafe {
            mixer::Mix_FadeInMusicPos(self.raw, loops as c_int, ms as c_int, position as c_double)
        };
        if ret == -1 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    // FIXME: make these class method?
    /// Returns current volume
    pub fn get_volume() -> i32 {
        unsafe { mixer::Mix_VolumeMusic(-1) as i32 }
    }

    /// Set the volume on a scale of 0 to 128.
    /// Values greater than 128 will use 128.
    pub fn set_volume(volume: i32) {
        // This shouldn't return anything. Use get_volume instead
        let _ = unsafe { mixer::Mix_VolumeMusic(volume as c_int) as i32 };
    }

    /// Pause the music playback.
    pub fn pause() {
        unsafe {
            mixer::Mix_PauseMusic();
        }
    }

    /// Unpause the music.
    pub fn resume() {
        unsafe {
            mixer::Mix_ResumeMusic();
        }
    }

    /// Rewind the music to the start.
    pub fn rewind() {
        unsafe {
            mixer::Mix_RewindMusic();
        }
    }

    /// Set the position of the currently playing music.
    pub fn set_pos(position: f64) -> Result<(), String> {
        let ret = unsafe { mixer::Mix_SetMusicPosition(position as c_double) };
        if ret == -1 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Setup a command line music player to use to play music.
    pub fn set_command(command: &str) -> Result<(), String> {
        let ret = unsafe {
            let c_command = CString::new(command).unwrap();
            mixer::Mix_SetMusicCMD(c_command.as_ptr())
        };
        if ret == -1 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Halt playback of music.
    pub fn halt() {
        unsafe {
            mixer::Mix_HaltMusic();
        }
    }

    /// Gradually fade out the music over ms milliseconds starting from now.
    pub fn fade_out(ms: i32) -> Result<(), String> {
        let ret = unsafe { mixer::Mix_FadeOutMusic(ms as c_int) };
        if ret == -1 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    // TODO: Mix_HookMusic
    // TODO: Mix_GetMusicHookData

    /// Sets up a function to be called when music playback is halted.
    ///
    /// # Examples
    ///
    /// ```
    /// fn after_music() {
    ///     println!("Music has ended");
    /// }
    ///
    /// sdl2::mixer::Music::hook_finished(after_music);
    /// ```
    pub fn hook_finished(f: fn()) {
        unsafe {
            MUSIC_FINISHED_HOOK = Some(f);
            mixer::Mix_HookMusicFinished(Some(c_music_finished_hook as extern "C" fn()));
        }
    }

    /// A previously set up function would no longer be called when music playback is halted.
    pub fn unhook_finished() {
        unsafe {
            mixer::Mix_HookMusicFinished(None);
            // unset from c, then rust, to avoid race condition
            MUSIC_FINISHED_HOOK = None;
        }
    }

    /// If music is actively playing, or not.
    pub fn is_playing() -> bool {
        unsafe { mixer::Mix_PlayingMusic() == 1 }
    }

    /// If music is paused, or not.
    pub fn is_paused() -> bool {
        unsafe { mixer::Mix_PausedMusic() == 1 }
    }

    /// If music is fading, or not.
    pub fn get_fading() -> Fading {
        let ret = unsafe { mixer::Mix_FadingMusic() as i32 } as c_uint;
        match ret {
            mixer::Mix_Fading_MIX_FADING_OUT    => Fading::FadingOut,
            mixer::Mix_Fading_MIX_FADING_IN     => Fading::FadingIn,
            mixer::Mix_Fading_MIX_NO_FADING | _ => Fading::NoFading
        }
    }
}

// 4.6 Effects

// TODO: Mix_RegisterEffect
// TODO: Mix_UnregisterEffect
// TODO: Mix_SetPostMix
