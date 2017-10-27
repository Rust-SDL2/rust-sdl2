use std::os::raw::{c_uint, c_int, c_char, c_double, c_void};
use sys;

pub type MIX_InitFlags = c_uint;
pub const MIX_INIT_FLAC: c_uint = 1;
pub const MIX_INIT_MOD: c_uint = 2;
pub const MIX_INIT_MODPLUG: c_uint = 4;
pub const MIX_INIT_MP3: c_uint = 8;
pub const MIX_INIT_OGG: c_uint = 16;
pub const MIX_INIT_FLUIDSYNTH: c_uint = 32;
#[repr(C)]
pub struct Struct_Mix_Chunk {
    pub allocated: c_int,
    pub abuf: *const u8,
    pub alen: u32,
    pub volume: u8,
}
pub type Mix_Chunk = Struct_Mix_Chunk;
pub type Mix_Fading = c_uint;
pub const MIX_NO_FADING: c_uint = 0;
pub const MIX_FADING_OUT: c_uint = 1;
pub const MIX_FADING_IN: c_uint = 2;
pub type Mix_MusicType = c_uint;
pub const MUS_NONE: c_uint = 0;
pub const MUS_CMD: c_uint = 1;
pub const MUS_WAV: c_uint = 2;
pub const MUS_MOD: c_uint = 3;
pub const MUS_MID: c_uint = 4;
pub const MUS_OGG: c_uint = 5;
pub const MUS_MP3: c_uint = 6;
pub const MUS_MP3_MAD: c_uint = 7;
pub const MUS_FLAC: c_uint = 8;
pub const MUS_MODPLUG: c_uint = 9;
pub type Struct__Mix_Music = c_void;
pub type Mix_Music = Struct__Mix_Music;
pub type Mix_EffectFunc_t = ::std::option::Option<extern "C" fn(arg1: c_int,
                                                                  arg2: *const c_void,
                                                                  arg3: c_int,
                                                                  arg4: *const c_void)
                                                                 >;
pub type Mix_EffectDone_t = ::std::option::Option<extern "C" fn(arg1: c_int,
                                                                  arg2: *const c_void)
                                                                 >;
