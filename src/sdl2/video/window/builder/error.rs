use std::error::Error;
use std::ffi::NulError;
use std::fmt;

#[derive(Debug)]
pub enum WindowBuildError {
    HeightOverflows(u32),
    WidthOverflows(u32),
    InvalidTitle(NulError),
    SdlError(String),
}

impl fmt::Display for WindowBuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::WindowBuildError::*;

        match *self {
            HeightOverflows(h) => write!(f, "Window height ({}) is too high.", h),
            WidthOverflows(w) => write!(f, "Window width ({}) is too high.", w),
            InvalidTitle(ref e) => write!(f, "Invalid window title: {}", e),
            SdlError(ref e) => write!(f, "SDL error: {}", e),
        }
    }
}

impl Error for WindowBuildError {
    fn description(&self) -> &str {
        use self::WindowBuildError::*;

        match *self {
            HeightOverflows(_) => "window height overflow",
            WidthOverflows(_) => "window width overflow",
            InvalidTitle(_) => "invalid window title",
            SdlError(ref e) => e,
        }
    }
}
