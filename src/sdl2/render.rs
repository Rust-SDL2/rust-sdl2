use video;
use video::Window;
use surface;
use surface::Surface;
use pixels;
use get_error;
use SdlResult;
use std::mem;
use std::ptr;
use std::raw;
use libc::{c_int, c_uint, uint32_t, c_float, c_double, c_void};
use rect::Point;
use rect::Rect;
use std::cell::{RefCell, RefMut};
use std::ffi::c_str_to_bytes;
use std::num::FromPrimitive;
use std::vec::Vec;
use std::borrow::ToOwned;
use std::marker::ContravariantLifetime;

use sys::render as ll;

#[derive(Copy, Clone)]
pub enum RenderDriverIndex {
    Auto,
    Index(i32)
}

#[derive(Copy, Clone, PartialEq, FromPrimitive)]
pub enum TextureAccess {
    Static = ll::SDL_TEXTUREACCESS_STATIC as isize,
    Streaming = ll::SDL_TEXTUREACCESS_STREAMING as isize,
    Target = ll::SDL_TEXTUREACCESS_TARGET as isize
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
    pub texture_formats: Vec<pixels::PixelFormatEnum>,
    pub max_texture_width: i32,
    pub max_texture_height: i32
}

#[derive(Copy, Clone, PartialEq, FromPrimitive)]
pub enum BlendMode {
    None = ll::SDL_BLENDMODE_NONE as isize,
    Blend = ll::SDL_BLENDMODE_BLEND as isize,
    Add = ll::SDL_BLENDMODE_ADD as isize,
    Mod = ll::SDL_BLENDMODE_MOD as isize
}

impl RendererInfo {
    pub fn from_ll(info: &ll::SDL_RendererInfo) -> RendererInfo {
        let actual_flags = RendererFlags::from_bits(info.flags).unwrap();

        unsafe {
            let texture_formats: Vec<pixels::PixelFormatEnum> = info.texture_formats[0..(info.num_texture_formats as usize)].iter().map(|&format| {
                FromPrimitive::from_i64(format as i64).unwrap()
            }).collect();

            RendererInfo {
                name: String::from_utf8_lossy(c_str_to_bytes(&info.name)).to_string(),
                flags: actual_flags,
                texture_formats: texture_formats,
                max_texture_width: info.max_texture_width as i32,
                max_texture_height: info.max_texture_height as i32
            }
        }
    }
}

pub enum RendererParent {
    Surface(Surface),
    Window(Window)
}

