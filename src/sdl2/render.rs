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
//! None of the draw methods in `Canvas` are expected to fail.
//! If they do, a panic is raised and the program is aborted.

use video::{Window, WindowContext};
use surface;
use surface::{Surface, SurfaceRef, SurfaceContext};
use pixels;
use pixels::PixelFormatEnum;
use get_error;
use std::fmt;
use std::error::Error;
use std::marker::PhantomData;
use std::mem;
use std::ops::Deref;
use std::ptr;
use std::rc::Rc;
use libc::{c_int, uint32_t, c_double, c_void};
use rect::Point;
use rect::Rect;
use std::ffi::CStr;
use num::FromPrimitive;
use std::vec::Vec;
use common::{validate_int, IntegerOrSdlError};

use sys::render as ll;

/// Contains the description of an error returned by SDL
#[derive(Debug)]
pub struct SdlError(String);

/// Possible errors returned by targetting a `Canvas` to render to a `Texture`
#[derive(Debug)]
pub enum TargetRenderError {
    SdlError(SdlError),
    NotSupported,
}

impl fmt::Display for SdlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &SdlError(ref e) = self;
        write!(f, "SDL error: {}", e)
    }
}

impl Error for SdlError {
    fn description(&self) -> &str {
        let &SdlError(ref e) = self;
        e
    }
}

impl fmt::Display for TargetRenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TargetRenderError::*;
        match *self {
            SdlError(ref e) => e.fmt(f),
            NotSupported => write!(f, "The renderer does not support the use of render targets"),
        }
    }
}

impl Error for TargetRenderError {
    fn description(&self) -> &str {
        use self::TargetRenderError::*;
        match *self {
            SdlError(ref e) => e.description(),
            NotSupported => "The renderer does not support the use of render targets",
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(i32)]
pub enum TextureAccess {
    Static = ll::SDL_TEXTUREACCESS_STATIC as i32,
    Streaming = ll::SDL_TEXTUREACCESS_STREAMING as i32,
    Target = ll::SDL_TEXTUREACCESS_TARGET as i32,
}

impl FromPrimitive for TextureAccess {
    fn from_i64(n: i64) -> Option<TextureAccess> {
        use self::TextureAccess::*;

        Some(match n as ll::SDL_TextureAccess {
                 ll::SDL_TEXTUREACCESS_STATIC => Static,
                 ll::SDL_TEXTUREACCESS_STREAMING => Streaming,
                 ll::SDL_TEXTUREACCESS_TARGET => Target,
                 _ => return None,
             })
    }

    fn from_u64(n: u64) -> Option<TextureAccess> {
        FromPrimitive::from_i64(n as i64)
    }
}

/// A structure that contains information on the capabilities of a render driver
/// or the current render context.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct RendererInfo {
    pub name: &'static str,
    pub flags: u32,
    pub texture_formats: Vec<PixelFormatEnum>,
    pub max_texture_width: u32,
    pub max_texture_height: u32,
}

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum BlendMode {
    None = ll::SDL_BLENDMODE_NONE as i32,
    Blend = ll::SDL_BLENDMODE_BLEND as i32,
    Add = ll::SDL_BLENDMODE_ADD as i32,
    Mod = ll::SDL_BLENDMODE_MOD as i32,
}

impl FromPrimitive for BlendMode {
    fn from_i64(n: i64) -> Option<BlendMode> {
        use self::BlendMode::*;

        Some(match n as ll::SDL_BlendMode {
                 ll::SDL_BLENDMODE_NONE => None,
                 ll::SDL_BLENDMODE_BLEND => Blend,
                 ll::SDL_BLENDMODE_ADD => Add,
                 ll::SDL_BLENDMODE_MOD => Mod,
                 _ => return Option::None,
             })
    }

    fn from_u64(n: u64) -> Option<BlendMode> {
        FromPrimitive::from_i64(n as i64)
    }
}

impl RendererInfo {
    pub unsafe fn from_ll(info: &ll::SDL_RendererInfo) -> RendererInfo {
        let texture_formats: Vec<PixelFormatEnum> =
            info.texture_formats[0..(info.num_texture_formats as usize)]
                .iter()
                .map(|&format| {
                         PixelFormatEnum::from_i64(format as i64)
                             .unwrap_or(PixelFormatEnum::Unknown)
                     })
                .collect();

        // The driver name is always a static string, compiled into SDL2.
        let name = CStr::from_ptr(info.name as *const _).to_str().unwrap();

        RendererInfo {
            name: name,
            flags: info.flags,
            texture_formats: texture_formats,
            max_texture_width: info.max_texture_width as u32,
            max_texture_height: info.max_texture_height as u32,
        }
    }
}

/// Manages what keeps a SDL_Renderer alive
///
/// When the `RendererContext` is dropped, it destroys the `SDL_Renderer`
pub struct RendererContext<T> {
    raw: *mut ll::SDL_Renderer,
    _target: Rc<T>,
}

impl<T> Drop for RendererContext<T> {
    fn drop(&mut self) {
        unsafe {
            ll::SDL_DestroyRenderer(self.raw);
        };
    }
}

impl<T> RendererContext<T> {
    /// Gets information about the rendering context.
    pub fn info(&self) -> RendererInfo {
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

    /// Gets the raw pointer to the SDL_Renderer
    pub fn raw(&self) -> *mut ll::SDL_Renderer {
        self.raw
    }

    pub unsafe fn from_ll(raw: *mut ll::SDL_Renderer, target: Rc<T>) -> Self {
        RendererContext {
            raw: raw,
            _target: target,
        }
    }

    unsafe fn set_raw_target(&self, raw_texture: *mut ll::SDL_Texture) -> Result<(), SdlError> {
        if ll::SDL_SetRenderTarget(self.raw, raw_texture) == 0 {
            Ok(())
        } else {
            Err(SdlError(get_error()))
        }
    }
}

impl<T: RenderTarget> Deref for Canvas<T> {
    type Target = RendererContext<T::Context>;

