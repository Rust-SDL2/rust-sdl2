/*!
A binding for SDL2_mixer.
 */

#![crate_id="sdl2_mixer#sdl2_mixer:0.1"]
#![crate_type = "lib"]
#![desc = "SDL2_mixer bindings and wrappers"]
#![comment = "SDL2_mixer bindings and wrappers"]
#![license = "MIT"]

#![feature(globs)]

extern crate libc;
extern crate sdl2;

use libc::{c_int, uint16_t, c_double};
use std::fmt;
use std::c_str::CString;
use std::default;
use std::ptr;
use sdl2::get_error;
use sdl2::rwops::RWops;

// Setup linking for all targets.
#[cfg(target_os="macos")]
mod mac {
    #[cfg(mac_framework)]
    #[link(kind="framework", name="SDL2_mixer")]
    extern {}

    #[cfg(not(mac_framework))]
    #[link(name="SDL2_mixer")]
    extern {}
}

#[cfg(target_os="win32")]
#[cfg(target_os="linux")]
#[cfg(target_os="freebsd")]
mod others {
    #[link(name="SDL2_mixer")]
    extern {}
}

#[allow(non_camel_case_types, dead_code)]
mod ffi;

pub static DEFAULT_CHANNELS : int = 2;
pub static DEFAULT_FREQUENCY : int = 22050;
pub static MAX_VOLUME : int = 128;


/// The version of the libSDL.so that you are linked to
#[deriving(Eq, Clone)]
pub struct SDLVersion {
    pub major: int,
    pub minor: int,
    pub patch: int,
}

impl fmt::Show for SDLVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f.buf, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl SDLVersion {
    fn from_sdl_version(sv: *ffi::SDL_version) -> SDLVersion {
        //! Converts a raw *SDL_version to SDLVersion
        unsafe {
            let v = *sv;
            SDLVersion{ major: v.major as int, minor: v.minor as int, patch: v.patch as int }
        }
    }
}

pub fn get_linked_version() -> SDLVersion {
    //! Returns the version of the dynamically linked SDL_mixer library
    unsafe {
        SDLVersion::from_sdl_version(ffi::Mix_Linked_Version())
    }
}

#[repr(C)]
#[deriving(Clone, Eq, Hash, Show)]
pub enum InitFlag {
    InitFlac       = ffi::MIX_INIT_FLAC as int,
    InitMod        = ffi::MIX_INIT_MOD as int,
    InitModPlug    = ffi::MIX_INIT_MODPLUG as int,
    InitMp3        = ffi::MIX_INIT_MP3 as int,
    InitOgg        = ffi::MIX_INIT_OGG as int,
    InitFluidSynth = ffi::MIX_INIT_FLUIDSYNTH as int,
}

pub fn init(flags: &[InitFlag]) -> ~[InitFlag] {
    //! Loads dynamic libraries and prepares them for use.  Flags should be
    //! one or more flags from InitFlag.
    //! It returns the flags successfully initialized, or [] on failure.
    let mut used = ~[];
    unsafe {
        let used_flags = ffi::Mix_Init(
            flags.iter().fold(0, |flags, &flag| {
                flags | flag as ffi::MIX_InitFlags
            }) as c_int
        );
        for flag in flags.iter() {
            if used_flags & *flag as c_int != 0 {
                used.push(*flag)
            }
        }
    }
    used
}

pub fn quit() {
    //! Cleans up all dynamically loaded library handles, freeing memory.
    unsafe { ffi::Mix_Quit(); }
}

pub fn open_audio(frequency: int, format: u16, channels: int, chunksize: int) -> Result<(), ~str> {
    //! Open the mixer with a certain audio format.
    let ret = unsafe { ffi::Mix_OpenAudio(frequency as c_int, format, channels as c_int, chunksize as c_int) };
    if ret == 0 { Ok(()) }
    else { Err(get_error()) }
}

pub fn close_audio() {
    //! Shutdown and cleanup the mixer API.
    unsafe { ffi::Mix_CloseAudio() }
}

