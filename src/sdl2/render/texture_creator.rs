use crate::common::validate_int;
use crate::get_error;
use crate::pixels::PixelFormatEnum;
use crate::surface::SurfaceRef;
use libc::c_int;
#[cfg(not(feature = "unsafe_textures"))]
use std::marker::PhantomData;
use std::rc::Rc;

use super::{RendererContext, Texture, TextureAccess};

mod error;
pub use self::error::TextureValueError;

/// Creates Textures that cannot outlive the creator
///
/// The `TextureCreator` does not hold a lifetime to its Canvas by design choice.
///
/// If a `Canvas` is dropped before its `TextureCreator`, it is still safe to use.
///
/// It is, however, useless.
///
/// Any `Texture` created here can only be drawn onto the original `Canvas`. A `Texture` used in a
/// `Canvas` must come from a `TextureCreator` coming from that same `Canvas`. Using a `Texture` to
/// render to a `Canvas` not being the parent of the `Texture`'s `TextureCreator` is undefined
/// behavior.
pub struct TextureCreator<T> {
    context: Rc<RendererContext<T>>,
    default_pixel_format: PixelFormatEnum,
}
/// Texture-creating methods for the renderer
impl<T> TextureCreator<T> {
    pub(super) fn new(
        context: Rc<RendererContext<T>>,
        default_pixel_format: PixelFormatEnum,
    ) -> Self {
        Self {
            context,
            default_pixel_format,
        }
    }

    // this can prevent introducing UB until
    // https://github.com/rust-lang/rust-clippy/issues/5953 is fixed
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn raw(&self) -> *mut sys::SDL_Renderer {
        self.context.raw()
    }

    pub fn default_pixel_format(&self) -> PixelFormatEnum {
        self.default_pixel_format
    }

    /// Creates a texture for a rendering context.
    ///
    /// If format is `None`, the format will be the one the parent Window or Surface uses.
    ///
    /// If format is `Some(pixel_format)`, the default will be overridden, and the texture will be
    /// created with the specified format if possible. If the PixelFormat is not supported, this
    /// will return an error.
    ///
    /// You should prefer the default format if possible to have performance gains and to avoid
    /// unsupported Pixel Formats that can cause errors. However, be careful with the default
    /// `PixelFormat` if you want to create transparent textures.
    pub fn create_texture<F>(
        &self,
        format: F,
        access: TextureAccess,
        width: u32,
        height: u32,
    ) -> Result<Texture, TextureValueError>
    where
        F: Into<Option<PixelFormatEnum>>,
    {
        use self::TextureValueError::*;
        let format: PixelFormatEnum = format.into().unwrap_or(self.default_pixel_format);
        let result = ll_create_texture(self.context.raw(), format, access, width, height)?;
        if result.is_null() {
            Err(SdlError(get_error()))
        } else {
            unsafe { Ok(self.raw_create_texture(result)) }
        }
    }

    #[inline]
    /// Shorthand for `create_texture(format, TextureAccess::Static, width, height)`
    pub fn create_texture_static<F>(
        &self,
        format: F,
        width: u32,
        height: u32,
    ) -> Result<Texture, TextureValueError>
    where
        F: Into<Option<PixelFormatEnum>>,
    {
        self.create_texture(format, TextureAccess::Static, width, height)
    }

    #[inline]
    /// Shorthand for `create_texture(format, TextureAccess::Streaming, width, height)`
    pub fn create_texture_streaming<F>(
        &self,
        format: F,
        width: u32,
        height: u32,
    ) -> Result<Texture, TextureValueError>
    where
        F: Into<Option<PixelFormatEnum>>,
    {
        self.create_texture(format, TextureAccess::Streaming, width, height)
    }

    #[inline]
    /// Shorthand for `create_texture(format, TextureAccess::Target, width, height)`
    pub fn create_texture_target<F>(
        &self,
        format: F,
        width: u32,
        height: u32,
    ) -> Result<Texture, TextureValueError>
    where
        F: Into<Option<PixelFormatEnum>>,
    {
        self.create_texture(format, TextureAccess::Target, width, height)
    }

    /// Creates a texture from an existing surface.
    ///
    /// # Remarks
    ///
    /// The access hint for the created texture is [`TextureAccess::Static`].
    ///
    /// ```no_run
    /// use sdl2::pixels::PixelFormatEnum;
    /// use sdl2::surface::Surface;
    /// use sdl2::render::{Canvas, Texture};
    /// use sdl2::video::Window;
    ///
    /// // We init systems.
    /// let sdl_context = sdl2::init().expect("failed to init SDL");
    /// let video_subsystem = sdl_context.video().expect("failed to get video context");
    ///
    /// // We create a window.
    /// let window = video_subsystem.window("sdl2 demo", 800, 600)
    ///     .build()
    ///     .expect("failed to build window");
    ///
    /// // We get the canvas from which we can get the `TextureCreator`.
    /// let mut canvas: Canvas<Window> = window.into_canvas()
    ///     .build()
    ///     .expect("failed to build window's canvas");
    /// let texture_creator = canvas.texture_creator();
    ///
    /// let surface = Surface::new(512, 512, PixelFormatEnum::RGB24).unwrap();
    /// let texture = texture_creator.create_texture_from_surface(surface).unwrap();
    /// ```
    #[doc(alias = "SDL_CreateTextureFromSurface")]
    pub fn create_texture_from_surface<S: AsRef<SurfaceRef>>(
        &self,
        surface: S,
    ) -> Result<Texture, TextureValueError> {
        use self::TextureValueError::*;
        let result = unsafe {
            sys::SDL_CreateTextureFromSurface(self.context.raw(), surface.as_ref().raw())
        };
        if result.is_null() {
            Err(SdlError(get_error()))
        } else {
            unsafe { Ok(self.raw_create_texture(result)) }
        }
    }

    /// Create a texture from its raw `SDL_Texture`.
    #[cfg(not(feature = "unsafe_textures"))]
    #[inline]
    pub const unsafe fn raw_create_texture(&self, raw: *mut sys::SDL_Texture) -> Texture {
        Texture {
            raw,
            _marker: PhantomData,
        }
    }

    /// Create a texture from its raw `SDL_Texture`. Should be used with care.
    #[cfg(feature = "unsafe_textures")]
    pub const unsafe fn raw_create_texture(&self, raw: *mut sys::SDL_Texture) -> Texture {
        Texture { raw }
    }
}

#[doc(alias = "SDL_CreateTexture")]
fn ll_create_texture(
    context: *mut sys::SDL_Renderer,
    pixel_format: PixelFormatEnum,
    access: TextureAccess,
    width: u32,
    height: u32,
) -> Result<*mut sys::SDL_Texture, TextureValueError> {
    use self::TextureValueError::*;
    let w = match validate_int(width, "width") {
        Ok(w) => w,
        Err(_) => return Err(WidthOverflows(width)),
    };
    let h = match validate_int(height, "height") {
        Ok(h) => h,
        Err(_) => return Err(HeightOverflows(height)),
    };

    // If the pixel format is YUV 4:2:0 and planar, the width and height must
    // be multiples-of-two. See issue #334 for details.
    match pixel_format {
        PixelFormatEnum::YV12 | PixelFormatEnum::IYUV => {
            if w % 2 != 0 || h % 2 != 0 {
                return Err(WidthMustBeMultipleOfTwoForFormat(width, pixel_format));
            }
        }
        _ => (),
    };

    Ok(unsafe { sys::SDL_CreateTexture(context, pixel_format as u32, access as c_int, w, h) })
}
