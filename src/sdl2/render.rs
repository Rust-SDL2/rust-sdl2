//! 2D accelerated rendering
//!
//! Official C documentation: https://wiki.libsdl.org/CategoryRender
//! # Introduction
//!
//! This module contains functions for 2D accelerated rendering.
//!
//! This API supports the following features:
//!
//! * single pixel points
//! * single pixel lines
//! * filled rectangles
//! * texture images
//! * All of these may be drawn in opaque, blended, or additive modes.
//!
//! The texture images can have an additional color tint or alpha modulation
//! applied to them, and may also be stretched with linear interpolation,
//! rotated or flipped/mirrored.
//!
//! For advanced functionality like particle effects or actual 3D you should use
//! SDL's OpenGL/Direct3D support or one of the many available 3D engines.
//!
//! This API is not designed to be used from multiple threads, see
//! [this bug](http://bugzilla.libsdl.org/show_bug.cgi?id=1995) for details.
//!
//! # Rust differences
//!
//! The Rust version of the render API deviates slightly from the original,
//! in order to be more idiomatic with Rust and to adhere to its notion of
//! memory safety.
//!
//! All `Texture` types are restricted to live for only as long as
//! the parent `Renderer`.
//! Consequentially, this means that `Renderer` never mutates and that all
//! drawing functionality is put behind interior mutability using
//! `RenderDrawer<'renderer>`.
//!
//! None of the draw methods in `RenderDrawer` are expected to fail.
//! If they do, a panic is raised and the program is aborted.

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
use libc::{c_int, uint32_t, c_double, c_void};
use rect::Point;
use rect::Rect;
use std::cell::{RefCell, RefMut};
use std::ffi::c_str_to_bytes;
use std::num::FromPrimitive;
use std::vec::Vec;
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

/// A structure that contains information on the capabilities of a render driver
/// or the current render context.
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
    pub unsafe fn from_ll(info: &ll::SDL_RendererInfo) -> RendererInfo {
        let actual_flags = RendererFlags::from_bits(info.flags).unwrap();

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

pub enum RendererParent {
    Surface(Surface),
    Window(Window)
}

/// 2D rendering context
pub struct Renderer {
    raw: *const ll::SDL_Renderer,
    parent: Option<RendererParent>,
    drawer_borrow: RefCell<()>
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe { ll::SDL_DestroyRenderer(self.raw) };
    }
}

impl Renderer {
    /// Creates a 2D rendering context for a window.
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
            unsafe {
                Ok(Renderer::from_ll(raw, RendererParent::Window(window)))
            }
        }
    }

    /// Creates a window and default renderer.
    pub fn new_with_window(width: i32, height: i32, window_flags: video::WindowFlags) -> SdlResult<Renderer> {
        use sys::video::SDL_Window;

        let raw_window: *const SDL_Window = ptr::null();
        let raw_renderer: *const ll::SDL_Renderer = ptr::null();
        let result = unsafe { ll::SDL_CreateWindowAndRenderer(width as c_int, height as c_int, window_flags.bits(), &raw_window, &raw_renderer) == 0};
        if result {
            let window = unsafe { Window::from_ll(raw_window, true) };
            unsafe {
                Ok(Renderer::from_ll(raw_renderer, RendererParent::Window(window)))
            }
        } else {
            Err(get_error())
        }
    }

    /// Creates a 2D software rendering context for a surface.
    pub fn from_surface(surface: surface::Surface) -> SdlResult<Renderer> {
        let raw_renderer = unsafe { ll::SDL_CreateSoftwareRenderer(surface.raw()) };
        if raw_renderer != ptr::null() {
            unsafe {
                Ok(Renderer::from_ll(raw_renderer, RendererParent::Surface(surface)))
            }
        } else {
            Err(get_error())
        }
    }

    /// Gets information about the rendering context.
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

    /// Gets the window or surface the rendering context was created from.
    #[inline]
    pub fn get_parent(&self) -> &RendererParent { self.parent.as_ref().unwrap() }

    #[inline]
    pub fn get_parent_as_window(&self) -> Option<&Window> {
        match self.get_parent() {
            &RendererParent::Window(ref window) => Some(window),
            _ => None
        }
    }

    #[inline]
    pub fn get_parent_as_surface(&self) -> Option<&Surface> {
        match self.get_parent() {
            &RendererParent::Surface(ref surface) => Some(surface),
            _ => None
        }
    }

    #[inline]
    pub fn unwrap_parent(mut self) -> RendererParent {
        use std::mem;
        mem::replace(&mut self.parent, None).unwrap()
    }

    #[inline]
    pub fn unwrap_parent_as_window(self) -> Option<Window> {
        match self.unwrap_parent() {
            RendererParent::Window(window) => Some(window),
            _ => None
        }
    }

    #[inline]
    pub fn unwrap_parent_as_surface(self) -> Option<Surface> {
        match self.unwrap_parent() {
            RendererParent::Surface(surface) => Some(surface),
            _ => None
        }
    }

    /// Provides drawing methods for the renderer.
    ///
    /// # Remarks
    /// This method is not `&mut self`.
    /// It uses interior mutability via `RenderDrawer<'renderer>` to preserve the
    /// Renderer's lifetime, and therefore the lifetimes of any Textures that
    /// belong to the Renderer.
    ///
    /// Only one `RenderDrawer` per `Renderer` can be active at a time.
    /// If this method is called and an existing `RenderDrawer` from this
    /// instance is active, the program will panic.
    ///
    /// # Examples
    /// ```no_run
    /// use sdl2::render::Renderer;
    /// use sdl2::rect::Rect;
    ///
    /// fn test_draw(renderer: &Renderer) {
    ///     let mut drawer = renderer.drawer();
    ///     drawer.clear();
    ///     drawer.draw_rect(Rect::new(50, 50, 150, 175));
    ///     drawer.present();
    /// }
    /// ```
    pub fn drawer(&self) -> RenderDrawer {
        match self.drawer_borrow.try_borrow_mut() {
            Some(borrow) => RenderDrawer::new(self.raw, borrow),
            None => panic!("Renderer drawer already borrowed")
        }
    }

    /// Unwraps the window or surface the rendering context was created from.
    pub unsafe fn raw(&self) -> *const ll::SDL_Renderer { self.raw }

    pub unsafe fn from_ll(raw: *const ll::SDL_Renderer, parent: RendererParent)
    -> Renderer
    {
        Renderer {
            raw: raw,
            parent: Some(parent),
            drawer_borrow: RefCell::new(())
        }
    }
}