    fn deref(&self) -> &RendererContext<T::Context> {
        self.context.as_ref()
    }
}

/// Represents structs which can be the target of a SDL_Renderer (or Canvas).
///
/// This is intended for internal use only. It should not be used outside of this crate,
/// but is still visible for documentation reasons.
pub trait RenderTarget {
    type Context;
}

impl<'s> RenderTarget for Surface<'s> {
    type Context = SurfaceContext<'s>;
}

impl<'r, 't, TC> RenderTarget for TextureTarget<'r, 't, TC> {
    type Context = TC;
}

/// Manages and owns a target (`Surface`, `Window`, or `Texture`) and allows drawing in it.
///
/// If the `Window` manipulates the shell of the Window, `Canvas<Window>` allows you to
/// manipulate both the shell and the inside of the window;
/// you can manipulate pixel by pixel (*not recommended*), lines, colored rectangles, or paste
/// `Texture`s to this `Canvas`.
///
/// Drawing to the `Canvas` does not take effect immediately, it draws to a buffer until you
/// call `present()`, where all the operations you did until the last `present()`
/// are updated to your target
///
/// Its context may be shared with the `TextureCreator`.
///
/// The context will not be dropped until all references of it are out of scope.
///
/// # Examples
///
/// ```rust,no_run
/// # use sdl2::render::Canvas;
/// # use sdl2::video::Window;
/// # use sdl2::pixels::Color;
/// # use sdl2::rect::Rect;
/// # let sdl_context = sdl2::init().unwrap();
/// # let video_subsystem = sdl_context.video().unwrap();
/// let window = video_subsystem.window("Example", 800, 600).build().unwrap();
///
/// // Let's create a Canvas which we will use to draw in our Window
/// let mut canvas : Canvas<Window> = window.into_canvas()
///     .present_vsync() //< this means the screen cannot
///     // render faster than your display rate (usually 60Hz or 144Hz)
///     .build().unwrap();
///
/// canvas.set_draw_color(Color::RGB(0, 0, 0));
/// // fills the canvas with the color we set in `set_draw_color`.
/// canvas.clear();
///
/// // change the color of our drawing with a gold-color ...
/// canvas.set_draw_color(Color::RGB(255, 210, 0));
/// // A draw a rectangle which almost fills our window with it !
/// canvas.fill_rect(Rect::new(10, 10, 780, 580));
///
/// // However the canvas has not been updated to the window yet,
/// // everything has been processed to an internal buffer,
/// // but if we want our buffer to be displayed on the window,
/// // we need to call `present`. We need to call this everytime
/// // we want to render a new frame on the window.
/// canvas.present();
/// // present does not "clear" the buffer, that means that
/// // you have to clear it yourself before rendering again,
/// // otherwise leftovers of what you've renderer before might
/// // show up on the window !
/// //
/// // A good rule of thumb is to `clear()`, draw every texture
/// // needed, and then `present()`; repeat this every new frame.
///
/// ```
pub struct Canvas<T: RenderTarget> {
    target: T,
    context: Rc<RendererContext<T::Context>>,
}

/// Alias for a `Canvas` that was created out of a `Surface`
pub type SurfaceCanvas<'s> = Canvas<Surface<'s>>;

/// Methods for the `SurfaceCanvas`.
impl<'s> Canvas<Surface<'s>> {
    /// Creates a 2D software rendering context for a surface.
    ///
    /// This method should only fail if SDL2 is not built with rendering
    /// support, or there's an out-of-memory error.
    pub fn from_surface(surface: surface::Surface<'s>) -> Result<Self, String> {
        let raw_renderer = unsafe { ll::SDL_CreateSoftwareRenderer(surface.raw()) };
        if raw_renderer != ptr::null_mut() {
            let context =
                Rc::new(unsafe { RendererContext::from_ll(raw_renderer, surface.context()) });
            Ok(Canvas {
                   target: surface,
                   context: context,
               })
        } else {
            Err(get_error())
        }
    }

    /// Gets a reference to the associated surface of the Canvas
    #[inline]
    pub fn surface(&self) -> &SurfaceRef {
        &self.target
    }

    /// Gets a mutable reference to the associated surface of the Canvas
    #[inline]
    pub fn surface_mut(&mut self) -> &mut SurfaceRef {
        &mut self.target
    }

    /// Gets the associated surface of the Canvas and destroys the Canvas
    #[inline]
    pub fn into_surface(self) -> Surface<'s> {
        self.target
    }

    /// Sets the render target to the provided texture
    /// Returns a handle for rendering methods to the target
    /// Returns `TargetRenderError::NotSupported`
    /// if the renderer does not support the use of render targets,
    /// Returns `TargetRenderError::SdlError` if SDL2 returned with an error code.
    /// The texture must be created with the texture access: `sdl2::render::TextureAccess::Target`.
    pub fn with_target<'r, 't, 'a>
        (&'r mut self,
         texture: &'t mut Texture<'a>)
         -> Result<TextureCanvas<'r, 't, SurfaceContext<'s>>, TargetRenderError> {
        self.internal_with_target(texture)
    }

    /// Returns a `TextureCreator` that can create Textures to be drawn on this `Canvas`
    ///
    /// This `TextureCreator` will share a reference to the renderer and target context.
    ///
    /// The target (i.e., `Window`) will not be destroyed and the SDL_Renderer will not be
    /// destroyed if the `TextureCreator` is still in scope.
    pub fn texture_creator(&self) -> TextureCreator<SurfaceContext<'s>> {
        TextureCreator {
            context: self.context.clone(),
            default_pixel_format: self.surface().pixel_format_enum(),
        }
    }
}

pub type WindowCanvas = Canvas<Window>;

impl RenderTarget for Window {
    type Context = WindowContext;
}

/// Methods for the `WindowCanvas`.
impl Canvas<Window> {
    /// Gets a reference to the associated window of the Canvas
    #[inline]
    pub fn window(&self) -> &Window {
        &self.target
    }

    /// Gets a mutable reference to the associated window of the Canvas
    #[inline]
    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.target
    }

    /// Gets the associated window of the Canvas and destroys the Canvas
    #[inline]
    pub fn into_window(self) -> Window {
        self.target
    }

    /// Sets the render target to the provided texture
    /// Returns a handle for rendering methods to the target
    /// Returns `TargetRenderError::NotSupported`
    /// if the renderer does not support the use of render targets,
    /// Returns `TargetRenderError::SdlError` if SDL2 returned with an error code.
    /// The texture must be created with the texture access: `sdl2::render::TextureAccess::Target`.
    pub fn with_target<'r, 't, 'a>
        (&'r mut self,
         texture: &'t mut Texture<'a>)
         -> Result<TextureCanvas<'r, 't, WindowContext>, TargetRenderError> {
        self.internal_with_target(texture)
    }

    /// Returns a `TextureCreator` that can create Textures to be drawn on this `Canvas`
    ///
    /// This `TextureCreator` will share a reference to the renderer and target context.
    ///
    /// The target (i.e., `Window`) will not be destroyed and the SDL_Renderer will not be
    /// destroyed if the `TextureCreator` is still in scope.
    pub fn texture_creator(&self) -> TextureCreator<WindowContext> {
        TextureCreator {
            context: self.context.clone(),
            default_pixel_format: self.window().window_pixel_format(),
        }
    }
}

impl<T: RenderTarget> Canvas<T> {
    /// Determine whether a window supports the use of render targets.
    pub fn render_target_supported(&self) -> bool {
        unsafe { ll::SDL_RenderTargetSupported(self.context.raw) == 1 }
    }

