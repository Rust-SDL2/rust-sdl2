use crate::video::GLProfile;

pub trait GLAttrTypeUtil {
    fn to_gl_value(self) -> i32;
    fn from_gl_value(value: i32) -> Self;
}

impl GLAttrTypeUtil for u8 {
    fn to_gl_value(self) -> i32 {
        self as i32
    }
    fn from_gl_value(value: i32) -> u8 {
        value as u8
    }
}

impl GLAttrTypeUtil for bool {
    fn to_gl_value(self) -> i32 {
        if self {
            1
        } else {
            0
        }
    }
    fn from_gl_value(value: i32) -> bool {
        value != 0
    }
}

impl GLAttrTypeUtil for GLProfile {
    fn to_gl_value(self) -> i32 {
        use self::GLProfile::*;

        match self {
            Unknown(i) => i,
            Core => 1,
            Compatibility => 2,
            GLES => 4,
        }
    }
    fn from_gl_value(value: i32) -> GLProfile {
        use self::GLProfile::*;

        match value {
            1 => Core,
            2 => Compatibility,
            4 => GLES,
            i => Unknown(i),
        }
    }
}
