/*!
Querying SDL Version
 */

use std::ffi::c_str_to_bytes;
use std::fmt;

pub use sys::version as ll;

/// A structure that contains information about the version of SDL in use.
#[derive(PartialEq, Copy, Clone)]
pub struct Version {
    /// major version
    pub major: int,
    /// minor version
    pub minor: int,
    /// update version (patchlevel)
    pub patch: int,
}

impl Version {
    /// Convert a raw *SDL_version to Version.
    pub fn from_ll(sv: *const ll::SDL_version) -> Version {
        unsafe {
            let ref v = *sv;
            Version{ major: v.major as int, minor: v.minor as int, patch: v.patch as int }
        }
    }
}

impl fmt::Show for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Get the version of SDL that is linked against your program.
pub fn get_version() -> Version {
    unsafe {
        let mut cver = ll::SDL_version { major: 0, minor: 0, patch: 0};
        ll::SDL_GetVersion(&mut cver);
        Version::from_ll(&cver)
    }
}

/// Get the code revision of SDL that is linked against your program.
pub fn get_revision() -> String {
    unsafe {
        let rev = ll::SDL_GetRevision();
        String::from_utf8_lossy(c_str_to_bytes(&rev)).to_string()
    }
}

/// Get the revision number of SDL that is linked against your program.
pub fn get_revision_number() -> int {
    unsafe {
        ll::SDL_GetRevisionNumber() as int
    }
}
