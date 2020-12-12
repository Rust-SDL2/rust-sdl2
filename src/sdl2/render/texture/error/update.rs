use std::error::Error;
use std::fmt;

use crate::pixels::PixelFormatEnum;

#[derive(Debug)]
pub enum UpdateTextureError {
    PitchOverflows(usize),
    PitchMustBeMultipleOfTwoForFormat(usize, PixelFormatEnum),
    XMustBeMultipleOfTwoForFormat(i32, PixelFormatEnum),
    YMustBeMultipleOfTwoForFormat(i32, PixelFormatEnum),
    WidthMustBeMultipleOfTwoForFormat(u32, PixelFormatEnum),
    HeightMustBeMultipleOfTwoForFormat(u32, PixelFormatEnum),
    SdlError(String),
}

impl fmt::Display for UpdateTextureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::UpdateTextureError::*;

        match *self {
            PitchOverflows(value) => write!(f, "Pitch overflows ({})", value),
            PitchMustBeMultipleOfTwoForFormat(value, format) => {
                write!(
                    f,
                    "Pitch must be multiple of two for pixel format '{:?}' ({})",
                    format, value
                )
            }
            XMustBeMultipleOfTwoForFormat(value, format) => {
                write!(
                    f,
                    "X must be multiple of two for pixel format '{:?}' ({})",
                    format, value
                )
            }
            YMustBeMultipleOfTwoForFormat(value, format) => {
                write!(
                    f,
                    "Y must be multiple of two for pixel format '{:?}' ({})",
                    format, value
                )
            }
            WidthMustBeMultipleOfTwoForFormat(value, format) => {
                write!(
                    f,
                    "Width must be multiple of two for pixel format '{:?}' ({})",
                    format, value
                )
            }
            HeightMustBeMultipleOfTwoForFormat(value, format) => {
                write!(
                    f,
                    "Height must be multiple of two for pixel format '{:?}' ({})",
                    format, value
                )
            }
            SdlError(ref e) => write!(f, "SDL error: {}", e),
        }
    }
}

impl Error for UpdateTextureError {
    fn description(&self) -> &str {
        use self::UpdateTextureError::*;

        match *self {
            PitchOverflows(_) => "pitch overflow",
            PitchMustBeMultipleOfTwoForFormat(..) => "pitch must be multiple of two",
            XMustBeMultipleOfTwoForFormat(..) => "x must be multiple of two",
            YMustBeMultipleOfTwoForFormat(..) => "y must be multiple of two",
            WidthMustBeMultipleOfTwoForFormat(..) => "width must be multiple of two",
            HeightMustBeMultipleOfTwoForFormat(..) => "height must be multiple of two",
            SdlError(ref e) => e,
        }
    }
}