pub struct Renderer {
    raw: *const ll::SDL_Renderer,
    parent: Option<RendererParent>,
    drawer: RefCell<RenderDrawer>
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe { ll::SDL_DestroyRenderer(self.raw) };
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
            Ok(Renderer {
                raw: raw,
                parent: Some(RendererParent::Window(window)),
                drawer: RefCell::new(RenderDrawer::new(raw))
            })
        }
    }

    pub fn new_with_window(width: i32, height: i32, window_flags: video::WindowFlags) -> SdlResult<Renderer> {
        use sys::video::SDL_Window;

        let raw_window: *const SDL_Window = ptr::null();
        let raw_renderer: *const ll::SDL_Renderer = ptr::null();
        let result = unsafe { ll::SDL_CreateWindowAndRenderer(width as c_int, height as c_int, window_flags.bits(), &raw_window, &raw_renderer) == 0};
        if result {
            let window = unsafe { Window::from_ll(raw_window, true) };
            Ok(Renderer {
                raw: raw_renderer,
                parent: Some(RendererParent::Window(window)),
                drawer: RefCell::new(RenderDrawer::new(raw_renderer))
            })
        } else {
            Err(get_error())
        }
    }

    pub fn from_surface(surface: surface::Surface) -> SdlResult<Renderer> {
        let raw_renderer = unsafe { ll::SDL_CreateSoftwareRenderer(surface.raw()) };
        if raw_renderer == ptr::null() {
            Ok(Renderer {
                raw: raw_renderer,
                parent: Some(RendererParent::Surface(surface)),
                drawer: RefCell::new(RenderDrawer::new(raw_renderer))
            })
        } else {
            Err(get_error())
        }
    }

    pub fn get_info(&self) -> RendererInfo {
        unsafe {
            let renderer_info_raw: ll::SDL_RendererInfo = mem::uninitialized();
            if ll::SDL_GetRendererInfo(self.raw, &renderer_info_raw) != 0 {
                // Should only fail on an invalid renderer
                panic!();
            } else {
                RendererInfo::from_ll(&renderer_info_raw)
            }
        }
    }

    #[inline]
    pub fn get_parent(&self) -> &RendererParent { self.parent.as_ref().unwrap() }

    #[inline]
    pub unsafe fn raw(&self) -> *const ll::SDL_Renderer { self.raw }

    #[inline]
    pub fn unwrap_parent(mut self) -> RendererParent {
        use std::mem;
        mem::replace(&mut self.parent, None).unwrap()
    }

    pub fn create_texture(&self, format: pixels::PixelFormatEnum, access: TextureAccess, size: (i32, i32)) -> SdlResult<Texture> {
        let (width, height) = size;
        let result = unsafe { ll::SDL_CreateTexture(self.raw, format as uint32_t, access as c_int, width as c_int, height as c_int) };
        if result == ptr::null() {
            Err(get_error())
        } else {
            Ok(Texture { raw: result, owned: true, _marker: ContravariantLifetime } )
        }
    }

    /// Shorthand for `create_texture(format, TextureAccess::Static, size)`
    pub fn create_texture_static(&self, format: pixels::PixelFormatEnum, size: (i32, i32)) -> SdlResult<Texture> {
        self.create_texture(format, TextureAccess::Static, size)
    }

    /// Shorthand for `create_texture(format, TextureAccess::Streaming, size)`
    pub fn create_texture_streaming(&self, format: pixels::PixelFormatEnum, size: (i32, i32)) -> SdlResult<Texture> {
        self.create_texture(format, TextureAccess::Streaming, size)
    }

    /// Shorthand for `create_texture(format, TextureAccess::Target, size)`
    pub fn create_texture_target(&self, format: pixels::PixelFormatEnum, size: (i32, i32)) -> SdlResult<Texture> {
        self.create_texture(format, TextureAccess::Target, size)
    }

    pub fn create_texture_from_surface(&self, surface: &surface::Surface) -> SdlResult<Texture> {
        let result = unsafe { ll::SDL_CreateTextureFromSurface(self.raw, surface.raw()) };
        if result == ptr::null() {
            Err(get_error())
        } else {
            Ok(Texture { raw: result, owned: true, _marker: ContravariantLifetime } )
        }
    }

    pub fn drawer(&self) -> RefMut<RenderDrawer> {
        match self.drawer.try_borrow_mut() {
            Some(drawer) => drawer,
            None => panic!("Renderer drawer already borrowed")
        }
    }
}

pub struct RenderDrawer {
    raw: *const ll::SDL_Renderer,
    render_target: RenderTarget
}

impl RenderDrawer {
    fn new(raw: *const ll::SDL_Renderer) -> RenderDrawer {
        RenderDrawer {
            raw: raw,
            render_target: RenderTarget { raw: raw }
        }
    }

    pub fn render_target_supported(&self) -> bool {
        unsafe { ll::SDL_RenderTargetSupported(self.raw) == 1 }
    }

    pub fn render_target(&mut self) -> Option<&mut RenderTarget> {
        if self.render_target_supported() { Some(&mut self.render_target) }
        else { None }
    }

    pub fn set_draw_color(&mut self, color: pixels::Color) {
        let ret = match color {
            pixels::Color::RGB(r, g, b) => {
                unsafe { ll::SDL_SetRenderDrawColor(self.raw, r, g, b, 255) }
            },
            pixels::Color::RGBA(r, g, b, a) => {
                unsafe { ll::SDL_SetRenderDrawColor(self.raw, r, g, b, a)  }
            }
        };
        // Should only fail on an invalid renderer
        if ret != 0 { panic!(get_error()) }
    }

    pub fn get_draw_color(&self) -> pixels::Color {
        let r: u8 = 0;
        let g: u8 = 0;
        let b: u8 = 0;
        let a: u8 = 0;
        let ret = unsafe { ll::SDL_GetRenderDrawColor(self.raw, &r, &g, &b, &a) };
        // Should only fail on an invalid renderer
        if ret != 0 { panic!(get_error()) }
        else { pixels::Color::RGBA(r, g, b, a) }
    }

    pub fn set_blend_mode(&mut self, blend: BlendMode) {
        let ret = unsafe { ll::SDL_SetRenderDrawBlendMode(self.raw, FromPrimitive::from_i64(blend as i64).unwrap()) };
        // Should only fail on an invalid renderer
        if ret != 0 { panic!(get_error()) }
    }

    pub fn get_blend_mode(&self) -> BlendMode {
        let blend = 0;
        let ret = unsafe { ll::SDL_GetRenderDrawBlendMode(self.raw, &blend) };
        // Should only fail on an invalid renderer
        if ret != 0 { panic!(get_error()) }
        else { FromPrimitive::from_i64(blend as i64).unwrap() }
    }