pub fn query_spec() -> Result<(int, u16, int), ~str> {
    //! Get the actual audio format in use by the opened audio device.
    let frequency : c_int = 0;
    let format : uint16_t = 0;
    let channels : c_int  = 0;
    let ret = unsafe { ffi::Mix_QuerySpec(&frequency, &format, &channels) };
    if ret == 0 {
        Ok((frequency as int, format as u16, channels as int))
    } else {
        Err(get_error())
    }
}

// 4.2 Samples

pub fn get_chunk_decoders_number() -> int {
    //! Get the number of sample chunk decoders available from the Mix_GetChunkDecoder function.
    unsafe { ffi::Mix_GetNumChunkDecoders() as int }
}

pub fn get_chunk_decoder(index: int) -> ~str {
    //! Get the name of the indexed sample chunk decoder.
     unsafe {
        let name = ffi::Mix_GetChunkDecoder(index as c_int);
        CString::new(name, false).as_str().unwrap().into_owned()
    }
}

/// The internal format for an audio chunk.
#[deriving(Eq)] #[allow(raw_pointer_deriving, visible_private_types)]
pub struct Chunk {
    pub raw: *ffi::Mix_Chunk,
    pub owned: bool
}

impl Drop for Chunk {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::Mix_FreeChunk(self.raw) }
        }
    }
}

impl Chunk {
    pub fn from_file(path: &Path) -> Result<~Chunk, ~str> {
        //! Load file for use as a sample.
        let raw = unsafe {
            ffi::Mix_LoadWAV_RW(try!(RWops::from_file(path, "wb")).raw, 0)
        };
        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(~Chunk{ raw: raw, owned: true })
        }
    }

    pub fn set_volume(&mut self, volume: int) -> int {
        //! Set chunk->volume to volume.
        unsafe {
            ffi::Mix_VolumeChunk(self.raw, volume as c_int) as int
        }
    }

    pub fn get_volume(&self) -> int {
        //! current volume for the chunk.
        unsafe {
            ffi::Mix_VolumeChunk(self.raw, -1) as int
        }
    }
}

/// Loader trait for RWops
pub trait LoaderRWops {
    /// Load src for use as a sample.
    fn load_wav(&self) -> Result<~Chunk, ~str>;
}

impl LoaderRWops for RWops {
    fn load_wav(&self) -> Result<~Chunk, ~str> {
        //! Load src for use as a sample.
        let raw = unsafe {
            ffi::Mix_LoadWAV_RW(self.raw, 0)
        };
        if raw == ptr::null() {
            Err(get_error())
        } else {
            Ok(~Chunk{ raw: raw, owned: true })
        }
    }
}


// 4.3 Channels

#[repr(C)]
#[deriving(Clone, Eq, Hash, Show, FromPrimitive)]
pub enum Fading {
    FadingNo  = ffi::MIX_NO_FADING as int,
    FadingOut = ffi::MIX_FADING_OUT as int,
    FadingIn  = ffi::MIX_FADING_IN as int
}

/// Sound effect channel.
pub struct Channel(int);

pub fn allocate_channels(numchans: int) -> int {
    //! Set the number of channels being mixed.
    unsafe {
        ffi::Mix_AllocateChannels(numchans as c_int) as int
    }
}

impl Channel {
    pub fn all() -> Channel {
        //! Represent for all channels (-1)
        Channel(-1)
    }

    pub fn post() -> Channel {
        //! This is the MIX_CHANNEL_POST (-2)
        Channel(-2)
    }

    pub fn set_volume(self, volume: int) -> int {
        //! Set the volume for any allocated channel.
        //! If channel is -1 then all channels at are set at once.
        let Channel(ch) = self;
        unsafe {
            ffi::Mix_Volume(ch as c_int, volume as c_int) as int
        }
    }

    pub fn get_volume(self) -> int {
        let Channel(ch) = self;
        unsafe {
            ffi::Mix_Volume(ch as c_int, -1) as int
        }
    }

    pub fn play(self, chunk: Chunk, loops: int) -> Result<Channel, ~str> {
        //! Play chunk on channel, or if channel is -1, pick the first free unreserved channel.
        self.play_timed(chunk, loops, -1)
    }

    pub fn play_timed(self, chunk: Chunk, loops: int, ticks: int) -> Result<Channel, ~str> {
        let Channel(ch) = self;
        let ret = unsafe {
            ffi::Mix_PlayChannelTimed(ch as c_int, chunk.raw, loops as c_int, ticks as c_int)
        };
        if ret == -1 {
            Err(get_error())
        } else {
            Ok(Channel(ret as int))
        }
    }

