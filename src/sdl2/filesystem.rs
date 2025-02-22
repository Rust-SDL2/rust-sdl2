use crate::{get_error, Error};
use libc::c_void;
use std::ffi::CStr;

use crate::sys;

#[doc(alias = "SDL_GetBasePath")]
pub fn base_path() -> Result<String, String> {
    let result = unsafe {
        let buf = sys::SDL_GetBasePath();
        let s = CStr::from_ptr(buf as *const _).to_str().unwrap().to_owned();
        sys::SDL_free(buf as *mut c_void);
        s
    };

    if result.is_empty() {
        Err(get_error())
    } else {
        Ok(result)
    }
}

// TODO: Change to OsStr or something?
/// Return the preferred directory for the application to write files on this
/// system, based on the given organization and application name.
#[doc(alias = "SDL_GetPrefPath")]
pub fn pref_path(org_name: &str, app_name: &str) -> Result<String, Error> {
    unsafe {
        let buf = sys::SDL_GetPrefPath(
            as_cstring!(org_name)?.as_ptr(),
            as_cstring!(app_name)?.as_ptr(),
        );
        if buf.is_null() {
            Err(Error::from_sdl_error())
        } else {
            Ok(CStr::from_ptr(buf as *const _).to_str().unwrap().to_owned())
        }
    }
}
