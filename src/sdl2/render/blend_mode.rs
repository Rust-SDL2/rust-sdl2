use std::convert::TryFrom;
use std::intrinsics::transmute;

use crate::sys::SDL_BlendMode::*;

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum BlendMode {
    None = SDL_BLENDMODE_NONE as i32,
    Blend = SDL_BLENDMODE_BLEND as i32,
    Add = SDL_BLENDMODE_ADD as i32,
    Mod = SDL_BLENDMODE_MOD as i32,
    Invalid = SDL_BLENDMODE_INVALID as i32,
}

impl TryFrom<u32> for BlendMode {
    type Error = ();

    fn try_from(n: u32) -> Result<Self, Self::Error> {
        use self::BlendMode::*;

        Ok(match unsafe { transmute(n) } {
            SDL_BLENDMODE_NONE => None,
            SDL_BLENDMODE_BLEND => Blend,
            SDL_BLENDMODE_ADD => Add,
            SDL_BLENDMODE_MOD => Mod,
            SDL_BLENDMODE_INVALID => Invalid,
        })
    }
}
