pub mod ll {
    use pixels::ll::SDL_PixelFormat;
    use rect::Rect;
    use std::libc::{c_int, c_void, uint32_t};
    pub type SDL_Rect = Rect;

    //SDL_surface.h
    pub struct SDL_BlitMap;

    pub struct SDL_Surface {
        flags: uint32_t,
        format: *SDL_PixelFormat,
        w: c_int,
        h: c_int,
        pitch: c_int,
        pixels: *c_void,
        userdata: *c_void,
        locked: c_int,
        lock_data: *c_void,
        clip_rect: SDL_Rect,
        map: *SDL_BlitMap,
        refcount: c_int
    }
}

#[deriving(Eq)]
pub struct Surface {
    raw: *ll::SDL_Surface,
    owned: bool
}
