use std::rc::Rc;

use crate::common::validate_int;
use crate::video::Window;
use crate::{get_error, IntegerOrSdlError};

use super::{Canvas, RendererContext, WindowCanvas};

/// The type that allows you to build Window-based renderers.
///
/// By default, the renderer builder will prioritize for a hardware-accelerated
/// renderer, which is probably what you want.
pub struct CanvasBuilder {
    window: Window,
    index: Option<u32>,
    renderer_flags: u32,
}

impl CanvasBuilder {
    /// Initializes a new `CanvasBuilder`.
    pub fn new(window: Window) -> CanvasBuilder {
        CanvasBuilder {
            window,
            // -1 means to initialize the first rendering driver supporting the
            // renderer flags
            index: None,
            // no flags gives priority to available SDL_RENDERER_ACCELERATED
            // renderers
            renderer_flags: 0,
        }
    }

    /// Builds the renderer.
    #[doc(alias = "SDL_CreateRenderer")]
    pub fn build(self) -> Result<WindowCanvas, IntegerOrSdlError> {
        use crate::common::IntegerOrSdlError::*;
        let index = match self.index {
            None => -1,
            Some(index) => validate_int(index, "index")?,
        };
        let raw = unsafe { sys::SDL_CreateRenderer(self.window.raw(), index, self.renderer_flags) };

        if raw.is_null() {
            Err(SdlError(get_error()))
        } else {
            let context = Rc::new(unsafe { RendererContext::from_ll(raw, self.window.context()) });
            let default_pixel_format = self.window.window_pixel_format();
            Ok(Canvas::new(context, self.window, default_pixel_format))
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
        self.renderer_flags |= sys::SDL_RendererFlags::SDL_RENDERER_SOFTWARE as u32;
        self
    }

    /// Set the renderer to use hardware acceleration.
    /// This flag is accumulative, and may be specified with other flags.
    pub fn accelerated(mut self) -> CanvasBuilder {
        self.renderer_flags |= sys::SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32;
        self
    }

    /// Synchronize renderer `present` method calls with the refresh rate.
    /// This flag is accumulative, and may be specified with other flags.
    pub fn present_vsync(mut self) -> CanvasBuilder {
        self.renderer_flags |= sys::SDL_RendererFlags::SDL_RENDERER_PRESENTVSYNC as u32;
        self
    }

    /// Set the renderer to support rendering to a texture.
    /// This flag is accumulative, and may be specified with other flags.
    pub fn target_texture(mut self) -> CanvasBuilder {
        self.renderer_flags |= sys::SDL_RendererFlags::SDL_RENDERER_TARGETTEXTURE as u32;
        self
    }
}
