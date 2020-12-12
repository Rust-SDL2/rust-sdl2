#[cfg(not(feature = "unsafe_textures"))]
use std::marker::PhantomData;

use crate::rect::Rect;
use crate::surface::Surface;
use crate::sys;

use super::{
    BlendMode, TextureCreator, TextureValueError, UpdateTextureError, UpdateTextureYUVError,
};

pub mod error;
mod internal;
use self::internal::InternalTexture;
mod query;
pub use self::query::TextureQuery;

/// A texture for a rendering context.
///
/// Every Texture is owned by a `TextureCreator` or `Canvas` (the latter is only possible with the
/// `unsafe_textures` feature).
///
/// # Differences between with and without `unsafe_textures` feature
///
/// Without the `unsafe_textures`, a texture is owned by a `TextureCreator` and a `Texture` cannot
/// outlive its parent `TextureCreator` thanks to lifetimes. A texture is destroyed via its `Drop`
/// implementation. While this is the most "Rust"-y way of doing things currently, it is pretty
/// cumbersome to use in some cases.
///
/// That is why the feature `unsafe_textures` was brought to life: the lifetimes are gone, meaning
/// that `Texture`s *can* outlive their parents. That means that the `Texture`s are not destroyed
/// on `Drop`, but destroyed when their parents are. That means if you create 10 000 textures with
/// this feature, they will only be destroyed after you drop the `Canvas` and every
/// `TextureCreator` linked to it. While this feature is enabled, this is the safest way to free
/// the memory taken by the `Texture`s, but there is still another, unsafe way to destroy the
/// `Texture` before its `Canvas`: the method `destroy`. This method is unsafe because *you* have
/// to make sure the parent `Canvas` or `TextureCreator` is still alive while calling this method.
///
/// **Calling the `destroy` method while no parent is alive is undefined behavior**
///
/// With the `unsafe_textures` feature, a `Texture` can be safely accessed (but not destroyed) after
/// the `Canvas` is dropped, but since any access (except `destroy`) requires the original `Canvas`,
/// it is not possible to access a `Texture` while the `Canvas` is dropped.
#[cfg(feature = "unsafe_textures")]
pub struct Texture {
    raw: *mut sys::SDL_Texture,
}

/// A texture for a rendering context.
///
/// Every Texture is owned by a `TextureCreator`. Internally, a texture is destroyed via its `Drop`
/// implementation. A texture can only be used by the `Canvas` it was originally created from, it
/// is undefined behavior otherwise.
#[cfg(not(feature = "unsafe_textures"))]
pub struct Texture<'r> {
    raw: *mut sys::SDL_Texture,
    _marker: PhantomData<&'r ()>,
}

#[cfg(not(feature = "unsafe_textures"))]
impl<'r> Drop for Texture<'r> {
    #[doc(alias = "SDL_DestroyTexture")]
    fn drop(&mut self) {
        unsafe {
            sys::SDL_DestroyTexture(self.raw);
        }
    }
}

#[cfg(feature = "unsafe_textures")]
impl Texture {
    /// Destroy the Texture and its representation
    /// in the Renderer. This will most likely
    /// mean that the renderer engine will free video
    /// memory that was allocated for this texture.
    ///
    /// This method is unsafe because since Texture does not have
    /// a lifetime, it is legal in Rust to make this texture live
    /// longer than the Renderer. It is however illegal to destroy a SDL_Texture
    /// after its SDL_Renderer, therefore this function is unsafe because
    /// of this.
    ///
    /// Note however that you don't *have* to destroy a Texture before its Canvas,
    /// since whenever Canvas is destroyed, the SDL implementation will automatically
    /// destroy all the children Textures of that Canvas.
    ///
    /// **Calling this method while no parent is alive is undefined behavior**
    pub unsafe fn destroy(self) {
        sys::SDL_DestroyTexture(self.raw)
    }
}

#[cfg(not(feature = "unsafe_textures"))]
impl<'r> Texture<'r> {
    #[inline]
    pub(super) const fn from_raw_create_texture(raw: *mut sys::SDL_Texture) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// Queries the attributes of the texture.
    #[inline]
    pub fn query(&self) -> TextureQuery {
        InternalTexture::new(self.raw).query()
    }

    /// Sets an additional color value multiplied into render copy operations.
    #[inline]
    pub fn set_color_mod(&mut self, red: u8, green: u8, blue: u8) {
        InternalTexture::new(self.raw).set_color_mod(red, green, blue)
    }

