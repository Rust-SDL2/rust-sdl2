use std::ffi::CString;
use std::old_io as io;
use std::old_io::IoResult;
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
            let path_c = CString::from_slice(path.as_vec()).as_ptr();
            let mode_c = CString::from_slice(mode.as_bytes()).as_ptr();
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

impl Reader for RWops {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let out_len = buf.len() as size_t;
        // FIXME: it's better to use as_mut_ptr().
        // number of objects read, or 0 at error or end of file.
        let ret = unsafe {
            ((*self.raw).read)(self.raw, buf.as_ptr() as *const c_void, 1, out_len)
        };
        if ret == 0 {
            Err(io::standard_error(io::EndOfFile))
        } else {
            Ok(ret as usize)
        }
    }
}

impl Writer for RWops {
    fn write_all(&mut self, buf: &[u8]) -> IoResult<()> {
        let in_len = buf.len() as size_t;
        let ret = unsafe {
            ((*self.raw).write)(self.raw, buf.as_ptr() as *const c_void, 1, in_len)
        };
        if ret == 0 {
            Err(io::standard_error(io::EndOfFile))
        } else if ret != in_len {
            // FIXME: what error should we return here?
            Err(io::standard_error(io::EndOfFile))
        } else {
            Ok(())
        }
    }
}

impl Seek for RWops {
    fn tell(&self) -> IoResult<u64> {
        let ret = unsafe {
            ((*self.raw).seek)(self.raw, 0, ll::RW_SEEK_CUR)
        };
        if ret == -1 {
            Err(io::IoError::last_error())
        } else {
            Ok(ret as u64)
        }
    }

    fn seek(&mut self, pos: i64, style: io::SeekStyle) -> IoResult<()> {
        // whence code is different from SeekStyle
        let whence = match style {
            io::SeekSet => ll::RW_SEEK_SET,
            io::SeekEnd => ll::RW_SEEK_END,
            io::SeekCur => ll::RW_SEEK_CUR
        };
        let ret = unsafe {
            ((*self.raw).seek)(self.raw, pos, whence)
        };
        if ret == -1 {
            Err(io::IoError::last_error())
        } else {
            Ok(())
        }
    }
}