extern "C" {
    pub fn Mix_Linked_Version() -> *const sys::SDL_version;
    pub fn Mix_Init(flags: c_int) -> c_int;
    pub fn Mix_Quit();
    pub fn Mix_OpenAudio(frequency: c_int,
                         format: u16,
                         channels: c_int,
                         chunksize: c_int)
                         -> c_int;
    pub fn Mix_AllocateChannels(numchans: c_int) -> c_int;
    pub fn Mix_QuerySpec(frequency: *mut c_int,
                         format: *mut u16,
                         channels: *mut c_int)
                         -> c_int;
    pub fn Mix_LoadWAV_RW(src: *mut sys::SDL_RWops, freesrc: c_int) -> *mut Mix_Chunk;
    pub fn Mix_LoadMUS(file: *const c_char) -> *mut Mix_Music;
    pub fn Mix_LoadMUS_RW(src: *mut sys::SDL_RWops, freesrc: c_int) -> *mut Mix_Music;
    pub fn Mix_LoadMUSType_RW(src: *mut sys::SDL_RWops,
                              type_: Mix_MusicType,
                              freesrc: c_int)
                              -> *mut Mix_Music;
    pub fn Mix_QuickLoad_WAV(mem: *mut u8) -> *mut Mix_Chunk;
    pub fn Mix_QuickLoad_RAW(mem: *mut u8, len: u32) -> *mut Mix_Chunk;
    pub fn Mix_FreeChunk(chunk: *mut Mix_Chunk);
    pub fn Mix_FreeMusic(music: *mut Mix_Music);
    pub fn Mix_GetNumChunkDecoders() -> c_int;
    pub fn Mix_GetChunkDecoder(index: c_int) -> *const c_char;
    pub fn Mix_GetNumMusicDecoders() -> c_int;
    pub fn Mix_GetMusicDecoder(index: c_int) -> *const c_char;
    pub fn Mix_GetMusicType(music: *const Mix_Music) -> Mix_MusicType;
    pub fn Mix_SetPostMix(mix_func: Option<unsafe extern "C" fn(udata: *mut c_void,
                                                                stream: *mut u8,
                                                                len: c_int)>,
                          arg: *mut c_void);
    pub fn Mix_HookMusic(mix_func: Option<unsafe extern "C" fn(udata: *mut c_void,
                                                               stream: *mut u8,
                                                               len: c_int)>,
                         arg: *mut c_void);
    pub fn Mix_HookMusicFinished(music_finished: Option<extern "C" fn()>);
    pub fn Mix_GetMusicHookData() -> *mut c_void;
    pub fn Mix_ChannelFinished(channel_finished: Option<extern "C" fn(channel: c_int)>);
    pub fn Mix_RegisterEffect(chan: c_int,
                              f: Mix_EffectFunc_t,
                              d: Mix_EffectDone_t,
                              arg: *mut c_void)
                              -> c_int;
    pub fn Mix_UnregisterEffect(channel: c_int, f: Mix_EffectFunc_t) -> c_int;
    pub fn Mix_UnregisterAllEffects(channel: c_int) -> c_int;
    pub fn Mix_SetPanning(channel: c_int, left: u8, right: u8) -> c_int;
    pub fn Mix_SetPosition(channel: c_int, angle: i16, distance: u8) -> c_int;
    pub fn Mix_SetDistance(channel: c_int, distance: u8) -> c_int;
    pub fn Mix_SetReverseStereo(channel: c_int, flip: c_int) -> c_int;
    pub fn Mix_ReserveChannels(num: c_int) -> c_int;
    pub fn Mix_GroupChannel(which: c_int, tag: c_int) -> c_int;
    pub fn Mix_GroupChannels(from: c_int, to: c_int, tag: c_int) -> c_int;
    pub fn Mix_GroupAvailable(tag: c_int) -> c_int;
    pub fn Mix_GroupCount(tag: c_int) -> c_int;
    pub fn Mix_GroupOldest(tag: c_int) -> c_int;
    pub fn Mix_GroupNewer(tag: c_int) -> c_int;
    pub fn Mix_PlayChannelTimed(channel: c_int,
                                chunk: *mut Mix_Chunk,
                                loops: c_int,
                                ticks: c_int)
                                -> c_int;
    pub fn Mix_PlayMusic(music: *mut Mix_Music, loops: c_int) -> c_int;
    pub fn Mix_FadeInMusic(music: *mut Mix_Music, loops: c_int, ms: c_int) -> c_int;
    pub fn Mix_FadeInMusicPos(music: *mut Mix_Music,
                              loops: c_int,
                              ms: c_int,
                              position: c_double)
                              -> c_int;
    pub fn Mix_FadeInChannelTimed(channel: c_int,
                                  chunk: *mut Mix_Chunk,
                                  loops: c_int,
                                  ms: c_int,
                                  ticks: c_int)
                                  -> c_int;
    pub fn Mix_Volume(channel: c_int, volume: c_int) -> c_int;
    pub fn Mix_VolumeChunk(chunk: *mut Mix_Chunk, volume: c_int) -> c_int;
    pub fn Mix_VolumeMusic(volume: c_int) -> c_int;
    pub fn Mix_HaltChannel(channel: c_int) -> c_int;
    pub fn Mix_HaltGroup(tag: c_int) -> c_int;
    pub fn Mix_HaltMusic() -> c_int;
    pub fn Mix_ExpireChannel(channel: c_int, ticks: c_int) -> c_int;
    pub fn Mix_FadeOutChannel(which: c_int, ms: c_int) -> c_int;
    pub fn Mix_FadeOutGroup(tag: c_int, ms: c_int) -> c_int;
    pub fn Mix_FadeOutMusic(ms: c_int) -> c_int;
    pub fn Mix_FadingMusic() -> Mix_Fading;
    pub fn Mix_FadingChannel(which: c_int) -> Mix_Fading;
    pub fn Mix_Pause(channel: c_int);
    pub fn Mix_Resume(channel: c_int);
    pub fn Mix_Paused(channel: c_int) -> c_int;
    pub fn Mix_PauseMusic();
    pub fn Mix_ResumeMusic();
    pub fn Mix_RewindMusic();
    pub fn Mix_PausedMusic() -> c_int;
    pub fn Mix_SetMusicPosition(position: c_double) -> c_int;
    pub fn Mix_Playing(channel: c_int) -> c_int;
    pub fn Mix_PlayingMusic() -> c_int;
    pub fn Mix_SetMusicCMD(command: *const c_char) -> c_int;
    pub fn Mix_SetSynchroValue(value: c_int) -> c_int;
    pub fn Mix_GetSynchroValue() -> c_int;
    pub fn Mix_SetSoundFonts(paths: *const c_char) -> c_int;
    pub fn Mix_GetSoundFonts() -> *const c_char;
    pub fn Mix_EachSoundFont(function: Option<unsafe extern "C" fn(arg1: *const c_char,
                                                                   arg2: *mut c_void) -> c_int>,
                             data: *mut c_void) -> c_int;
    pub fn Mix_GetChunk(channel: c_int) -> *mut Mix_Chunk;
    pub fn Mix_CloseAudio();
}
