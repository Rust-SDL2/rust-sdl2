use std::ffi::CString;
use std::io;
use std::path::Path;
use std::marker::PhantomData;
use libc::{c_int, size_t, c_char};
use libc::c_void;
use crate::get_error;
use std::mem::transmute;

use crate::sys;

/// A structure that provides an abstract interface to stream I/O.
pub struct RWops<'a> {
    raw: *mut sys::SDL_RWops,
    _marker: PhantomData<&'a ()>
}

impl<'a> RWops<'a> {
    pub unsafe fn raw(&self) -> *mut sys::SDL_RWops { self.raw }

    pub unsafe fn from_ll<'b>(raw: *mut sys::SDL_RWops) -> RWops<'b> {
        RWops {
            raw,
            _marker: PhantomData
        }
    }

    /// Creates an SDL file stream.
    pub fn from_file<P: AsRef<Path>>(path: P, mode: &str) -> Result<RWops <'static>, String> {
        let raw = unsafe {
            let path_c = CString::new(path.as_ref().to_str().unwrap()).unwrap();
            let mode_c = CString::new(mode).unwrap();
            sys::SDL_RWFromFile(path_c.as_ptr() as *const c_char, mode_c.as_ptr() as *const c_char)
        };

        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(RWops {
                raw,
                _marker: PhantomData
            })
        }
    }

    /// Prepares a read-only memory buffer for use with `RWops`.
    ///
    /// This method can only fail if the buffer size is zero.
    pub fn from_bytes(buf: &'a [u8]) -> Result<RWops <'a>, String> {
        let raw = unsafe {
            sys::SDL_RWFromConstMem(buf.as_ptr() as *const c_void, buf.len() as c_int)
        };

        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(RWops {
                raw,
                _marker: PhantomData
            })
        }
    }

    /// Reads a `Read` object into a buffer and then passes it to `RWops.from_bytes`.
    ///
    /// The buffer must be provided to this function and must live as long as the
    /// `RWops`, but the `RWops` does not take ownership of it.
    pub fn from_read<T>(r: &mut T, buffer: &'a mut Vec<u8>) -> Result<RWops<'a>, String>
        where T: io::Read + Sized {
        match r.read_to_end(buffer) {
            Ok(_size) => RWops::from_bytes(buffer),
            Err(ioerror) => {
                let msg = format!("IO error: {}", ioerror);
                Err(msg)
            }
        }
    }

    /// Prepares a read-write memory buffer for use with `RWops`.
    ///
    /// This method can only fail if the buffer size is zero.
    pub fn from_bytes_mut(buf: &'a mut [u8]) -> Result<RWops <'a>, String> {
        let raw = unsafe {
            sys::SDL_RWFromMem(buf.as_ptr() as *mut c_void, buf.len() as c_int)
        };

        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(RWops {
                raw,
                _marker: PhantomData
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
            v => Some(v as usize)
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
            ((*self.raw).read.unwrap())(self.raw, buf.as_ptr() as *mut c_void, 1, out_len as sys::size_t)
        };
        Ok(ret as usize)
    }
}

impl<'a> io::Write for RWops<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let in_len = buf.len() as size_t;
        let ret = unsafe {
            ((*self.raw).write.unwrap())(self.raw, buf.as_ptr() as *const c_void, 1, in_len as sys::size_t)
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
            io::SeekFrom::Start(pos) => (sys::RW_SEEK_SET, pos as i64),
            io::SeekFrom::End(pos) => (sys::RW_SEEK_END, pos),
            io::SeekFrom::Current(pos) => (sys::RW_SEEK_CUR, pos)
        };
        let ret = unsafe {
            ((*self.raw).seek.unwrap())(self.raw, offset, transmute(whence))
        };
        if ret == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(ret as u64)
        }
    }
}
