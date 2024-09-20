use crate::get_error;
use libc::c_void;
use std::error;
use std::ffi::{CStr, CString, NulError};
use std::fmt;

use crate::sys;

#[doc(alias = "SDL_GetBasePath")]
pub fn base_path() -> Result<String, String> {
    unsafe {
        let buf = sys::SDL_GetBasePath();
        if buf.is_null() {
            Err(get_error())
        } else {
            let s = CStr::from_ptr(buf).to_str().unwrap().to_owned();
            sys::SDL_free(buf as *mut c_void);
            Ok(s)
        }
    }
}

#[derive(Debug, Clone)]
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
            SdlError(ref e) => write!(f, "SDL error: {}", e),
        }
    }
}

impl error::Error for PrefPathError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::InvalidOrganizationName(err) => Some(err),
            Self::InvalidApplicationName(err) => Some(err),
            Self::SdlError(_) => None,
        }
    }
}

// TODO: Change to OsStr or something?
/// Return the preferred directory for the application to write files on this
/// system, based on the given organization and application name.
#[doc(alias = "SDL_GetPrefPath")]
pub fn pref_path(org_name: &str, app_name: &str) -> Result<String, PrefPathError> {
    use self::PrefPathError::*;

    let org = CString::new(org_name).map_err(InvalidOrganizationName)?;
    let app = CString::new(app_name).map_err(InvalidApplicationName)?;

    unsafe {
        let buf = sys::SDL_GetPrefPath(org.as_ptr(), app.as_ptr());
        if buf.is_null() {
            Err(SdlError(get_error()))
        } else {
            let ret = CStr::from_ptr(buf).to_str().unwrap().to_owned();
            sys::SDL_free(buf as *mut c_void);
            Ok(ret)
        }
    }
}
