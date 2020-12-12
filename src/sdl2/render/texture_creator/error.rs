use std::error::Error;
use std::fmt;

use crate::pixels::PixelFormatEnum;

#[derive(Debug)]
pub enum TextureValueError {
    WidthOverflows(u32),
    HeightOverflows(u32),
    WidthMustBeMultipleOfTwoForFormat(u32, PixelFormatEnum),
    SdlError(String),
}

impl fmt::Display for TextureValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TextureValueError::*;

        match *self {
            WidthOverflows(value) => write!(f, "Integer width overflows ({})", value),
            HeightOverflows(value) => write!(f, "Integer height overflows ({})", value),
            WidthMustBeMultipleOfTwoForFormat(value, format) => {
                write!(
                    f,
                    "Texture width must be multiple of two for pixel format '{:?}' ({})",
                    format, value
                )
            }
            SdlError(ref e) => write!(f, "SDL error: {}", e),
        }
    }
}

impl Error for TextureValueError {
    fn description(&self) -> &str {
        use self::TextureValueError::*;

        match *self {
            WidthOverflows(_) => "texture width overflow",
            HeightOverflows(_) => "texture height overflow",
            WidthMustBeMultipleOfTwoForFormat(..) => "texture width must be multiple of two",
            SdlError(ref e) => e,
        }
    }
}
