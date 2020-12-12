use std::ffi::CStr;

use crate::sys;

#[derive(Copy, Clone)]
#[doc(alias = "SDL_GetVideoDriver")]
pub struct DriverIterator {
    length: i32,
    index: i32,
}

impl Iterator for DriverIterator {
    type Item = &'static str;

    #[inline]
    fn next(&mut self) -> Option<&'static str> {
        if self.index >= self.length {
            None
        } else {
            use std::str;

            unsafe {
                let buf = sys::SDL_GetVideoDriver(self.index);
                assert!(!buf.is_null());
                self.index += 1;

                Some(str::from_utf8(CStr::from_ptr(buf as *const _).to_bytes()).unwrap())
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let l = self.length as usize;
        (l, Some(l))
    }
}

impl ExactSizeIterator for DriverIterator {}

/// Gets an iterator of all video drivers compiled into the SDL2 library.
#[inline]
#[doc(alias = "SDL_GetVideoDriver")]
pub fn drivers() -> DriverIterator {
    // This function is thread-safe and doesn't require the video subsystem to be initialized.
    // The list of drivers are read-only and statically compiled into SDL2, varying by platform.

    // SDL_GetNumVideoDrivers can never return a negative value.
    DriverIterator {
        length: unsafe { sys::SDL_GetNumVideoDrivers() },
        index: 0,
    }
}
