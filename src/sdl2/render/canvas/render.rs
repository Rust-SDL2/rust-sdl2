use crate::common::{validate_int, IntegerOrSdlError};
use crate::get_error;
use crate::pixels;
use crate::rect::Point;
use crate::rect::Rect;
use crate::sys::SDL_BlendMode;

use libc::c_void;
use libc::{c_double, c_int};
use std::mem;
use std::mem::{transmute, MaybeUninit};
use std::ptr;

use super::super::{BlendMode, RenderTarget, Texture};
use super::Canvas;

/// Drawing methods
impl<T: RenderTarget> Canvas<T> {
    // this can prevent introducing UB until
    // https://github.com/rust-lang/rust-clippy/issues/5953 is fixed
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn raw(&self) -> *mut sys::SDL_Renderer {
        self.context.raw()
    }

    /// Sets the color used for drawing operations (Rect, Line and Clear).
    #[doc(alias = "SDL_SetRenderDrawColor")]
    pub fn set_draw_color<C: Into<pixels::Color>>(&mut self, color: C) {
        let (r, g, b, a) = color.into().rgba();
        let ret = unsafe { sys::SDL_SetRenderDrawColor(self.raw(), r, g, b, a) };
        // Should only fail on an invalid renderer
        if ret != 0 {
            panic!(get_error())
        }
    }

    /// Gets the color used for drawing operations (Rect, Line and Clear).
    #[doc(alias = "SDL_GetRenderDrawColor")]
    pub fn draw_color(&self) -> pixels::Color {
        let (mut r, mut g, mut b, mut a) = (0, 0, 0, 0);
        let ret = unsafe {
            sys::SDL_GetRenderDrawColor(self.context.raw(), &mut r, &mut g, &mut b, &mut a)
        };
        // Should only fail on an invalid renderer
        if ret != 0 {
            panic!(get_error())
        } else {
            pixels::Color::RGBA(r, g, b, a)
        }
    }

    /// Sets the blend mode used for drawing operations (Fill and Line).
    #[doc(alias = "SDL_SetRenderDrawBlendMode")]
    pub fn set_blend_mode(&mut self, blend: BlendMode) {
        let ret =
            unsafe { sys::SDL_SetRenderDrawBlendMode(self.context.raw(), transmute(blend as u32)) };
        // Should only fail on an invalid renderer
        if ret != 0 {
            panic!(get_error())
        }
    }

    /// Gets the blend mode used for drawing operations.
    #[doc(alias = "SDL_GetRenderDrawBlendMode")]
    pub fn blend_mode(&self) -> BlendMode {
        let mut blend: MaybeUninit<SDL_BlendMode> = mem::MaybeUninit::uninit();
        let ret =
            unsafe { sys::SDL_GetRenderDrawBlendMode(self.context.raw(), blend.as_mut_ptr()) };
        // Should only fail on an invalid renderer
        if ret != 0 {
            panic!(get_error())
        } else {
            use std::convert::TryFrom;
            let blend = unsafe { blend.assume_init() };
            BlendMode::try_from(blend as u32).unwrap()
        }
    }