    fn internal_with_target<'r, 't, 'a>
        (&'r mut self,
         texture: &'t mut Texture<'a>)
         -> Result<TextureCanvas<'r, 't, T::Context>, TargetRenderError> {
        if self.render_target_supported() {
            unsafe { self.set_raw_target(texture.raw) }
                .map_err(|e| TargetRenderError::SdlError(e))?;
            Ok(TextureCanvas {
                   context: self.context.clone(),
                   target: TextureTarget {
                       raw_renderer: &self.context.raw,
                       _texture_marker: PhantomData,
                       _texture_target: PhantomData,
                   },
               })
        } else {
            Err(TargetRenderError::NotSupported)
        }
    }
}

/// Creates Textures that cannot outlive the creator
///
/// The `TextureCreator` does not hold a lifetime to its Canvas by design choice.
/// If a `Canvas` is dropped before its `TextureCreator`, it is still safe to use.
/// It is, however, useless. Any `Texture` created here can only be drawn onto the original `Canvas`
pub struct TextureCreator<T> {
    context: Rc<RendererContext<T>>,
    default_pixel_format: PixelFormatEnum,
}

/// The type that allows you to build Window-based renderers.
///
/// By default, the renderer builder will prioritize for a hardware-accelerated
/// renderer.
pub struct CanvasBuilder {
    window: Window,
    index: Option<u32>,
    renderer_flags: u32,
}

impl CanvasBuilder {
    /// Initializes a new `CanvasBuilder`.
    pub fn new(window: Window) -> CanvasBuilder {
        CanvasBuilder {
            window: window,
            // -1 means to initialize the first rendering driver supporting the
            // renderer flags
            index: None,
            // no flags gives priority to available SDL_RENDERER_ACCELERATED
            // renderers
            renderer_flags: 0,
        }
    }

    /// Builds the renderer.
    pub fn build(self) -> Result<WindowCanvas, IntegerOrSdlError> {
        use common::IntegerOrSdlError::*;
        let index = match self.index {
            None => -1,
            Some(index) => try!(validate_int(index, "index")),
        };
        let raw = unsafe { ll::SDL_CreateRenderer(self.window.raw(), index, self.renderer_flags) };

        if raw.is_null() {
            Err(SdlError(get_error()))
        } else {
            let context = Rc::new(unsafe { RendererContext::from_ll(raw, self.window.context()) });
            Ok(Canvas {
                   context: context,
                   target: self.window,
               })
        }
    }

    /// Sets the index of the rendering driver to initialize.
    /// If you desire the first rendering driver to support the flags provided,
    /// or if you're translating code from C which passes -1 for the index,
    /// **do not** invoke the `index` method.
    pub fn index(mut self, index: u32) -> CanvasBuilder {
        self.index = Some(index);
        self
    }

    /// Set the renderer to a software fallback.
    /// This flag is accumulative, and may be specified with other flags.
    pub fn software(mut self) -> CanvasBuilder {
        self.renderer_flags |= ll::SDL_RENDERER_SOFTWARE as u32;
        self
    }

    /// Set the renderer to use hardware acceleration.
    /// This flag is accumulative, and may be specified with other flags.
    pub fn accelerated(mut self) -> CanvasBuilder {
        self.renderer_flags |= ll::SDL_RENDERER_ACCELERATED as u32;
        self
    }

    /// Synchronize renderer `present` method calls with the refresh rate.
    /// This flag is accumulative, and may be specified with other flags.
    pub fn present_vsync(mut self) -> CanvasBuilder {
        self.renderer_flags |= ll::SDL_RENDERER_PRESENTVSYNC as u32;
        self
    }

    /// Set the renderer to support rendering to a texture.
    /// This flag is accumulative, and may be specified with other flags.
    pub fn target_texture(mut self) -> CanvasBuilder {
        self.renderer_flags |= ll::SDL_RENDERER_TARGETTEXTURE as u32;
        self
    }
}

#[derive(Debug)]
pub enum TextureValueError {
    WidthOverflows(u32),
    HeightOverflows(u32),
    WidthMustBeMultipleOfTwoForFormat(u32, PixelFormatEnum),
    SdlError(String),
}

impl fmt::Display for TextureValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TextureValueError::*;

        match *self {
            WidthOverflows(value) => write!(f, "Integer width overflows ({})", value),
            HeightOverflows(value) => write!(f, "Integer height overflows ({})", value),
            WidthMustBeMultipleOfTwoForFormat(value, format) => {
                write!(f,
                       "Texture width must be multiple of two for pixel format '{:?}' ({})",
                       format,
                       value)
            }
            SdlError(ref e) => write!(f, "SDL error: {}", e),
        }
    }
}

impl Error for TextureValueError {
    fn description(&self) -> &str {
        use self::TextureValueError::*;

        match *self {
            WidthOverflows(_) => "texture width overflow",
            HeightOverflows(_) => "texture height overflow",
            WidthMustBeMultipleOfTwoForFormat(..) => "texture width must be multiple of two",
            SdlError(ref e) => e,
        }
    }
}

/// Texture-creating methods for the renderer
impl<T> TextureCreator<T> {
    pub fn raw(&self) -> *mut ll::SDL_Renderer {
        self.context.raw()
    }

    pub fn default_pixel_format(&self) -> PixelFormatEnum {
        self.default_pixel_format.clone()
    }

    /// Creates a texture for a rendering context.
    ///
    /// If format is `None`, the format will be the one the parent Window or Surface uses.
    ///
    /// If format is `Some(pixel_format)` the default will be overridden and the texture will be
    /// created with the specified format if possible. If the PixelFormat is not supported, this
    /// will return an error.
    ///
    /// You should prefer the default format if possible to have performance gains and to avoid
    /// unsupported Pixel Formats that can cause errors. However, be careful with the default
    /// `PixelFormat` if you want to create transparent textures.
    pub fn create_texture<F>(&self,
                             format: F,
                             access: TextureAccess,
                             width: u32,
                             height: u32)
                             -> Result<Texture, TextureValueError>
        where F: Into<Option<PixelFormatEnum>>
    {
        use self::TextureValueError::*;
        let w = match validate_int(width, "width") {
            Ok(w) => w,
            Err(_) => return Err(WidthOverflows(width)),
        };
        let h = match validate_int(height, "height") {
            Ok(h) => h,
            Err(_) => return Err(HeightOverflows(height)),
        };
        let format: PixelFormatEnum = format.into().unwrap_or(self.default_pixel_format);

        // If the pixel format is YUV 4:2:0 and planar, the width and height must
        // be multiples-of-two. See issue #334 for details.
        match format {
            PixelFormatEnum::YV12 |
            PixelFormatEnum::IYUV => {
                if w % 2 != 0 || h % 2 != 0 {
                    return Err(WidthMustBeMultipleOfTwoForFormat(width, format));
                }
            }
            _ => (),
        }

        let result = unsafe {
            ll::SDL_CreateTexture(self.context.raw, format as uint32_t, access as c_int, w, h)
        };
        if result == ptr::null_mut() {
            Err(SdlError(get_error()))
        } else {
            unsafe { Ok(self.raw_create_texture(result)) }
        }
    }

