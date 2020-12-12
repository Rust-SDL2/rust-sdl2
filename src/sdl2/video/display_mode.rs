use std::ptr;

use libc::c_int;

use crate::pixels::PixelFormatEnum;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DisplayMode {
    pub format: PixelFormatEnum,
    pub w: i32,
    pub h: i32,
    pub refresh_rate: i32,
}

impl DisplayMode {
    pub fn new(format: PixelFormatEnum, w: i32, h: i32, refresh_rate: i32) -> DisplayMode {
        DisplayMode {
            format,
            w,
            h,
            refresh_rate,
        }
    }

    pub fn from_ll(raw: &sys::SDL_DisplayMode) -> DisplayMode {
        use std::convert::TryFrom;
        DisplayMode::new(
            PixelFormatEnum::try_from(raw.format as u32).unwrap_or(PixelFormatEnum::Unknown),
            raw.w as i32,
            raw.h as i32,
            raw.refresh_rate as i32,
        )
    }

    pub fn to_ll(&self) -> sys::SDL_DisplayMode {
        sys::SDL_DisplayMode {
            format: self.format as u32,
            w: self.w as c_int,
            h: self.h as c_int,
            refresh_rate: self.refresh_rate as c_int,
            driverdata: ptr::null_mut(),
        }
    }
}