    /// Clears the current rendering target with the drawing color.
    #[doc(alias = "SDL_RenderClear")]
    pub fn clear(&mut self) {
        let ret = unsafe { sys::SDL_RenderClear(self.context.raw()) };
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
    #[doc(alias = "SDL_RenderPresent")]
    pub fn present(&mut self) {
        self.context.present()
    }

    /// Gets the output size of a rendering context.
    #[doc(alias = "SDL_GetRendererOutputSize")]
    pub fn output_size(&self) -> Result<(u32, u32), String> {
        let mut width = 0;
        let mut height = 0;

        let result =
            unsafe { sys::SDL_GetRendererOutputSize(self.context.raw(), &mut width, &mut height) };

        if result == 0 {
            Ok((width as u32, height as u32))
        } else {
            Err(get_error())
        }
    }

    /// Sets a device independent resolution for rendering.
    #[doc(alias = "SDL_RenderSetLogicalSize")]
    pub fn set_logical_size(&mut self, width: u32, height: u32) -> Result<(), IntegerOrSdlError> {
        use crate::common::IntegerOrSdlError::*;
        let width = validate_int(width, "width")?;
        let height = validate_int(height, "height")?;
        let result = unsafe { sys::SDL_RenderSetLogicalSize(self.context.raw(), width, height) };
        match result {
            0 => Ok(()),
            _ => Err(SdlError(get_error())),
        }
    }

    /// Gets device independent resolution for rendering.
    #[doc(alias = "SDL_RenderGetLogicalSize")]
    pub fn logical_size(&self) -> (u32, u32) {
        let mut width = 0;
        let mut height = 0;

        unsafe { sys::SDL_RenderGetLogicalSize(self.context.raw(), &mut width, &mut height) };

        (width as u32, height as u32)
    }

    /// Sets the drawing area for rendering on the current target.
    #[doc(alias = "SDL_RenderSetViewport")]
    pub fn set_viewport<R: Into<Option<Rect>>>(&mut self, rect: R) {
        let ptr = match rect.into() {
            Some(ref rect) => rect.raw(),
            None => ptr::null(),
        };
        let ret = unsafe { sys::SDL_RenderSetViewport(self.context.raw(), ptr) };
        if ret != 0 {
            panic!("Could not set viewport: {}", get_error())
        }
    }

    /// Gets the drawing area for the current target.
    #[doc(alias = "SDL_RenderGetViewport")]
    pub fn viewport(&self) -> Rect {
        let mut rect = mem::MaybeUninit::uninit();
        unsafe { sys::SDL_RenderGetViewport(self.context.raw(), rect.as_mut_ptr()) };
        let rect = unsafe { rect.assume_init() };
        Rect::from_ll(rect)
    }

    /// Sets the clip rectangle for rendering on the specified target.
    ///
    /// If the rectangle is `None`, clipping will be disabled.
    #[doc(alias = "SDL_RenderSetClipRect")]
    pub fn set_clip_rect<R: Into<Option<Rect>>>(&mut self, rect: R) {
        let ret = unsafe {
            sys::SDL_RenderSetClipRect(
                self.context.raw(),
                match rect.into() {
                    Some(ref rect) => rect.raw(),
                    None => ptr::null(),
                },
            )
        };
        if ret != 0 {
            panic!("Could not set clip rect: {}", get_error())
        }
    }

    /// Gets the clip rectangle for the current target.
    ///
    /// Returns `None` if clipping is disabled.
    #[doc(alias = "SDL_RenderGetClipRect")]
    pub fn clip_rect(&self) -> Option<Rect> {
        let mut raw = mem::MaybeUninit::uninit();
        unsafe { sys::SDL_RenderGetClipRect(self.context.raw(), raw.as_mut_ptr()) };
        let raw = unsafe { raw.assume_init() };
        if raw.w == 0 || raw.h == 0 {
            None
        } else {
            Some(Rect::from_ll(raw))
        }
    }

    /// Sets the drawing scale for rendering on the current target.
    #[doc(alias = "SDL_RenderSetScale")]
    pub fn set_scale(&mut self, scale_x: f32, scale_y: f32) -> Result<(), String> {
        let ret = unsafe { sys::SDL_RenderSetScale(self.context.raw(), scale_x, scale_y) };
        // Should only fail on an invalid renderer
        if ret != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Gets the drawing scale for the current target.
    #[doc(alias = "SDL_RenderGetScale")]
    pub fn scale(&self) -> (f32, f32) {
        let mut scale_x = 0.0;
        let mut scale_y = 0.0;
        unsafe { sys::SDL_RenderGetScale(self.context.raw(), &mut scale_x, &mut scale_y) };
        (scale_x, scale_y)
    }

    /// Draws a point on the current rendering target.
    /// Errors if drawing fails for any reason (e.g. driver failure)
    #[doc(alias = "SDL_RenderDrawPoint")]
    pub fn draw_point<P: Into<Point>>(&mut self, point: P) -> Result<(), String> {
        let point = point.into();
        let result = unsafe { sys::SDL_RenderDrawPoint(self.context.raw(), point.x(), point.y()) };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Draws multiple points on the current rendering target.
    /// Errors if drawing fails for any reason (e.g. driver failure)
    #[doc(alias = "SDL_RenderDrawPoints")]
    pub fn draw_points<'a, P: Into<&'a [Point]>>(&mut self, points: P) -> Result<(), String> {
        let points = points.into();
        let result = unsafe {
            sys::SDL_RenderDrawPoints(
                self.context.raw(),
                Point::raw_slice(points),
                points.len() as c_int,
            )
        };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Draws a line on the current rendering target.
    /// Errors if drawing fails for any reason (e.g. driver failure)
    #[doc(alias = "SDL_RenderDrawLine")]
    pub fn draw_line<P1: Into<Point>, P2: Into<Point>>(
        &mut self,
        start: P1,
        end: P2,
    ) -> Result<(), String> {
        let start = start.into();
        let end = end.into();
        let result = unsafe {
            sys::SDL_RenderDrawLine(self.context.raw(), start.x(), start.y(), end.x(), end.y())
        };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Draws a series of connected lines on the current rendering target.
    /// Errors if drawing fails for any reason (e.g. driver failure)
    #[doc(alias = "SDL_RenderDrawLines")]
    pub fn draw_lines<'a, P: Into<&'a [Point]>>(&mut self, points: P) -> Result<(), String> {
        let points = points.into();
        let result = unsafe {
            sys::SDL_RenderDrawLines(
                self.context.raw(),
                Point::raw_slice(points),
                points.len() as c_int,
            )
        };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Draws a rectangle on the current rendering target.
    /// Errors if drawing fails for any reason (e.g. driver failure)
    #[doc(alias = "SDL_RenderDrawRect")]
    pub fn draw_rect(&mut self, rect: Rect) -> Result<(), String> {
        let result = unsafe { sys::SDL_RenderDrawRect(self.context.raw(), rect.raw()) };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Draws some number of rectangles on the current rendering target.
    /// Errors if drawing fails for any reason (e.g. driver failure)
    #[doc(alias = "SDL_RenderDrawRects")]
    pub fn draw_rects(&mut self, rects: &[Rect]) -> Result<(), String> {
        let result = unsafe {
            sys::SDL_RenderDrawRects(
                self.context.raw(),
                Rect::raw_slice(rects),
                rects.len() as c_int,
            )
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
    #[doc(alias = "SDL_RenderFillRect")]
    pub fn fill_rect<R: Into<Option<Rect>>>(&mut self, rect: R) -> Result<(), String> {
        let result = unsafe {
            sys::SDL_RenderFillRect(
                self.context.raw(),
                rect.into().as_ref().map(|r| r.raw()).unwrap_or(ptr::null()),
            )
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
    #[doc(alias = "SDL_RenderFillRects")]
    pub fn fill_rects(&mut self, rects: &[Rect]) -> Result<(), String> {
        let result = unsafe {
            sys::SDL_RenderFillRects(
                self.context.raw(),
                Rect::raw_slice(rects),
                rects.len() as c_int,
            )
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
    #[doc(alias = "SDL_RenderCopy")]
    pub fn copy<R1, R2>(&mut self, texture: &Texture, src: R1, dst: R2) -> Result<(), String>
    where
        R1: Into<Option<Rect>>,
        R2: Into<Option<Rect>>,
    {
        let ret = unsafe {
            sys::SDL_RenderCopy(
                self.context.raw(),
                texture.raw(),
                match src.into() {
                    Some(ref rect) => rect.raw(),
                    None => ptr::null(),
                },
                match dst.into() {
                    Some(ref rect) => rect.raw(),
                    None => ptr::null(),
                },
            )
        };

        if ret != 0 {
            Err(get_error())
        } else {
            Ok(())
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
    /// Errors if drawing fails for any reason (e.g. driver failure),
    /// if the provided texture does not belong to the renderer,
    /// or if the driver does not support RenderCopyEx.
    #[doc(alias = "SDL_RenderCopyEx")]
    pub fn copy_ex<R1, R2, P>(
        &mut self,
        texture: &Texture,
        src: R1,
        dst: R2,
        angle: f64,
        center: P,
        flip_horizontal: bool,
        flip_vertical: bool,
    ) -> Result<(), String>
    where
        R1: Into<Option<Rect>>,
        R2: Into<Option<Rect>>,
        P: Into<Option<Point>>,
    {
        use crate::sys::SDL_RendererFlip::*;
        let flip = unsafe {
            match (flip_horizontal, flip_vertical) {
                (false, false) => SDL_FLIP_NONE,
                (true, false) => SDL_FLIP_HORIZONTAL,
                (false, true) => SDL_FLIP_VERTICAL,
                (true, true) => transmute::<u32, sys::SDL_RendererFlip>(
                    transmute::<sys::SDL_RendererFlip, u32>(SDL_FLIP_HORIZONTAL)
                        | transmute::<sys::SDL_RendererFlip, u32>(SDL_FLIP_VERTICAL),
                ),
            }
        };

        let ret = unsafe {
            sys::SDL_RenderCopyEx(
                self.context.raw(),
                texture.raw(),
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
                flip,
            )
        };

        if ret != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Reads pixels from the current rendering target.
    /// # Remarks
    /// WARNING: This is a very slow operation, and should not be used frequently.
    #[doc(alias = "SDL_RenderReadPixels")]
    pub fn read_pixels<R: Into<Option<Rect>>>(
        &self,
        rect: R,
        format: pixels::PixelFormatEnum,
    ) -> Result<Vec<u8>, String> {
        unsafe {
            let rect = rect.into();
            let (actual_rect, w, h) = match rect {
                Some(ref rect) => (rect.raw(), rect.width() as usize, rect.height() as usize),
                None => {
                    let (w, h) = self.output_size()?;
                    (ptr::null(), w as usize, h as usize)
                }
            };

            let pitch = w * format.byte_size_per_pixel(); // calculated pitch
            let size = format.byte_size_of_pixels(w * h);
            let mut pixels = Vec::with_capacity(size);
            pixels.set_len(size);

            // Pass the interior of `pixels: Vec<u8>` to SDL
            let ret = {
                sys::SDL_RenderReadPixels(
                    self.context.raw(),
                    actual_rect,
                    format as u32,
                    pixels.as_mut_ptr() as *mut c_void,
                    pitch as c_int,
                )
            };

            if ret == 0 {
                Ok(pixels)
            } else {
                Err(get_error())
            }
        }
    }

    /// Creates a texture for a rendering context.
    ///
    /// If format is `None`, the format will be the one the parent Window or Surface uses.
    ///
    /// If format is `Some(pixel_format)`
    /// created with the specified format if possible. If the PixelFormat is not supported, this
    /// will return an error.
    ///
    /// You should prefer the default format if possible to have performance gains and to avoid
    /// unsupported Pixel Formats that can cause errors. However, be careful with the default
    /// `PixelFormat` if you want to create transparent textures.
    ///
    /// # Notes
    ///
    /// Note that this method is only accessible in Canvas with the `unsafe_textures` feature,
    /// because lifetimes otherwise prevent `Canvas` from creating and accessing `Texture`s at the
    /// same time.
    #[cfg(feature = "unsafe_textures")]
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

    /// Shorthand for `create_texture(format, TextureAccess::Static, width, height)`
    ///
    /// # Notes
    ///
    /// Note that this method is only accessible in Canvas with the `unsafe_textures` feature.
    #[cfg(feature = "unsafe_textures")]
    #[inline]
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

    /// Shorthand for `create_texture(format, TextureAccess::Streaming, width, height)`
    ///
    /// # Notes
    ///
    /// Note that this method is only accessible in Canvas with the `unsafe_textures` feature.
    #[cfg(feature = "unsafe_textures")]
    #[inline]
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

    /// Shorthand for `create_texture(format, TextureAccess::Target, width, height)`
    ///
    /// # Notes
    ///
    /// Note that this method is only accessible in Canvas with the `unsafe_textures` feature.
    #[cfg(feature = "unsafe_textures")]
    #[inline]
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
    /// The access hint for the created texture is `TextureAccess::Static`.
    ///
    /// # Notes
    ///
    /// Note that this method is only accessible in Canvas with the `unsafe_textures` feature.
    #[cfg(feature = "unsafe_textures")]
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

    #[cfg(feature = "unsafe_textures")]
    /// Create a texture from its raw `SDL_Texture`. Should be used with care.
    ///
    /// # Notes
    ///
    /// Note that this method is only accessible in Canvas with the `unsafe_textures` feature.
    pub unsafe fn raw_create_texture(&self, raw: *mut sys::SDL_Texture) -> Texture {
        Texture { raw }
    }
}
