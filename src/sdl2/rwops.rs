use std::ffi::CString;
use std::io;
use std::path::Path;
use std::marker::PhantomData;
use std::slice;
use libc::{c_void, c_int, size_t, c_char, int64_t};
use get_error;

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
    pub fn from_file<P: AsRef<Path>>(path: P, mode: &str) -> Result<RWops <'static>, String> {
        let raw = unsafe {
            let path_c = CString::new(path.as_ref().to_str().unwrap()).unwrap();
            let mode_c = CString::new(mode).unwrap();
            ll::SDL_RWFromFile(path_c.as_ptr() as *const c_char, mode_c.as_ptr() as *const c_char)
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
    pub fn from_bytes(buf: &'a [u8]) -> Result<RWops <'a>, String> {
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

    /// Create an SDL RWops object from a Rust stream.
    pub fn from_stream<R: io::Read + io::Seek + 'static>(reader: R) -> RWops<'static> {
        CustomRWopsBuilder::new(reader)
            .with_seek(|reader, from| reader.seek(from))
            .with_read(|reader, buf| reader.read(buf))
            .build()
    }

    /// Prepares a read-write memory buffer for use with `RWops`.
    ///
    /// This method can only fail if the buffer size is zero.
    pub fn from_bytes_mut(buf: &'a mut [u8]) -> Result<RWops <'a>, String> {
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

/// Builder for creating custom RWops implementations.
pub struct CustomRWopsBuilder<T> {
    ptr: *mut T,
    size: Option<Box<FnMut() -> i64>>,
    seek: Option<Box<FnMut(io::SeekFrom) -> io::Result<u64>>>,
    read: Option<Box<FnMut(&mut [u8]) -> io::Result<usize>>>,
    write: Option<Box<FnMut(&[u8]) -> io::Result<usize>>>,
}

impl<T: 'static> CustomRWopsBuilder<T> {
    /// Create a new custom RWops builder around a value.
    pub fn new(value: T) -> Self {
        let ptr = Box::into_raw(Box::new(value));

        Self {
            ptr: ptr,
            size: None,
            seek: None,
            read: None,
            write: None,
        }
    }

    /// Set the callback for fetching the size.
    pub fn with_size<F: FnMut(&mut T) -> i64 + 'static>(mut self, mut f: F) -> Self {
        let ptr = self.ptr;
        self.size = Some(Box::new(move || unsafe {
            f(&mut *ptr)
        }));

        self
    }

    /// Set the callback for seeking.
    pub fn with_seek<F: FnMut(&mut T, io::SeekFrom) -> io::Result<u64> + 'static>(mut self, mut f: F) -> Self {
        let ptr = self.ptr;
        self.seek = Some(Box::new(move |from| unsafe {
            f(&mut *ptr, from)
        }));

        self
    }

    /// Set the callback for reading.
    pub fn with_read<F: FnMut(&mut T, &mut [u8]) -> io::Result<usize> + 'static>(mut self, mut f: F) -> Self {
        let ptr = self.ptr;
        self.read = Some(Box::new(move |buf| unsafe {
            f(&mut *ptr, buf)
        }));

        self
    }

    /// Set the callback for writing.
    pub fn with_write<F: FnMut(&mut T, &[u8]) -> io::Result<usize> + 'static>(mut self, mut f: F) -> Self {
        let ptr = self.ptr;
        self.write = Some(Box::new(move |buf| unsafe {
            f(&mut *ptr, buf)
        }));

        self
    }

    /// Create a RWops from the builder.
    pub fn build(self) -> RWops<'static> {
        struct Callbacks {
            size: Option<Box<FnMut() -> i64>>,
            seek: Option<Box<FnMut(io::SeekFrom) -> io::Result<u64>>>,
            read: Option<Box<FnMut(&mut [u8]) -> io::Result<usize>>>,
            write: Option<Box<FnMut(&[u8]) -> io::Result<usize>>>,
            drop: Box<Fn()>,
        }

        unsafe fn get_callbacks<'a>(ptr: *mut ll::SDL_RWops) -> *mut *mut Callbacks {
            &(*ptr).hidden as *const _ as *mut _
        }

        extern "C" fn stream_size(rwops: *mut ll::SDL_RWops) -> int64_t {
            let mut callbacks = unsafe {
                &mut **get_callbacks(rwops)
            };

            callbacks.size
                .as_mut()
                .map(|f| f())
                .unwrap_or(-1)
        }

        extern "C" fn stream_seek(rwops: *mut ll::SDL_RWops, offset: int64_t, whence: c_int) -> int64_t {
            let mut callbacks = unsafe {
                &mut **get_callbacks(rwops)
            };

            let from = match whence {
                SEEK_SET => io::SeekFrom::Start(offset as u64),
                SEEK_CUR => io::SeekFrom::Current(offset),
                SEEK_END => io::SeekFrom::End(offset),
                _ => return -1,
            };

            callbacks.seek
                .as_mut()
                .and_then(|f| f(from).ok())
                .map(|pos| pos as i64)
                .unwrap_or(-1)
        }

        extern "C" fn stream_read(rwops: *mut ll::SDL_RWops, ptr: *mut c_void, size: size_t, maxnum: size_t) -> size_t {
            let mut callbacks = unsafe {
                &mut **get_callbacks(rwops)
            };

            let buf = unsafe {
                slice::from_raw_parts_mut(ptr as *mut u8, size * maxnum)
            };

            callbacks.read
                .as_mut()
                .and_then(|f| f(buf).ok())
                .unwrap_or(0)
        }

        extern "C" fn stream_write(rwops: *mut ll::SDL_RWops, ptr: *const c_void, size: size_t, maxnum: size_t) -> size_t {
            let mut callbacks = unsafe {
                &mut **get_callbacks(rwops)
            };

            let buf = unsafe {
                slice::from_raw_parts(ptr as *mut u8, size * maxnum)
            };

            callbacks.write
                .as_mut()
                .and_then(|f| f(buf).ok())
                .unwrap_or(0)
        }

        extern "C" fn stream_close(rwops: *mut ll::SDL_RWops) -> c_int {
            if !rwops.is_null() {
                let callbacks = unsafe {
                    &mut **get_callbacks(rwops)
                };

                (callbacks.drop)();

                unsafe {
                    ll::SDL_FreeRW(rwops);
                }
            }
            0
        }

        let value_ptr = self.ptr;
        let callbacks = Callbacks {
            size: self.size,
            seek: self.seek,
            read: self.read,
            write: self.write,
            drop: Box::new(move || unsafe {
                Box::from_raw(value_ptr);
            }),
        };

        unsafe {
            let rwops_ptr = ll::SDL_AllocRW();

            *get_callbacks(rwops_ptr) = Box::into_raw(Box::new(callbacks));
            (*rwops_ptr).type_ = 0;
            (*rwops_ptr).size = stream_size;
            (*rwops_ptr).seek = stream_seek;
            (*rwops_ptr).read = stream_read;
            (*rwops_ptr).write = stream_write;
            (*rwops_ptr).close = stream_close;

            RWops::from_ll(rwops_ptr)
        }
    }
}
