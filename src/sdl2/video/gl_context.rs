use crate::sys;

pub struct GLContext {
    raw: sys::SDL_GLContext,
}

impl Drop for GLContext {
    #[doc(alias = "SDL_GL_DeleteContext")]
    fn drop(&mut self) {
        unsafe { sys::SDL_GL_DeleteContext(self.raw) }
    }
}

impl GLContext {
    pub(super) unsafe fn from_raw(raw: sys::SDL_GLContext) -> Self {
        Self { raw }
    }

    /// Returns true if the OpenGL context is the current one in the thread.
    #[doc(alias = "SDL_GL_GetCurrentContext")]
    pub fn is_current(&self) -> bool {
        let current_raw = unsafe { sys::SDL_GL_GetCurrentContext() };
        self.raw == current_raw
    }
}

impl_raw_accessors!((GLContext, sys::SDL_GLContext));