    /// Shorthand for `create_texture(format, TextureAccess::Static, width, height)`
    pub fn create_texture_static<F>(&self,
                                    format: F,
                                    width: u32,
                                    height: u32)
                                    -> Result<Texture, TextureValueError>
        where F: Into<Option<PixelFormatEnum>>
    {
        self.create_texture(format, TextureAccess::Static, width, height)
    }

    /// Shorthand for `create_texture(format, TextureAccess::Streaming, width, height)`
    pub fn create_texture_streaming<F>(&self,
                                       format: F,
                                       width: u32,
                                       height: u32)
                                       -> Result<Texture, TextureValueError>
        where F: Into<Option<PixelFormatEnum>>
    {
        self.create_texture(format, TextureAccess::Streaming, width, height)
    }

    /// Shorthand for `create_texture(format, TextureAccess::Target, width, height)`
    pub fn create_texture_target<F>(&self,
                                    format: F,
                                    width: u32,
                                    height: u32)
                                    -> Result<Texture, TextureValueError>
        where F: Into<Option<PixelFormatEnum>>
    {
        self.create_texture(format, TextureAccess::Target, width, height)
    }

    /// Creates a texture from an existing surface.
    /// # Remarks
    /// The access hint for the created texture is `TextureAccess::Static`.
    pub fn create_texture_from_surface<S: AsRef<SurfaceRef>>
        (&self,
         surface: S)
         -> Result<Texture, TextureValueError> {
        use self::TextureValueError::*;
        let result =
            unsafe { ll::SDL_CreateTextureFromSurface(self.context.raw, surface.as_ref().raw()) };
        if result == ptr::null_mut() {
            Err(SdlError(get_error()))
        } else {
            unsafe { Ok(self.raw_create_texture(result)) }
        }
    }

    pub unsafe fn raw_create_texture(&self, raw: *mut ll::SDL_Texture) -> Texture {
        Texture {
            raw: raw,
            _marker: PhantomData,
        }
    }
}

pub struct TextureTarget<'r, 't, TC> {
    raw_renderer: &'r *mut ll::SDL_Renderer,
    _texture_marker: PhantomData<&'t ()>,
    // unfortunately there is no way to know which kind of Renderer we have here at compile time,
    // so this PhantomData is here to keep track of that.
    _texture_target: PhantomData<TC>,
}

impl<'r, 't, TC> Drop for TextureTarget<'r, 't, TC> {
    // `Drop` cannot be specialized. Get around this through run-time check of Target Kind
    fn drop(&mut self) {
        unsafe {
            ll::SDL_SetRenderTarget(*self.raw_renderer, ptr::null_mut());
        }
    }
}

/// Drawing methods
impl<T: RenderTarget> Canvas<T> {
    pub fn raw(&self) -> *mut ll::SDL_Renderer {
        self.context.raw()
    }

    /// Sets the color used for drawing operations (Rect, Line and Clear).
    pub fn set_draw_color(&mut self, color: pixels::Color) {
        let (r, g, b, a) = color.rgba();
        let ret = unsafe { ll::SDL_SetRenderDrawColor(self.raw, r, g, b, a) };
        // Should only fail on an invalid renderer
        if ret != 0 {
            panic!(get_error())
        }
    }

    /// Gets the color used for drawing operations (Rect, Line and Clear).
    pub fn draw_color(&self) -> pixels::Color {
        let (mut r, mut g, mut b, mut a) = (0, 0, 0, 0);
        let ret =
            unsafe { ll::SDL_GetRenderDrawColor(self.context.raw, &mut r, &mut g, &mut b, &mut a) };
        // Should only fail on an invalid renderer
        if ret != 0 {
            panic!(get_error())
        } else {
            pixels::Color::RGBA(r, g, b, a)
        }
    }

    /// Sets the blend mode used for drawing operations (Fill and Line).
    pub fn set_blend_mode(&mut self, blend: BlendMode) {
        let ret = unsafe {
            ll::SDL_SetRenderDrawBlendMode(self.context.raw,
                                           FromPrimitive::from_i64(blend as i64).unwrap())
        };
        // Should only fail on an invalid renderer
        if ret != 0 {
            panic!(get_error())
        }
    }

    /// Gets the blend mode used for drawing operations.
    pub fn blend_mode(&self) -> BlendMode {
        let mut blend = 0;
        let ret = unsafe { ll::SDL_GetRenderDrawBlendMode(self.context.raw, &mut blend) };
        // Should only fail on an invalid renderer
        if ret != 0 {
            panic!(get_error())
        } else {
            FromPrimitive::from_i64(blend as i64).unwrap()
        }
    }

    /// Clears the current rendering target with the drawing color.
    pub fn clear(&mut self) {
        let ret = unsafe { ll::SDL_RenderClear(self.context.raw) };
        if ret != 0 {
            panic!("Could not clear: {}", get_error())
        }
    }

    /// Updates the screen with any rendering performed since the previous call.
    ///
    /// SDL's rendering functions operate on a backbuffer; that is, calling a
    /// rendering function such as `draw_line()` does not directly put a line on
    /// the screen, but rather updates the backbuffer.
    /// As such, you compose your entire scene and present the composed
    /// backbuffer to the screen as a complete picture.
    pub fn present(&mut self) {
        unsafe { ll::SDL_RenderPresent(self.context.raw) }
    }

    /// Gets the output size of a rendering context.
    pub fn output_size(&self) -> Result<(u32, u32), String> {
        let mut width = 0;
        let mut height = 0;

        let result =
            unsafe { ll::SDL_GetRendererOutputSize(self.context.raw, &mut width, &mut height) };

        if result == 0 {
            Ok((width as u32, height as u32))
        } else {
            Err(get_error())
        }
    }

    /// Sets a device independent resolution for rendering.
    pub fn set_logical_size(&mut self, width: u32, height: u32) -> Result<(), IntegerOrSdlError> {
        use common::IntegerOrSdlError::*;
        let width = try!(validate_int(width, "width"));
        let height = try!(validate_int(height, "height"));
        let result = unsafe { ll::SDL_RenderSetLogicalSize(self.context.raw, width, height) };
        match result {
            0 => Ok(()),
            _ => Err(SdlError(get_error())),
        }
    }

    /// Gets device independent resolution for rendering.
    pub fn logical_size(&self) -> (u32, u32) {
        let mut width = 0;
        let mut height = 0;

        unsafe { ll::SDL_RenderGetLogicalSize(self.context.raw, &mut width, &mut height) };

        (width as u32, height as u32)
    }

    /// Sets the drawing area for rendering on the current target.
    pub fn set_viewport<R: Into<Option<Rect>>>(&mut self, rect: R) {
        let ptr = match rect.into() {
            Some(ref rect) => rect.raw(),
            None => ptr::null(),
        };
        let ret = unsafe { ll::SDL_RenderSetViewport(self.context.raw, ptr) };
        if ret != 0 {
            panic!("Could not set viewport: {}", get_error())
        }
    }