    pub fn fade_in(self, chunk: Chunk, loops: int, ms: int) -> Result<Channel, ~str> {
        //! Play chunk on channel, or if channel is -1, pick the first free unreserved channel.
        self.fade_in_timed(chunk, loops, ms, -1)
    }

    pub fn fade_in_timed(self, chunk: Chunk, loops: int, ms: int, ticks: int) -> Result<Channel, ~str> {
        let Channel(ch) = self;
        let ret = unsafe {
            ffi::Mix_FadeInChannelTimed(ch as c_int, chunk.raw, loops as c_int, ms as c_int, ticks as c_int)
        };
        if ret == -1 {
            Err(get_error())
        } else {
            Ok(Channel(ret as int))
        }
    }

    pub fn pause(self) {
        //! Pause channel, or all playing channels if -1 is passed in.
        let Channel(ch) = self;
        unsafe { ffi::Mix_Pause(ch as c_int); }
    }

    pub fn resume(self) {
        //! Unpause channel, or all playing and paused channels if -1 is passed in.
        let Channel(ch) = self;
        unsafe { ffi::Mix_Resume(ch as c_int); }
    }

    pub fn halt(self) {
        //! Halt channel playback
        let Channel(ch) = self;
        unsafe { ffi::Mix_HaltChannel(ch as c_int); }
    }

    pub fn expire(self, ticks: int) -> int {
        //! Halt channel playback, after ticks milliseconds.
        let Channel(ch) = self;
        unsafe { ffi::Mix_ExpireChannel(ch as c_int, ticks as c_int) as int }
    }

    pub fn fade_out(self, ms: int) -> int {
        //! Gradually fade out which channel over ms milliseconds starting from now.
        let Channel(ch) = self;
        unsafe { ffi::Mix_FadeOutChannel(ch as c_int, ms as c_int) as int }
    }

    // TODO: Mix_ChannelFinished

    pub fn is_playing(self) -> bool {
        //! if channel is playing, or not.
        let Channel(ch) = self;
        unsafe { ffi::Mix_Playing(ch as c_int) != 0 }
    }

    pub fn is_paused(self) -> bool {
        //!  if channel is paused, or not.
        let Channel(ch) = self;
        unsafe { ffi::Mix_Paused(ch as c_int) != 0 }
    }

    pub fn get_fading(self) -> Fading {
        //! if channel is fading in, out, or not
        let Channel(ch) = self;
        let ret = unsafe { ffi::Mix_FadingChannel(ch as c_int) as int };
        FromPrimitive::from_int(ret).unwrap()
    }

    pub fn get_chunk(self) -> Option<Chunk> {
        //! Get the most recent sample chunk pointer played on channel.
        let Channel(ch) = self;
        let raw = unsafe { ffi::Mix_GetChunk(ch as c_int) };
        if raw.is_null() {
            None
        } else {
            Some( Chunk { raw: raw, owned: false } )
        }
    }

