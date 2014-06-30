use std::str;
use SdlResult;
use get_error;

#[allow(non_camel_case_types)]
pub mod ll {
    use libc::{c_char};

    extern "C" {
        pub fn SDL_GetBasePath() -> *const c_char;
        pub fn SDL_GetPrefPath(arg: *const c_char, app: *const c_char) -> *const c_char;
    }
}

pub fn get_base_path() -> SdlResult<String> {
    let result = unsafe {
        let cstr = ll::SDL_GetBasePath();
        str::raw::from_c_str(cstr)
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
        str::raw::from_c_str(cstr)
    };

    if result.len() == 0 {
        Err(get_error())
    } else {
        Ok(result)
    }
}