    /// Gets the drawing area for the current target.
    pub fn viewport(&self) -> Rect {
        let mut rect = unsafe { mem::uninitialized() };
        unsafe { ll::SDL_RenderGetViewport(self.context.raw, &mut rect) };
        Rect::from_ll(rect)
    }

    /// Sets the clip rectangle for rendering on the specified target.
    ///
    /// If the rectangle is `None`, clipping will be disabled.
    pub fn set_clip_rect<R: Into<Option<Rect>>>(&mut self, rect: R) {
        let ret = unsafe {
            ll::SDL_RenderSetClipRect(self.context.raw,
                                      match rect.into() {
                                          Some(ref rect) => rect.raw(),
                                          None => ptr::null(),
                                      })
        };
        if ret != 0 {
            panic!("Could not set clip rect: {}", get_error())
        }
    }

    /// Gets the clip rectangle for the current target.
    ///
    /// Returns `None` if clipping is disabled.
    pub fn clip_rect(&self) -> Option<Rect> {
        let mut raw = unsafe { mem::uninitialized() };
        unsafe { ll::SDL_RenderGetClipRect(self.context.raw, &mut raw) };
        if raw.w == 0 || raw.h == 0 {
            None
        } else {
            Some(Rect::from_ll(raw))
        }
    }

    /// Sets the drawing scale for rendering on the current target.
    pub fn set_scale(&mut self, scale_x: f32, scale_y: f32) -> Result<(), String> {
        let ret = unsafe { ll::SDL_RenderSetScale(self.context.raw, scale_x, scale_y) };
        // Should only fail on an invalid renderer
        if ret != 0 { Err(get_error()) } else { Ok(()) }
    }

    /// Gets the drawing scale for the current target.
    pub fn scale(&self) -> (f32, f32) {
        let mut scale_x = 0.0;
        let mut scale_y = 0.0;
        unsafe { ll::SDL_RenderGetScale(self.context.raw, &mut scale_x, &mut scale_y) };
        (scale_x, scale_y)
    }

