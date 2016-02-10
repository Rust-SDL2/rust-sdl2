use std::ffi::{CString, NulError};
use std::ops::Add;

/// A given ID was too big (made the i32 representation become negative),
/// or something else bad happened in the C layer.
#[derive(Debug)]
pub enum IdOrSdlError {
    IdTooBig(u32),
    SdlError(String)
}

/// Validates and converts the given u32 to a positive C integer.
pub fn validate_int(value: u32) -> Option<::libc::c_int> {
    // Many SDL functions will accept `int` values, even if it doesn't make sense 
    // for the values to be negative.
    // In the cases that SDL doesn't check negativity, passing negative values 
    // could be unsafe.
    // For example, `SDL_JoystickGetButton` uses the index argument to access an 
    // array without checking if it's negative, which could potentially lead to 
    // segmentation faults.
    if value >= 1 << 31 { 
        None
    } else { 
        Some(value as ::libc::c_int)
    }
}

#[cfg(test)]
mod test {
}
