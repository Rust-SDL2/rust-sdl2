use crate::sys::SDL_AudioStatus;

use std::convert::TryFrom;
use std::mem;

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AudioStatus {
    Stopped = SDL_AudioStatus::SDL_AUDIO_STOPPED as i32,
    Playing = SDL_AudioStatus::SDL_AUDIO_PLAYING as i32,
    Paused = SDL_AudioStatus::SDL_AUDIO_PAUSED as i32,
}

impl TryFrom<u32> for AudioStatus {
    type Error = ();

    fn try_from(n: u32) -> Result<Self, Self::Error> {
        use self::AudioStatus::*;
        use crate::sys::SDL_AudioStatus::*;

        Ok(match unsafe { mem::transmute(n) } {
            SDL_AUDIO_STOPPED => Stopped,
            SDL_AUDIO_PLAYING => Playing,
            SDL_AUDIO_PAUSED => Paused,
        })
    }
}
