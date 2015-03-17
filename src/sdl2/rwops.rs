use std::ffi::{CString, AsOsStr};
use std::io;
use std::path::Path;
use libc::{c_void, c_int, size_t};
use get_error;
use SdlResult;

use sys::rwops as ll;

#[derive(PartialEq)] #[allow(raw_pointer_derive)]
pub struct RWops {
    raw: *const ll::SDL_RWops,
    close_on_drop: bool
}

impl_raw_accessors!((RWops, *const ll::SDL_RWops));
impl_owned_accessors!((RWops, close_on_drop));

/// A structure that provides an abstract interface to stream I/O.
impl RWops {
    pub fn from_file(path: &Path, mode: &str) -> SdlResult<RWops> {
        let raw = unsafe {
            let path_c = CString::new(
                path.as_os_str().to_str().unwrap()).unwrap().as_ptr();
            let mode_c = CString::new(mode).unwrap().as_ptr();
            ll::SDL_RWFromFile(path_c, mode_c)
        };
        if raw.is_null() { Err(get_error()) }
        else { Ok(RWops{raw: raw, close_on_drop: true}) }
    }

    pub fn from_bytes(buf: &[u8]) -> SdlResult<RWops> {
        let raw = unsafe {
            ll::SDL_RWFromConstMem(buf.as_ptr() as *const c_void, buf.len() as c_int)
        };
        if raw.is_null() { Err(get_error()) }
        else { Ok(RWops{raw: raw, close_on_drop: false}) }
    }

    pub fn len(&self) -> usize {
        unsafe {
            ((*self.raw).size)(self.raw) as usize
        }
    }
}

impl Drop for RWops {
    fn drop(&mut self) {
        // TODO: handle close error
        if self.close_on_drop {
            let ret = unsafe { ((*self.raw).close)(self.raw) };
            if ret != 0 {
                println!("error {} when closing RWopt", get_error());
            }
        }
    }
}

impl io::Read for RWops {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let out_len = buf.len() as size_t;
        // FIXME: it's better to use as_mut_ptr().
        // number of objects read, or 0 at error or end of file.
        let ret = unsafe {
            ((*self.raw).read)(self.raw, buf.as_ptr() as *const c_void, 1, out_len)
        };
        Ok(ret as usize)
    }
}

impl io::Write for RWops {
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

impl io::Seek for RWops {
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
