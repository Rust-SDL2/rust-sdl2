use std::error;
use std::error::Error;
use std::ffi::NulError;
use sdl2::ErrorMessage;
use std::fmt;

/// The result of an SDL2_TTF font operation.
pub type SdlTtfResult<T> = Result<T, SdlTtfError>;

/// A font-related error.
#[derive(Debug)]
pub enum SdlTtfError {
    /// A Latin-1 encoded byte string is invalid.
    InvalidLatin1Text(NulError),
    /// A SDL2-related error occured.
    SdlError(ErrorMessage),
}

impl error::Error for SdlTtfError {
    fn description(&self) -> &str {
        match self {
            &SdlTtfError::InvalidLatin1Text(ref error) => {
                error.description()
            },
            &SdlTtfError::SdlError(ref message) => {
                message.description()
            },
        }
    }

    fn cause<'a>(&'a self) -> Option<&'a error::Error> {
        match self {
            &SdlTtfError::InvalidLatin1Text(ref error) => {
                Some(error)
            },
            &SdlTtfError::SdlError(_) => {
                None
            },
        }
    }
}

impl fmt::Display for SdlTtfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &SdlTtfError::InvalidLatin1Text(ref err) => {
                write!(f, "Invalid Latin-1 bytes: {}", err.description())
            },
            &SdlTtfError::SdlError(ref msg) => {
                write!(f, "SDL2 error: {}", msg)
            },
        }
        
    }
}