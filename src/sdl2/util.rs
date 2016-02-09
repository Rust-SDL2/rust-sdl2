use std::ffi::{CString, NulError};
use std::ops::Add;

use SdlResult;
use ErrorMessage;

/// Validates and converts the given u32 to a positive C integer.
pub fn validate_int(value: u32) -> Result<::libc::c_int, String> {
    // Many SDL functions will accept `int` values, even if it doesn't make sense 
    // for the values to be negative.
    // In the cases that SDL doesn't check negativity, passing negative values 
    // could be unsafe.
    // For example, `SDL_JoystickGetButton` uses the index argument to access an 
    // array without checking if it's negative, which could potentially lead to 
    // segmentation faults.
    if value >= 1 << 31 { 
        Err(format!("`{}` is out of bounds.", value))
    } else { 
        Ok(value as ::libc::c_int)
    }
}

pub struct CheckedInteger {
    value: u32,
}
impl CheckedInteger {
    pub fn new(value: u32) -> Result<CheckedInteger, String> {
        if value >= 1 << 31 { 
            Err(format!("The value '{}' is too big for a C int.", value))
        } else { 
            Ok(CheckedInteger { value: value } )
        }
    }
    
    pub fn add(&self, value: u32) -> Result<CheckedInteger, String> {
        if let Some(new) = self.value.checked_add(value) {
            CheckedInteger::new(new)
        } else {
            Err("The combined value overflowed".to_owned())
        }
    }
    
    pub fn sub(&self, value: u32) -> Result<CheckedInteger, String> {
        if let Some(new) = self.value.checked_add(value) {
            CheckedInteger::new(new)
        } else {
            Err("The combined value underflowed".to_owned())
        }
    }
    
    pub fn value(&self) -> u32 {
        self.value
    }
}


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
        self.or(Err(ErrorMessage("argument string cannot contain an interior nul byte".into())))
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