/// Texture-creating methods for the renderer
impl Renderer {
    /// Creates a texture for a rendering context.
    ///
    /// `size` is the width and height of the texture.
    pub fn create_texture(&self, format: pixels::PixelFormatEnum, access: TextureAccess, size: (i32, i32)) -> SdlResult<Texture> {
        let (width, height) = size;
        let result = unsafe { ll::SDL_CreateTexture(self.raw, format as uint32_t, access as c_int, width as c_int, height as c_int) };
        if result == ptr::null() {
            Err(get_error())
        } else {
            unsafe { Ok(Texture::from_ll(result)) }
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

    /// Creates a texture from an existing surface.
    /// # Remarks
    /// The access hint for the created texture is `TextureAccess::Static`.
    pub fn create_texture_from_surface(&self, surface: &surface::Surface) -> SdlResult<Texture> {
        let result = unsafe { ll::SDL_CreateTextureFromSurface(self.raw, surface.raw()) };
        if result == ptr::null() {
            Err(get_error())
        } else {
            unsafe { Ok(Texture::from_ll(result)) }
        }
    }
}

/// Drawing functionality for the render context.
pub struct RenderDrawer<'renderer> {
    raw: *const ll::SDL_Renderer,
    _borrow: RefMut<'renderer, ()>
}

/// Render target methods for the drawer
impl<'renderer> RenderDrawer<'renderer> {
    fn new<'l>(raw: *const ll::SDL_Renderer, borrow: RefMut<'l, ()>) -> RenderDrawer<'l> {
        RenderDrawer {
            raw: raw,
            _borrow: borrow
        }
    }

    /// Determine whether a window supports the use of render targets.
    pub fn render_target_supported(&self) -> bool {
        unsafe { ll::SDL_RenderTargetSupported(self.raw) == 1 }
    }

    /// Gets the render target handle.
    ///
    /// Returns `None` if the window does not support the use of render targets.
    pub fn render_target(&mut self) -> Option<RenderTarget> {
        if self.render_target_supported() {
            Some(RenderTarget {
                raw: self.raw,
                _marker: ContravariantLifetime
            })
        } else {
            None
        }
    }
}

/// Drawing methods
impl<'renderer> RenderDrawer<'renderer> {
    /// Sets the color used for drawing operations (Rect, Line and Clear).
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

    /// Gets the color used for drawing operations (Rect, Line and Clear).
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

