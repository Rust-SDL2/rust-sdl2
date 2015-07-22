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
//! ---
//!
//! None of the draw methods in `Renderer` are expected to fail.
//! If they do, a panic is raised and the program is aborted.

use Sdl;
use video::{Window, WindowProperties, WindowPropertiesGetters};
use surface;
use surface::{Surface, SurfaceRef};
use pixels;
use pixels::PixelFormatEnum;
use get_error;
use SdlResult;
use std::mem;
use std::ptr;
use libc::{c_int, uint32_t, c_double, c_void};
use rect::Point;
use rect::Rect;
use std::cell::UnsafeCell;
use std::ffi::CStr;
use num::FromPrimitive;
use std::vec::Vec;
use std::rc::Rc;

use sys::render as ll;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum TextureAccess {
    Static = ll::SDL_TEXTUREACCESS_STATIC as isize,
    Streaming = ll::SDL_TEXTUREACCESS_STREAMING as isize,
    Target = ll::SDL_TEXTUREACCESS_TARGET as isize
}

impl FromPrimitive for TextureAccess {
    fn from_i64(n: i64) -> Option<TextureAccess> {
        use self::TextureAccess::*;

        Some( match n as ll::SDL_TextureAccess {
            ll::SDL_TEXTUREACCESS_STATIC    => Static,
            ll::SDL_TEXTUREACCESS_STREAMING => Streaming,
            ll::SDL_TEXTUREACCESS_TARGET    => Target,
            _                               => return None,
        })
    }

    fn from_u64(n: u64) -> Option<TextureAccess> { FromPrimitive::from_i64(n as i64) }
}

/// A structure that contains information on the capabilities of a render driver
/// or the current render context.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct RendererInfo {
    pub name: String,
    pub flags: u32,
    pub texture_formats: Vec<pixels::PixelFormatEnum>,
    pub max_texture_width: u32,
    pub max_texture_height: u32
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum BlendMode {
    None = ll::SDL_BLENDMODE_NONE as isize,
    Blend = ll::SDL_BLENDMODE_BLEND as isize,
    Add = ll::SDL_BLENDMODE_ADD as isize,
    Mod = ll::SDL_BLENDMODE_MOD as isize
}

impl FromPrimitive for BlendMode {
    fn from_i64(n: i64) -> Option<BlendMode> {
        use self::BlendMode::*;

        Some( match n as ll::SDL_BlendMode {
            ll::SDL_BLENDMODE_NONE  => None,
            ll::SDL_BLENDMODE_BLEND => Blend,
            ll::SDL_BLENDMODE_ADD   => Add,
            ll::SDL_BLENDMODE_MOD   => Mod,
            _                       => return Option::None,
        })
    }

    fn from_u64(n: u64) -> Option<BlendMode> { FromPrimitive::from_i64(n as i64) }
}

impl RendererInfo {
    pub unsafe fn from_ll(info: &ll::SDL_RendererInfo) -> RendererInfo {
        let texture_formats: Vec<pixels::PixelFormatEnum> = info.texture_formats[0..(info.num_texture_formats as usize)].iter().map(|&format| {
            FromPrimitive::from_i64(format as i64).unwrap()
        }).collect();

        RendererInfo {
            name: String::from_utf8_lossy(CStr::from_ptr(info.name).to_bytes()).to_string(),
            flags: info.flags,
            texture_formats: texture_formats,
            max_texture_width: info.max_texture_width as u32,
            max_texture_height: info.max_texture_height as u32
        }
    }
}

pub enum RendererParent<'a> {
    Surface(Surface<'a>),
    Window(Window)
}

/// 2D rendering context
pub struct Renderer<'a> {
    raw: *mut ll::SDL_Renderer,
    parent: Option<RendererParent<'a>>,
    is_alive: Rc<UnsafeCell<bool>>
}

impl<'a> Drop for Renderer<'a> {
    fn drop(&mut self) {
        unsafe {
            *self.is_alive.get() = false;
            ll::SDL_DestroyRenderer(self.raw);
        };
    }
}

