use std::ops::Deref;
use std::rc::Rc;

use crate::pixels::PixelFormatEnum;

use super::{RenderTarget, RendererContext, TargetRenderError, Texture};

pub mod render;
pub mod surface;
pub mod window;

/// Manages and owns a target (`Surface` or `Window`) and allows drawing in it.
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
/// // we need to call `present`. We need to call this every time
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
    default_pixel_format: PixelFormatEnum,
}

impl<T: RenderTarget> Canvas<T> {
    /// Determine whether a window supports the use of render targets.
    #[doc(alias = "SDL_RenderTargetSupported")]
    pub fn render_target_supported(&self) -> bool {
        self.context.is_supported()
    }

    /// Temporarily sets the target of `Canvas` to a `Texture`. This effectively allows rendering
    /// to a `Texture` in any way you want: you can make a `Texture` a combination of other
    /// `Texture`s, be a complex geometry form with the `gfx` module, ... You can draw pixel by
    /// pixel in it if you want, so you can do basically anything with that `Texture`.
    ///
    /// If you want to set the content of multiple `Texture` at once the most efficient way
    /// possible, *don't* make a loop and call this function every time and use
    /// `with_multiple_texture_canvas` instead. Using `with_texture_canvas` is actually
    /// inefficient because the target is reset to the source (the `Window` or the `Surface`)
    /// at the end of this function, but using it in a loop would make this reset useless.
    /// Plus, the check that render_target is actually supported on that `Canvas` is also
    /// done every time, leading to useless checks.
    ///
    /// # Notes
    ///
    /// Note that the `Canvas` in the closure is exactly the same as the one you call this
    /// function with, meaning that you can call every function of your original `Canvas`.
    ///
    /// That means you can also call `with_texture_canvas` and `with_multiple_texture_canvas` from
    /// the inside of the closure. Even though this is useless and inefficient, this is totally
    /// safe to do and allowed.
    ///
    /// Since the render target is now a Texture, some calls of Canvas might return another result
    /// than if the target was to be the original source. For instance `output_size` will return
    /// this size of the current `Texture` in the closure, but the size of the `Window` or
    /// `Surface` outside of the closure.
    ///
    /// You do not need to call `present` after drawing in the Canvas in the closure, the changes
    /// are applied directly to the `Texture` instead of a hidden buffer.
    ///
    /// # Errors
    ///
    /// * returns `TargetRenderError::NotSupported`
    /// if the renderer does not support the use of render targets
    /// * returns `TargetRenderError::SdlError` if SDL2 returned with an error code.
    ///
    /// The texture *must* be created with the texture access:
    /// `sdl2::render::TextureAccess::Target`.
    /// Using a texture which was not created with the texture access `Target` is undefined
    /// behavior.
    ///
    /// # Examples
    ///
    /// The example below changes a newly created `Texture` to be a 150-by-150 black texture with a
    /// 50-by-50 red square in the middle.
    ///
    /// ```rust,no_run
    /// # use sdl2::render::{Canvas, Texture};
    /// # use sdl2::video::Window;
    /// # use sdl2::pixels::Color;
    /// # use sdl2::rect::Rect;
    /// # let mut canvas : Canvas<Window> = unimplemented!();
    /// let texture_creator = canvas.texture_creator();
    /// let mut texture = texture_creator
    ///     .create_texture_target(texture_creator.default_pixel_format(), 150, 150)
    ///     .unwrap();
    /// let result = canvas.with_texture_canvas(&mut texture, |texture_canvas| {
    ///     texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
    ///     texture_canvas.clear();
    ///     texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
    ///     texture_canvas.fill_rect(Rect::new(50, 50, 50, 50)).unwrap();
    /// });
    /// ```
    ///

