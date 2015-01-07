use video;
use video::Window;
use surface;
use surface::Surface;
use pixels;
use get_error;
use SdlResult;
use std::ptr;
use libc;
use libc::{c_int, uint32_t, c_float, c_double, c_void, size_t};
use ::c_vec::CVec;
use rect::Point;
use rect::Rect;
use std::ffi::c_str_to_bytes;
use std::num::FromPrimitive;
use std::vec::Vec;
use std::borrow::ToOwned;

pub use sys::render as ll;

#[derive(Copy, Clone)]
pub enum RenderDriverIndex {
    Auto,
    Index(int)
}

#[derive(Copy, Clone, PartialEq, FromPrimitive)]
pub enum TextureAccess {
    Static = ll::SDL_TEXTUREACCESS_STATIC as int,
    Streaming = ll::SDL_TEXTUREACCESS_STREAMING as int,
    Target = ll::SDL_TEXTUREACCESS_TARGET as int
}

bitflags! {
    flags RendererFlags: u32 {
        const SOFTWARE = ll::SDL_RENDERER_SOFTWARE as u32,
        const ACCELERATED = ll::SDL_RENDERER_ACCELERATED as u32,
        const PRESENTVSYNC = ll::SDL_RENDERER_PRESENTVSYNC as u32,
        const TARGETTEXTURE = ll::SDL_RENDERER_TARGETTEXTURE as u32
    }
}

#[derive(PartialEq)]
pub struct RendererInfo {
    pub name: String,
    pub flags: RendererFlags,
    pub texture_formats: Vec<pixels::PixelFormatFlag>,
    pub max_texture_width: int,
    pub max_texture_height: int
}

#[derive(Copy, Clone, PartialEq, FromPrimitive)]
pub enum BlendMode {
    None = ll::SDL_BLENDMODE_NONE as int,
    Blend = ll::SDL_BLENDMODE_BLEND as int,
    Add = ll::SDL_BLENDMODE_ADD as int,
    Mod = ll::SDL_BLENDMODE_MOD as int
}

#[derive(Copy, Clone, PartialEq)]
pub enum RendererFlip {
    None = ll::SDL_FLIP_NONE as int,
    Horizontal = ll::SDL_FLIP_HORIZONTAL as int,
    Vertical = ll::SDL_FLIP_VERTICAL as int,
}

impl RendererInfo {
    pub fn from_ll(info: &ll::SDL_RendererInfo) -> RendererInfo {
        let actual_flags = RendererFlags::from_bits(info.flags).unwrap();

        unsafe {
            let texture_formats: Vec<pixels::PixelFormatFlag> = info.texture_formats[0..(info.num_texture_formats as uint)].iter().map(|&format| {
                FromPrimitive::from_i64(format as i64).unwrap()
            }).collect();

            RendererInfo {
                name: String::from_utf8_lossy(c_str_to_bytes(&info.name)).to_string(),
                flags: actual_flags,
                texture_formats: texture_formats,
                max_texture_width: info.max_texture_width as int,
                max_texture_height: info.max_texture_height as int
            }
        }
    }
}

pub enum RendererParent {
    Surface(Surface),
    Window(Window)
}

#[allow(raw_pointer_derive)]
pub struct Renderer {
    raw: *const ll::SDL_Renderer,
    parent: Option<RendererParent>,
    owned: bool
}

#[unsafe_destructor]
impl Drop for Renderer {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ll::SDL_DestroyRenderer(self.raw);
            }
        }
    }
}

impl Renderer {
    pub fn from_window(window: Window, index: RenderDriverIndex, renderer_flags: RendererFlags) -> SdlResult<Renderer> {
        let index = match index {
            RenderDriverIndex::Auto => -1,
            RenderDriverIndex::Index(x) => x
        };

        let raw = unsafe {
            ll::SDL_CreateRenderer(window.raw(), index as c_int, renderer_flags.bits())
        };

        if raw == ptr::null() {
            Err(get_error())
        } else {
            Ok(Renderer{ raw: raw, parent: Some(RendererParent::Window(window)), owned: true,})
        }
    }

