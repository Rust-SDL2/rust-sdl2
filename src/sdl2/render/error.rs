use std::error::Error;
use std::fmt;

/// Contains the description of an error returned by SDL
#[derive(Debug)]
pub struct SdlError(pub(super) String);

/// Possible errors returned by targeting a `Canvas` to render to a `Texture`
#[derive(Debug)]
pub enum TargetRenderError {
    SdlError(SdlError),
    NotSupported,
}

impl fmt::Display for SdlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &SdlError(ref e) = self;
        write!(f, "SDL error: {}", e)
    }
}

impl Error for SdlError {
    fn description(&self) -> &str {
        let &SdlError(ref e) = self;
        e
    }
}

impl fmt::Display for TargetRenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TargetRenderError::*;
        match *self {
            SdlError(ref e) => e.fmt(f),
            NotSupported => write!(f, "The renderer does not support the use of render targets"),
        }
    }
}

impl Error for TargetRenderError {
    fn description(&self) -> &str {
        use self::TargetRenderError::*;
        match *self {
            SdlError(self::SdlError(ref e)) => e.as_str(),
            NotSupported => "The renderer does not support the use of render targets",
        }
    }
}
