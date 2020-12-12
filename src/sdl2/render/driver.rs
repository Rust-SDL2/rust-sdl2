use std::mem;

use crate::sys;

use super::RendererInfo;

#[derive(Copy, Clone)]
pub struct DriverIterator {
    length: i32,
    index: i32,
}

impl Iterator for DriverIterator {
    type Item = RendererInfo;

    #[inline]
    #[doc(alias = "SDL_GetRenderDriverInfo")]
    fn next(&mut self) -> Option<RendererInfo> {
        if self.index >= self.length {
            None
        } else {
            let mut out = mem::MaybeUninit::uninit();
            let result = unsafe { sys::SDL_GetRenderDriverInfo(self.index, out.as_mut_ptr()) == 0 };
            assert!(result, 0);
            self.index += 1;

            unsafe { Some(RendererInfo::from_ll(&out.assume_init())) }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let l = self.length as usize;
        (l, Some(l))
    }
}

impl ExactSizeIterator for DriverIterator {}

/// Gets an iterator of all render drivers compiled into the SDL2 library.
#[inline]
#[doc(alias = "SDL_GetNumRenderDrivers")]
pub fn drivers() -> DriverIterator {
    // This function is thread-safe and doesn't require the video subsystem to be initialized.
    // The list of drivers are read-only and statically compiled into SDL2, varying by platform.

    // SDL_GetNumRenderDrivers can never return a negative value.
    DriverIterator {
        length: unsafe { sys::SDL_GetNumRenderDrivers() },
        index: 0,
    }
}
