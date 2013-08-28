pub mod ll {
    use pixels::ll::SDL_PixelFormat;
    use pixels::ll::SDL_Palette;
    use render::ll::SDL_BlendMode;
    use rwops::ll::SDL_RWops;
    use rect::Rect;
    use std::libc::{c_int, c_void, uint32_t, uint8_t};
    pub type SDL_Rect = Rect;
    pub type SDL_bool = c_int;

    pub type SDL_SurfaceFlag = c_int;

    pub static SDL_SWSURFACE: SDL_SurfaceFlag = 0;
    pub static SDL_PREALLOC: SDL_SurfaceFlag = 0x00000001;
    pub static SDL_RLEACCEL: SDL_SurfaceFlag = 0x00000002;
    pub static SDL_DONTFREE: SDL_SurfaceFlag = 0x00000004;

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

    externfn!(fn SDL_CreateRGBSurface(flags: uint32_t, width: c_int, height: c_int, depth: c_int, Rmask: uint32_t, Gmask: uint32_t, Bmask: uint32_t, Amask: uint32_t) ->  *SDL_Surface)
    externfn!(fn SDL_CreateRGBSurfaceFrom(pixels: *c_void, width: c_int, height: c_int, depth: c_int, pitch: c_int, Rmask: uint32_t, Gmask: uint32_t, Bmask: uint32_t, Amask: uint32_t) ->  *SDL_Surface)
    externfn!(fn SDL_FreeSurface(surface: *SDL_Surface))
    externfn!(fn SDL_SetSurfacePalette(surface: *SDL_Surface, palette: *SDL_Palette) -> c_int)
    externfn!(fn SDL_LockSurface(surface: *SDL_Surface) -> c_int)
    externfn!(fn SDL_UnlockSurface(surface: *SDL_Surface))
    externfn!(fn SDL_LoadBMP_RW(src: *SDL_RWops, freesrc: c_int) ->  *SDL_Surface)
    externfn!(fn SDL_SaveBMP_RW(surface: *SDL_Surface, dst: *SDL_RWops, freedst: c_int) -> c_int)
    externfn!(fn SDL_SetSurfaceRLE(surface: *SDL_Surface, flag: c_int) -> c_int)
    externfn!(fn SDL_SetColorKey(surface: *SDL_Surface, flag: c_int, key: uint32_t) -> c_int)
    externfn!(fn SDL_GetColorKey(surface: *SDL_Surface, key: *uint32_t) -> c_int)
    externfn!(fn SDL_SetSurfaceColorMod(surface: *SDL_Surface, r: uint8_t, g: uint8_t, b: uint8_t) -> c_int)
    externfn!(fn SDL_GetSurfaceColorMod(surface: *SDL_Surface, r: *uint8_t, g: *uint8_t, b: *uint8_t ) -> c_int)
    externfn!(fn SDL_SetSurfaceAlphaMod(surface: *SDL_Surface, alpha: uint8_t) -> c_int)
    externfn!(fn SDL_GetSurfaceAlphaMod(surface: *SDL_Surface, alpha: *uint8_t ) -> c_int)
    externfn!(fn SDL_SetSurfaceBlendMode(surface: *SDL_Surface, blendMode: SDL_BlendMode) -> c_int)
    externfn!(fn SDL_GetSurfaceBlendMode(surface: *SDL_Surface, blendMode: *SDL_BlendMode) -> c_int)
    externfn!(fn SDL_SetClipRect(surface: *SDL_Surface, rect: *SDL_Rect) ->  SDL_bool)
    externfn!(fn SDL_GetClipRect(surface: *SDL_Surface, rect: *SDL_Rect))
    externfn!(fn SDL_ConvertSurface(src: *SDL_Surface, fmt: *SDL_PixelFormat, flags: uint32_t) ->  *SDL_Surface)
    externfn!(fn SDL_ConvertSurfaceFormat(src: *SDL_Surface, pixel_format: uint32_t, flags: uint32_t) ->  *SDL_Surface)
    externfn!(fn SDL_ConvertPixels(width: c_int, height: c_int, src_format: uint32_t, src: *c_void, src_pitch: c_int, dst_format: uint32_t, dst: *c_void, dst_pitch: c_int) -> c_int)
    externfn!(fn SDL_FillRect(dst: *SDL_Surface, rect: *SDL_Rect, color: uint32_t) -> c_int)
    externfn!(fn SDL_FillRects(dst: *SDL_Surface, rects: *SDL_Rect, count: c_int, color: uint32_t) -> c_int)
    externfn!(fn SDL_UpperBlit(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *SDL_Surface, dstrect: *SDL_Rect) -> c_int)
    externfn!(fn SDL_LowerBlit(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *SDL_Surface, dstrect: *SDL_Rect) -> c_int)
    externfn!(fn SDL_SoftStretch(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *SDL_Surface, dstrect: *SDL_Rect) -> c_int)
    externfn!(fn SDL_UpperBlitScaled(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *SDL_Surface, dstrect: *SDL_Rect) -> c_int)
    externfn!(fn SDL_LowerBlitScaled(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *SDL_Surface, dstrect: *SDL_Rect) -> c_int)
}

#[deriving(Eq)]
pub struct Surface {
    raw: *ll::SDL_Surface,
    owned: bool
}

impl Drop for Surface{
    fn drop(&self) {
        if self.owned {
            unsafe {
                ll::SDL_FreeSurface(self.raw);
            }
        }
    }
}
