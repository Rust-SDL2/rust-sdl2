use std::ffi::CStr;

#[doc(alias = "SDL_GetAudioDriver")]
#[derive(Copy, Clone)]
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
            unsafe {
                let buf = sys::SDL_GetAudioDriver(self.index);
                assert!(!buf.is_null());
                self.index += 1;

                Some(CStr::from_ptr(buf as *const _).to_str().unwrap())
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

/// Gets an iterator of all audio drivers compiled into the SDL2 library.
#[doc(alias = "SDL_GetAudioDriver")]
#[inline]
pub fn drivers() -> DriverIterator {
    // This function is thread-safe and doesn't require the audio subsystem to be initialized.
    // The list of drivers are read-only and statically compiled into SDL2, varying by platform.

    // SDL_GetNumAudioDrivers can never return a negative value.
    DriverIterator {
        length: unsafe { sys::SDL_GetNumAudioDrivers() },
        index: 0,
    }
}