/// The type that allows you to build Window-based renderers.
///
/// By default, the renderer builder will prioritize for a hardware-accelerated renderer.
pub struct RendererBuilder {
    window: Window,
    index: Option<u32>,
    renderer_flags: u32
}

impl RendererBuilder {
    /// Initializes a new `RendererBuilder`.
    pub fn new(window: Window) -> RendererBuilder {
        RendererBuilder {
            window: window,
            // -1 means to initialize the first rendering driver supporting the renderer flags
            index: None,
            // no flags gives priority to available SDL_RENDERER_ACCELERATED renderers
            renderer_flags: 0
        }
    }

    /// Builds the renderer.
    pub fn build(self) -> SdlResult<Renderer<'static>> {
        let index = match self.index {
            None => -1,
            Some(index) => try!(u32_to_int!(index))
        };
        let raw = unsafe {
            ll::SDL_CreateRenderer(self.window.raw(), index, self.renderer_flags)
        };

        if raw.is_null() {
            Err(get_error())
        } else {
            unsafe {
                Ok(Renderer::from_ll(raw, RendererParent::Window(self.window)))
            }
        }
    }

    /// Sets the index of the rendering driver to initialize.
    pub fn index(mut self, index: u32) -> RendererBuilder {
        self.index = Some(index);
        self
    }

    /// Set the renderer to a software fallback.
    pub fn software(mut self) -> RendererBuilder {
        self.renderer_flags |= ll::SDL_RENDERER_SOFTWARE as u32;
        self
    }

    /// Set the renderer to use hardware acceleration.
    pub fn accelerated(mut self) -> RendererBuilder {
        self.renderer_flags |= ll::SDL_RENDERER_ACCELERATED as u32;
        self
    }

    /// Synchronize renderer present with the refresh rate.
    pub fn present_vsync(mut self) -> RendererBuilder {
        self.renderer_flags |= ll::SDL_RENDERER_PRESENTVSYNC as u32;
        self
    }

    /// Set the renderer to support rendering to a texture.
    pub fn target_texture(mut self) -> RendererBuilder {
        self.renderer_flags |= ll::SDL_RENDERER_TARGETTEXTURE as u32;
        self
    }
}

