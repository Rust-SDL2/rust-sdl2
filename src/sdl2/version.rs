/*!
Querying SDL Version
 */

use std::ffi::CStr;
use std::fmt;

use sys::version as ll;

/// A structure that contains information about the version of SDL in use.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Version {
    /// major version
    pub major: u8,
    /// minor version
    pub minor: u8,
    /// update version (patchlevel)
    pub patch: u8,
}

impl Version {
    /// Convert a raw *SDL_version to Version.
    pub fn from_ll(v: ll::SDL_version) -> Version {
        Version { major: v.major, minor: v.minor, patch: v.patch }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Get the version of SDL that is linked against your program.
pub fn version() -> Version {
    unsafe {
        let mut cver = ll::SDL_version { major: 0, minor: 0, patch: 0};
        ll::SDL_GetVersion(&mut cver);
        Version::from_ll(cver)
    }
}

/// Get the code revision of SDL that is linked against your program.
pub fn revision() -> String {
    unsafe {
        let rev = ll::SDL_GetRevision();
        String::from_utf8_lossy(CStr::from_ptr(rev).to_bytes()).to_string()
    }
}

/// Get the revision number of SDL that is linked against your program.
pub fn revision_number() -> i32 {
    unsafe {
        ll::SDL_GetRevisionNumber()
    }
}