    /// Draws a point on the current rendering target.
    /// Errors if drawing fails for any reason (e.g. driver failure)
    pub fn draw_point<P: Into<Point>>(&mut self, point: P) -> Result<(), String> {
        let point = point.into();
        let result = unsafe { ll::SDL_RenderDrawPoint(self.context.raw, point.x(), point.y()) };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Draws multiple points on the current rendering target.
    /// Errors if drawing fails for any reason (e.g. driver failure)
    pub fn draw_points<'a, P: Into<&'a [Point]>>(&mut self, points: P) -> Result<(), String> {
        let points = points.into();
        let result = unsafe {
            ll::SDL_RenderDrawPoints(self.context.raw,
                                     Point::raw_slice(points),
                                     points.len() as c_int)
        };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Draws a line on the current rendering target.
    /// Errors if drawing fails for any reason (e.g. driver failure)
    pub fn draw_line<P1: Into<Point>, P2: Into<Point>>(&mut self,
                                                       start: P1,
                                                       end: P2)
                                                       -> Result<(), String> {
        let start = start.into();
        let end = end.into();
        let result = unsafe {
            ll::SDL_RenderDrawLine(self.context.raw, start.x(), start.y(), end.x(), end.y())
        };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Draws a series of connected lines on the current rendering target.
    /// Errors if drawing fails for any reason (e.g. driver failure)
    pub fn draw_lines<'a, P: Into<&'a [Point]>>(&mut self, points: P) -> Result<(), String> {
        let points = points.into();
        let result = unsafe {
            ll::SDL_RenderDrawLines(self.context.raw,
                                    Point::raw_slice(points),
                                    points.len() as c_int)
        };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Draws a rectangle on the current rendering target.
    /// Errors if drawing fails for any reason (e.g. driver failure)
    pub fn draw_rect(&mut self, rect: Rect) -> Result<(), String> {
        let result = unsafe { ll::SDL_RenderDrawRect(self.context.raw, rect.raw()) };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Draws some number of rectangles on the current rendering target.
    /// Errors if drawing fails for any reason (e.g. driver failure)
    pub fn draw_rects(&mut self, rects: &[Rect]) -> Result<(), String> {
        let result = unsafe {
            ll::SDL_RenderDrawRects(self.context.raw,
                                    Rect::raw_slice(rects),
                                    rects.len() as c_int)
        };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Fills a rectangle on the current rendering target with the drawing
    /// color.
    /// Passing None will fill the entire rendering target.
    /// Errors if drawing fails for any reason (e.g. driver failure)
    pub fn fill_rect<R: Into<Option<Rect>>>(&mut self, rect: R) -> Result<(), String> {
        let result = unsafe {
            ll::SDL_RenderFillRect(self.context.raw,
                                   rect.into()
                                       .as_ref()
                                       .map(|r| r.raw())
                                       .unwrap_or(ptr::null()))
        };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Fills some number of rectangles on the current rendering target with
    /// the drawing color.
    /// Errors if drawing fails for any reason (e.g. driver failure)
    pub fn fill_rects(&mut self, rects: &[Rect]) -> Result<(), String> {
        let result = unsafe {
            ll::SDL_RenderFillRects(self.context.raw,
                                    Rect::raw_slice(rects),
                                    rects.len() as c_int)
        };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Copies a portion of the texture to the current rendering target.
    ///
    /// * If `src` is `None`, the entire texture is copied.
    /// * If `dst` is `None`, the texture will be stretched to fill the given
    ///   rectangle.
    ///
    /// Errors if drawing fails for any reason (e.g. driver failure),
    /// or if the provided texture does not belong to the renderer.
    pub fn copy<R1, R2>(&mut self, texture: &Texture, src: R1, dst: R2) -> Result<(), String>
        where R1: Into<Option<Rect>>,
              R2: Into<Option<Rect>>
    {
        let ret = unsafe {
            ll::SDL_RenderCopy(self.context.raw,
                               texture.raw,
                               match src.into() {
                                   Some(ref rect) => rect.raw(),
                                   None => ptr::null(),
                               },
                               match dst.into() {
                                   Some(ref rect) => rect.raw(),
                                   None => ptr::null(),
                               })
        };

        if ret != 0 { Err(get_error()) } else { Ok(()) }
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
    /// Errors if drawing fails for any reason (e.g. driver failure),
    /// if the provided texture does not belong to the renderer,
    /// or if the driver does not support RenderCopyEx.
    pub fn copy_ex<R1, R2, P>(&mut self,
                              texture: &Texture,
                              src: R1,
                              dst: R2,
                              angle: f64,
                              center: P,
                              flip_horizontal: bool,
                              flip_vertical: bool)
                              -> Result<(), String>
        where R1: Into<Option<Rect>>,
              R2: Into<Option<Rect>>,
              P: Into<Option<Point>>
    {
        let flip = match (flip_horizontal, flip_vertical) {
            (false, false) => ll::SDL_FLIP_NONE,
            (true, false) => ll::SDL_FLIP_HORIZONTAL,
            (false, true) => ll::SDL_FLIP_VERTICAL,
            (true, true) => ll::SDL_FLIP_HORIZONTAL | ll::SDL_FLIP_VERTICAL,
        };

        let ret = unsafe {
            ll::SDL_RenderCopyEx(self.context.raw,
                                 texture.raw,
                                 match src.into() {
                                     Some(ref rect) => rect.raw(),
                                     None => ptr::null(),
                                 },
                                 match dst.into() {
                                     Some(ref rect) => rect.raw(),
                                     None => ptr::null(),
                                 },
                                 angle as c_double,
                                 match center.into() {
                                     Some(ref point) => point.raw(),
                                     None => ptr::null(),
                                 },
                                 flip)
        };

        if ret != 0 { Err(get_error()) } else { Ok(()) }
    }

    /// Reads pixels from the current rendering target.
    /// # Remarks
    /// WARNING: This is a very slow operation, and should not be used frequently.
    pub fn read_pixels<R: Into<Option<Rect>>>(&self,
                                              rect: R,
                                              format: pixels::PixelFormatEnum)
                                              -> Result<Vec<u8>, String> {
        unsafe {
            let rect = rect.into();
            let (actual_rect, w, h) = match rect {
                Some(ref rect) => (rect.raw(), rect.width() as usize, rect.height() as usize),
                None => {
                    let (w, h) = try!(self.output_size());
                    (ptr::null(), w as usize, h as usize)
                }
            };

            let pitch = w * format.byte_size_per_pixel(); // calculated pitch
            let size = format.byte_size_of_pixels(w * h);
            let mut pixels = Vec::with_capacity(size);
            pixels.set_len(size);

            // Pass the interior of `pixels: Vec<u8>` to SDL
            let ret = {
                ll::SDL_RenderReadPixels(self.context.raw,
                                         actual_rect,
                                         format as uint32_t,
                                         pixels.as_mut_ptr() as *mut c_void,
                                         pitch as c_int)
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
/// use sdl2::video::WindowContext;
/// use sdl2::render::{Texture, TextureCreator, WindowCanvas};
///
/// // Draw a red rectangle to a new texture
/// fn draw_to_texture<'c>(t: &'c TextureCreator<WindowContext>, c: &mut WindowCanvas)
///                        -> Texture<'c> {
///     let mut texture = t.create_texture_target(PixelFormatEnum::RGBA8888, 512, 512)
///                       .unwrap();
///     {
///         let mut target = c.with_target(&mut texture)
///                           .expect("This platform doesn't support render targets");
///
///         // Start drawing
///         target.clear();
///         target.set_draw_color(Color::RGB(255, 0, 0));
///         target.fill_rect(Rect::new(100, 100, 256, 256));
///    }
///    texture
/// }
/// ```
pub type TextureCanvas<'r, 't, TC> = Canvas<TextureTarget<'r, 't, TC>>;

impl<'r, 't, TC> Canvas<TextureTarget<'r, 't, TC>> {
    /// Replace the target of the `TextureCanvas` with a different `Texture`
    ///
    /// Returns the new `TextureCanvas` and releases the `&mut` borrow on the old `Texture`
    pub fn with_target<'nt, 'a>(mut self,
                                texture: &'nt mut Texture<'a>)
                                -> Result<TextureCanvas<'r, 'nt, TC>, SdlError> {
        unsafe { self.set_raw_target(texture.raw) }?;
        let context = mem::replace(&mut self.context, unsafe { mem::zeroed() });
        let raw_renderer = mem::replace(&mut self.target.raw_renderer, unsafe { mem::zeroed() });
        mem::forget(self);
        Ok(TextureCanvas {
               context: context,
               target: TextureTarget {
                   raw_renderer: raw_renderer,
                   _texture_marker: PhantomData,
                   _texture_target: PhantomData,
               },
           })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TextureQuery {
    pub format: pixels::PixelFormatEnum,
    pub access: TextureAccess,
    pub width: u32,
    pub height: u32,
}

/// A texture for a rendering context.
///
/// Every Texture is owned by a `TextureCreator`.
/// A `Texture` cannot outlive the `TextureCreator`
///
/// A `Texture` can be safely accessed after the `Canvas` is dropped.
pub struct Texture<'r> {
    raw: *mut ll::SDL_Texture,
    _marker: PhantomData<&'r ()>,
}

impl<'r> Drop for Texture<'r> {
    fn drop(&mut self) {
        unsafe {
            ll::SDL_DestroyTexture(self.raw);
        }
    }
}

#[derive(Debug)]
pub enum UpdateTextureError {
    PitchOverflows(usize),
    PitchMustBeMultipleOfTwoForFormat(usize, PixelFormatEnum),
    XMustBeMultipleOfTwoForFormat(i32, PixelFormatEnum),
    YMustBeMultipleOfTwoForFormat(i32, PixelFormatEnum),
    WidthMustBeMultipleOfTwoForFormat(u32, PixelFormatEnum),
    HeightMustBeMultipleOfTwoForFormat(u32, PixelFormatEnum),
    SdlError(String),
}

impl fmt::Display for UpdateTextureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::UpdateTextureError::*;

        match *self {
            PitchOverflows(value) => write!(f, "Pitch overflows ({})", value),
            PitchMustBeMultipleOfTwoForFormat(value, format) => {
                write!(f,
                       "Pitch must be multiple of two for pixel format '{:?}' ({})",
                       format,
                       value)
            }
            XMustBeMultipleOfTwoForFormat(value, format) => {
                write!(f,
                       "X must be multiple of two for pixel format '{:?}' ({})",
                       format,
                       value)
            }
            YMustBeMultipleOfTwoForFormat(value, format) => {
                write!(f,
                       "Y must be multiple of two for pixel format '{:?}' ({})",
                       format,
                       value)
            }
            WidthMustBeMultipleOfTwoForFormat(value, format) => {
                write!(f,
                       "Width must be multiple of two for pixel format '{:?}' ({})",
                       format,
                       value)
            }
            HeightMustBeMultipleOfTwoForFormat(value, format) => {
                write!(f,
                       "Height must be multiple of two for pixel format '{:?}' ({})",
                       format,
                       value)
            }
            SdlError(ref e) => write!(f, "SDL error: {}", e),
        }
    }
}

impl Error for UpdateTextureError {
    fn description(&self) -> &str {
        use self::UpdateTextureError::*;

        match *self {
            PitchOverflows(_) => "pitch overflow",
            PitchMustBeMultipleOfTwoForFormat(..) => "pitch must be multiple of two",
            XMustBeMultipleOfTwoForFormat(..) => "x must be multiple of two",
            YMustBeMultipleOfTwoForFormat(..) => "y must be multiple of two",
            WidthMustBeMultipleOfTwoForFormat(..) => "width must be multiple of two",
            HeightMustBeMultipleOfTwoForFormat(..) => "height must be multiple of two",
            SdlError(ref e) => e,
        }
    }
}

#[derive(Debug)]
pub enum UpdateTextureYUVError {
    PitchOverflows { plane: &'static str, value: usize },
    InvalidPlaneLength {
        plane: &'static str,
        length: usize,
        pitch: usize,
        height: usize,
    },
    XMustBeMultipleOfTwoForFormat(i32),
    YMustBeMultipleOfTwoForFormat(i32),
    WidthMustBeMultipleOfTwoForFormat(u32),
    HeightMustBeMultipleOfTwoForFormat(u32),
    RectNotInsideTexture(Rect),
    SdlError(String),
}

impl fmt::Display for UpdateTextureYUVError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::UpdateTextureYUVError::*;

        match *self {
            PitchOverflows { plane, value } => {
                write!(f, "Pitch overflows on {} plane ({})", plane, value)
            }
            InvalidPlaneLength {
                plane,
                length,
                pitch,
                height,
            } => {
                write!(f,
                       "The {} plane is wrong length ({}, should be {} * {})",
                       plane,
                       length,
                       pitch,
                       height)
            }
            XMustBeMultipleOfTwoForFormat(value) => {
                write!(f, "X must be multiple of two ({})", value)
            }
            YMustBeMultipleOfTwoForFormat(value) => {
                write!(f, "Y must be multiple of two ({})", value)
            }
            WidthMustBeMultipleOfTwoForFormat(value) => {
                write!(f, "Width must be multiple of two ({})", value)
            }
            HeightMustBeMultipleOfTwoForFormat(value) => {
                write!(f, "Height must be multiple of two ({})", value)
            }
            RectNotInsideTexture(_) => write!(f, "Rect must be inside texture"),
            SdlError(ref e) => write!(f, "SDL error: {}", e),
        }
    }
}

impl Error for UpdateTextureYUVError {
    fn description(&self) -> &str {
        use self::UpdateTextureYUVError::*;

        match *self {
            PitchOverflows { .. } => "pitch overflow",
            InvalidPlaneLength { .. } => "invalid plane length",
            XMustBeMultipleOfTwoForFormat(_) => "x must be multiple of two",
            YMustBeMultipleOfTwoForFormat(_) => "y must be multiple of two",
            WidthMustBeMultipleOfTwoForFormat(_) => "width must be multiple of two",
            HeightMustBeMultipleOfTwoForFormat(_) => "height must be multiple of two",
            RectNotInsideTexture(_) => "rect must be inside texture",
            SdlError(ref e) => e,
        }
    }
}

impl<'r> Texture<'r> {
    /// Queries the attributes of the texture.
    pub fn query(&self) -> TextureQuery {
        let mut format = 0;
        let mut access = 0;
        let mut width = 0;
        let mut height = 0;

        let ret = unsafe {
            ll::SDL_QueryTexture(self.raw, &mut format, &mut access, &mut width, &mut height)
        };
        // Should only fail on an invalid texture
        if ret != 0 {
            panic!(get_error())
        } else {
            TextureQuery {
                format: FromPrimitive::from_i64(format as i64).unwrap(),
                access: FromPrimitive::from_i64(access as i64).unwrap(),
                width: width as u32,
                height: height as u32,
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
    pub fn color_mod(&self) -> (u8, u8, u8) {
        let (mut r, mut g, mut b) = (0, 0, 0);
        let ret = unsafe { ll::SDL_GetTextureColorMod(self.raw, &mut r, &mut g, &mut b) };

        // Should only fail on an invalid texture
        if ret != 0 {
            panic!(get_error())
        } else {
            (r, g, b)
        }
    }

    /// Sets an additional alpha value multiplied into render copy operations.
    pub fn set_alpha_mod(&mut self, alpha: u8) {
        let ret = unsafe { ll::SDL_SetTextureAlphaMod(self.raw, alpha) };

        if ret != 0 {
            panic!("Error setting alpha mod: {}", get_error())
        }
    }

    /// Gets the additional alpha value multiplied into render copy operations.
    pub fn alpha_mod(&self) -> u8 {
        let mut alpha = 0;
        let ret = unsafe { ll::SDL_GetTextureAlphaMod(self.raw, &mut alpha) };

        // Should only fail on an invalid texture
        if ret != 0 { panic!(get_error()) } else { alpha }
    }

    /// Sets the blend mode for a texture, used by `Renderer::copy()`.
    pub fn set_blend_mode(&mut self, blend: BlendMode) {
        let ret = unsafe {
            ll::SDL_SetTextureBlendMode(self.raw, FromPrimitive::from_i64(blend as i64).unwrap())
        };

        if ret != 0 {
            panic!("Error setting blend: {}", get_error())
        }
    }

    /// Gets the blend mode used for texture copy operations.
    pub fn blend_mode(&self) -> BlendMode {
        let mut blend = 0;
        let ret = unsafe { ll::SDL_GetTextureBlendMode(self.raw, &mut blend) };

        // Should only fail on an invalid texture
        if ret != 0 {
            panic!(get_error())
        } else {
            FromPrimitive::from_i64(blend as i64).unwrap()
        }
    }

    /// Updates the given texture rectangle with new pixel data.
    ///
    /// `pitch` is the number of bytes in a row of pixel data, including padding
    /// between lines
    ///
    /// * If `rect` is `None`, the entire texture is updated.
    pub fn update<R>(&mut self,
                     rect: R,
                     pixel_data: &[u8],
                     pitch: usize)
                     -> Result<(), UpdateTextureError>
        where R: Into<Option<Rect>>
    {
        use self::UpdateTextureError::*;
        let rect = rect.into();
        let rect_raw_ptr = match rect {
            Some(ref rect) => rect.raw(),
            None => ptr::null(),
        };

        // Check if the rectangle's position or size is odd, and if the pitch is odd.
        // This needs to be done in case the texture's pixel format is planar YUV.
        // See issue #334 for details.
        let TextureQuery { format, .. } = self.query();
        match format {
            PixelFormatEnum::YV12 |
            PixelFormatEnum::IYUV => {
                match rect {
                    Some(r) => {
                        if r.x() % 2 != 0 {
                            return Err(XMustBeMultipleOfTwoForFormat(r.x(), format));
                        } else if r.y() % 2 != 0 {
                            return Err(YMustBeMultipleOfTwoForFormat(r.y(), format));
                        } else if r.width() % 2 != 0 {
                            return Err(WidthMustBeMultipleOfTwoForFormat(r.width(), format));
                        } else if r.height() % 2 != 0 {
                            return Err(HeightMustBeMultipleOfTwoForFormat(r.height(), format));
                        }
                    }
                    _ => {}
                };
                if pitch % 2 != 0 {
                    return Err(PitchMustBeMultipleOfTwoForFormat(pitch, format));
                }
            }
            _ => {}
        }

        let pitch = match validate_int(pitch as u32, "pitch") {
            Ok(p) => p,
            Err(_) => return Err(PitchOverflows(pitch)),
        };

        let result = unsafe {
            ll::SDL_UpdateTexture(self.raw,
                                  rect_raw_ptr,
                                  pixel_data.as_ptr() as *const _,
                                  pitch)
        };

        if result != 0 {
            Err(SdlError(get_error()))
        } else {
            Ok(())
        }
    }

    /// Updates a rectangle within a planar YV12 or IYUV texture with new pixel data.
    pub fn update_yuv<R>(&mut self,
                         rect: R,
                         y_plane: &[u8],
                         y_pitch: usize,
                         u_plane: &[u8],
                         u_pitch: usize,
                         v_plane: &[u8],
                         v_pitch: usize)
                         -> Result<(), UpdateTextureYUVError>
        where R: Into<Option<Rect>>
    {
        use self::UpdateTextureYUVError::*;

        let rect = rect.into();

        let rect_raw_ptr = match rect {
            Some(ref rect) => rect.raw(),
            None => ptr::null(),
        };

        match rect {
            Some(ref r) => {
                if r.x() % 2 != 0 {
                    return Err(XMustBeMultipleOfTwoForFormat(r.x()));
                } else if r.y() % 2 != 0 {
                    return Err(YMustBeMultipleOfTwoForFormat(r.y()));
                } else if r.width() % 2 != 0 {
                    return Err(WidthMustBeMultipleOfTwoForFormat(r.width()));
                } else if r.height() % 2 != 0 {
                    return Err(HeightMustBeMultipleOfTwoForFormat(r.height()));
                }
            }
            _ => {}
        };

        // If the destination rectangle lies outside the texture boundaries,
        // SDL_UpdateYUVTexture will write outside allocated texture memory.
        let tex_info = self.query();
        if let Some(ref r) = rect {
            let tex_rect = Rect::new(0, 0, tex_info.width, tex_info.height);
            let inside = match r.intersection(tex_rect) {
                Some(intersection) => intersection == *r,
                None => false,
            };
            // The destination rectangle cannot lie outside the texture boundaries
            if !inside {
                return Err(RectNotInsideTexture(r.clone()));
            }
        }

        // We need the height in order to check the array slice lengths.
        // Checking the lengths can prevent buffer overruns in SDL_UpdateYUVTexture.
        let height = match rect {
            Some(ref r) => r.height(),
            None => tex_info.height,
        } as usize;

        //let wrong_length =
        if y_plane.len() != (y_pitch * height) {
            return Err(InvalidPlaneLength {
                           plane: "y",
                           length: y_plane.len(),
                           pitch: y_pitch,
                           height: height,
                       });
        }
        if u_plane.len() != (u_pitch * height / 2) {
            return Err(InvalidPlaneLength {
                           plane: "u",
                           length: u_plane.len(),
                           pitch: u_pitch,
                           height: height / 2,
                       });
        }
        if v_plane.len() != (v_pitch * height / 2) {
            return Err(InvalidPlaneLength {
                           plane: "v",
                           length: v_plane.len(),
                           pitch: v_pitch,
                           height: height / 2,
                       });
        }

        let y_pitch = match validate_int(y_pitch as u32, "y_pitch") {
            Ok(p) => p,
            Err(_) => {
                return Err(PitchOverflows {
                               plane: "y",
                               value: y_pitch,
                           })
            }
        };
        let u_pitch = match validate_int(u_pitch as u32, "u_pitch") {
            Ok(p) => p,
            Err(_) => {
                return Err(PitchOverflows {
                               plane: "u",
                               value: u_pitch,
                           })
            }
        };
        let v_pitch = match validate_int(v_pitch as u32, "v_pitch") {
            Ok(p) => p,
            Err(_) => {
                return Err(PitchOverflows {
                               plane: "v",
                               value: v_pitch,
                           })
            }
        };

        let result = unsafe {
            ll::SDL_UpdateYUVTexture(self.raw,
                                     rect_raw_ptr,
                                     y_plane.as_ptr(),
                                     y_pitch,
                                     u_plane.as_ptr(),
                                     u_pitch,
                                     v_plane.as_ptr(),
                                     v_pitch)
        };
        if result != 0 {
            Err(SdlError(get_error()))
        } else {
            Ok(())
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
    pub fn with_lock<F, R, R2>(&mut self, rect: R2, func: F) -> Result<R, String>
        where F: FnOnce(&mut [u8], usize) -> R,
              R2: Into<Option<Rect>>
    {
        // Call to SDL to populate pixel data
        let loaded = unsafe {
            let q = self.query();
            let mut pixels = ptr::null_mut();
            let mut pitch = 0;

            let (rect_raw_ptr, height) = match rect.into() {
                Some(ref rect) => (rect.raw(), rect.height() as usize),
                None => (ptr::null(), q.height as usize),
            };

            let ret = ll::SDL_LockTexture(self.raw, rect_raw_ptr, &mut pixels, &mut pitch);
            if ret == 0 {
                let size = q.format
                    .byte_size_from_pitch_and_height(pitch as usize, height);
                Ok((::std::slice::from_raw_parts_mut(pixels as *mut u8, size), pitch))
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
        if ll::SDL_GL_UnbindTexture(self.raw) != 0 {
            panic!("OpenGL texture unbinding not supported");
        }
    }

    /// Binds and unbinds an OpenGL/ES/ES2 texture from the current context.
    pub fn gl_with_bind<R, F: FnOnce(f32, f32) -> R>(&mut self, f: F) -> R {
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

    pub fn raw(&self) -> *mut ll::SDL_Texture {
        self.raw
    }
}

#[derive(Copy, Clone)]
pub struct DriverIterator {
    length: i32,
    index: i32,
}

impl Iterator for DriverIterator {
    type Item = RendererInfo;

    #[inline]
    fn next(&mut self) -> Option<RendererInfo> {
        if self.index >= self.length {
            None
        } else {
            let mut out = unsafe { mem::uninitialized() };
            let result = unsafe { ll::SDL_GetRenderDriverInfo(self.index, &mut out) == 0 };
            assert!(result, 0);
            self.index += 1;

            unsafe { Some(RendererInfo::from_ll(&out)) }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let l = self.length as usize;
        (l, Some(l))
    }
}

impl ExactSizeIterator for DriverIterator {}

/// Gets an iterator of all render drivers compiled into the SDL2 library.
#[inline]
pub fn drivers() -> DriverIterator {
    // This function is thread-safe and doesn't require the video subsystem to be initialized.
    // The list of drivers are read-only and statically compiled into SDL2, varying by platform.

    // SDL_GetNumRenderDrivers can never return a negative value.
    DriverIterator {
        length: unsafe { ll::SDL_GetNumRenderDrivers() },
        index: 0,
    }
}