impl<'a> Renderer<'a> {
    /// Creates a 2D software rendering context for a surface.
    ///
    /// This method should only fail if SDL2 is not built with rendering support, or there's
    /// an out-of-memory error.
    pub fn from_surface(surface: surface::Surface<'a>) -> SdlResult<Renderer<'a>> {
        let raw_renderer = unsafe { ll::SDL_CreateSoftwareRenderer(surface.raw()) };
        if raw_renderer != ptr::null_mut() {
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
            let mut renderer_info_raw = mem::uninitialized();
            if ll::SDL_GetRendererInfo(self.raw, &mut renderer_info_raw) != 0 {
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
    pub fn get_parent_as_surface(&self) -> Option<&SurfaceRef> {
        match self.get_parent() {
            &RendererParent::Surface(ref surface) => Some(surface),
            _ => None
        }
    }

    /// Accesses the Window properties, such as the position, size and title of a Window.
    /// Returns None if the renderer is not associated with a Window.
    pub fn window_properties<'b>(&'b mut self, sdl: &'b Sdl) -> Option<WindowProperties<'b>>
    {
        match self.parent.as_mut() {
            Some(&mut RendererParent::Window(ref mut window)) => Some(window.properties(sdl)),
            _ => None
        }
    }

    /// Accesses the Window getters, such as the position, size and title of a Window.
    /// Returns None if the renderer is not associated with a Window.
    pub fn window_properties_getters(&self) -> Option<WindowPropertiesGetters>
    {
        match self.parent.as_ref() {
            Some(&RendererParent::Window(ref window)) => Some(window.properties_getters()),
            _ => None
        }
    }

    #[inline]
    pub fn unwrap_parent(mut self) -> RendererParent<'a> {
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
    pub fn unwrap_parent_as_surface(self) -> Option<Surface<'a>> {
        match self.unwrap_parent() {
            RendererParent::Surface(surface) => Some(surface),
            _ => None
        }
    }

    /// Unwraps the window or surface the rendering context was created from.
    pub unsafe fn raw(&self) -> *mut ll::SDL_Renderer { self.raw }

    pub unsafe fn from_ll(raw: *mut ll::SDL_Renderer, parent: RendererParent)
    -> Renderer
    {
        Renderer {
            raw: raw,
            parent: Some(parent),
            is_alive: Rc::new(UnsafeCell::new(true))
        }
    }
}

/// Texture-creating methods for the renderer
impl<'a> Renderer<'a> {
    /// Creates a texture for a rendering context.
    ///
    /// `size` is the width and height of the texture.
    pub fn create_texture(&self, format: pixels::PixelFormatEnum, access: TextureAccess, (width, height): (u32, u32)) -> SdlResult<Texture> {
        let width = try!(u32_to_int!(width));
        let height = try!(u32_to_int!(height));

        // If the pixel format is YUV 4:2:0 and planar, the width and height must
        // be multiples-of-two. See issue #334 for details.
        match format {
            PixelFormatEnum::YV12 | PixelFormatEnum::IYUV => {
                if width % 2 != 0 || height % 2 != 0 {
                    return Err(format!("The width and height must be multiples-of-two for planar YUV 4:2:0 pixel formats"));
                }
            },
            _ => ()
        }

        let result = unsafe { ll::SDL_CreateTexture(self.raw, format as uint32_t, access as c_int, width, height) };
        if result == ptr::null_mut() {
            Err(get_error())
        } else {
            unsafe { Ok(Texture::from_ll(self, result)) }
        }
    }

    /// Shorthand for `create_texture(format, TextureAccess::Static, size)`
    pub fn create_texture_static(&self, format: pixels::PixelFormatEnum, size: (u32, u32)) -> SdlResult<Texture> {
        self.create_texture(format, TextureAccess::Static, size)
    }

    /// Shorthand for `create_texture(format, TextureAccess::Streaming, size)`
    pub fn create_texture_streaming(&self, format: pixels::PixelFormatEnum, size: (u32, u32)) -> SdlResult<Texture> {
        self.create_texture(format, TextureAccess::Streaming, size)
    }

    /// Shorthand for `create_texture(format, TextureAccess::Target, size)`
    pub fn create_texture_target(&self, format: pixels::PixelFormatEnum, size: (u32, u32)) -> SdlResult<Texture> {
        self.create_texture(format, TextureAccess::Target, size)
    }

    /// Creates a texture from an existing surface.
    /// # Remarks
    /// The access hint for the created texture is `TextureAccess::Static`.
    pub fn create_texture_from_surface<S: AsRef<SurfaceRef>>(&self, surface: S) -> SdlResult<Texture> {
        let result = unsafe { ll::SDL_CreateTextureFromSurface(self.raw, surface.as_ref().raw()) };
        if result == ptr::null_mut() {
            Err(get_error())
        } else {
            unsafe { Ok(Texture::from_ll(self, result)) }
        }
    }
}

/// Render target methods
impl<'a> Renderer<'a> {
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
                is_renderer_alive: &self.is_alive
            })
        } else {
            None
        }
    }
}

/// Drawing methods
impl<'a> Renderer<'a> {
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
        let (mut r, mut g, mut b, mut a) = (0, 0, 0, 0);
        let ret = unsafe { ll::SDL_GetRenderDrawColor(self.raw, &mut r, &mut g, &mut b, &mut a) };
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
        let mut blend = 0;
        let ret = unsafe { ll::SDL_GetRenderDrawBlendMode(self.raw, &mut blend) };
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
    pub fn get_output_size(&self) -> SdlResult<(u32, u32)> {
        let mut width = 0;
        let mut height = 0;

        let result = unsafe { ll::SDL_GetRendererOutputSize(self.raw, &mut width, &mut height) == 0 };

        if result {
            Ok((width as u32, height as u32))
        } else {
            Err(get_error())
        }
    }