    /// Gets the additional color value multiplied into render copy operations.
    #[inline]
    pub fn color_mod(&self) -> (u8, u8, u8) {
        InternalTexture::new(self.raw).color_mod()
    }

    /// Sets an additional alpha value multiplied into render copy operations.
    #[inline]
    pub fn set_alpha_mod(&mut self, alpha: u8) {
        InternalTexture::new(self.raw).set_alpha_mod(alpha)
    }

    /// Gets the additional alpha value multiplied into render copy operations.
    #[inline]
    pub fn alpha_mod(&self) -> u8 {
        InternalTexture::new(self.raw).alpha_mod()
    }

    /// Sets the blend mode used for drawing operations (Fill and Line).
    #[inline]
    pub fn set_blend_mode(&mut self, blend: BlendMode) {
        InternalTexture::new(self.raw).set_blend_mode(blend)
    }

    /// Gets the blend mode used for texture copy operations.
    #[inline]
    pub fn blend_mode(&self) -> BlendMode {
        InternalTexture::new(self.raw).blend_mode()
    }

    /// Updates the given texture rectangle with new pixel data.
    ///
    /// `pitch` is the number of bytes in a row of pixel data, including padding
    /// between lines
    ///
    /// * If `rect` is `None`, the entire texture is updated.
    #[inline]
    pub fn update<R>(
        &mut self,
        rect: R,
        pixel_data: &[u8],
        pitch: usize,
    ) -> Result<(), UpdateTextureError>
    where
        R: Into<Option<Rect>>,
    {
        InternalTexture::new(self.raw).update(rect, pixel_data, pitch)
    }

    /// Updates a rectangle within a planar YV12 or IYUV texture with new pixel data.
    #[inline]
    pub fn update_yuv<R>(
        &mut self,
        rect: R,
        y_plane: &[u8],
        y_pitch: usize,
        u_plane: &[u8],
        u_pitch: usize,
        v_plane: &[u8],
        v_pitch: usize,
    ) -> Result<(), UpdateTextureYUVError>
    where
        R: Into<Option<Rect>>,
    {
        InternalTexture::new(self.raw)
            .update_yuv(rect, y_plane, y_pitch, u_plane, u_pitch, v_plane, v_pitch)
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
    #[inline]
    pub fn with_lock<F, R, R2>(&mut self, rect: R2, func: F) -> Result<R, String>
    where
        F: FnOnce(&mut [u8], usize) -> R,
        R2: Into<Option<Rect>>,
    {
        InternalTexture::new(self.raw).with_lock(rect, func)
    }

    /// Binds an OpenGL/ES/ES2 texture to the current
    /// context for use with when rendering OpenGL primitives directly.
    #[inline]
    pub unsafe fn gl_bind_texture(&mut self) -> (f32, f32) {
        InternalTexture::new(self.raw).gl_bind_texture()
    }

    /// Unbinds an OpenGL/ES/ES2 texture from the current context.
    #[inline]
    pub unsafe fn gl_unbind_texture(&mut self) {
        InternalTexture::new(self.raw).gl_unbind_texture()
    }

    /// Binds and unbinds an OpenGL/ES/ES2 texture from the current context.
    #[inline]
    pub fn gl_with_bind<R, F: FnOnce(f32, f32) -> R>(&mut self, f: F) -> R {
        InternalTexture::new(self.raw).gl_with_bind(f)
    }

    #[inline]
    // this can prevent introducing UB until
    // https://github.com/rust-lang/rust-clippy/issues/5953 is fixed
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub const fn raw(&self) -> *mut sys::SDL_Texture {
        self.raw
    }

    /// A convenience function for [`TextureCreator::create_texture_from_surface`].
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
    /// let texture = Texture::from_surface(&surface, &texture_creator).unwrap();
    /// ```
    #[cfg(not(feature = "unsafe_textures"))]
    pub fn from_surface<'a, T>(
        surface: &Surface,
        texture_creator: &'a TextureCreator<T>,
    ) -> Result<Texture<'a>, TextureValueError> {
        texture_creator.create_texture_from_surface(surface)
    }

    /// A convenience function for [`TextureCreator::create_texture_from_surface`].
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
    /// let texture = Texture::from_surface(&surface, &texture_creator).unwrap();
    /// ```
    #[cfg(feature = "unsafe_textures")]
    pub fn from_surface<T>(
        surface: &Surface,
        texture_creator: &TextureCreator<T>,
    ) -> Result<Texture, TextureValueError> {
        texture_creator.create_texture_from_surface(surface)
    }
}

