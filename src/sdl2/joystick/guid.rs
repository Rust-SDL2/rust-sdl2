use crate::sys;

use libc::c_char;
use std::ffi::{CStr, CString, NulError};
use std::fmt::{Display, Error, Formatter};

/// Wrapper around a `SDL_JoystickGUID`, a globally unique identifier
/// for a joystick.
#[derive(Copy, Clone)]
pub struct Guid {
    raw: sys::SDL_JoystickGUID,
}

impl PartialEq for Guid {
    fn eq(&self, other: &Guid) -> bool {
        self.raw.data == other.raw.data
    }
}

impl Eq for Guid {}

impl Guid {
    pub(super) fn new(raw: sys::SDL_JoystickGUID) -> Self {
        Self { raw }
    }

    /// Create a GUID from a string representation.
    #[doc(alias = "SDL_JoystickGetGUIDFromString")]
    pub fn from_string(guid: &str) -> Result<Guid, NulError> {
        let guid = CString::new(guid)?;

        let raw = unsafe { sys::SDL_JoystickGetGUIDFromString(guid.as_ptr() as *const c_char) };

        Ok(Guid { raw })
    }

    /// Return `true` if GUID is full 0s
    pub fn is_zero(&self) -> bool {
        for &i in &self.raw.data {
            if i != 0 {
                return false;
            }
        }

        true
    }

    /// Return a String representation of GUID
    #[doc(alias = "SDL_JoystickGetGUIDString")]
    pub fn string(&self) -> String {
        // Doc says "buf should supply at least 33bytes". I took that
        // to mean that 33bytes should be enough in all cases, but
        // maybe I'm wrong?
        let mut buf = [0; 33];

        let len = buf.len() as i32;
        let c_str = buf.as_mut_ptr();

        unsafe {
            sys::SDL_JoystickGetGUIDString(self.raw, c_str, len);
        }

        // The buffer should always be NUL terminated (the
        // documentation doesn't explicitly say it but I checked the
        // code)
        if c_str.is_null() {
            String::new()
        } else {
            unsafe {
                CStr::from_ptr(c_str as *const _)
                    .to_str()
                    .unwrap()
                    .to_string()
            }
        }
    }

    /// Return a copy of the internal SDL_JoystickGUID
    pub fn raw(self) -> sys::SDL_JoystickGUID {
        self.raw
    }
}

impl Display for Guid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.string())
    }
}