    /// Sets a device independent resolution for rendering.
    pub fn set_logical_size(&mut self, width: u32, height: u32) -> SdlResult<()> {
        let width = try!(u32_to_int!(width));
        let height = try!(u32_to_int!(height));
        let result = unsafe { ll::SDL_RenderSetLogicalSize(self.raw, width, height) };
        match result {
            0 => Ok(()),
            _ => Err(get_error())
        }
    }

    /// Gets device independent resolution for rendering.
    pub fn get_logical_size(&self) -> (u32, u32) {
        let mut width = 0;
        let mut height = 0;

        unsafe { ll::SDL_RenderGetLogicalSize(self.raw, &mut width, &mut height) };

        (width as u32, height as u32)
    }

    /// Sets the drawing area for rendering on the current target.
    pub fn set_viewport(&mut self, rect: Option<Rect>) {
        let ptr = match rect {
            Some(ref rect) => rect.raw(),
            None => ptr::null()
        };
        let ret = unsafe { ll::SDL_RenderSetViewport(self.raw, ptr) };
        if ret != 0 { panic!("Could not set viewport: {}", get_error()) }
    }

    /// Gets the drawing area for the current target.
    pub fn get_viewport(&self) -> Rect {
        let mut rect = unsafe { mem::uninitialized() };
        unsafe { ll::SDL_RenderGetViewport(self.raw, &mut rect) };
        Rect::from_ll(rect).unwrap().unwrap()
    }

    /// Sets the clip rectangle for rendering on the specified target.
    ///
    /// If the rectangle is `None`, clipping will be disabled.
    pub fn set_clip_rect(&mut self, rect: Option<Rect>) {
        let ret = unsafe {
            ll::SDL_RenderSetClipRect(
                self.raw,
                match rect {
                    Some(ref rect) => rect.raw(),
                    None => ptr::null()
                }
            )
        };
        if ret != 0 { panic!("Could not set clip rect: {}", get_error()) }
    }

    /// Gets the clip rectangle for the current target.
    ///
    /// Returns `None` if clipping is disabled.
    pub fn get_clip_rect(&self) -> Option<Rect> {
        let mut rect = unsafe { mem::uninitialized() };
        unsafe { ll::SDL_RenderGetClipRect(self.raw, &mut rect) };
        Rect::from_ll(rect).unwrap()
    }

    /// Sets the drawing scale for rendering on the current target.
    pub fn set_scale(&mut self, scale_x: f32, scale_y: f32) {
        let ret = unsafe { ll::SDL_RenderSetScale(self.raw, scale_x, scale_y) };
        // Should only fail on an invalid renderer
        if ret != 0 { panic!(get_error()) }
    }

    /// Gets the drawing scale for the current target.
    pub fn get_scale(&self) -> (f32, f32) {
        let mut scale_x = 0.0;
        let mut scale_y = 0.0;
        unsafe { ll::SDL_RenderGetScale(self.raw, &mut scale_x, &mut scale_y) };
        (scale_x, scale_y)
    }

    /// Draws a point on the current rendering target.
    /// # Panics
    /// Panics if drawing fails for any reason (e.g. driver failure)
    pub fn draw_point(&mut self, point: Point) {
        unsafe {
            if ll::SDL_RenderDrawPoint(self.raw, point.x(), point.y()) != 0 {
                panic!("Error drawing point: {}", get_error())
            }
        }
    }

    /// Draws multiple points on the current rendering target.
    /// # Panics
    /// Panics if drawing fails for any reason (e.g. driver failure)
    pub fn draw_points(&mut self, points: &[Point]) {
        unsafe {
            if ll::SDL_RenderDrawPoints(self.raw, Point::raw_slice(points), points.len() as c_int) != 0 {
                panic!("Error drawing points: {}", get_error())
            }
        }
    }

    // Draws a line on the current rendering target.
    /// # Panics
    /// Panics if drawing fails for any reason (e.g. driver failure)
    pub fn draw_line(&mut self, start: Point, end: Point) {
        unsafe {
            if ll::SDL_RenderDrawLine(self.raw, start.x(), start.y(), end.x(), end.y()) != 0 {
                panic!("Error drawing line: {}", get_error())
            }
        }
    }

