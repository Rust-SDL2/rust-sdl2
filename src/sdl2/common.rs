use std::ffi::{CString, NulError};
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    Sdl(String),
    /// An integer was larger than [`i32::MAX`] in a parameter, and it can't be converted to a C int
    IntOverflow(&'static str, u32),
    /// A null byte was found within a parameter, and it can't be sent to SDL
    InvalidString(NulError, &'static str),
}

impl Error {
    pub fn from_sdl_error() -> Self {
        Self::Sdl(crate::get_error())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sdl(msg) => write!(f, "SDL error: {msg}"),
            Self::IntOverflow(name, value) => write!(f, "Integer '{name}' overflows: {value}"),
            Self::InvalidString(name, nul) => write!(f, "Invalid string '{name}': {nul}"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Sdl(..) | Self::IntOverflow(..) => None,
            Self::InvalidString(nul, _) => Some(nul),
        }
    }
}

pub fn validate_string(str: impl Into<Vec<u8>>, name: &'static str) -> Result<CString, Error> {
    match CString::new(str) {
        Ok(c) => Ok(c),
        Err(nul) => Err(Error::InvalidString(nul, name)),
    }
}

/// Validates and converts the given u32 to a positive C integer.
pub fn validate_int(value: u32, name: &'static str) -> Result<libc::c_int, Error> {
    // Many SDL functions will accept `int` values, even if it doesn't make sense
    // for the values to be negative.
    // In the cases that SDL doesn't check negativity, passing negative values
    // could be unsafe.
    // For example, `SDL_JoystickGetButton` uses the index argument to access an
    // array without checking if it's negative, which could potentially lead to
    // segmentation faults.
    if value > libc::c_int::MAX as u32 {
        Err(Error::IntOverflow(name, value))
    } else {
        Ok(value as libc::c_int)
    }
}
