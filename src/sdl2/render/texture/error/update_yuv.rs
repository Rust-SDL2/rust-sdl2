use std::error::Error;
use std::fmt;

use crate::rect::Rect;

#[derive(Debug)]
pub enum UpdateTextureYUVError {
    PitchOverflows {
        plane: &'static str,
        value: usize,
    },
    InvalidPlaneLength {
        plane: &'static str,
        length: usize,
        pitch: usize,
        height: usize,
    },
    XMustBeMultipleOfTwoForFormat(i32),
    YMustBeMultipleOfTwoForFormat(i32),
    WidthMustBeMultipleOfTwoForFormat(u32),
    HeightMustBeMultipleOfTwoForFormat(u32),
    RectNotInsideTexture(Rect),
    SdlError(String),
}

impl fmt::Display for UpdateTextureYUVError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::UpdateTextureYUVError::*;

        match *self {
            PitchOverflows { plane, value } => {
                write!(f, "Pitch overflows on {} plane ({})", plane, value)
            }
            InvalidPlaneLength {
                plane,
                length,
                pitch,
                height,
            } => {
                write!(
                    f,
                    "The {} plane is wrong length ({}, should be {} * {})",
                    plane, length, pitch, height
                )
            }
            XMustBeMultipleOfTwoForFormat(value) => {
                write!(f, "X must be multiple of two ({})", value)
            }
            YMustBeMultipleOfTwoForFormat(value) => {
                write!(f, "Y must be multiple of two ({})", value)
            }
            WidthMustBeMultipleOfTwoForFormat(value) => {
                write!(f, "Width must be multiple of two ({})", value)
            }
            HeightMustBeMultipleOfTwoForFormat(value) => {
                write!(f, "Height must be multiple of two ({})", value)
            }
            RectNotInsideTexture(_) => write!(f, "Rect must be inside texture"),
            SdlError(ref e) => write!(f, "SDL error: {}", e),
        }
    }
}

impl Error for UpdateTextureYUVError {
    fn description(&self) -> &str {
        use self::UpdateTextureYUVError::*;

        match *self {
            PitchOverflows { .. } => "pitch overflow",
            InvalidPlaneLength { .. } => "invalid plane length",
            XMustBeMultipleOfTwoForFormat(_) => "x must be multiple of two",
            YMustBeMultipleOfTwoForFormat(_) => "y must be multiple of two",
            WidthMustBeMultipleOfTwoForFormat(_) => "width must be multiple of two",
            HeightMustBeMultipleOfTwoForFormat(_) => "height must be multiple of two",
            RectNotInsideTexture(_) => "rect must be inside texture",
            SdlError(ref e) => e,
        }
    }
}
