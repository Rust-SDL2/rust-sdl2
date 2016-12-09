use std::ffi::{CStr, CString, NulError};
use std::fmt;
use get_error;
use libc::c_char;

use sys::filesystem as ll;

pub fn base_path() -> Result<String, String> {
    let result = unsafe {
        let buf = ll::SDL_GetBasePath();
        CStr::from_ptr(buf as *const _).to_str().unwrap().to_owned()
    };

    if result.len() == 0 {
        Err(get_error())
    } else {
        Ok(result)
    }
}

#[derive(Debug)]
pub enum PrefPathError {
    InvalidOrganizationName(NulError),
    InvalidApplicationName(NulError),
    SdlError(String),
}

impl fmt::Display for PrefPathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::PrefPathError::*;

        match *self {
            InvalidOrganizationName(ref e) => write!(f, "Invalid organization name: {}", e),
            InvalidApplicationName(ref e) => write!(f, "Invalid application name: {}", e),
            SdlError(ref e) => write!(f, "SDL error: {}", e)
        }
    }
}

// TODO: Change to OsStr or something?
/// Return the preferred directory for the application to write files on this
/// system, based on the given organization and application name.
pub fn pref_path(org_name: &str, app_name: &str)
        -> Result<String, PrefPathError> {
    use self::PrefPathError::*;
    let result = unsafe {
        let org = match CString::new(org_name) {
            Ok(s) => s,
            Err(err) => return Err(InvalidOrganizationName(err)),
        };
        let app = match CString::new(app_name) {
            Ok(s) =>s,
            Err(err) => return Err(InvalidApplicationName(err)),
        };
        let buf = ll::SDL_GetPrefPath(org.as_ptr() as *const c_char, app.as_ptr() as *const c_char);
        CStr::from_ptr(buf as *const _).to_str().unwrap().to_owned()
    };

    if result.len() == 0 {
        Err(SdlError(get_error()))
    } else {
        Ok(result)
    }
}
