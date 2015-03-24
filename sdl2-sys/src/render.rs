use libc::{c_int, c_uint, c_char, c_void, c_float, c_double};
use libc::{uint8_t, uint32_t};
use rect::Point;
use rect::Rect;

use surface::SDL_Surface;
use video::SDL_Window;

pub type SDL_Point = Point;
pub type SDL_Rect = Rect;
pub type SDL_bool = c_int;

//SDL_render.h
pub type SDL_RendererFlags = c_uint;
pub const SDL_RENDERER_SOFTWARE : SDL_RendererFlags = 0x00000001;
pub const SDL_RENDERER_ACCELERATED : SDL_RendererFlags = 0x00000002;
pub const SDL_RENDERER_PRESENTVSYNC : SDL_RendererFlags = 0x00000004;
pub const SDL_RENDERER_TARGETTEXTURE : SDL_RendererFlags = 0x00000008;

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct SDL_RendererInfo
{
    pub name: *const c_char,
    pub flags: uint32_t,
    pub num_texture_formats: uint32_t,
    pub texture_formats: [uint32_t; 16],
    pub max_texture_width: c_int,
    pub max_texture_height: c_int,
}

pub type SDL_TextureAccess = c_uint;
pub const SDL_TEXTUREACCESS_STATIC : SDL_TextureAccess = 0;
pub const SDL_TEXTUREACCESS_STREAMING : SDL_TextureAccess = 1;
pub const SDL_TEXTUREACCESS_TARGET : SDL_TextureAccess = 2;

pub type SDL_TextureModulate = c_uint;
pub const SDL_TEXTUREMODULATE_NONE : SDL_TextureModulate = 0x00000000;
pub const SDL_TEXTUREMODULATE_COLOR : SDL_TextureModulate = 0x00000001;
pub const SDL_TEXTUREMODULATE_ALPHA : SDL_TextureModulate = 0x00000002;

pub type SDL_RendererFlip = c_uint;
pub const SDL_FLIP_NONE : SDL_RendererFlip = 0x00000000;
pub const SDL_FLIP_HORIZONTAL : SDL_RendererFlip = 0x00000001;
pub const SDL_FLIP_VERTICAL : SDL_RendererFlip = 0x00000002;

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct SDL_Renderer;
#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct SDL_Texture;

//SDL_blendmode.h
pub type SDL_BlendMode = c_int;
pub const SDL_BLENDMODE_NONE : SDL_BlendMode = 0x00000000;
pub const SDL_BLENDMODE_BLEND : SDL_BlendMode = 0x00000001;
pub const SDL_BLENDMODE_ADD : SDL_BlendMode = 0x00000002;
pub const SDL_BLENDMODE_MOD : SDL_BlendMode = 0x00000004;

