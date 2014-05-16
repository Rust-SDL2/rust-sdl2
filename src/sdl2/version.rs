use std::fmt;
use std::mem;
use std::c_str::CString;


#[allow(non_camel_case_types)]
pub mod ll {
    use libc::{uint8_t, c_char, c_int};
    pub struct SDL_version {
        pub major: uint8_t,
        pub minor: uint8_t,
        pub patch: uint8_t,
    }
    extern "C" {
        pub fn SDL_GetVersion(ver: *mut SDL_version);
        pub fn SDL_GetRevision() -> *c_char;
        pub fn SDL_GetRevisionNumber() -> c_int;
    }
}


#[deriving(Eq, Clone)]
pub struct Version {
    pub major: int,
    pub minor: int,
    pub patch: int,
}

impl Version {
    pub fn from_ll(sv: *ll::SDL_version) -> Version {
        //! Converts a raw *SDL_version to Version
        unsafe {
            let v = *sv;
            Version{ major: v.major as int, minor: v.minor as int, patch: v.patch as int }
        }
    }
}

impl fmt::Show for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

pub fn get_version() -> Version {
    unsafe {
        let mut cver = mem::init::<ll::SDL_version>();
        ll::SDL_GetVersion(&mut cver);
        Version::from_ll(&cver)
    }
}

pub fn get_revision() -> ~str {
    unsafe {
        let ret = ll::SDL_GetRevision();
        CString::new(ret, false).as_str().unwrap().into_owned()
    }
}

pub fn get_revision_number() -> int {
    unsafe {
        ll::SDL_GetRevisionNumber() as int
    }
}
