use std::mem;
use std::rc::Rc;

use crate::get_error;
use crate::sys;

use super::{RendererInfo, SdlError};

/// Manages what keeps a `SDL_Renderer` alive
///
/// When the `RendererContext` is dropped, it destroys the `SDL_Renderer`
pub struct RendererContext<T> {
    raw: *mut sys::SDL_Renderer,
    _target: Rc<T>,
}

impl<T> Drop for RendererContext<T> {
    #[doc(alias = "SDL_DestroyRenderer")]
    fn drop(&mut self) {
        unsafe {
            sys::SDL_DestroyRenderer(self.raw);
        };
    }
}

impl<T> RendererContext<T> {
    /// Gets information about the rendering context.
    #[doc(alias = "SDL_GetRendererInfo")]
    pub fn info(&self) -> RendererInfo {
        let mut renderer_info_raw = mem::MaybeUninit::uninit();
        let result =
            unsafe { sys::SDL_GetRendererInfo(self.raw, renderer_info_raw.as_mut_ptr()) != 0 };

        if result {
            // Should only fail on an invalid renderer
            panic!();
        } else {
            unsafe {
                let renderer_info_raw = renderer_info_raw.assume_init();
                RendererInfo::from_ll(&renderer_info_raw)
            }
        }
    }

    /// Gets the raw pointer to the SDL_Renderer
    // this can prevent introducing UB until
    // https://github.com/rust-lang/rust-clippy/issues/5953 is fixed
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn raw(&self) -> *mut sys::SDL_Renderer {
        self.raw
    }

    pub unsafe fn from_ll(raw: *mut sys::SDL_Renderer, target: Rc<T>) -> Self {
        RendererContext {
            raw,
            _target: target,
        }
    }

    pub(super) unsafe fn set_raw_target(
        &self,
        raw_texture: *mut sys::SDL_Texture,
    ) -> Result<(), SdlError> {
        if sys::SDL_SetRenderTarget(self.raw, raw_texture) == 0 {
            Ok(())
        } else {
            Err(SdlError(get_error()))
        }
    }

    pub(super) fn get_raw_target(&self) -> *mut sys::SDL_Texture {
        unsafe { sys::SDL_GetRenderTarget(self.raw) }
    }

    pub(super) fn is_supported(&self) -> bool {
        unsafe { sys::SDL_RenderTargetSupported(self.raw) == sys::SDL_bool::SDL_TRUE }
    }

    pub(super) fn present(&self) {
        unsafe { sys::SDL_RenderPresent(self.raw) }
    }
}