extern "C" {
    pub fn SDL_GetNumRenderDrivers() -> c_int;
    pub fn SDL_GetRenderDriverInfo(index: c_int, info: *const SDL_RendererInfo) -> c_int;
    pub fn SDL_CreateWindowAndRenderer(width: c_int, height: c_int, window_flags: uint32_t, window: *const *const SDL_Window, renderer: *const *const SDL_Renderer) -> c_int;
    pub fn SDL_CreateRenderer(window: *const SDL_Window, index: c_int, flags: uint32_t) -> *const SDL_Renderer;
    pub fn SDL_CreateSoftwareRenderer(surface: *const SDL_Surface) -> *const SDL_Renderer;
    pub fn SDL_GetRenderer(window: *const SDL_Window) -> *const SDL_Renderer;
    pub fn SDL_GetRendererInfo(renderer: *const SDL_Renderer, info: *const SDL_RendererInfo) -> c_int;
    pub fn SDL_GetRendererOutputSize(renderer: *const SDL_Renderer, w: *const c_int, h: *const c_int) -> c_int;
    pub fn SDL_CreateTexture(renderer: *const SDL_Renderer, format: uint32_t, access: c_int, w: c_int, h: c_int) -> *const SDL_Texture;
    pub fn SDL_CreateTextureFromSurface(renderer: *const SDL_Renderer, surface: *const SDL_Surface) -> *const SDL_Texture;
    pub fn SDL_QueryTexture(texture: *const SDL_Texture, format: *const uint32_t, access: *const c_int, w: *const c_int, h: *const c_int) -> c_int;
    pub fn SDL_SetTextureColorMod(texture: *const SDL_Texture, r: uint8_t, g: uint8_t, b: uint8_t) -> c_int;
    pub fn SDL_GetTextureColorMod(texture: *const SDL_Texture, r: *const uint8_t, g: *const uint8_t, b: *const uint8_t) -> c_int;
    pub fn SDL_SetTextureAlphaMod(texture: *const SDL_Texture, alpha: uint8_t) -> c_int;
    pub fn SDL_GetTextureAlphaMod(texture: *const SDL_Texture, alpha: *const uint8_t) -> c_int;
    pub fn SDL_SetTextureBlendMode(texture: *const SDL_Texture, blendMode: SDL_BlendMode) -> c_int;
    pub fn SDL_GetTextureBlendMode(texture: *const SDL_Texture, blendMode: *const SDL_BlendMode) -> c_int;
    pub fn SDL_UpdateTexture(texture: *const SDL_Texture, rect: *const SDL_Rect, pixels: *const c_void, pitch: c_int) -> c_int;
    pub fn SDL_UpdateYUVTexture(texture: *const SDL_Texture, rect: *const SDL_Rect, Yplane: *const uint8_t, Ypitch: c_int, Uplane: *const uint8_t, Upitch: c_int, Vplane: *const uint8_t, Vpitch: c_int) -> c_int;
    pub fn SDL_LockTexture(texture: *const SDL_Texture, rect: *const SDL_Rect, pixels: *mut *mut c_void, pitch: *mut c_int) -> c_int;
    pub fn SDL_UnlockTexture(texture: *const SDL_Texture);
    pub fn SDL_RenderTargetSupported(renderer: *const SDL_Renderer) -> SDL_bool;
    pub fn SDL_SetRenderTarget(renderer: *const SDL_Renderer, texture: *const SDL_Texture) -> c_int;
    pub fn SDL_GetRenderTarget(renderer: *const SDL_Renderer) -> *const SDL_Texture;
    pub fn SDL_RenderSetLogicalSize(renderer: *const SDL_Renderer, w: c_int, h: c_int) -> c_int;
    pub fn SDL_RenderGetLogicalSize(renderer: *const SDL_Renderer, w: *const c_int, h: *const c_int);
    pub fn SDL_RenderSetViewport(renderer: *const SDL_Renderer, rect: *const SDL_Rect) -> c_int;
    pub fn SDL_RenderGetViewport(renderer: *const SDL_Renderer, rect: *const SDL_Rect);
    pub fn SDL_RenderSetClipRect(renderer: *const SDL_Renderer, rect: *const SDL_Rect) -> c_int;
    pub fn SDL_RenderGetClipRect(renderer: *const SDL_Renderer, rect: *const SDL_Rect);
    pub fn SDL_RenderSetScale(renderer: *const SDL_Renderer, scaleX: c_float, scaleY: c_float) -> c_int;
    pub fn SDL_RenderGetScale(renderer: *const SDL_Renderer, scaleX: *const c_float, scaleY: *const c_float);
    pub fn SDL_SetRenderDrawColor(renderer: *const SDL_Renderer, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;
    pub fn SDL_GetRenderDrawColor(renderer: *const SDL_Renderer, r: *const uint8_t, g: *const uint8_t, b: *const uint8_t, a: *const uint8_t) -> c_int;
    pub fn SDL_SetRenderDrawBlendMode(renderer: *const SDL_Renderer, blendMode: SDL_BlendMode) -> c_int;
    pub fn SDL_GetRenderDrawBlendMode(renderer: *const SDL_Renderer, blendMode: *const SDL_BlendMode) -> c_int;
    pub fn SDL_RenderClear(renderer: *const SDL_Renderer) -> c_int;
    pub fn SDL_RenderDrawPoint(renderer: *const SDL_Renderer, x: c_int, y: c_int) -> c_int;
    pub fn SDL_RenderDrawPoints(renderer: *const SDL_Renderer, Points: *const SDL_Point, count: c_int) -> c_int;
    pub fn SDL_RenderDrawLine(renderer: *const SDL_Renderer, x1: c_int, y1: c_int, x2: c_int, y2: c_int) -> c_int;
    pub fn SDL_RenderDrawLines(renderer: *const SDL_Renderer, Points: *const SDL_Point, count: c_int) -> c_int;
    pub fn SDL_RenderDrawRect(renderer: *const SDL_Renderer, rect: *const SDL_Rect) -> c_int;
    pub fn SDL_RenderDrawRects(renderer: *const SDL_Renderer, rects: *const SDL_Rect, count: c_int) -> c_int;
    pub fn SDL_RenderFillRect(renderer: *const SDL_Renderer, rect: *const SDL_Rect) -> c_int;
    pub fn SDL_RenderFillRects(renderer: *const SDL_Renderer, rects: *const SDL_Rect, count: c_int) -> c_int;
    pub fn SDL_RenderCopy(renderer: *const SDL_Renderer, texture: *const SDL_Texture, srcrect: *const SDL_Rect, dstrect: *const SDL_Rect) -> c_int;
    pub fn SDL_RenderCopyEx(renderer: *const SDL_Renderer, texture: *const SDL_Texture, srcrect: *const SDL_Rect, dstrect: *const SDL_Rect, angle: c_double, center: *const SDL_Point, flip: SDL_RendererFlip) -> c_int;
    pub fn SDL_RenderReadPixels(renderer: *const SDL_Renderer, rect: *const SDL_Rect, format: uint32_t, pixels: *mut c_void, pitch: c_int) -> c_int;
    pub fn SDL_RenderPresent(renderer: *const SDL_Renderer);
    pub fn SDL_DestroyTexture(texture: *const SDL_Texture);
    pub fn SDL_DestroyRenderer(renderer: *const SDL_Renderer);
    pub fn SDL_GL_BindTexture(texture: *const SDL_Texture, texw: *const c_float, texh: *const c_float) -> c_int;
    pub fn SDL_GL_UnbindTexture(texture: *const SDL_Texture) -> c_int;
}
