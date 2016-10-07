use libc::{c_int, c_uint, c_char, c_void, c_float, c_double};
use libc::{uint8_t, uint32_t};
use rect::{SDL_Point, SDL_Rect};
use surface::SDL_Surface;
use video::SDL_Window;
use sdl::SDL_bool;

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
pub struct SDL_Renderer(c_void);
#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct SDL_Texture(c_void);

//SDL_blendmode.h
pub type SDL_BlendMode = c_int;
pub const SDL_BLENDMODE_NONE : SDL_BlendMode = 0x00000000;
pub const SDL_BLENDMODE_BLEND : SDL_BlendMode = 0x00000001;
pub const SDL_BLENDMODE_ADD : SDL_BlendMode = 0x00000002;
pub const SDL_BLENDMODE_MOD : SDL_BlendMode = 0x00000004;

extern "C" {
    pub fn SDL_GetNumRenderDrivers() -> c_int;
    pub fn SDL_GetRenderDriverInfo(index: c_int, info: *mut SDL_RendererInfo) -> c_int;
    pub fn SDL_CreateWindowAndRenderer(width: c_int, height: c_int, window_flags: uint32_t, window: *mut *mut SDL_Window, renderer: *mut *mut SDL_Renderer) -> c_int;
    pub fn SDL_CreateRenderer(window: *mut SDL_Window, index: c_int, flags: uint32_t) -> *mut SDL_Renderer;
    pub fn SDL_CreateSoftwareRenderer(surface: *mut SDL_Surface) -> *mut SDL_Renderer;
    pub fn SDL_GetRenderer(window: *mut SDL_Window) -> *mut SDL_Renderer;
    pub fn SDL_GetRendererInfo(renderer: *mut SDL_Renderer, info: *mut SDL_RendererInfo) -> c_int;
    pub fn SDL_GetRendererOutputSize(renderer: *mut SDL_Renderer, w: *mut c_int, h: *mut c_int) -> c_int;
    pub fn SDL_CreateTexture(renderer: *mut SDL_Renderer, format: uint32_t, access: c_int, w: c_int, h: c_int) -> *mut SDL_Texture;
    pub fn SDL_CreateTextureFromSurface(renderer: *mut SDL_Renderer, surface: *mut SDL_Surface) -> *mut SDL_Texture;
    pub fn SDL_QueryTexture(texture: *mut SDL_Texture, format: *mut uint32_t, access: *mut c_int, w: *mut c_int, h: *mut c_int) -> c_int;
    pub fn SDL_SetTextureColorMod(texture: *mut SDL_Texture, r: uint8_t, g: uint8_t, b: uint8_t) -> c_int;
    pub fn SDL_GetTextureColorMod(texture: *mut SDL_Texture, r: *mut uint8_t, g: *mut uint8_t, b: *mut uint8_t) -> c_int;
    pub fn SDL_SetTextureAlphaMod(texture: *mut SDL_Texture, alpha: uint8_t) -> c_int;
    pub fn SDL_GetTextureAlphaMod(texture: *mut SDL_Texture, alpha: *mut uint8_t) -> c_int;
    pub fn SDL_SetTextureBlendMode(texture: *mut SDL_Texture, blendMode: SDL_BlendMode) -> c_int;
    pub fn SDL_GetTextureBlendMode(texture: *mut SDL_Texture, blendMode: *mut SDL_BlendMode) -> c_int;
    pub fn SDL_UpdateTexture(texture: *mut SDL_Texture, rect: *const SDL_Rect, pixels: *const c_void, pitch: c_int) -> c_int;
    pub fn SDL_UpdateYUVTexture(texture: *mut SDL_Texture, rect: *const SDL_Rect, Yplane: *const uint8_t, Ypitch: c_int, Uplane: *const uint8_t, Upitch: c_int, Vplane: *const uint8_t, Vpitch: c_int) -> c_int;
    pub fn SDL_LockTexture(texture: *mut SDL_Texture, rect: *const SDL_Rect, pixels: *mut *mut c_void, pitch: *mut c_int) -> c_int;
    pub fn SDL_UnlockTexture(texture: *mut SDL_Texture);
    pub fn SDL_RenderTargetSupported(renderer: *mut SDL_Renderer) -> SDL_bool;
    pub fn SDL_SetRenderTarget(renderer: *mut SDL_Renderer, texture: *mut SDL_Texture) -> c_int;
    pub fn SDL_GetRenderTarget(renderer: *mut SDL_Renderer) -> *mut SDL_Texture;
    pub fn SDL_RenderSetLogicalSize(renderer: *mut SDL_Renderer, w: c_int, h: c_int) -> c_int;
    pub fn SDL_RenderGetLogicalSize(renderer: *mut SDL_Renderer, w: *mut c_int, h: *mut c_int);
    pub fn SDL_RenderSetViewport(renderer: *mut SDL_Renderer, rect: *const SDL_Rect) -> c_int;
    pub fn SDL_RenderGetViewport(renderer: *mut SDL_Renderer, rect: *mut SDL_Rect);
    pub fn SDL_RenderSetClipRect(renderer: *mut SDL_Renderer, rect: *const SDL_Rect) -> c_int;
    pub fn SDL_RenderGetClipRect(renderer: *mut SDL_Renderer, rect: *mut SDL_Rect);
    pub fn SDL_RenderSetScale(renderer: *mut SDL_Renderer, scaleX: c_float, scaleY: c_float) -> c_int;
    pub fn SDL_RenderGetScale(renderer: *mut SDL_Renderer, scaleX: *mut c_float, scaleY: *mut c_float);
    pub fn SDL_SetRenderDrawColor(renderer: *mut SDL_Renderer, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int;
    pub fn SDL_GetRenderDrawColor(renderer: *mut SDL_Renderer, r: *mut uint8_t, g: *mut uint8_t, b: *mut uint8_t, a: *mut uint8_t) -> c_int;
    pub fn SDL_SetRenderDrawBlendMode(renderer: *mut SDL_Renderer, blendMode: SDL_BlendMode) -> c_int;
    pub fn SDL_GetRenderDrawBlendMode(renderer: *mut SDL_Renderer, blendMode: *mut SDL_BlendMode) -> c_int;
    pub fn SDL_RenderClear(renderer: *mut SDL_Renderer) -> c_int;
    pub fn SDL_RenderDrawPoint(renderer: *mut SDL_Renderer, x: c_int, y: c_int) -> c_int;
    pub fn SDL_RenderDrawPoints(renderer: *mut SDL_Renderer, Points: *const SDL_Point, count: c_int) -> c_int;
    pub fn SDL_RenderDrawLine(renderer: *mut SDL_Renderer, x1: c_int, y1: c_int, x2: c_int, y2: c_int) -> c_int;
    pub fn SDL_RenderDrawLines(renderer: *mut SDL_Renderer, Points: *const SDL_Point, count: c_int) -> c_int;
    pub fn SDL_RenderDrawRect(renderer: *mut SDL_Renderer, rect: *const SDL_Rect) -> c_int;
    pub fn SDL_RenderDrawRects(renderer: *mut SDL_Renderer, rects: *const SDL_Rect, count: c_int) -> c_int;
    pub fn SDL_RenderFillRect(renderer: *mut SDL_Renderer, rect: *const SDL_Rect) -> c_int;
    pub fn SDL_RenderFillRects(renderer: *mut SDL_Renderer, rects: *const SDL_Rect, count: c_int) -> c_int;
    pub fn SDL_RenderCopy(renderer: *mut SDL_Renderer, texture: *mut SDL_Texture, srcrect: *const SDL_Rect, dstrect: *const SDL_Rect) -> c_int;
    pub fn SDL_RenderCopyEx(renderer: *mut SDL_Renderer, texture: *mut SDL_Texture, srcrect: *const SDL_Rect, dstrect: *const SDL_Rect, angle: c_double, center: *const SDL_Point, flip: SDL_RendererFlip) -> c_int;
    pub fn SDL_RenderReadPixels(renderer: *mut SDL_Renderer, rect: *const SDL_Rect, format: uint32_t, pixels: *mut c_void, pitch: c_int) -> c_int;
    pub fn SDL_RenderPresent(renderer: *mut SDL_Renderer);
    pub fn SDL_DestroyTexture(texture: *mut SDL_Texture);
    pub fn SDL_DestroyRenderer(renderer: *mut SDL_Renderer);
    pub fn SDL_GL_BindTexture(texture: *mut SDL_Texture, texw: *mut c_float, texh: *mut c_float) -> c_int;
    pub fn SDL_GL_UnbindTexture(texture: *mut SDL_Texture) -> c_int;
}
