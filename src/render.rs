use video;
use surface;
use pixels;
use get_error;
use std::ptr;
use std::libc::c_int;

pub mod ll {

    use std::libc::{c_int, c_char, c_void, c_float, c_double};
    use std::libc::{uint8_t, uint32_t};
    use rect::Rect;
    use rect::Point;

    use surface::ll::SDL_Surface;
    use video::ll::SDL_Window;

    pub type SDL_Rect = Rect;
    pub type SDL_Point = Point;
    pub type SDL_bool = c_int;

    //SDL_render.h
    pub enum SDL_RendererFlags {
        SDL_RENDERER_SOFTWARE = 0x00000001,
        SDL_RENDERER_ACCELERATED = 0x00000002,
        SDL_RENDERER_PRESENTVSYNC = 0x00000004,
        SDL_RENDERER_TARGETTEXTURE = 0x00000008
    }

    pub struct SDL_RendererInfo
    {
        name: *c_char,
        flags: uint32_t,
        num_texture_formats: uint32_t,
        texture_formats: [uint32_t, ..16],
        max_texture_width: c_int,
        max_texture_height: c_int,
    }

    pub enum SDL_TextureAccess {
        SDL_TEXTUREACCESS_STATIC,
        SDL_TEXTUREACCESS_STREAMING,
        SDL_TEXTUREACCESS_TARGET
    }

    pub enum SDL_TextureModulate {
        SDL_TEXTUREMODULATE_NONE = 0x00000000,
        SDL_TEXTUREMODULATE_COLOR = 0x00000001,
        SDL_TEXTUREMODULATE_ALPHA = 0x00000002
    }

    pub enum SDL_RendererFlip {
        SDL_FLIP_NONE = 0x00000000,
        SDL_FLIP_HORIZONTAL = 0x00000001,
        SDL_FLIP_VERTICAL = 0x00000002
    }

    pub struct SDL_Renderer;
    pub struct SDL_Texture;

    //SDL_blendmode.h
    pub enum SDL_BlendMode {
        SDL_BLENDMODE_NONE = 0x00000000,
        SDL_BLENDMODE_BLEND = 0x00000001,
        SDL_BLENDMODE_ADD = 0x00000002,
        SDL_BLENDMODE_MOD = 0x00000004
    }