    /// Sets the blend mode used for drawing operations (Fill and Line).
    pub fn set_blend_mode(&mut self, blend: BlendMode) {
        let ret = unsafe { ll::SDL_SetRenderDrawBlendMode(self.raw, FromPrimitive::from_i64(blend as i64).unwrap()) };
        // Should only fail on an invalid renderer
        if ret != 0 { panic!(get_error()) }
    }

    /// Gets the blend mode used for drawing operations.
    pub fn get_blend_mode(&self) -> BlendMode {
        let blend = 0;
        let ret = unsafe { ll::SDL_GetRenderDrawBlendMode(self.raw, &blend) };
        // Should only fail on an invalid renderer
        if ret != 0 { panic!(get_error()) }
        else { FromPrimitive::from_i64(blend as i64).unwrap() }
    }

    /// Clears the current rendering target with the drawing color.
    pub fn clear(&mut self) {
        let ret = unsafe { ll::SDL_RenderClear(self.raw) };
        if ret != 0 { panic!("Could not clear: {}", get_error()) }
    }

    /// Updates the screen with any rendering performed since the previous call.
    ///
    /// SDL's rendering functions operate on a backbuffer; that is, calling a
    /// rendering function such as `draw_line()` does not directly put a line on
    /// the screen, but rather updates the backbuffer.
    /// As such, you compose your entire scene and present the composed
    /// backbuffer to the screen as a complete picture.
    pub fn present(&mut self) {
        unsafe { ll::SDL_RenderPresent(self.raw) }
    }

    /// Gets the output size of a rendering context.
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

    /// Sets a device independent resolution for rendering.
    pub fn set_logical_size(&mut self, width: i32, height: i32) {
        let ret = unsafe { ll::SDL_RenderSetLogicalSize(self.raw, width as c_int, height as c_int) };
        if ret != 0 { panic!("Could not set logical size: {}", get_error()) }
    }

    /// Gets device independent resolution for rendering.
    pub fn get_logical_size(&self) -> (i32, i32) {

        let width: c_int = 0;
        let height: c_int = 0;

        unsafe { ll::SDL_RenderGetLogicalSize(self.raw, &width, &height) };

        (width as i32, height as i32)
    }

    /// Sets the drawing area for rendering on the current target.
    pub fn set_viewport(&mut self, rect: Option<Rect>) {
        let ptr = match rect {
            Some(ref rect) => rect as *const _,
            None => ptr::null()
        };
        let ret = unsafe { ll::SDL_RenderSetViewport(self.raw, ptr) };
        if ret != 0 { panic!("Could not set viewport: {}", get_error()) }
    }

    /// Gets the drawing area for the current target.
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

    /// Sets the clip rectangle for rendering on the specified target.
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

    /// Gets the clip rectangle for the current target.
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

    /// Sets the drawing scale for rendering on the current target.
    pub fn set_scale(&mut self, scale_x: f32, scale_y: f32) {
        let ret = unsafe { ll::SDL_RenderSetScale(self.raw, scale_x, scale_y) };
        // Should only fail on an invalid renderer
        if ret != 0 { panic!(get_error()) }
    }

    /// Gets the drawing scale for the current target.
    pub fn get_scale(&self) -> (f32, f32) {
        let scale_x = 0.0;
        let scale_y = 0.0;
        unsafe { ll::SDL_RenderGetScale(self.raw, &scale_x, &scale_y) };
        (scale_x, scale_y)
    }

    /// Draws a point on the current rendering target.
    /// # Panics
    /// Panics if drawing fails for any reason (e.g. driver failure)
    pub fn draw_point(&mut self, point: Point) {
        unsafe {
            if ll::SDL_RenderDrawPoint(self.raw, point.x, point.y) != 0 {
                panic!("Error drawing point: {}", get_error())
            }
        }
    }

    /// Draws multiple points on the current rendering target.
    /// # Panics
    /// Panics if drawing fails for any reason (e.g. driver failure)
    pub fn draw_points(&mut self, points: &[Point]) {
        unsafe {
            if ll::SDL_RenderDrawPoints(self.raw, points.as_ptr(), points.len() as c_int) != 0 {
                panic!("Error drawing points: {}", get_error())
            }
        }
    }

    // Draws a line on the current rendering target.
    /// # Panics
    /// Panics if drawing fails for any reason (e.g. driver failure)
    pub fn draw_line(&mut self, start: Point, end: Point) {
        unsafe {
            if ll::SDL_RenderDrawLine(self.raw, start.x, start.y, end.x, end.y) != 0 {
                panic!("Error drawing line: {}", get_error())
            }
        }
    }

