use pixels::SDL_PixelFormat;
use pixels::SDL_Palette;
use rwops::SDL_RWops;
use rect::SDL_Rect;
use libc::{c_int, c_void, uint32_t, uint8_t};
use render::SDL_BlendMode;
use sdl::SDL_bool;

pub type SDL_SurfaceFlag = uint32_t;

pub const SDL_SWSURFACE: SDL_SurfaceFlag = 0;
pub const SDL_PREALLOC: SDL_SurfaceFlag = 0x00000001;
pub const SDL_RLEACCEL: SDL_SurfaceFlag = 0x00000002;
pub const SDL_DONTFREE: SDL_SurfaceFlag = 0x00000004;

//SDL_surface.h
#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct SDL_BlitMap(c_void);

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct SDL_Surface {
    pub flags: uint32_t,
    pub format: *mut SDL_PixelFormat,
    pub w: c_int,
    pub h: c_int,
    pub pitch: c_int,
    pub pixels: *mut c_void,
    pub userdata: *mut c_void,
    pub locked: c_int,
    pub lock_data: *mut c_void,
    pub clip_rect: SDL_Rect,
    pub map: *mut SDL_BlitMap,
    pub refcount: c_int
}

extern "C" {
    pub fn SDL_CreateRGBSurface(flags: uint32_t, width: c_int, height: c_int, depth: c_int, Rmask: uint32_t, Gmask: uint32_t, Bmask: uint32_t, Amask: uint32_t) ->  *mut SDL_Surface;
    pub fn SDL_CreateRGBSurfaceFrom(pixels: *mut c_void, width: c_int, height: c_int, depth: c_int, pitch: c_int, Rmask: uint32_t, Gmask: uint32_t, Bmask: uint32_t, Amask: uint32_t) ->  *mut SDL_Surface;
    pub fn SDL_FreeSurface(surface: *mut SDL_Surface);
    pub fn SDL_SetSurfacePalette(surface: *mut SDL_Surface, palette: *mut SDL_Palette) -> c_int;
    pub fn SDL_LockSurface(surface: *mut SDL_Surface) -> c_int;
    pub fn SDL_UnlockSurface(surface: *mut SDL_Surface);
    pub fn SDL_LoadBMP_RW(src: *mut SDL_RWops, freesrc: c_int) ->  *mut SDL_Surface;
    pub fn SDL_SaveBMP_RW(surface: *mut SDL_Surface, dst: *mut SDL_RWops, freedst: c_int) -> c_int;
    pub fn SDL_SetSurfaceRLE(surface: *mut SDL_Surface, flag: c_int) -> c_int;
    pub fn SDL_SetColorKey(surface: *mut SDL_Surface, flag: c_int, key: uint32_t) -> c_int;
    pub fn SDL_GetColorKey(surface: *mut SDL_Surface, key: *mut uint32_t) -> c_int;
    pub fn SDL_SetSurfaceColorMod(surface: *mut SDL_Surface, r: uint8_t, g: uint8_t, b: uint8_t) -> c_int;
    pub fn SDL_GetSurfaceColorMod(surface: *mut SDL_Surface, r: *mut uint8_t, g: *mut uint8_t, b: *mut uint8_t ) -> c_int;
    pub fn SDL_SetSurfaceAlphaMod(surface: *mut SDL_Surface, alpha: uint8_t) -> c_int;
    pub fn SDL_GetSurfaceAlphaMod(surface: *mut SDL_Surface, alpha: *mut uint8_t ) -> c_int;
    pub fn SDL_SetSurfaceBlendMode(surface: *mut SDL_Surface, blendMode: SDL_BlendMode) -> c_int;
    pub fn SDL_GetSurfaceBlendMode(surface: *mut SDL_Surface, blendMode: *mut SDL_BlendMode) -> c_int;
    pub fn SDL_SetClipRect(surface: *mut SDL_Surface, rect: *const SDL_Rect) ->  SDL_bool;
    pub fn SDL_GetClipRect(surface: *mut SDL_Surface, rect: *mut SDL_Rect);
    pub fn SDL_ConvertSurface(src: *mut SDL_Surface, fmt: *mut SDL_PixelFormat, flags: uint32_t) ->  *mut SDL_Surface;
    pub fn SDL_ConvertSurfaceFormat(src: *mut SDL_Surface, pixel_format: uint32_t, flags: uint32_t) ->  *mut SDL_Surface;
    pub fn SDL_ConvertPixels(width: c_int, height: c_int, src_format: uint32_t, src: *const c_void, src_pitch: c_int, dst_format: uint32_t, dst: *mut c_void, dst_pitch: c_int) -> c_int;
    pub fn SDL_FillRect(dst: *mut SDL_Surface, rect: *const SDL_Rect, color: uint32_t) -> c_int;
    pub fn SDL_FillRects(dst: *mut SDL_Surface, rects: *const SDL_Rect, count: c_int, color: uint32_t) -> c_int;
    pub fn SDL_UpperBlit(src: *mut SDL_Surface, srcrect: *const SDL_Rect, dst: *mut SDL_Surface, dstrect: *mut SDL_Rect) -> c_int;
    pub fn SDL_LowerBlit(src: *mut SDL_Surface, srcrect: *mut SDL_Rect, dst: *mut SDL_Surface, dstrect: *mut SDL_Rect) -> c_int;
    pub fn SDL_SoftStretch(src: *mut SDL_Surface, srcrect: *mut SDL_Rect, dst: *mut SDL_Surface, dstrect: *mut SDL_Rect) -> c_int;
    pub fn SDL_UpperBlitScaled(src: *mut SDL_Surface, srcrect: *const SDL_Rect, dst: *mut SDL_Surface, dstrect: *mut SDL_Rect) -> c_int;
    pub fn SDL_LowerBlitScaled(src: *mut SDL_Surface, srcrect: *mut SDL_Rect, dst: *mut SDL_Surface, dstrect: *mut SDL_Rect) -> c_int;
}