    /// Draws a series of connected lines on the current rendering target.
    /// # Panics
    /// Panics if drawing fails for any reason (e.g. driver failure)
    pub fn draw_lines(&mut self, points: &[Point]) {
        unsafe {
            if ll::SDL_RenderDrawLines(self.raw, Point::raw_slice(points), points.len() as c_int) != 0 {
                panic!("Error drawing lines: {}", get_error())
            }
        }
    }

    /// Draws a rectangle on the current rendering target.
    /// # Panics
    /// Panics if drawing fails for any reason (e.g. driver failure)
    pub fn draw_rect(&mut self, rect: Rect) {
        unsafe {
            if ll::SDL_RenderDrawRect(self.raw, rect.raw()) != 0 {
                panic!("Error drawing rect: {}", get_error())
            }
        }
    }

    /// Draws some number of rectangles on the current rendering target.
    /// # Panics
    /// Panics if drawing fails for any reason (e.g. driver failure)
    pub fn draw_rects(&mut self, rects: &[Rect]) {
        unsafe {
            if ll::SDL_RenderDrawRects(self.raw, Rect::raw_slice(rects), rects.len() as c_int) != 0 {
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
            if ll::SDL_RenderFillRect(self.raw, rect.raw()) != 0 {
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
            if ll::SDL_RenderFillRects(self.raw, Rect::raw_slice(rects), rects.len() as c_int) != 0 {
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
        texture.check_renderer();

        let ret = unsafe {
            ll::SDL_RenderCopy(
                self.raw,
                texture.raw,
                match src {
                    Some(ref rect) => rect.raw(),
                    None => ptr::null()
                },
                match dst {
                    Some(ref rect) => rect.raw(),
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
        texture.check_renderer();

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
                    Some(ref rect) => rect.raw(),
                    None => ptr::null()
                },
                match dst {
                    Some(ref rect) => rect.raw(),
                    None => ptr::null()
                },
                angle as c_double,
                match center {
                    Some(ref point) => point.raw(),
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
                Some(ref rect) => (rect.raw(), rect.width() as usize, rect.height() as usize),
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
                ll::SDL_RenderReadPixels(self.raw, actual_rect, format as uint32_t, pixels.as_mut_ptr() as *mut c_void, pitch as c_int)
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
///
/// # Example
/// ```no_run
/// use sdl2::pixels::{Color, PixelFormatEnum};
/// use sdl2::rect::Rect;
/// use sdl2::render::{Renderer, Texture};
///
/// // Draw a red rectangle to a new texture
/// fn draw_to_texture(r: &mut Renderer) -> Texture {
///     r.render_target()
///         .expect("This platform doesn't support render targets")
///         .create_and_set(PixelFormatEnum::RGBA8888, (512, 512));
///
///     // Start drawing
///     r.clear();
///     r.set_draw_color(Color::RGB(255, 0, 0));
///     r.fill_rect(Rect::new_unwrap(100, 100, 256, 256));
///
///     let texture: Option<Texture> = r.render_target().unwrap().reset().unwrap();
///     texture.unwrap()
/// }
/// ```
pub struct RenderTarget<'renderer> {
    raw: *mut ll::SDL_Renderer,
    is_renderer_alive: &'renderer Rc<UnsafeCell<bool>>
}

impl<'renderer> RenderTarget<'renderer> {
    /// Resets the render target to the default render target.
    ///
    /// The old render target is returned if the function is successful.
    pub fn reset(&mut self) -> SdlResult<Option<Texture>> {
        unsafe {
            let old_texture_raw = ll::SDL_GetRenderTarget(self.raw);

            if ll::SDL_SetRenderTarget(self.raw, ptr::null_mut()) == 0 {
                Ok(match old_texture_raw.is_null() {
                    true => None,
                    false => Some(Texture {
                        raw: old_texture_raw,
                        is_renderer_alive: self.is_renderer_alive.clone()
                    })
                })
            } else {
                Err(get_error())
            }
        }
    }

    /// Sets the render target to the provided texture.
    /// The texture must be created with the texture access: `sdl2::render::TextureAccess::Target`.
    ///
    /// The old render target is returned if the function is successful.
    pub fn set(&mut self, texture: Texture) -> SdlResult<Option<Texture>> {
        texture.check_renderer();

        unsafe {
            let old_texture_raw = ll::SDL_GetRenderTarget(self.raw);

            if ll::SDL_SetRenderTarget(self.raw, texture.raw) == 0 {
                texture.forget();
                Ok(match old_texture_raw.is_null() {
                    true => None,
                    false => Some(Texture {
                        raw: old_texture_raw,
                        is_renderer_alive: self.is_renderer_alive.clone()
                    })
                })
            } else {
                Err(get_error())
            }
        }
    }

    /// Creates a new texture and sets it as the render target.
    ///
    /// The old render target is returned if the function is successful.
    pub fn create_and_set(&mut self, format: pixels::PixelFormatEnum, (width, height): (u32, u32)) -> SdlResult<Option<Texture>> {
        let width = try!(u32_to_int!(width));
        let height = try!(u32_to_int!(height));

        let new_texture_raw = unsafe {
            let access = ll::SDL_TEXTUREACCESS_TARGET;
            ll::SDL_CreateTexture(self.raw, format as uint32_t, access as c_int, width, height)
        };

        if new_texture_raw == ptr::null_mut() {
            Err(get_error())
        } else {
            unsafe {
                let old_texture_raw = ll::SDL_GetRenderTarget(self.raw);

                if ll::SDL_SetRenderTarget(self.raw, new_texture_raw) == 0 {
                    Ok(match old_texture_raw.is_null() {
                        true => None,
                        false => Some(Texture {
                            raw: old_texture_raw,
                            is_renderer_alive: self.is_renderer_alive.clone()
                        })
                    })
                } else {
                    Err(get_error())
                }
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TextureQuery {
    pub format: pixels::PixelFormatEnum,
    pub access: TextureAccess,
    pub width: u32,
    pub height: u32
}

/// A texture for a rendering context.
///
/// Every Texture is owned by a Renderer.
/// If a Texture is accessed after the corresponding Renderer is dropped, then
/// the program will panic (clarification: will not crash).
///
/// A Texture can be safely dropped before or after the Renderer is dropped.
pub struct Texture {
    raw: *mut ll::SDL_Texture,
    is_renderer_alive: Rc<UnsafeCell<bool>>
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            if *self.is_renderer_alive.get() {
                ll::SDL_DestroyTexture(self.raw);
            }
        }
    }
}

impl Texture {
    #[inline]
    fn check_renderer(&self) {
        let alive = unsafe { *self.is_renderer_alive.get() };
        if !alive {
            panic!("renderer has been destroyed; cannot use Texture");
        }
    }

    /// Doesn't free the Texture, but decrements its `is_renderer_alive` box.
    fn forget(self) {
        unsafe {
            let _is_renderer_alive: Rc<UnsafeCell<bool>> = mem::transmute_copy(&self.is_renderer_alive);
            mem::forget(self);

            // is_renderer_alive gets deref'd
        }
    }

    /// Queries the attributes of the texture.
    pub fn query(&self) -> TextureQuery {
        self.check_renderer();

        let mut format = 0;
        let mut access = 0;
        let mut width = 0;
        let mut height = 0;

        let ret = unsafe { ll::SDL_QueryTexture(self.raw, &mut format, &mut access, &mut width, &mut height) };
        // Should only fail on an invalid texture
        if ret != 0 {
            panic!(get_error())
        } else {
            TextureQuery {
               format: FromPrimitive::from_i64(format as i64).unwrap(),
               access: FromPrimitive::from_i64(access as i64).unwrap(),
               width: width as u32,
               height: height as u32
            }
        }
    }

    /// Sets an additional color value multiplied into render copy operations.
    pub fn set_color_mod(&mut self, red: u8, green: u8, blue: u8) {
        self.check_renderer();

        let ret = unsafe { ll::SDL_SetTextureColorMod(self.raw, red, green, blue) };

        if ret != 0 {
            panic!("Error setting color mod: {}", get_error())
        }
    }

    /// Gets the additional color value multiplied into render copy operations.
    pub fn get_color_mod(&self) -> (u8, u8, u8) {
        self.check_renderer();

        let (mut r, mut g, mut b) = (0, 0, 0);
        let ret = unsafe { ll::SDL_GetTextureColorMod(self.raw, &mut r, &mut g, &mut b) };

        // Should only fail on an invalid texture
        if ret != 0 { panic!(get_error()) }
        else { (r, g, b) }
    }

    /// Sets an additional alpha value multiplied into render copy operations.
    pub fn set_alpha_mod(&mut self, alpha: u8) {
        self.check_renderer();

        let ret = unsafe { ll::SDL_SetTextureAlphaMod(self.raw, alpha) };

        if ret != 0 {
            panic!("Error setting alpha mod: {}", get_error())
        }
    }

    /// Gets the additional alpha value multiplied into render copy operations.
    pub fn get_alpha_mod(&self) -> u8 {
        self.check_renderer();

        let mut alpha = 0;
        let ret = unsafe { ll::SDL_GetTextureAlphaMod(self.raw, &mut alpha) };

        // Should only fail on an invalid texture
        if ret != 0 { panic!(get_error()) }
        else { alpha }
    }

    /// Sets the blend mode for a texture, used by `Renderer::copy()`.
    pub fn set_blend_mode(&mut self, blend: BlendMode) {
        self.check_renderer();

        let ret = unsafe { ll::SDL_SetTextureBlendMode(self.raw, FromPrimitive::from_i64(blend as i64).unwrap()) };

        if ret != 0 {
            panic!("Error setting blend: {}", get_error())
        }
    }

    /// Gets the blend mode used for texture copy operations.
    pub fn get_blend_mode(&self) -> BlendMode {
        self.check_renderer();

        let mut blend = 0;
        let ret = unsafe { ll::SDL_GetTextureBlendMode(self.raw, &mut blend) };

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
    pub fn update(&mut self, rect: Option<Rect>, pixel_data: &[u8], pitch: usize) -> SdlResult<()> {
        self.check_renderer();

        let ret = unsafe {
            let rect_raw_ptr = match rect {
                Some(ref rect) => rect.raw(),
                None => ptr::null()
            };

            // Check if the rectangle's position or size is odd, and if the pitch is odd.
            // This needs to be done in case the texture's pixel format is planar YUV.
            // See issue #334 for details.
            let rect_is_odd = match rect {
                Some(r) => (r.x() % 2 != 0) || (r.y() % 2 != 0) || (r.width() % 2 != 0) || (r.height() % 2 != 0),
                None => false
            };
            let pitch_is_odd = pitch % 2 != 0;

            if rect_is_odd || pitch_is_odd {
                // Query the texture's format
                match self.query() {
                    TextureQuery { format: PixelFormatEnum::YV12, .. } |
                    TextureQuery { format: PixelFormatEnum::IYUV, .. } => {
                        return Err(format!("The rectangle dimensions and pitch must be multiples-of-two for planar YUV 4:2:0 pixel formats"));
                    },
                    _ => ()
                }
            }

            let pitch = try!(usize_to_int!(pitch));

            ll::SDL_UpdateTexture(self.raw, rect_raw_ptr, pixel_data.as_ptr() as *const _, pitch)
        };

        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    /// Updates a rectangle within a planar YV12 or IYUV texture with new pixel data.
    pub fn update_yuv(&mut self, rect: Option<Rect>, y_plane: &[u8], y_pitch: usize, u_plane: &[u8], u_pitch: usize, v_plane: &[u8], v_pitch: usize) -> SdlResult<()> {
        self.check_renderer();

        let rect_raw_ptr = match rect {
            Some(ref rect) => rect.raw(),
            None => ptr::null()
        };

        let rect_is_odd = match rect {
            Some(r) => (r.x() % 2 != 0) || (r.y() % 2 != 0) || (r.width() % 2 != 0) || (r.height() % 2 != 0),
            None => false
        };

        if rect_is_odd {
            return Err(format!("The rectangle dimensions must be multiples-of-two for planar YUV 4:2:0 pixel formats"));
        }

        // We need the height in order to check the array slice lengths.
        // Checking the lengths can prevent buffer overruns in SDL_UpdateYUVTexture.
        let height = match rect {
            Some(r) => r.height(),
            None => self.query().height
        } as usize;

        let wrong_length =
            (y_plane.len() != (y_pitch * height)) ||
            (u_plane.len() != (u_pitch * height/2)) ||
            (v_plane.len() != (v_pitch * height/2));

        if wrong_length {
            return Err(format!("One or more of the plane lengths is not correct (should be pitch * height)."));
        }

        let y_pitch = try!(usize_to_int!(y_pitch));
        let u_pitch = try!(usize_to_int!(u_pitch));
        let v_pitch = try!(usize_to_int!(v_pitch));

        unsafe {
            let result = ll::SDL_UpdateYUVTexture(
                self.raw,
                rect_raw_ptr,
                y_plane.as_ptr(),
                y_pitch,
                u_plane.as_ptr(),
                u_pitch,
                v_plane.as_ptr(),
                v_pitch
            );

            if result == 0 { Ok(()) }
            else { Err(get_error()) }
        }
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
        self.check_renderer();

        // Call to SDL to populate pixel data
        let loaded = unsafe {
            let q = self.query();
            let mut pixels = ptr::null_mut();
            let mut pitch = 0;

            let (rect_raw_ptr, height) = match rect {
                Some(ref rect) => (rect.raw(), rect.height() as usize),
                None => (ptr::null(), q.height as usize)
            };

            let ret = ll::SDL_LockTexture(self.raw, rect_raw_ptr, &mut pixels, &mut pitch);
            if ret == 0 {
                let size = q.format.byte_size_from_pitch_and_height(pitch as usize, height);
                Ok( (::std::slice::from_raw_parts_mut(pixels as *mut u8, size ), pitch) )
            } else {
                Err(get_error())
            }
        };

        match loaded {
            Ok((interior, pitch)) => {
                let result;
                unsafe {
                    result = func(interior, pitch as usize);
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
        self.check_renderer();

        let mut texw = 0.0;
        let mut texh = 0.0;

        if ll::SDL_GL_BindTexture(self.raw, &mut texw, &mut texh) == 0 {
            (texw, texh)
        } else {
            panic!("OpenGL texture binding not supported");
        }
    }

    /// Unbinds an OpenGL/ES/ES2 texture from the current context.
    pub unsafe fn gl_unbind_texture(&mut self) {
        self.check_renderer();

        if ll::SDL_GL_UnbindTexture(self.raw) != 0 {
            panic!("OpenGL texture unbinding not supported");
        }
    }

    /// Binds and unbinds an OpenGL/ES/ES2 texture from the current context.
    pub fn gl_with_bind<R, F: FnOnce(f32, f32) -> R>(&mut self, f: F) -> R {
        self.check_renderer();

        unsafe {
            let mut texw = 0.0;
            let mut texh = 0.0;

            if ll::SDL_GL_BindTexture(self.raw, &mut texw, &mut texh) == 0 {
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

    pub unsafe fn from_ll(renderer: &Renderer, raw: *mut ll::SDL_Texture) -> Texture {
        Texture {
            raw: raw,
            is_renderer_alive: renderer.is_alive.clone()
        }
    }

    pub unsafe fn raw(&self) -> *mut ll::SDL_Texture { self.raw }
}


pub fn get_num_render_drivers() -> SdlResult<u32> {
    let result = unsafe { ll::SDL_GetNumRenderDrivers() };
    if result > 0 {
        Ok(result as u32)
    } else {
        Err(get_error())
    }
}

pub fn get_render_driver_info(index: u32) -> SdlResult<RendererInfo> {
    let mut out = unsafe { mem::uninitialized() };
    let index = try!(u32_to_int!(index));
    let result = unsafe { ll::SDL_GetRenderDriverInfo(index, &mut out) == 0 };
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