    /// Draws a series of connected lines on the current rendering target.
    /// # Panics
    /// Panics if drawing fails for any reason (e.g. driver failure)
    pub fn draw_lines(&mut self, points: &[Point]) {
        unsafe {
            if ll::SDL_RenderDrawLines(self.raw, points.as_ptr(), points.len() as c_int) != 0 {
                panic!("Error drawing lines: {}", get_error())
            }
        }
    }

    /// Draws a rectangle on the current rendering target.
    /// # Panics
    /// Panics if drawing fails for any reason (e.g. driver failure)
    pub fn draw_rect(&mut self, rect: Rect) {
        unsafe {
            if ll::SDL_RenderDrawRect(self.raw, &rect) != 0 {
                panic!("Error drawing rect: {}", get_error())
            }
        }
    }

    /// Draws some number of rectangles on the current rendering target.
    /// # Panics
    /// Panics if drawing fails for any reason (e.g. driver failure)
    pub fn draw_rects(&mut self, rects: &[Rect]) {
        unsafe {
            if ll::SDL_RenderDrawRects(self.raw, rects.as_ptr(), rects.len() as c_int) != 0 {
                panic!("Error drawing rects: {}", get_error())
            }
        }
    }

    /// Fills a rectangle on the current rendering target with the drawing
    /// color.
    /// # Panics
    /// Panics if drawing fails for any reason (e.g. driver failure)
    pub fn fill_rect(&mut self, rect: Rect) {
        unsafe {
            if ll::SDL_RenderFillRect(self.raw, &rect) != 0 {
                panic!("Error filling rect: {}", get_error())
            }
        }
    }

    /// Fills some number of rectangles on the current rendering target with
    /// the drawing color.
    /// # Panics
    /// Panics if drawing fails for any reason (e.g. driver failure)
    pub fn fill_rects(&mut self, rects: &[Rect]) {
        unsafe {
            if ll::SDL_RenderFillRects(self.raw, rects.as_ptr(), rects.len() as c_int) != 0 {
                panic!("Error filling rects: {}", get_error())
            }
        }
    }

