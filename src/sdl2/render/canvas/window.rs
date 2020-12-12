use std::rc::Rc;

use crate::pixels::PixelFormatEnum;
use crate::render::RendererContext;
use crate::video::{Window, WindowContext};

use super::super::TextureCreator;
use super::Canvas;

/// Methods for the `WindowCanvas`.
impl Canvas<Window> {
    pub(super) fn new(
        context: Rc<RendererContext<WindowContext>>,
        target: Window,
        default_pixel_format: PixelFormatEnum,
    ) -> Self {
        Self {
            context,
            target,
            default_pixel_format,
        }
    }

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

    #[inline]
    pub fn default_pixel_format(&self) -> PixelFormatEnum {
        self.window().window_pixel_format()
    }

    /// Returns a `TextureCreator` that can create Textures to be drawn on this `Canvas`
    ///
    /// This `TextureCreator` will share a reference to the renderer and target context.
    ///
    /// The target (i.e., `Window`) will not be destroyed and the SDL_Renderer will not be
    /// destroyed if the `TextureCreator` is still in scope.
    pub fn texture_creator(&self) -> TextureCreator<WindowContext> {
        TextureCreator::new(self.context.clone(), self.default_pixel_format())
    }
}