    pub fn unregister_all_effects(self) -> Result<(), ~str> {
        //! This removes all effects registered to channel.
        let Channel(ch) = self;
        let ret = unsafe { ffi::Mix_UnregisterAllEffects(ch as c_int) };
        if ret == 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    pub fn set_panning(self, left: u8, right: u8) -> Result<(), ~str> {
        let Channel(ch) = self;
        let ret = unsafe { ffi::Mix_SetPanning(ch as c_int, left, right) };
        if ret == 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    pub fn set_distance(self, distance: u8) -> Result<(), ~str> {
        //! This effect simulates a simple attenuation of volume due to distance.
        let Channel(ch) = self;
        let ret = unsafe { ffi::Mix_SetDistance(ch as c_int, distance) };
        if ret == 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    pub fn set_position(self, angle: i16, distance: u8) -> Result<(), ~str> {
        //! This effect emulates a simple 3D audio effect.
        let Channel(ch) = self;
        let ret = unsafe { ffi::Mix_SetPosition(ch as c_int, angle, distance) };
        if ret == 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    pub fn set_reverse_stereo(self, flip: bool) -> Result<(), ~str> {
        //! Simple reverse stereo, swaps left and right channel sound.
        let Channel(ch) = self;
        let ret = unsafe { ffi::Mix_SetReverseStereo(ch as c_int, flip as c_int) };
        if ret == 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }
}

pub fn get_playing_channels_number() -> int {
    unsafe { ffi::Mix_Playing(-1) as int }
}

pub fn get_paused_channels_number() -> int {
    unsafe { ffi::Mix_Paused(-1) as int }
}

// 4.4 Groups

pub fn reserve_channels(num: int) -> int {
    //! Reserve num channels from being used when playing samples when
    //! passing in -1 as a channel number to playback functions.
    unsafe { ffi::Mix_ReserveChannels(num as c_int) as int }
}

/// Sound effect channel grouping.
pub struct Group(int);

impl default::Default for Group {
    fn default() -> Group {
        Group(-1)
    }
}

impl Group {
    pub fn add_channels_range(self, from: int, to: int) -> int {
        //! Add channels starting at from up through to to group tag,
        //! or reset it's group to the default group tag (-1).
        let Group(g) = self;
        unsafe { ffi::Mix_GroupChannels(from as c_int, to as c_int, g as c_int) as int }
    }

    pub fn add_channel(self, Channel(ch): Channel) -> bool {
        //! Add which channel to group tag, or reset it's group to the default group tag
        let Group(g) = self;
        unsafe { ffi::Mix_GroupChannel(ch as c_int, g as c_int) == 1 }
    }

    pub fn count(self) -> int {
        //! Count the number of channels in group
        let Group(g) = self;
        unsafe { ffi::Mix_GroupCount(g as c_int) as int }
    }

    pub fn find_available(self) -> Option<Channel> {
        //! Find the first available (not playing) channel in group
        let Group(g) = self;
        let ret = unsafe { ffi::Mix_GroupAvailable(g as c_int) as int };
        if ret == -1 {
            None
        } else {
            Some(Channel(ret))
        }
    }

    pub fn find_oldest(self) -> Option<Channel> {
        //! Find the oldest actively playing channel in group
        let Group(g) = self;
        let ret = unsafe { ffi::Mix_GroupOldest(g as c_int) as int };
        if ret == -1 {
            None
        } else {
            Some(Channel(ret))
        }
    }

    pub fn find_newest(self) -> Option<Channel> {
        //! Find the newest, most recently started, actively playing channel in group.
        let Group(g) = self;
        let ret = unsafe { ffi::Mix_GroupNewer(g as c_int) as int };
        if ret == -1 {
            None
        } else {
            Some(Channel(ret))
        }
    }

    pub fn fade_out(self, ms: int) -> int {
        //! Gradually fade out channels in group over some milliseconds starting from now.
        //! Returns the number of channels set to fade out.
        let Group(g) = self;
        unsafe { ffi::Mix_FadeOutGroup(g as c_int, ms as c_int) as int }
    }

    pub fn halt(self) {
        //! Halt playback on all channels in group.
        let Group(g) = self;
        unsafe { ffi::Mix_HaltGroup(g as c_int); }
    }
}

// 4.5 Music

pub fn get_music_decoders_number() -> int {
    //! Get the number of music decoders available.
    unsafe { ffi::Mix_GetNumMusicDecoders() as int }
}

pub fn get_music_decoder(index: int) -> ~str {
    //! Get the name of the indexed music decoder.
    unsafe {
        let name = ffi::Mix_GetMusicDecoder(index as c_int);
        CString::new(name, false).as_str().unwrap().into_owned()
    }
}


#[repr(C)]
#[deriving(Clone, Eq, Hash, Show, FromPrimitive)]
pub enum MusicType {
    TypeNone    = ffi::MUS_NONE as int,
    TypeCmd     = ffi::MUS_CMD as int,
    TypeWav     = ffi::MUS_WAV as int,
    TypeMod     = ffi::MUS_MOD as int,
    TypeMid     = ffi::MUS_MID as int,
    TypeOgg     = ffi::MUS_OGG as int,
    TypeMp3     = ffi::MUS_MP3 as int,
    TypeMp3Mad  = ffi::MUS_MP3_MAD as int,
    TypeFlac    = ffi::MUS_FLAC as int,
    TypeModPlug = ffi::MUS_MODPLUG as int
}

/// This is an opaque data type used for Music data.
#[deriving(Eq)] #[allow(raw_pointer_deriving, visible_private_types)]
pub struct Music {
    pub raw: *ffi::Mix_Music,
    pub owned: bool
}

impl Drop for Music {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::Mix_FreeMusic(self.raw) };
        }
    }
}

impl Music {
    pub fn from_file(path: &Path) -> Result<~Music, ~str> {
        //! Load music file to use.
        let raw = unsafe {
            ffi::Mix_LoadMUS(path.to_c_str().unwrap())
        };
        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(~Music{ raw: raw, owned: true })
        }
    }

