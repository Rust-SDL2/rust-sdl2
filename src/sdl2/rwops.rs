use std::ffi::CString;
use std::io;
use std::path::Path;
use std::marker::PhantomData;
use libc::{c_void, c_int, size_t};
use get_error;
use SdlResult;

use sys::rwops as ll;

/// A structure that provides an abstract interface to stream I/O.
pub struct RWops<'a> {
    raw: *mut ll::SDL_RWops,
    _marker: PhantomData<&'a ()>
}

impl<'a> RWops<'a> {
    pub unsafe fn raw(&self) -> *mut ll::SDL_RWops { self.raw }

    pub unsafe fn from_ll<'b>(raw: *mut ll::SDL_RWops) -> RWops<'b> {
        RWops {
            raw: raw,
            _marker: PhantomData
        }
    }

    /// Creates an SDL file stream.
    pub fn from_file<P: AsRef<Path>>(path: P, mode: &str) -> SdlResult<RWops<'static>> {
        let raw = unsafe {
            let path_c = CString::new(path.as_ref().to_str().unwrap()).unwrap();
            let mode_c = CString::new(mode).unwrap();
            ll::SDL_RWFromFile(path_c.as_ptr(), mode_c.as_ptr())
        };

        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(RWops {
                raw: raw,
                _marker: PhantomData
            })
        }
    }

    /// Prepares a read-only memory buffer for use with `RWops`.
    ///
    /// This method can only fail if the buffer size is zero.
    pub fn from_bytes(buf: &'a [u8]) -> SdlResult<RWops<'a>> {
        let raw = unsafe {
            ll::SDL_RWFromConstMem(buf.as_ptr() as *const c_void, buf.len() as c_int)
        };

        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(RWops {
                raw: raw,
                _marker: PhantomData
            })
        }
    }

    /// Prepares a read-write memory buffer for use with `RWops`.
    ///
    /// This method can only fail if the buffer size is zero.
    pub fn from_bytes_mut(buf: &'a mut [u8]) -> SdlResult<RWops<'a>> {
        let raw = unsafe {
            ll::SDL_RWFromMem(buf.as_ptr() as *mut c_void, buf.len() as c_int)
        };

        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(RWops {
                raw: raw,
                _marker: PhantomData
            })
        }
    }

    /// Gets the stream's total size in bytes.
    ///
    /// Returns `None` if the stream size can't be determined
    /// (either because it doesn't make sense for the stream type, or there was an error).
    pub fn len(&self) -> Option<usize> {
        let result = unsafe { ((*self.raw).size)(self.raw) };

        match result {
            -1 => None,
            v => Some(v as usize)
        }
    }
}

impl<'a> Drop for RWops<'a> {
    fn drop(&mut self) {
        let ret = unsafe { ((*self.raw).close)(self.raw) };
        if ret != 0 {
            panic!(get_error());
        }
    }
}

impl<'a> io::Read for RWops<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let out_len = buf.len() as size_t;
        // FIXME: it's better to use as_mut_ptr().
        // number of objects read, or 0 at error or end of file.
        let ret = unsafe {
            ((*self.raw).read)(self.raw, buf.as_ptr() as *mut c_void, 1, out_len)
        };
        Ok(ret as usize)
    }
}

impl<'a> io::Write for RWops<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let in_len = buf.len() as size_t;
        let ret = unsafe {
            ((*self.raw).write)(self.raw, buf.as_ptr() as *const c_void, 1, in_len)
        };
        Ok(ret as usize)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> io::Seek for RWops<'a> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        // whence code is different from SeekStyle
        let (whence, offset) = match pos {
            io::SeekFrom::Start(pos) => (ll::RW_SEEK_SET, pos as i64),
            io::SeekFrom::End(pos) => (ll::RW_SEEK_END, pos),
            io::SeekFrom::Current(pos) => (ll::RW_SEEK_CUR, pos)
        };
        let ret = unsafe {
            ((*self.raw).seek)(self.raw, offset, whence)
        };
        if ret == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(ret as u64)
        }
    }
}
