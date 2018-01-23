//! Framerate control

use libc;
use libc::{c_void, uint32_t, size_t};
use std::mem;
use ::get_error;
use super::ffi;

/// Structure holding the state and timing information of the framerate controller.
pub struct FPSManager {
    raw: *mut ffi::FPSmanager,
}

impl FPSManager {
    /// Create the framerate manager.
    pub fn new() -> FPSManager {
        unsafe {
            let size = mem::size_of::<ffi::FPSmanager>() as size_t;
            let raw = libc::malloc(size) as *mut ffi::FPSmanager;
            ffi::SDL_initFramerate(raw);
            FPSManager { raw: raw }
        }
    }

    /// Set the framerate in Hz.
    pub fn set_framerate(&mut self, rate: u32) -> Result<(), String> {
        let ret = unsafe { ffi::SDL_setFramerate(self.raw, rate as uint32_t) };
        match ret {
            0 => Ok(()),
            _ => Err(get_error())
        }
    }

    /// Return the current target framerate in Hz.
    pub fn get_framerate(&self) -> i32 {
        // will not get an error
        unsafe { ffi::SDL_getFramerate(self.raw) as i32 }
    }

    /// Return the current framecount.
    pub fn get_frame_count(&self) -> i32 {
        // will not get an error
        unsafe { ffi::SDL_getFramecount(self.raw) as i32 }
    }

    /// Delay execution to maintain a constant framerate and calculate fps.
    pub fn delay(&mut self) -> u32 {
        unsafe { ffi::SDL_framerateDelay(self.raw) as u32 }
    }
}

impl Drop for FPSManager {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut c_void) }
    }
}
