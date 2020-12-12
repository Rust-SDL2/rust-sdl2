use std::rc::Rc;

use crate::get_error;
use crate::render::{RendererContext, TextureCreator};
use crate::surface::{self, Surface, SurfaceContext, SurfaceRef};

use super::Canvas;

/// Methods for the `SurfaceCanvas`.
impl<'s> Canvas<Surface<'s>> {
    /// Creates a 2D software rendering context for a surface.
    ///
    /// This method should only fail if SDL2 is not built with rendering
    /// support, or there's an out-of-memory error.
    #[doc(alias = "SDL_CreateSoftwareRenderer")]
    pub fn from_surface(surface: surface::Surface<'s>) -> Result<Self, String> {
        let raw_renderer = unsafe { sys::SDL_CreateSoftwareRenderer(surface.raw()) };
        if !raw_renderer.is_null() {
            let context =
                Rc::new(unsafe { RendererContext::from_ll(raw_renderer, surface.context()) });
            let default_pixel_format = surface.pixel_format_enum();
            Ok(Canvas {
                target: surface,
                context,
                default_pixel_format,
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

    /// Returns a `TextureCreator` that can create Textures to be drawn on this `Canvas`
    ///
    /// This `TextureCreator` will share a reference to the renderer and target context.
    ///
    /// The target (i.e., `Window`) will not be destroyed and the SDL_Renderer will not be
    /// destroyed if the `TextureCreator` is still in scope.
    pub fn texture_creator(&self) -> TextureCreator<SurfaceContext<'s>> {
        TextureCreator::new(self.context.clone(), self.default_pixel_format)
    }
}
