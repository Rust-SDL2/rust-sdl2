use crate::get_error;
use alloc::string::String;
use libc::c_void;
use libc::{c_char, c_int, size_t};
use alloc::ffi::CString;
use core::marker::PhantomData;

use crate::sys;

/// A structure that provides an abstract interface to stream I/O.
pub struct RWops<'a> {
    raw: *mut sys::SDL_RWops,
    _marker: PhantomData<&'a ()>,
}

impl<'a> RWops<'a> {
    // this can prevent introducing UB until
    // https://github.com/rust-lang/rust-clippy/issues/5953 is fixed
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub unsafe fn raw(&self) -> *mut sys::SDL_RWops {
        self.raw
    }

    pub unsafe fn from_ll<'b>(raw: *mut sys::SDL_RWops) -> RWops<'b> {
        RWops {
            raw,
            _marker: PhantomData,
        }
    }

    /// Creates an SDL file stream.
    #[doc(alias = "SDL_RWFromFile")]
    pub fn from_file(path: &str, mode: &str) -> Result<RWops<'static>, String> {
        let raw = unsafe {
            let path_c = CString::new(path).unwrap();
            let mode_c = CString::new(mode).unwrap();
            sys::SDL_RWFromFile(
                path_c.as_ptr() as *const c_char,
                mode_c.as_ptr() as *const c_char,
            )
        };

        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(RWops {
                raw,
                _marker: PhantomData,
            })
        }
    }

    /// Prepares a read-only memory buffer for use with `RWops`.
    ///
    /// This method can only fail if the buffer size is zero.
    #[doc(alias = "SDL_RWFromConstMem")]
    pub fn from_bytes(buf: &'a [u8]) -> Result<RWops<'a>, String> {
        let raw =
            unsafe { sys::SDL_RWFromConstMem(buf.as_ptr() as *const c_void, buf.len() as c_int) };

        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(RWops {
                raw,
                _marker: PhantomData,
            })
        }
    }

    /// Prepares a read-write memory buffer for use with `RWops`.
    ///
    /// This method can only fail if the buffer size is zero.
    #[doc(alias = "SDL_RWFromMem")]
    pub fn from_bytes_mut(buf: &'a mut [u8]) -> Result<RWops<'a>, String> {
        let raw = unsafe { sys::SDL_RWFromMem(buf.as_ptr() as *mut c_void, buf.len() as c_int) };

        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(RWops {
                raw,
                _marker: PhantomData,
            })
        }
    }

    /// Gets the stream's total size in bytes.
    ///
    /// Returns `None` if the stream size can't be determined
    /// (either because it doesn't make sense for the stream type, or there was an error).
    pub fn len(&self) -> Option<usize> {
        let result = unsafe { ((*self.raw).size.unwrap())(self.raw) };

        match result {
            -1 => None,
            v => Some(v as usize),
        }
    }

    // Tells if the stream is empty
    pub fn is_empty(&self) -> bool {
        match self.len() {
            Some(s) => s == 0,
            None => true,
        }
    }
}

impl<'a> Drop for RWops<'a> {
    fn drop(&mut self) {
        let ret = unsafe { ((*self.raw).close.unwrap())(self.raw) };
        if ret != 0 {
            panic!("{}", get_error());
        }
    }
}

