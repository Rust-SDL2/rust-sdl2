use std::c_str::ToCStr;

use SdlResult;
use get_error;

pub use sys::filesystem as ll;

pub fn get_base_path() -> SdlResult<String> {
    let result = unsafe {
        let cstr = ll::SDL_GetBasePath();
        String::from_raw_buf(cstr as *const u8)
    };

    if result.len() == 0 {
        Err(get_error())
    } else {
        Ok(result)
    }
}

pub fn get_pref_path(org: &str, app: &str) -> SdlResult<String> {
    let result = unsafe {
        let cstr =
            org.with_c_str(|org_cstr| {
            app.with_c_str(|app_cstr| {
                ll::SDL_GetPrefPath(org_cstr, app_cstr)
            })});
        String::from_raw_buf(cstr as *const u8)
    };

    if result.len() == 0 {
        Err(get_error())
    } else {
        Ok(result)
    }
}

