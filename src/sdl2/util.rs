use std::ffi::{CString, NulError};

use SdlResult;

pub trait CStringExt {
    /// Returns an SDL error if the string contains a nul byte.
    fn unwrap_or_sdlresult(self) -> SdlResult<CString>;

    /// Removes any nul bytes so that they can be displayed in a C string.
    ///
    /// * Use this with functions that use the string for display purposes (such as Window titles or error messages).
    /// * Do not use this with functions that use the string as a lookup value (such as device names).
    fn remove_nul(self) -> CString;
}

impl CStringExt for Result<CString, NulError> {
    fn unwrap_or_sdlresult(self) -> SdlResult<CString> {
        self.or(Err(format!("argument string cannot contain an interior nul byte")))
    }

    fn remove_nul(self) -> CString {
        match self {
            Ok(value) => value,
            Err(e) => {
                let original_vec = e.into_vec();
                let v = original_vec.into_iter().filter(|&c| c != 0).collect();

                unsafe { CString::from_vec_unchecked(v) }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::CStringExt;
    use std::ffi::CString;

    #[test]
    fn test_cstring_ext() {
        assert!(CString::new("FooBar").unwrap_or_sdlresult().is_ok());
        assert!(CString::new("Foo\0Bar").unwrap_or_sdlresult().is_err());
        assert_eq!(CString::new("Foo\0Bar\0").remove_nul(), CString::new("FooBar").unwrap());
    }
}