    pub fn clear(&mut self) {
        let ret = unsafe { ll::SDL_RenderClear(self.raw) };
        if ret != 0 { panic!("Could not clear: {}", get_error()) }
    }

    pub fn present(&mut self) {
        unsafe { ll::SDL_RenderPresent(self.raw) }
    }

    pub fn get_output_size(&self) -> SdlResult<(i32, i32)> {
        let width: c_int = 0;
        let height: c_int = 0;

        let result = unsafe { ll::SDL_GetRendererOutputSize(self.raw, &width, &height) == 0 };

        if result {
            Ok((width as i32, height as i32))
        } else {
            Err(get_error())
        }
    }

    pub fn set_logical_size(&mut self, width: i32, height: i32) {
        let ret = unsafe { ll::SDL_RenderSetLogicalSize(self.raw, width as c_int, height as c_int) };
        if ret != 0 { panic!("Could not set logical size: {}", get_error()) }
    }

    pub fn get_logical_size(&self) -> (i32, i32) {

        let width: c_int = 0;
        let height: c_int = 0;

        unsafe { ll::SDL_RenderGetLogicalSize(self.raw, &width, &height) };

        (width as i32, height as i32)
    }

    pub fn set_viewport(&mut self, rect: Option<Rect>) {
        let ptr = match rect {
            Some(ref rect) => rect as *const _,
            None => ptr::null()
        };
        let ret = unsafe { ll::SDL_RenderSetViewport(self.raw, ptr) };
        if ret != 0 { panic!("Could not set viewport: {}", get_error()) }
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

    pub fn set_clip_rect(&mut self, rect: Option<Rect>) {
        let ret = unsafe {
            ll::SDL_RenderSetClipRect(
                self.raw,
                match rect {
                    Some(ref rect) => rect as *const _,
                    None => ptr::null()
                }
            )
        };
        if ret != 0 { panic!("Could not set clip rect: {}", get_error()) }
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

    pub fn set_scale(&mut self, scale_x: f64, scale_y: f64) {
        let ret = unsafe { ll::SDL_RenderSetScale(self.raw, scale_x as c_float, scale_y as c_float) };
        // Should only fail on an invalid renderer
        if ret != 0 { panic!(get_error()) }
    }

    pub fn get_scale(&self) -> (f64, f64) {
        let scale_x: c_float = 0.0;
        let scale_y: c_float = 0.0;
        unsafe { ll::SDL_RenderGetScale(self.raw, &scale_x, &scale_y) };
        (scale_x as f64, scale_y as f64)
    }

    pub fn draw_point(&mut self, point: Point) {
        unsafe {
            if ll::SDL_RenderDrawPoint(self.raw, point.x, point.y) != 0 {
                panic!("Error drawing point: {}", get_error())
            }
        }
    }

    pub fn draw_points(&mut self, points: &[Point]) {
        unsafe {
            if ll::SDL_RenderDrawPoints(self.raw, points.as_ptr(), points.len() as c_int) != 0 {
                panic!("Error drawing points: {}", get_error())
            }
        }
    }

    pub fn draw_line(&mut self, start: Point, end: Point) {
        unsafe {
            if ll::SDL_RenderDrawLine(self.raw, start.x, start.y, end.x, end.y) != 0 {
                panic!("Error drawing line: {}", get_error())
            }
        }
    }

    pub fn draw_lines(&mut self, points: &[Point]) {
        unsafe {
            if ll::SDL_RenderDrawLines(self.raw, points.as_ptr(), points.len() as c_int) != 0 {
                panic!("Error drawing lines: {}", get_error())
            }
        }
    }

    pub fn draw_rect(&mut self, rect: &Rect) {
        unsafe {
            if ll::SDL_RenderDrawRect(self.raw, rect) != 0 {
                panic!("Error drawing rect: {}", get_error())
            }
        }
    }

    pub fn draw_rects(&mut self, rects: &[Rect]) {
        unsafe {
            if ll::SDL_RenderDrawRects(self.raw, rects.as_ptr(), rects.len() as c_int) != 0 {
                panic!("Error drawing rects: {}", get_error())
            }
        }
    }

    pub fn fill_rect(&mut self, rect: &Rect) {
        unsafe {
            if ll::SDL_RenderFillRect(self.raw, rect) != 0 {
                panic!("Error filling rect: {}", get_error())
            }
        }
    }

    pub fn fill_rects(&mut self, rects: &[Rect]) {
        unsafe {
            if ll::SDL_RenderFillRects(self.raw, rects.as_ptr(), rects.len() as c_int) != 0 {
                panic!("Error filling rects: {}", get_error())
            }
        }
    }

    pub fn copy(&mut self, texture: &mut Texture, src: Option<Rect>, dst: Option<Rect>) {
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

        if ret != 0 {
            panic!("Error copying texture: {}", get_error())
        }
    }

    pub fn copy_ex(&mut self, texture: &mut Texture, src: Option<Rect>, dst: Option<Rect>, angle: f64, center: Option<Point>, (flip_horizontal, flip_vertical): (bool, bool)) {
        let flip = match (flip_horizontal, flip_vertical) {
            (false, false) => ll::SDL_FLIP_NONE,
            (true, false) => ll::SDL_FLIP_HORIZONTAL,
            (false, true) => ll::SDL_FLIP_VERTICAL,
            (true, true) => ll::SDL_FLIP_HORIZONTAL | ll::SDL_FLIP_VERTICAL,
        };

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
                flip
            )
        };

        if ret != 0 {
            panic!("Error copying texture (ex): {}", get_error())
        }
    }

