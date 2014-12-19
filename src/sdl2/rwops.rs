use std::io;
use std::io::IoResult;
use libc::{c_void, c_int, size_t};
use get_error;
use SdlResult;

#[allow(non_camel_case_types)]
pub mod ll {
    use libc::{c_uchar, uint32_t, c_char, FILE, c_void};
    use libc::{c_int, int64_t, size_t};

    #[allow(dead_code)]
    #[repr(C)]
    struct SDL_RWops_Anon {
        data: [c_uchar, ..24],
    }

    pub type SDL_bool = c_int;

    pub static RW_SEEK_SET: c_int = 0;
    pub static RW_SEEK_CUR: c_int = 1;
    pub static RW_SEEK_END: c_int = 2;

    #[allow(dead_code)]
    #[repr(C)]
    pub struct SDL_RWops {
        pub size:  extern "C" fn(context: *const SDL_RWops) -> int64_t,
        pub seek:  extern "C" fn(context: *const SDL_RWops, offset: int64_t, whence: c_int) -> int64_t,
        pub read:  extern "C" fn(context: *const SDL_RWops, ptr: *const c_void,
                                 size: size_t, maxnum: size_t) -> size_t,
        pub write: extern "C" fn(context: *const SDL_RWops, ptr: *const c_void,
                                 size: size_t, maxnum: size_t) -> size_t,
        pub close: extern "C" fn(context: *const SDL_RWops) -> c_int,
        pub _type: uint32_t,
        hidden: SDL_RWops_Anon
    }

    extern "C" {
        pub fn SDL_RWFromFile(file: *const c_char, mode: *const c_char) -> *const SDL_RWops;
        pub fn SDL_RWFromFP(fp: *const FILE, autoclose: SDL_bool) -> *const SDL_RWops;
        pub fn SDL_RWFromMem(mem: *const c_void, size: c_int) -> *const SDL_RWops;
        pub fn SDL_RWFromConstMem(mem: *const c_void, size: c_int) -> *const SDL_RWops;

        pub fn SDL_AllocRW() -> *const SDL_RWops;
        pub fn SDL_FreeRW(area: *const SDL_RWops);
    }
}

#[deriving(PartialEq)] #[allow(raw_pointer_deriving)]
pub struct RWops {
    raw: *const ll::SDL_RWops,
    close_on_drop: bool
}

impl_raw_accessors!(RWops, *const ll::SDL_RWops);
impl_owned_accessors!(RWops, close_on_drop);

/// A structure that provides an abstract interface to stream I/O.
impl RWops {
    pub fn from_file(path: &Path, mode: &str) -> SdlResult<RWops> {
        let raw = unsafe {
            ll::SDL_RWFromFile(path.to_c_str().into_inner(), mode.to_c_str().into_inner())
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

    pub fn len(&self) -> uint {
        unsafe {
            ((*self.raw).size)(self.raw) as uint
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
    fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
        let out_len = buf.len() as size_t;
        // FIXME: it's better to use as_mut_ptr().
        // number of objects read, or 0 at error or end of file.
        let ret = unsafe {
            ((*self.raw).read)(self.raw, buf.as_ptr() as *const c_void, 1, out_len)
        };
        if ret == 0 {
            Err(io::standard_error(io::EndOfFile))
        } else {
            Ok(ret as uint)
        }
    }
}

impl Writer for RWops {
    fn write(&mut self, buf: &[u8]) -> IoResult<()> {
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