#[cfg(feature = "unsafe_textures")]
impl Texture {
    /// Queries the attributes of the texture.
    #[inline]
    pub fn query(&self) -> TextureQuery {
        InternalTexture::new(self.raw).query()
    }

    /// Sets an additional color value multiplied into render copy operations.
    #[inline]
    pub fn set_color_mod(&mut self, red: u8, green: u8, blue: u8) {
        InternalTexture::new(self.raw).set_color_mod(red, green, blue)
    }

    /// Gets the additional color value multiplied into render copy operations.
    #[inline]
    pub fn color_mod(&self) -> (u8, u8, u8) {
        InternalTexture::new(self.raw).color_mod()
    }

    /// Sets an additional alpha value multiplied into render copy operations.
    #[inline]
    pub fn set_alpha_mod(&mut self, alpha: u8) {
        InternalTexture::new(self.raw).set_alpha_mod(alpha)
    }

    /// Gets the additional alpha value multiplied into render copy operations.
    #[inline]
    pub fn alpha_mod(&self) -> u8 {
        InternalTexture::new(self.raw).alpha_mod()
    }

    /// Sets the blend mode used for drawing operations (Fill and Line).
    #[inline]
    pub fn set_blend_mode(&mut self, blend: BlendMode) {
        InternalTexture::new(self.raw).set_blend_mode(blend)
    }

    /// Gets the blend mode used for texture copy operations.
    #[inline]
    pub fn blend_mode(&self) -> BlendMode {
        InternalTexture::new(self.raw).blend_mode()
    }

    /// Updates the given texture rectangle with new pixel data.
    ///
    /// `pitch` is the number of bytes in a row of pixel data, including padding
    /// between lines
    ///
    /// * If `rect` is `None`, the entire texture is updated.
    #[inline]
    pub fn update<R>(
        &mut self,
        rect: R,
        pixel_data: &[u8],
        pitch: usize,
    ) -> Result<(), UpdateTextureError>
    where
        R: Into<Option<Rect>>,
    {
        InternalTexture::new(self.raw).update(rect, pixel_data, pitch)
    }

    /// Updates a rectangle within a planar YV12 or IYUV texture with new pixel data.
    #[inline]
    pub fn update_yuv<R>(
        &mut self,
        rect: R,
        y_plane: &[u8],
        y_pitch: usize,
        u_plane: &[u8],
        u_pitch: usize,
        v_plane: &[u8],
        v_pitch: usize,
    ) -> Result<(), UpdateTextureYUVError>
    where
        R: Into<Option<Rect>>,
    {
        InternalTexture::new(self.raw)
            .update_yuv(rect, y_plane, y_pitch, u_plane, u_pitch, v_plane, v_pitch)
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
    #[inline]
    pub fn with_lock<F, R, R2>(&mut self, rect: R2, func: F) -> Result<R, String>
    where
        F: FnOnce(&mut [u8], usize) -> R,
        R2: Into<Option<Rect>>,
    {
        InternalTexture::new(self.raw).with_lock(rect, func)
    }

    /// Binds an OpenGL/ES/ES2 texture to the current
    /// context for use with when rendering OpenGL primitives directly.
    #[inline]
    pub unsafe fn gl_bind_texture(&mut self) -> (f32, f32) {
        InternalTexture::new(self.raw).gl_bind_texture()
    }

    /// Unbinds an OpenGL/ES/ES2 texture from the current context.
    #[inline]
    pub unsafe fn gl_unbind_texture(&mut self) {
        InternalTexture::new(self.raw).gl_unbind_texture()
    }

    /// Binds and unbinds an OpenGL/ES/ES2 texture from the current context.
    #[inline]
    pub fn gl_with_bind<R, F: FnOnce(f32, f32) -> R>(&mut self, f: F) -> R {
        InternalTexture::new(self.raw).gl_with_bind(f)
    }

    #[inline]
    // this can prevent introducing UB until
    // https://github.com/rust-lang/rust-clippy/issues/5953 is fixed
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub const fn raw(&self) -> *mut sys::SDL_Texture {
        self.raw
    }
}