    pub fn read_pixels(&self, rect: Option<Rect>, format: pixels::PixelFormatEnum) -> SdlResult<Vec<u8>> {
        unsafe {
            let (actual_rect, w, h) = match rect {
                Some(ref rect) => (rect as *const _, rect.w as usize, rect.h as usize),
                None => {
                    let (w, h) = try!(self.get_output_size());
                    (ptr::null(), w as usize, h as usize)
                }
            };

            let pitch = w * format.byte_size_per_pixel(); // calculated pitch
            let size = format.byte_size_of_pixels(w * h);
            let mut pixels = Vec::with_capacity(size);
            pixels.set_len(size);

            // Pass the interior of `pixels: Vec<u8>` to SDL
            let ret = {
                let pixels_ref: raw::Slice<u8> = mem::transmute(pixels.as_slice());
                ll::SDL_RenderReadPixels(self.raw, actual_rect, format as uint32_t, pixels_ref.data as *mut c_void, pitch as c_int)
            };

            if ret == 0 {
                Ok(pixels)
            } else {
                Err(get_error())
            }
        }
    }
}

pub struct RenderTarget {
    raw: *const ll::SDL_Renderer
}

impl RenderTarget {
    /// Resets the render target to the default render target.
    pub fn reset(&mut self) -> SdlResult<()> {
        unsafe {
            if ll::SDL_SetRenderTarget(self.raw, ptr::null()) == 0 {
                Ok(())
            } else {
                Err(get_error())
            }
        }
    }

    /// Sets the render target to the provided texture.
    /// The texture must be created with the texture access: `sdl2::render::TextureAccess::Target`.
    pub fn set(&mut self, texture: Texture) -> SdlResult<()> {
        unsafe {
            if ll::SDL_SetRenderTarget(self.raw, texture.raw) == 0 {
                Ok(())
            } else {
                Err(get_error())
            }
        }
    }

    /// Creates a new texture and sets it as the render target.
    pub fn create_and_set(&mut self, format: pixels::PixelFormatEnum, width: i32, height: i32) -> SdlResult<Texture> {
        let new_texture_raw = unsafe {
            let access = ll::SDL_TEXTUREACCESS_TARGET;
            ll::SDL_CreateTexture(self.raw, format as uint32_t, access as c_int, width as c_int, height as c_int)
        };

        if new_texture_raw == ptr::null() {
            Err(get_error())
        } else {
            unsafe {
                if ll::SDL_SetRenderTarget(self.raw, new_texture_raw) == 0 {
                    Ok(Texture {
                        raw: new_texture_raw,
                        owned: false,
                        _marker: ContravariantLifetime
                    })
                } else {
                    Err(get_error())
                }
            }
        }
    }

    /// Gets the current render target.
    /// Returns None if the default render target is set.
    pub fn get(&mut self) -> Option<Texture> {
        let texture_raw = unsafe {  ll::SDL_GetRenderTarget(self.raw) };

        if texture_raw == ptr::null() {
            None
        } else {
            Some(Texture {
                raw: texture_raw,
                owned: false,
                _marker: ContravariantLifetime
            })
        }
    }
}

#[derive(Copy, Clone)]
pub struct TextureQuery {
    pub format: pixels::PixelFormatEnum,
    pub access: TextureAccess,
    pub width: i32,
    pub height: i32
}