    pub fn get_type(&self) -> MusicType {
        //! The file format encoding of the music.
        let ret = unsafe { ffi::Mix_GetMusicType(self.raw) as int };
        FromPrimitive::from_int(ret).unwrap()
    }

    pub fn play(&self, loops: int) -> Result<(), ~str> {
        //! Play the loaded music loop times through from start to finish.
        let ret = unsafe {
            ffi::Mix_PlayMusic(self.raw, loops as c_int)
        };
        if ret == -1 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    pub fn fade_in(&self, loops: int, ms: int) -> Result<(), ~str> {
        //! Fade in over ms milliseconds of time, the loaded music,
        //! playing it loop times through from start to finish.
        let ret = unsafe {
            ffi::Mix_FadeInMusic(self.raw, loops as c_int, ms as c_int)
        };
        if ret == -1 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    pub fn fade_in_from_pos(&self, loops: int, ms: int, position: f64) -> Result<(), ~str> {
        //! Fade in over ms milliseconds of time, from position.
        let ret = unsafe {
            ffi::Mix_FadeInMusicPos(self.raw, loops as c_int, ms as c_int, position as c_double)
        };
        if ret == -1 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    // FIXME: make these class method?
    pub fn get_volume() -> int {
        unsafe { ffi::Mix_VolumeMusic(-1) as int }
    }

    pub fn set_volume(volume: int) -> int {
        //! Set the volume.
        unsafe { ffi::Mix_VolumeMusic(volume as c_int) as int }
    }

    pub fn pause() {
        //! Pause the music playback.
        unsafe { ffi::Mix_PauseMusic(); }
    }

    pub fn resume() {
        //! Unpause the music.
        unsafe { ffi::Mix_ResumeMusic(); }
    }

    pub fn rewind() {
        //! Rewind the music to the start.
        unsafe { ffi::Mix_RewindMusic(); }
    }

    pub fn set_pos(position: f64) -> Result<(), ~str> {
        //! Set the position of the currently playing music.
        let ret = unsafe {
            ffi::Mix_SetMusicPosition(position as c_double)
        };
        if ret == -1 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    pub fn set_command(command: &str) -> Result<(), ~str> {
        //! Setup a command line music player to use to play music.
        let ret = unsafe {
            ffi::Mix_SetMusicCMD(command.to_c_str().unwrap())
        };
        if ret == -1 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    pub fn halt() {
        //! Halt playback of music.
        unsafe { ffi::Mix_HaltMusic(); }
    }

    pub fn fade_out(ms: int) -> Result<(), ~str> {
        //! Gradually fade out the music over ms milliseconds starting from now.
        let ret = unsafe {
            ffi::Mix_FadeOutMusic(ms as c_int)
        };
        if ret == -1 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    // TODO: Mix_HookMusic
    // TODO: Mix_HookMusicFinished
    // TODO: Mix_GetMusicHookData

    pub fn is_playing() -> bool {
        //! If music is actively playing, or not.
        unsafe { ffi::Mix_PlayingMusic() == 1 }
    }

    pub fn is_paused() -> bool {
        //! If music is paused, or not.
        unsafe { ffi::Mix_PausedMusic() == 1 }
    }

    pub fn get_fading() -> Fading {
        //! If music is fading, or not.
        let ret = unsafe { ffi::Mix_FadingMusic() as int };
        FromPrimitive::from_int(ret).unwrap()
    }
}

// 4.6 Effects

// TODO: Mix_RegisterEffect
// TODO: Mix_UnregisterEffect
// TODO: Mix_SetPostMix
