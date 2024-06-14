//! Querying SDL Version

use std::ffi::CStr;
use std::fmt;

use crate::sys;

/// A structure that contains a version of SDL.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, PartialOrd, Ord)]
pub struct Version {
    /// major version
    pub major: u8,
    /// minor version
    pub minor: u8,
    /// update version (patchlevel)
    pub patch: u8,
}

impl Version {
    /// The version of SDL that was used to generate the bindings. This may differ from the version
    /// used at runtime, use [`version`] to get that.
    pub const COMPILE_TIME_VERSION: Self = Self {
        major: sys::SDL_MAJOR_VERSION as u8,
        minor: sys::SDL_MINOR_VERSION as u8,
        patch: sys::SDL_PATCHLEVEL as u8,
    };

    /// Convert a raw SDL_version to Version.
    pub fn from_ll(v: sys::SDL_version) -> Version {
        Version {
            major: v.major,
            minor: v.minor,
            patch: v.patch,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Get the version of SDL that is linked against your program.
#[doc(alias = "SDL_GetVersion")]
pub fn version() -> Version {
    unsafe {
        let mut cver = sys::SDL_version {
            major: 0,
            minor: 0,
            patch: 0,
        };
        sys::SDL_GetVersion(&mut cver);
        Version::from_ll(cver)
    }
}

/// Get the code revision of SDL that is linked against your program.
#[doc(alias = "SDL_GetRevision")]
pub fn revision() -> String {
    unsafe {
        let rev = sys::SDL_GetRevision();
        CStr::from_ptr(rev as *const _).to_str().unwrap().to_owned()
    }
}

/// Get the revision number of SDL that is linked against your program.
#[doc(alias = "SDL_GetRevisionNumber")]
pub fn revision_number() -> i32 {
    unsafe { sys::SDL_GetRevisionNumber() }
}
