use std::ffi::{c_str_to_bytes, CString};
use SdlResult;
use get_error;

pub use sys::filesystem as ll;

pub fn get_base_path() -> SdlResult<String> {
    let result = unsafe {
        let buf = ll::SDL_GetBasePath();
        String::from_utf8_lossy(c_str_to_bytes(&buf)).to_string()
    };

    if result.len() == 0 {
        Err(get_error())
    } else {
        Ok(result)
    }
}

pub fn get_pref_path(org: &str, app: &str) -> SdlResult<String> {
    let result = unsafe {
        let org_cstr = CString::from_slice(org.as_bytes()).as_ptr();
        let app_cstr = CString::from_slice(app.as_bytes()).as_ptr();
        let buf = ll::SDL_GetPrefPath(org_cstr, app_cstr);
        String::from_utf8_lossy(c_str_to_bytes(&buf)).to_string()
    };

    if result.len() == 0 {
        Err(get_error())
    } else {
        Ok(result)
    }
}

