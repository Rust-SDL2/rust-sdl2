use libc::c_int;
use std::ops::{Deref, DerefMut};

use crate::get_error;
use crate::rect::Rect;
use crate::surface::SurfaceRef;

use super::Window;

pub struct WindowSurfaceRef<'a>(&'a mut SurfaceRef, &'a Window);

impl<'a> Deref for WindowSurfaceRef<'a> {
    type Target = SurfaceRef;

    #[inline]
    fn deref(&self) -> &SurfaceRef {
        self.0
    }
}

impl<'a> DerefMut for WindowSurfaceRef<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut SurfaceRef {
        &mut self.0
    }
}

impl<'a> WindowSurfaceRef<'a> {
    pub(super) fn new(surface_ref: &'a mut SurfaceRef, window: &'a Window) -> Self {
        Self(surface_ref, window)
    }

    /// Updates the change made to the inner Surface to the Window it was created from.
    ///
    /// This would effectively be the theoretical equivalent of `present` from a Canvas.
    #[doc(alias = "SDL_UpdateWindowSurface")]
    pub fn update_window(&self) -> Result<(), String> {
        unsafe {
            if sys::SDL_UpdateWindowSurface(self.1.raw()) == 0 {
                Ok(())
            } else {
                Err(get_error())
            }
        }
    }

    /// Same as `update_window`, but only update the parts included in `rects` to the Window it was
    /// created from.
    #[doc(alias = "SDL_UpdateWindowSurfaceRects")]
    pub fn update_window_rects(&self, rects: &[Rect]) -> Result<(), String> {
        unsafe {
            if sys::SDL_UpdateWindowSurfaceRects(
                self.1.raw(),
                Rect::raw_slice(rects),
                rects.len() as c_int,
            ) == 0
            {
                Ok(())
            } else {
                Err(get_error())
            }
        }
    }

    /// Gives up this WindowSurfaceRef, allowing to use the window freely again. Before being
    /// destroyed, calls `update_window` one last time.
    ///
    /// If you don't want to `update_window` one last time, simply Drop this struct. However
    /// beware, since the Surface will still be in the state you left it the next time you will
    /// call `window.surface()` again.
    pub fn finish(self) -> Result<(), String> {
        self.update_window()
    }
}