#[derive(PartialEq)] #[allow(raw_pointer_derive)]
pub struct Texture<'renderer> {
    raw: *const ll::SDL_Texture,
    owned: bool,
    /// Textures cannot live longer than the Renderer it was born from: 'a
    /// All SDL textures contain an internal reference to a Renderer
    _marker: ContravariantLifetime<'renderer>
}

#[unsafe_destructor]
impl<'renderer> Drop for Texture<'renderer> {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ll::SDL_DestroyTexture(self.raw);
            }
        }
    }
}

impl<'renderer> Texture<'renderer> {

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
               width: width as i32,
               height: height as i32
            })
        } else {
            Err(get_error())
        }
    }

    pub fn set_color_mod(&mut self, red: u8, green: u8, blue: u8) -> SdlResult<()> {
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

    pub fn set_alpha_mod(&mut self, alpha: u8) -> SdlResult<()> {
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

    pub fn set_blend_mode(&mut self, blend: BlendMode) -> SdlResult<()> {
        let ret = unsafe { ll::SDL_SetTextureBlendMode(self.raw, FromPrimitive::from_i64(blend as i64).unwrap()) };

        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn get_blend_mode(&self) -> SdlResult<BlendMode> {
        let blend = 0;
        let result = unsafe { ll::SDL_GetTextureBlendMode(self.raw, &blend) == 0 };
        if result {
            Ok(FromPrimitive::from_i64(blend as i64).unwrap())
        } else {
            Err(get_error())
        }
    }

    pub fn update(&mut self, rect: Option<Rect>, pixel_data: &[u8], pitch: i32) -> SdlResult<()> {
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

    /// Locks the texture for **write-only** pixel access.
    /// The texture must have been created with streaming access.
    ///
    /// `F` is a function that is passed the write-only texture buffer,
    /// and the pitch of the texture (size of a row in bytes).
    /// # Remarks
    /// As an optimization, the pixels made available for editing don't
    /// necessarily contain the old texture data.
    /// This is a write-only operation, and if you need to keep a copy of the
    /// texture data you should do that at the application level.
    pub fn with_lock<F, R>(&mut self, rect: Option<Rect>, func: F) -> SdlResult<R>
    where F: FnOnce(&mut [u8], usize) -> R
    {
        // Call to SDL to populate pixel data
        let loaded = unsafe {
            let q = try!(self.query());
            let pixels : *const c_void = ptr::null();
            let pitch = 0;
            let size = q.format.byte_size_of_pixels((q.width * q.height) as usize);

            let actual_rect = match rect {
                Some(ref rect) => rect as *const _,
                None => ptr::null()
            };

            let ret = ll::SDL_LockTexture(self.raw, actual_rect, &pixels, &pitch);
            if ret == 0 {
                Ok( (raw::Slice { data: pixels as *const u8, len: size }, pitch) )
            } else {
                Err(get_error())
            }
        };

        match loaded {
            Ok((interior, pitch)) => {
                let result;
                unsafe {
                    result = func(mem::transmute(interior), pitch as usize);
                    ll::SDL_UnlockTexture(self.raw);
                }
                Ok(result)
            }
            Err(e) => Err(e),
        }
    }

    pub unsafe fn gl_bind_texture(&mut self) -> (f32, f32) {
        unsafe {
            let texw = 0.0;
            let texh = 0.0;

            if ll::SDL_GL_BindTexture(self.raw, &texw, &texh) == 0 {
                (texw, texh)
            } else {
                panic!("OpenGL texture binding not supported");
            }
        }
    }

    pub unsafe fn gl_unbind_texture(&mut self) {
        unsafe {
            if ll::SDL_GL_UnbindTexture(self.raw) != 0 {
                panic!("OpenGL texture unbinding not supported");
            }
        }
    }

    pub fn gl_with_bind<R, F: FnOnce(f32, f32) -> R>(&mut self, f: F) -> R {
        unsafe {
            let texw = 0.0;
            let texh = 0.0;

            if ll::SDL_GL_BindTexture(self.raw, &texw, &texh) == 0 {
                let return_value = f(texw, texh);

                if ll::SDL_GL_UnbindTexture(self.raw) == 0 {
                    return_value
                } else {
                    // This should never happen...
                    panic!();
                }
            } else {
                panic!("OpenGL texture binding not supported");
            }
        }
    }
}


pub fn get_num_render_drivers() -> SdlResult<i32> {
    let result = unsafe { ll::SDL_GetNumRenderDrivers() };
    if result > 0 {
        Ok(result as i32)
    } else {
        Err(get_error())
    }
}

pub fn get_render_driver_info(index: i32) -> SdlResult<RendererInfo> {
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
