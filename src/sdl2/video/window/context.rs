use crate::VideoSubsystem;

/// Holds a `SDL_Window`
///
/// When the `WindowContext` is dropped, it destroys the `SDL_Window`
pub struct WindowContext {
    pub(super) subsystem: VideoSubsystem,
    pub(super) raw: *mut sys::SDL_Window,
}

impl Drop for WindowContext {
    #[inline]
    #[doc(alias = "SDL_DestroyWindow")]
    fn drop(&mut self) {
        unsafe { sys::SDL_DestroyWindow(self.raw) };
    }
}

impl WindowContext {
    #[inline]
    /// Unsafe if the `*mut SDL_Window` is used after the `WindowContext` is dropped
    pub unsafe fn from_ll(subsystem: VideoSubsystem, raw: *mut sys::SDL_Window) -> WindowContext {
        WindowContext {
            subsystem: subsystem.clone(),
            raw,
        }
    }
}