    /// Copies a portion of the texture to the current rendering target.
    ///
    /// * If `src` is `None`, the entire texture is copied.
    /// * If `dst` is `None`, the texture will be stretched to fill the given
    ///   rectangle.
    ///
    /// # Panics
    /// Panics if drawing fails for any reason (e.g. driver failure),
    /// or if the provided texture does not belong to the renderer.
    pub fn copy(&mut self, texture: &Texture, src: Option<Rect>, dst: Option<Rect>) {
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

    /// Copies a portion of the texture to the current rendering target,
    /// optionally rotating it by angle around the given center and also
    /// flipping it top-bottom and/or left-right.
    ///
    /// * If `src` is `None`, the entire texture is copied.
    /// * If `dst` is `None`, the texture will be stretched to fill the given
    ///   rectangle.
    /// * If `center` is `None`, rotation will be done around the center point
    ///   of `dst`, or `src` if `dst` is None.
    ///
    /// # Panics
    /// Panics if drawing fails for any reason (e.g. driver failure),
    /// if the provided texture does not belong to the renderer,
    /// or if the driver does not support RenderCopyEx.
    pub fn copy_ex(&mut self, texture: &Texture, src: Option<Rect>, dst: Option<Rect>, angle: f64, center: Option<Point>, (flip_horizontal, flip_vertical): (bool, bool)) {
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

    /// Reads pixels from the current rendering target.
    /// # Remarks
    /// WARNING: This is a very slow operation, and should not be used frequently.
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

/// A handle for getting/setting the render target of the render context.
pub struct RenderTarget<'render_drawer> {
    raw: *const ll::SDL_Renderer,
    _marker: ContravariantLifetime<'render_drawer>
}

impl<'render_drawer> RenderTarget<'render_drawer> {
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

/// A texture for a rendering context.
///
/// Textures are owned by and cannot live longer than the parent `Renderer`.
/// Each texture is bound to the `'renderer` contravariant lifetime.
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
    /// Queries the attributes of the texture.
    pub fn query(&self) -> TextureQuery {
        let format: uint32_t = 0;
        let access: c_int = 0;
        let width: c_int = 0;
        let height: c_int = 0;

        let ret = unsafe { ll::SDL_QueryTexture(self.raw, &format, &access, &width, &height) };
        // Should only fail on an invalid texture
        if ret != 0 {
            panic!(get_error())
        } else {
            TextureQuery {
               format: FromPrimitive::from_i64(format as i64).unwrap(),
               access: FromPrimitive::from_i64(access as i64).unwrap(),
               width: width as i32,
               height: height as i32
            }
        }
    }

    /// Sets an additional color value multiplied into render copy operations.
    pub fn set_color_mod(&mut self, red: u8, green: u8, blue: u8) {
        let ret = unsafe { ll::SDL_SetTextureColorMod(self.raw, red, green, blue) };

        if ret != 0 {
            panic!("Error setting color mod: {}", get_error())
        }
    }

    /// Gets the additional color value multiplied into render copy operations.
    pub fn get_color_mod(&self) -> (u8, u8, u8) {
        let r = 0;
        let g = 0;
        let b = 0;
        let ret = unsafe { ll::SDL_GetTextureColorMod(self.raw, &r, &g, &b) };

        // Should only fail on an invalid texture
        if ret != 0 { panic!(get_error()) }
        else { (r, g, b) }
    }

    /// Sets an additional alpha value multiplied into render copy operations.
    pub fn set_alpha_mod(&mut self, alpha: u8) {
        let ret = unsafe { ll::SDL_SetTextureAlphaMod(self.raw, alpha) };

        if ret != 0 {
            panic!("Error setting alpha mod: {}", get_error())
        }
    }

    /// Gets the additional alpha value multiplied into render copy operations.
    pub fn get_alpha_mod(&self) -> u8 {
        let alpha = 0;
        let ret = unsafe { ll::SDL_GetTextureAlphaMod(self.raw, &alpha) };

        // Should only fail on an invalid texture
        if ret != 0 { panic!(get_error()) }
        else { alpha }
    }

    /// Sets the blend mode for a texture, used by `RenderDrawer::copy()`.
    pub fn set_blend_mode(&mut self, blend: BlendMode) {
        let ret = unsafe { ll::SDL_SetTextureBlendMode(self.raw, FromPrimitive::from_i64(blend as i64).unwrap()) };

        if ret != 0 {
            panic!("Error setting blend: {}", get_error())
        }
    }

    /// Gets the blend mode used for texture copy operations.
    pub fn get_blend_mode(&self) -> BlendMode {
        let blend = 0;
        let ret = unsafe { ll::SDL_GetTextureBlendMode(self.raw, &blend) };

        // Should only fail on an invalid texture
        if ret != 0 { panic!(get_error()) }
        else { FromPrimitive::from_i64(blend as i64).unwrap() }
    }

    /// Updates the given texture rectangle with new pixel data.
    ///
    /// `pitch` is the number of bytes in a row of pixel data, including padding
    /// between lines
    ///
    /// * If `rect` is `None`, the entire texture is updated.
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
            let q = self.query();
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

    /// Binds an OpenGL/ES/ES2 texture to the current
    /// context for use with when rendering OpenGL primitives directly.
    pub unsafe fn gl_bind_texture(&mut self) -> (f32, f32) {
        let texw = 0.0;
        let texh = 0.0;

        if ll::SDL_GL_BindTexture(self.raw, &texw, &texh) == 0 {
            (texw, texh)
        } else {
            panic!("OpenGL texture binding not supported");
        }
    }

    /// Unbinds an OpenGL/ES/ES2 texture from the current context.
    pub unsafe fn gl_unbind_texture(&mut self) {
        if ll::SDL_GL_UnbindTexture(self.raw) != 0 {
            panic!("OpenGL texture unbinding not supported");
        }
    }

    /// Binds and unbinds an OpenGL/ES/ES2 texture from the current context.
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

    pub unsafe fn from_ll<'l>(raw: *const ll::SDL_Texture) -> Texture<'l> {
        Texture {
            raw: raw,
            owned: true,
            _marker: ContravariantLifetime
        }
    }

    #[unstable="Will likely be removed with ownership reform"]
    pub unsafe fn from_ll_unowned<'l>(raw: *const ll::SDL_Texture) -> Texture<'l> {
        Texture {
            raw: raw,
            owned: false,
            _marker: ContravariantLifetime
        }
    }

    pub unsafe fn raw(&self) -> *const ll::SDL_Texture { self.raw }
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
        unsafe { Ok(RendererInfo::from_ll(&out)) }
    } else {
        Err(get_error())
    }
}

/*
    //TODO: Figure out how to support this with our current struct format
    pub fn SDL_GetRenderer(window: *SDL_Window) -> *SDL_Renderer;
*/