    externfn!(fn SDL_GetNumRenderDrivers() -> c_int)
    externfn!(fn SDL_GetRenderDriverInfo(index: c_int, info: *SDL_RendererInfo) -> c_int)
    externfn!(fn SDL_CreateWindowAndRenderer(width: c_int, height: c_int, window_flags: uint32_t, window: **SDL_Window, renderer: **SDL_Renderer) -> c_int)
    externfn!(fn SDL_CreateRenderer(window: *SDL_Window, index: c_int, flags: uint32_t) -> *SDL_Renderer)
    externfn!(fn SDL_CreateSoftwareRenderer(surface: *SDL_Surface) -> *SDL_Renderer)
    externfn!(fn SDL_GetRenderer(window: *SDL_Window) -> *SDL_Renderer)
    externfn!(fn SDL_GetRendererInfo(renderer: *SDL_Renderer, info: *SDL_RendererInfo) -> c_int)
    externfn!(fn SDL_GetRendererOutputSize(renderer: *SDL_Renderer, w: *c_int, h: *c_int) -> c_int)
    externfn!(fn SDL_CreateTexture(renderer: *SDL_Renderer, format: uint32_t, access: c_int, w: c_int, h: c_int) -> *SDL_Texture)
    externfn!(fn SDL_CreateTextureFromSurface(renderer: *SDL_Renderer, surface: *SDL_Surface) -> *SDL_Texture)
    externfn!(fn SDL_QueryTexture(texture: *SDL_Texture, format: *uint32_t, access: *c_int, w: *c_int, h: *c_int) -> c_int)
    externfn!(fn SDL_SetTextureColorMod(texture: *SDL_Texture, r: uint8_t, g: uint8_t, b: uint8_t) -> c_int)
    externfn!(fn SDL_GetTextureColorMod(texture: *SDL_Texture, r: *uint8_t, g: *uint8_t, b: *uint8_t) -> c_int)
    externfn!(fn SDL_SetTextureAlphaMod(texture: *SDL_Texture, alpha: uint8_t) -> c_int)
    externfn!(fn SDL_GetTextureAlphaMod(texture: *SDL_Texture, alpha: *uint8_t) -> c_int)
    externfn!(fn SDL_SetTextureBlendMode(texture: *SDL_Texture, blendMode: SDL_BlendMode) -> c_int)
    externfn!(fn SDL_GetTextureBlendMode(texture: *SDL_Texture, blendMode: *SDL_BlendMode) -> c_int)
    externfn!(fn SDL_UpdateTexture(texture: *SDL_Texture, rect: *SDL_Rect, pixels: *c_void, pitch: c_int) -> c_int)
    externfn!(fn SDL_LockTexture(texture: *SDL_Texture, rect: *SDL_Rect, pixels: **c_void, pitch: *c_int) -> c_int)
    externfn!(fn SDL_UnlockTexture(texture: *SDL_Texture))
    externfn!(fn SDL_RenderTargetSupported(renderer: *SDL_Renderer) -> SDL_bool)
    externfn!(fn SDL_SetRenderTarget(renderer: *SDL_Renderer, texture: *SDL_Texture) -> c_int)
    externfn!(fn SDL_GetRenderTarget(renderer: *SDL_Renderer) -> *SDL_Texture)
    externfn!(fn SDL_RenderSetLogicalSize(renderer: *SDL_Renderer, w: c_int, h: c_int) -> c_int)
    externfn!(fn SDL_RenderGetLogicalSize(renderer: *SDL_Renderer, w: *c_int, h: *c_int))
    externfn!(fn SDL_RenderSetViewport(renderer: *SDL_Renderer, rect: *SDL_Rect) -> c_int)
    externfn!(fn SDL_RenderGetViewport(renderer: *SDL_Renderer, rect: *SDL_Rect))
    externfn!(fn SDL_RenderSetClipRect(renderer: *SDL_Renderer, rect: *SDL_Rect) -> c_int)
    externfn!(fn SDL_RenderGetClipRect(renderer: *SDL_Renderer, rect: *SDL_Rect))
    externfn!(fn SDL_RenderSetScale(renderer: *SDL_Renderer, scaleX: c_float, scaleY: c_float) -> c_int)
    externfn!(fn SDL_RenderGetScale(renderer: *SDL_Renderer, scaleX: *c_float, scaleY: *c_float))
    externfn!(fn SDL_SetRenderDrawColor(renderer: *SDL_Renderer, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> c_int)
    externfn!(fn SDL_GetRenderDrawColor(renderer: *SDL_Renderer, r: *uint8_t, g: *uint8_t, b: *uint8_t, a: *uint8_t) -> c_int)
    externfn!(fn SDL_SetRenderDrawBlendMode(renderer: *SDL_Renderer, blendMode: SDL_BlendMode) -> c_int)
    externfn!(fn SDL_GetRenderDrawBlendMode(renderer: *SDL_Renderer, blendMode: *SDL_BlendMode) -> c_int)
    externfn!(fn SDL_RenderClear(renderer: *SDL_Renderer) -> c_int)
    externfn!(fn SDL_RenderDrawPoint(renderer: *SDL_Renderer, x: c_int, y: c_int) -> c_int)
    externfn!(fn SDL_RenderDrawPoints(renderer: *SDL_Renderer, Points: *SDL_Point, count: c_int) -> c_int)
    externfn!(fn SDL_RenderDrawLine(renderer: *SDL_Renderer, x1: c_int, y1: c_int, x2: c_int, y2: c_int) -> c_int)
    externfn!(fn SDL_RenderDrawLines(renderer: *SDL_Renderer, Points: *SDL_Point, count: c_int) -> c_int)
    externfn!(fn SDL_RenderDrawRect(renderer: *SDL_Renderer, rect: *SDL_Rect) -> c_int)
    externfn!(fn SDL_RenderDrawRects(renderer: *SDL_Renderer, rects: *SDL_Rect, count: c_int) -> c_int)
    externfn!(fn SDL_RenderFillRect(renderer: *SDL_Renderer, rect: *SDL_Rect) -> c_int)
    externfn!(fn SDL_RenderFillRects(renderer: *SDL_Renderer, rects: *SDL_Rect, count: c_int) -> c_int)
    externfn!(fn SDL_RenderCopy(renderer: *SDL_Renderer, texture: *SDL_Texture, srcrect: *SDL_Rect, dstrect: *SDL_Rect) -> c_int)
    externfn!(fn SDL_RenderCopyEx(renderer: *SDL_Renderer, texture: *SDL_Texture, srcrect: *SDL_Rect, dstrect: *SDL_Rect, angle: c_double, center: *SDL_Point, flip: SDL_RendererFlip) -> c_int)
    externfn!(fn SDL_RenderReadPixels(renderer: *SDL_Renderer, rect: *SDL_Rect, format: uint32_t, pixels: *c_void, pitch: c_int) -> c_int)
    externfn!(fn SDL_RenderPresent(renderer: *SDL_Renderer))
    externfn!(fn SDL_DestroyTexture(texture: *SDL_Texture))
    externfn!(fn SDL_DestroyRenderer(renderer: *SDL_Renderer))
    externfn!(fn SDL_GL_BindTexture(texture: *SDL_Texture, texw: *c_float, texh: *c_float) -> c_int)
    externfn!(fn SDL_GL_UnbindTexture(texture: *SDL_Texture) -> c_int)
}

pub enum RenderDriverIndex {
    DriverAuto,
    DriverIndex(int)
}

#[deriving(Eq)]
pub enum RendererFlags {
    Software = ll::SDL_RENDERER_SOFTWARE as int,
    Accelerated = ll::SDL_RENDERER_ACCELERATED as int,
    PresentVSync = ll::SDL_RENDERER_PRESENTVSYNC as int,
    TargetTexture = ll::SDL_RENDERER_TARGETTEXTURE as int
}

#[deriving(Eq)]
pub struct Renderer {
    raw: *ll::SDL_Renderer,
    parent: Either<~video::Window, ~surface::Surface>,
    owned: bool
}

impl Drop for Renderer {
    fn drop(&self) {
        if self.owned {
            unsafe {
                ll::SDL_DestroyRenderer(self.raw);
            }
        }
    }
}

impl Renderer {
    pub fn from_window(window: ~video::Window, index: RenderDriverIndex, renderer_flags: &[RendererFlags]) -> Result<~Renderer, ~str> {
        let flags = renderer_flags.iter().fold(0u32, |flags, flag| { flags | *flag as u32 });
        let index = match index {
            DriverAuto => -1,
            DriverIndex(x) => x
        };

        let raw = unsafe {
            ll::SDL_CreateRenderer(window.raw, index as c_int, flags)
        };

        if raw == ptr::null() {
            Err(get_error())
        } else {
            Ok(~Renderer{ raw: raw, parent: Left(window), owned: true,})
        }
    }

    pub fn set_draw_color(&self, color: pixels::Color) -> bool {
        match color {
            pixels::RGB(r, g, b) => {
                unsafe { ll::SDL_SetRenderDrawColor(self.raw, r, g, b, 255) == 0 }
            },
            pixels::RGBA(r, g, b, a) => {
                unsafe { ll::SDL_SetRenderDrawColor(self.raw, r, g, b, a) == 0 }
            }
        }
    }

    pub fn clear(&self) -> bool {
        unsafe { ll::SDL_RenderClear(self.raw) == 0 }
    }

    pub fn present(&self) {
        unsafe { ll::SDL_RenderPresent(self.raw) }
    }
}
