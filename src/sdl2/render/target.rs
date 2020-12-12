use super::Canvas;

use crate::surface::{Surface, SurfaceContext};
use crate::video::{Window, WindowContext};

/// Represents structs which can be the target of a `SDL_Renderer` (or Canvas).
///
/// This is intended for internal use only. It should not be used outside of this crate,
/// but is still visible for documentation reasons.
pub trait RenderTarget {
    type Context;
}

/// Alias for a `Canvas` that was created out of a `Surface`
pub type SurfaceCanvas<'s> = Canvas<Surface<'s>>;

impl<'s> RenderTarget for Surface<'s> {
    type Context = SurfaceContext<'s>;
}

pub type WindowCanvas = Canvas<Window>;

impl RenderTarget for Window {
    type Context = WindowContext;
}