    pub fn with_texture_canvas<F>(
        &mut self,
        texture: &mut Texture,
        f: F,
    ) -> Result<(), TargetRenderError>
    where
        for<'r> F: FnOnce(&'r mut Canvas<T>),
    {
        if self.render_target_supported() {
            let target = self.get_raw_target();
            unsafe { self.set_raw_target(texture.raw) }.map_err(TargetRenderError::SdlError)?;
            f(self);
            unsafe { self.set_raw_target(target) }.map_err(TargetRenderError::SdlError)?;
            Ok(())
        } else {
            Err(TargetRenderError::NotSupported)
        }
    }

    /// Same as `with_texture_canvas`, but allows to change multiple `Texture`s at once with the
    /// least amount of overhead. It means that between every iteration the Target is not reset to
    /// the source, and that the fact that the Canvas supports render target isn't checked every
    /// iteration either; the check is actually only done once, at the beginning, avoiding useless
    /// checks.
    ///
    /// The closure is run once for every `Texture` sent as parameter.
    ///
    /// The main changes from `with_texture_canvas` is that is takes an `Iterator` of `(&mut
    /// Texture, U)`, where U is a type defined by the user. The closure takes a `&mut Canvas`, and
    /// `&U` as arguments instead of a simple `&mut Canvas`. This user-defined type allows you to
    /// keep track of what to do with the Canvas you have received in the closure.
    ///
    /// You will usually want to keep track of the number, a property, or anything that will allow
    /// you to uniquely track this `Texture`, but it can also be an empty struct or `()` as well!
    ///
    /// # Examples
    ///
    /// Let's create two textures, one which will be yellow, and the other will be white
    ///
    /// ```rust,no_run
    /// # use sdl2::pixels::Color;
    /// # use sdl2::rect::Rect;
    /// # use sdl2::video::Window;
    /// # use sdl2::render::{Canvas, Texture};
    /// # let mut canvas : Canvas<Window> = unimplemented!();
    /// let texture_creator = canvas.texture_creator();
    /// enum TextureColor {
    ///     Yellow,
    ///     White,
    /// };
    ///
    /// let mut square_texture1 : Texture =
    ///     texture_creator.create_texture_target(None, 100, 100).unwrap();
    /// let mut square_texture2 : Texture =
    ///     texture_creator.create_texture_target(None, 100, 100).unwrap();
    /// let textures : Vec<(&mut Texture, TextureColor)> = vec![
    ///     (&mut square_texture1, TextureColor::Yellow),
    ///     (&mut square_texture2, TextureColor::White)
    /// ];
    /// let result : Result<(), _> =
    ///     canvas.with_multiple_texture_canvas(textures.iter(), |texture_canvas, user_context| {
    ///     match *user_context {
    ///         TextureColor::White => {
    ///             texture_canvas.set_draw_color(Color::RGB(255, 255, 255));
    ///         },
    ///         TextureColor::Yellow => {
    ///             texture_canvas.set_draw_color(Color::RGB(255, 255, 0));
    ///         }
    ///     };
    ///     texture_canvas.clear();
    /// });
    /// // square_texture1 is now Yellow and square_texture2 is now White!
    /// ```
    ///
    ///
    #[cfg(not(feature = "unsafe_textures"))]
    pub fn with_multiple_texture_canvas<'t: 'a, 'a: 's, 's, I, F, U: 's>(
        &mut self,
        textures: I,
        mut f: F,
    ) -> Result<(), TargetRenderError>
    where
        for<'r> F: FnMut(&'r mut Canvas<T>, &U),
        I: Iterator<Item = &'s (&'a mut Texture<'t>, U)>,
    {
        if self.render_target_supported() {
            let target = self.get_raw_target();
            for &(ref texture, ref user_context) in textures {
                unsafe { self.set_raw_target(texture.raw) }.map_err(TargetRenderError::SdlError)?;
                f(self, user_context);
            }
            // reset the target to its source
            unsafe { self.set_raw_target(target) }.map_err(TargetRenderError::SdlError)?;
            Ok(())
        } else {
            Err(TargetRenderError::NotSupported)
        }
    }

    #[cfg(feature = "unsafe_textures")]
    pub fn with_multiple_texture_canvas<'a: 's, 's, I, F, U: 's>(
        &mut self,
        textures: I,
        mut f: F,
    ) -> Result<(), TargetRenderError>
    where
        for<'r> F: FnMut(&'r mut Canvas<T>, &U),
        I: Iterator<Item = &'s (&'a mut Texture, U)>,
    {
        if self.render_target_supported() {
            for &(ref texture, ref user_context) in textures {
                unsafe { self.set_raw_target(texture.raw) }
                    .map_err(|e| TargetRenderError::SdlError(e))?;
                f(self, &user_context);
            }
            // reset the target to its source
            unsafe { self.set_raw_target(ptr::null_mut()) }
                .map_err(|e| TargetRenderError::SdlError(e))?;
            Ok(())
        } else {
            Err(TargetRenderError::NotSupported)
        }
    }
}

impl<T: RenderTarget> Deref for Canvas<T> {
    type Target = RendererContext<T::Context>;

    fn deref(&self) -> &RendererContext<T::Context> {
        self.context.as_ref()
    }
}
