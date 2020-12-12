use std::convert::TryFrom;
use std::intrinsics::transmute;

use crate::sys::SDL_TextureAccess;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(i32)]
pub enum TextureAccess {
    Static = SDL_TextureAccess::SDL_TEXTUREACCESS_STATIC as i32,
    Streaming = SDL_TextureAccess::SDL_TEXTUREACCESS_STREAMING as i32,
    Target = SDL_TextureAccess::SDL_TEXTUREACCESS_TARGET as i32,
}

impl TryFrom<u32> for TextureAccess {
    type Error = ();

    fn try_from(n: u32) -> Result<Self, Self::Error> {
        use self::TextureAccess::*;
        use crate::sys::SDL_TextureAccess::*;

        Ok(match unsafe { transmute(n) } {
            SDL_TEXTUREACCESS_STATIC => Static,
            SDL_TEXTUREACCESS_STREAMING => Streaming,
            SDL_TEXTUREACCESS_TARGET => Target,
        })
    }
}