    pub fn new_with_window(width: int, height: int, window_flags: video::WindowFlags) -> SdlResult<Renderer> {
        let raw_window: *const video::ll::SDL_Window = ptr::null();
        let raw_renderer: *const ll::SDL_Renderer = ptr::null();
        let result = unsafe { ll::SDL_CreateWindowAndRenderer(width as c_int, height as c_int, window_flags.bits(), &raw_window, &raw_renderer) == 0};
        if result {
            let window = unsafe { Window::from_ll(raw_window, true) };
            Ok(Renderer {
                raw: raw_renderer,
                parent: Some(RendererParent::Window(window)),
                owned: true
            })
        } else {
            Err(get_error())
        }
    }

    pub fn from_surface(surface: surface::Surface) -> SdlResult<Renderer> {
        let result = unsafe { ll::SDL_CreateSoftwareRenderer(surface.raw()) };
        if result == ptr::null() {
            Ok(Renderer {
                raw: result,
                parent: Some(RendererParent::Surface(surface)),
                owned: true
            })
        } else {
            Err(get_error())
        }
    }

    #[inline]
    pub fn get_parent<'a>(&'a self) -> &'a RendererParent { self.parent.as_ref().unwrap() }

    #[inline]
    pub fn unwrap_parent(mut self) -> RendererParent {
        use std::mem;
        mem::replace(&mut self.parent, None).unwrap()
    }

    #[inline]
    pub fn raw(&self) -> *const ll::SDL_Renderer { self.raw }

    #[inline]
    pub fn owned(&self) -> bool { self.owned }

    pub fn set_draw_color(&self, color: pixels::Color) -> SdlResult<()> {
        let ret = match color {
            pixels::Color::RGB(r, g, b) => {
                unsafe { ll::SDL_SetRenderDrawColor(self.raw, r, g, b, 255) }
            },
            pixels::Color::RGBA(r, g, b, a) => {
                unsafe { ll::SDL_SetRenderDrawColor(self.raw, r, g, b, a)  }
            }
        };
        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn get_draw_color(&self) -> SdlResult<pixels::Color> {
        let r: u8 = 0;
        let g: u8 = 0;
        let b: u8 = 0;
        let a: u8 = 0;
        let result = unsafe { ll::SDL_GetRenderDrawColor(self.raw, &r, &g, &b, &a) == 0 };
        if result {
            Ok(pixels::Color::RGBA(r, g, b, a))
        } else {
            Err(get_error())
        }
    }

    pub fn clear(&self) -> SdlResult<()> {
        let ret = unsafe { ll::SDL_RenderClear(self.raw) };
        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn present(&self) {
        unsafe { ll::SDL_RenderPresent(self.raw) }
    }

    pub fn get_output_size(&self) -> SdlResult<(int, int)> {
        let width: c_int = 0;
        let height: c_int = 0;

        let result = unsafe { ll::SDL_GetRendererOutputSize(self.raw, &width, &height) == 0 };

        if result {
            Ok((width as int, height as int))
        } else {
            Err(get_error())
        }
    }

    pub fn create_texture(&self, format: pixels::PixelFormatFlag, access: TextureAccess, width: int, height: int) -> SdlResult<Texture> {
        let result = unsafe { ll::SDL_CreateTexture(self.raw, format as uint32_t, access as c_int, width as c_int, height as c_int) };
        if result == ptr::null() {
            Err(get_error())
        } else {
            Ok(Texture { raw: result, owned: true } )
        }
    }

    pub fn create_texture_from_surface(&self, surface: &surface::Surface) -> SdlResult<Texture> {
        let result = unsafe { ll::SDL_CreateTextureFromSurface(self.raw, surface.raw()) };
        if result == ptr::null() {
            Err(get_error())
        } else {
            Ok(Texture { raw: result, owned: true } )
        }
    }

    pub fn render_target_supported(&self) -> bool {
        unsafe { ll::SDL_RenderTargetSupported(self.raw) == 1 }
    }

    pub fn set_render_target(&self, texture: Option<&Texture>) -> SdlResult<()> {
        unsafe {
            let actual_texture = match texture {
                Some(texture) => texture.raw,
                None => ptr::null()
            };
            if ll::SDL_SetRenderTarget(self.raw, actual_texture) == 0 {
                Ok(())
            } else {
                Err(get_error())
            }
        }
    }

    pub fn get_render_target(&self) -> Option<Texture> {
        let raw = unsafe { ll::SDL_GetRenderTarget(self.raw) };

        if raw == ptr::null() {
            None
        } else {
            Some(Texture{
                raw: raw,
                owned: false
            })
        }
    }

    pub fn set_logical_size(&self, width: int, height: int) -> SdlResult<()> {
        let ret = unsafe { ll::SDL_RenderSetLogicalSize(self.raw, width as c_int, height as c_int) };

        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn get_logical_size(&self) -> (int, int) {

        let width: c_int = 0;
        let height: c_int = 0;

        unsafe { ll::SDL_RenderGetLogicalSize(self.raw, &width, &height) };

        (width as int, height as int)
    }

    pub fn set_viewport(&self, rect: Option<Rect>) -> SdlResult<()> {
        let ptr = match rect {
            Some(ref rect) => rect as *const _,
            None => ptr::null()
        };
        let ret = unsafe { ll::SDL_RenderSetViewport(self.raw, ptr) };

        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn get_viewport(&self) -> Rect {
        let rect = Rect{
            x: 0,
            y: 0,
            w: 0,
            h: 0
        };
        unsafe { ll::SDL_RenderGetViewport(self.raw, &rect) };
        rect
    }

    pub fn set_clip_rect(&self, rect: Option<Rect>) -> SdlResult<()> {
        let ret = unsafe { 
            ll::SDL_RenderSetClipRect(
                self.raw, 
                match rect {
                    Some(ref rect) => rect as *const _,
                    None => ptr::null()
                }
            ) 
        };

        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn get_clip_rect(&self) -> Rect {
        let rect = Rect{
            x: 0,
            y: 0,
            w: 0,
            h: 0
        };
        unsafe { ll::SDL_RenderGetClipRect(self.raw, &rect) };
        rect
    }

    pub fn set_scale(&self, scale_x: f64, scale_y: f64) -> SdlResult<()> {
        let ret = unsafe { ll::SDL_RenderSetScale(self.raw, scale_x as c_float, scale_y as c_float) };

        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn get_scale(&self) -> (f64, f64) {
        let scale_x: c_float = 0.0;
        let scale_y: c_float = 0.0;
        unsafe { ll::SDL_RenderGetScale(self.raw, &scale_x, &scale_y) };
        (scale_x as f64, scale_y as f64)
    }

    pub fn draw_point(&self, point: Point) -> SdlResult<()> {
        let ret = unsafe { ll::SDL_RenderDrawPoint(self.raw, point.x, point.y) };

        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn draw_points(&self, points: &[Point]) -> SdlResult<()> {
        let ret = unsafe {
            ll::SDL_RenderDrawPoints(self.raw, points.as_ptr(), points.len() as c_int)
        };

        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn draw_line(&self, start: Point, end: Point) -> SdlResult<()> {
        let ret = unsafe { ll::SDL_RenderDrawLine(self.raw, start.x, start.y, end.x, end.y) };

        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn draw_lines(&self, points: &[Point]) -> SdlResult<()> {
        let ret = unsafe {
            ll::SDL_RenderDrawLines(self.raw, points.as_ptr(), points.len() as c_int)
        };

        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn draw_rect(&self, rect: &Rect) -> SdlResult<()> {
        let ret = unsafe { ll::SDL_RenderDrawRect(self.raw, rect) };

        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn draw_rects(&self, rects: &[Rect]) -> SdlResult<()> {
        let ret = unsafe {
            ll::SDL_RenderDrawRects(self.raw, rects.as_ptr(), rects.len() as c_int)
        };

        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn fill_rect(&self, rect: &Rect) -> SdlResult<()> {
        let ret = unsafe { ll::SDL_RenderFillRect(self.raw, rect) };

        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn fill_rects(&self, rects: &[Rect]) -> SdlResult<()> {
        let ret = unsafe {
            ll::SDL_RenderFillRects(self.raw, rects.as_ptr(), rects.len() as c_int)
        };

        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn copy(&self, texture: &Texture, src: Option<Rect>, dst: Option<Rect>) -> SdlResult<()> {
        let ret = unsafe {
            ll::SDL_RenderCopy(
                self.raw,
                texture.raw,
                match src {
                    Some(ref rect) => rect as *const _,
                    None => ptr::null()
                },
                match dst {
                    Some(ref rect) => rect as *const _,
                    None => ptr::null()
                }
            )
        };

        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    //TODO: Check whether RendererFlip is supposed to be combinable
    pub fn copy_ex(&self, texture: &Texture, src: Option<Rect>, dst: Option<Rect>, angle: f64, center: Option<Point>, flip: RendererFlip) -> SdlResult<()> {
        let ret = unsafe {
            ll::SDL_RenderCopyEx(
                self.raw,
                texture.raw,
                match src {
                    Some(ref rect) => rect as *const _,
                    None => ptr::null()
                },
                match dst {
                    Some(ref rect) => rect as *const _,
                    None => ptr::null()
                },
                angle as c_double,
                match center {
                    Some(ref point) => point as *const _,
                    None => ptr::null()
                },
                FromPrimitive::from_i64(flip as i64).unwrap()
            )
        };

        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn read_pixels(&self, rect: Option<Rect>, format: pixels::PixelFormatFlag) -> SdlResult<CVec<u8>> {
        unsafe {
            let (actual_rect, w, h) = match rect {
                Some(ref rect) => (rect as *const _, rect.w as uint, rect.h as uint),
                None => {
                    let (w, h) = try!(self.get_output_size());
                    (ptr::null(), w as uint, h as uint)
                }
            };
            let size = format.byte_size_of_pixels(w * h);
            let pixels = libc::malloc(size as size_t) as *mut u8;
            let pitch = w * format.byte_size_per_pixel(); // calculated pitch
            let ret = ll::SDL_RenderReadPixels(self.raw, actual_rect, format as uint32_t, pixels as *mut c_void, pitch as c_int);
            if ret == 0 {
                let pixels = ptr::Unique(pixels);
                Ok(CVec::new_with_dtor(pixels.0 as *mut u8, size, move || {
                    libc::free(pixels.0 as *mut c_void)
                }))
            } else {
                Err(get_error())
            }
        }
    }
}


#[derive(Copy, Clone)]
pub struct TextureQuery {
    pub format: pixels::PixelFormatFlag,
    pub access: TextureAccess,
    pub width: int,
    pub height: int
}

#[derive(PartialEq)] #[allow(raw_pointer_derive)]
pub struct Texture {
    pub raw: *const ll::SDL_Texture,
    pub owned: bool
}

impl Drop for Texture {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ll::SDL_DestroyTexture(self.raw);
            }
        }
    }
}

impl Texture {

    pub fn query(&self) -> SdlResult<TextureQuery> {
        let format: uint32_t = 0;
        let access: c_int = 0;
        let width: c_int = 0;
        let height: c_int = 0;

        let result = unsafe { ll::SDL_QueryTexture(self.raw, &format, &access, &width, &height) == 0 };
        if result {
            Ok(TextureQuery {
               format: FromPrimitive::from_i64(format as i64).unwrap(),
               access: FromPrimitive::from_i64(access as i64).unwrap(),
               width: width as int,
               height: height as int
            })
        } else {
            Err(get_error())
        }
    }

    pub fn set_color_mod(&self, red: u8, green: u8, blue: u8) -> SdlResult<()> {
        let ret = unsafe { ll::SDL_SetTextureColorMod(self.raw, red, green, blue) };

        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn get_color_mod(&self) -> SdlResult<(u8, u8, u8)> {
        let r = 0;
        let g = 0;
        let b = 0;
        let result = unsafe { ll::SDL_GetTextureColorMod(self.raw, &r, &g, &b) == 0 };

        if result {
            Ok((r, g, b))
        } else {
            Err(get_error())
        }
    }

    pub fn set_alpha_mod(&self, alpha: u8) -> SdlResult<()> {
        let ret = unsafe { ll::SDL_SetTextureAlphaMod(self.raw, alpha) };

        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn get_alpha_mod(&self) -> SdlResult<u8> {
        let alpha = 0;
        let result = unsafe { ll::SDL_GetTextureAlphaMod(self.raw, &alpha) == 0 };

        if result {
            Ok(alpha)
        } else {
            Err(get_error())
        }
    }

    pub fn set_blend_mode(&self, blend: BlendMode) -> SdlResult<()> {
        let ret = unsafe { ll::SDL_SetTextureBlendMode(self.raw, FromPrimitive::from_i64(blend as i64).unwrap()) };

        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn get_blend_mode(&self) -> SdlResult<BlendMode> {
        let blend: i64 = 0;
        let result = unsafe { ll::SDL_GetTextureBlendMode(self.raw, &FromPrimitive::from_i64(blend as i64).unwrap()) == 0 };
        if result {
            Ok(FromPrimitive::from_i64(blend as i64).unwrap())
        } else {
            Err(get_error())
        }
    }

    pub fn update(&self, rect: Option<Rect>, pixel_data: &[u8], pitch: int) -> SdlResult<()> {
        let ret = unsafe {
            let actual_rect = match rect {
                Some(ref rect) => rect as *const _,
                None => ptr::null()
            };

            ll::SDL_UpdateTexture(self.raw, actual_rect, pixel_data.as_ptr() as *const _, pitch as c_int)
        };

        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    fn unsafe_lock(&self, rect: Option<Rect>) -> SdlResult<(CVec<u8>, i32)> {
        let q = try!(self.query());
        unsafe {
            let actual_rect = match rect {
                Some(ref rect) => rect as *const _,
                None => ptr::null()
            };
            let pixels : *const c_void = ptr::null();
            let pitch = 0i32;
            let ret = ll::SDL_LockTexture(self.raw, actual_rect, &pixels, &pitch);
            let size = q.format.byte_size_of_pixels((q.width * q.height) as uint);
            if ret == 0 {
                Ok((CVec::new(pixels as *mut u8, size), pitch))
            } else {
                Err(get_error())
            }
        }
    }

    pub fn with_lock<F: FnOnce(CVec<u8>, i32) -> ()>(&self, rect: Option<Rect>, func: F) -> SdlResult<()> {
        match self.unsafe_lock(rect) {
            Ok((cvec, pitch)) => {
                func(cvec, pitch);
                self.unlock();
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn unlock(&self) {
        unsafe { ll::SDL_UnlockTexture(self.raw) }
    }

    pub fn gl_bind_texture(&self) -> SdlResult<(f64, f64)> {
        let texw: c_float = 0.0;
        let texh: c_float = 0.0;

        let result = unsafe {
            ll::SDL_GL_BindTexture(self.raw, &texw, &texh) == 0
        };

        if result {
            Ok((texw as f64, texh as f64))
        } else {
            Err("Operation not supported".to_owned())
        }
    }

    pub fn gl_unbind_texture(&self) -> bool {
        unsafe { ll::SDL_GL_UnbindTexture(self.raw) == 0 }
    }

    pub fn gl_with_bind<R, F: FnOnce(f64, f64) -> R>(&self, f: F) -> R {
        unsafe {
            let texw: c_float = 0.0;
            let texh: c_float = 0.0;
            if ll::SDL_GL_BindTexture(self.raw, &texw, &texh) != 0 { panic!("could not bind texture"); }
            let rv = f(texw as f64, texh as f64);
            ll::SDL_GL_UnbindTexture(self.raw);
            rv
        }
    }
}


pub fn get_num_render_drivers() -> SdlResult<int> {
    let result = unsafe { ll::SDL_GetNumRenderDrivers() };
    if result > 0 {
        Ok(result as int)
    } else {
        Err(get_error())
    }
}

pub fn get_render_driver_info(index: int) -> SdlResult<RendererInfo> {
    let out = ll::SDL_RendererInfo {
        name: ptr::null(),
        flags: 0,
        num_texture_formats: 0,
        texture_formats: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        max_texture_width: 0,
        max_texture_height: 0,
    };
    let result = unsafe { ll::SDL_GetRenderDriverInfo(index as c_int, &out) == 0 };
    if result {
        Ok(RendererInfo::from_ll(&out))
    } else {
        Err(get_error())
    }
}

/*
    //TODO: Figure out how to support this with our current struct format
    pub fn SDL_GetRenderer(window: *SDL_Window) -> *SDL_Renderer;
*/
