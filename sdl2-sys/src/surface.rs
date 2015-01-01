use pixels::SDL_PixelFormat;
use pixels::SDL_Palette;
use rwops::SDL_RWops;
use rect::Rect;
use libc::{c_int, c_void, uint32_t, uint8_t};
pub use render::SDL_BlendMode;

pub type SDL_bool = c_int;
pub type SDL_Rect = Rect;

pub type SDL_SurfaceFlag = c_int;

pub const SDL_SWSURFACE: SDL_SurfaceFlag = 0;
pub const SDL_PREALLOC: SDL_SurfaceFlag = 0x00000001;
pub const SDL_RLEACCEL: SDL_SurfaceFlag = 0x00000002;
pub const SDL_DONTFREE: SDL_SurfaceFlag = 0x00000004;

//SDL_surface.h
#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct SDL_BlitMap;

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct SDL_Surface {
    pub flags: uint32_t,
    pub format: *const SDL_PixelFormat,
    pub w: c_int,
    pub h: c_int,
    pub pitch: c_int,
    pub pixels: *const c_void,
    pub userdata: *const c_void,
    pub locked: c_int,
    pub lock_data: *const c_void,
    pub clip_rect: SDL_Rect,
    pub map: *const SDL_BlitMap,
    pub refcount: c_int
}

extern "C" {
    pub fn SDL_CreateRGBSurface(flags: uint32_t, width: c_int, height: c_int, depth: c_int, Rmask: uint32_t, Gmask: uint32_t, Bmask: uint32_t, Amask: uint32_t) ->  *const SDL_Surface;
    pub fn SDL_CreateRGBSurfaceFrom(pixels: *const c_void, width: c_int, height: c_int, depth: c_int, pitch: c_int, Rmask: uint32_t, Gmask: uint32_t, Bmask: uint32_t, Amask: uint32_t) ->  *const SDL_Surface;
    pub fn SDL_FreeSurface(surface: *const SDL_Surface);
    pub fn SDL_SetSurfacePalette(surface: *const SDL_Surface, palette: *const SDL_Palette) -> c_int;
    pub fn SDL_LockSurface(surface: *const SDL_Surface) -> c_int;
    pub fn SDL_UnlockSurface(surface: *const SDL_Surface);
    pub fn SDL_LoadBMP_RW(src: *const SDL_RWops, freesrc: c_int) ->  *const SDL_Surface;
    pub fn SDL_SaveBMP_RW(surface: *const SDL_Surface, dst: *const SDL_RWops, freedst: c_int) -> c_int;
    pub fn SDL_SetSurfaceRLE(surface: *const SDL_Surface, flag: c_int) -> c_int;
    pub fn SDL_SetColorKey(surface: *const SDL_Surface, flag: c_int, key: uint32_t) -> c_int;
    pub fn SDL_GetColorKey(surface: *const SDL_Surface, key: *const uint32_t) -> c_int;
    pub fn SDL_SetSurfaceColorMod(surface: *const SDL_Surface, r: uint8_t, g: uint8_t, b: uint8_t) -> c_int;
    pub fn SDL_GetSurfaceColorMod(surface: *const SDL_Surface, r: *const uint8_t, g: *const uint8_t, b: *const uint8_t ) -> c_int;
    pub fn SDL_SetSurfaceAlphaMod(surface: *const SDL_Surface, alpha: uint8_t) -> c_int;
    pub fn SDL_GetSurfaceAlphaMod(surface: *const SDL_Surface, alpha: *const uint8_t ) -> c_int;
    pub fn SDL_SetSurfaceBlendMode(surface: *const SDL_Surface, blendMode: SDL_BlendMode) -> c_int;
    pub fn SDL_GetSurfaceBlendMode(surface: *const SDL_Surface, blendMode: *const SDL_BlendMode) -> c_int;
    pub fn SDL_SetClipRect(surface: *const SDL_Surface, rect: *const SDL_Rect) ->  SDL_bool;
    pub fn SDL_GetClipRect(surface: *const SDL_Surface, rect: *const SDL_Rect);
    pub fn SDL_ConvertSurface(src: *const SDL_Surface, fmt: *const SDL_PixelFormat, flags: uint32_t) ->  *const SDL_Surface;
    pub fn SDL_ConvertSurfaceFormat(src: *const SDL_Surface, pixel_format: uint32_t, flags: uint32_t) ->  *const SDL_Surface;
    pub fn SDL_ConvertPixels(width: c_int, height: c_int, src_format: uint32_t, src: *const c_void, src_pitch: c_int, dst_format: uint32_t, dst: *const c_void, dst_pitch: c_int) -> c_int;
    pub fn SDL_FillRect(dst: *const SDL_Surface, rect: *const SDL_Rect, color: uint32_t) -> c_int;
    pub fn SDL_FillRects(dst: *const SDL_Surface, rects: *const SDL_Rect, count: c_int, color: uint32_t) -> c_int;
    pub fn SDL_UpperBlit(src: *const SDL_Surface, srcrect: *const SDL_Rect, dst: *const SDL_Surface, dstrect: *const SDL_Rect) -> c_int;
    pub fn SDL_LowerBlit(src: *const SDL_Surface, srcrect: *const SDL_Rect, dst: *const SDL_Surface, dstrect: *const SDL_Rect) -> c_int;
    pub fn SDL_SoftStretch(src: *const SDL_Surface, srcrect: *const SDL_Rect, dst: *const SDL_Surface, dstrect: *const SDL_Rect) -> c_int;
    pub fn SDL_UpperBlitScaled(src: *const SDL_Surface, srcrect: *const SDL_Rect, dst: *const SDL_Surface, dstrect: *const SDL_Rect) -> c_int;
    pub fn SDL_LowerBlitScaled(src: *const SDL_Surface, srcrect: *const SDL_Rect, dst: *const SDL_Surface, dstrect: *const SDL_Rect) -> c_int;
}
