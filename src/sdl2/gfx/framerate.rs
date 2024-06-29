//! Framerate control

use get_error;
use std::mem::MaybeUninit;
use sys::gfx;

/// Structure holding the state and timing information of the framerate controller.
#[derive(Debug, Clone)]
pub struct FPSManager {
    raw: gfx::framerate::FPSmanager,
}

impl Default for FPSManager {
    fn default() -> Self {
        Self::new()
    }
}

impl FPSManager {
    /// Create the framerate manager.
    pub fn new() -> FPSManager {
        unsafe {
            let mut raw = MaybeUninit::uninit();
            gfx::framerate::SDL_initFramerate(raw.as_mut_ptr());
            FPSManager {
                raw: raw.assume_init(),
            }
        }
    }

    /// Set the framerate in Hz.
    pub fn set_framerate(&mut self, rate: u32) -> Result<(), String> {
        let ret = unsafe { gfx::framerate::SDL_setFramerate(&mut self.raw, rate) };
        match ret {
            0 => Ok(()),
            _ => Err(get_error()),
        }
    }

    /// Return the current target framerate in Hz.
    pub fn get_framerate(&self) -> i32 {
        // will not get an error
        // SAFETY: SDL_getFramerate will not mutate self.raw, even though it accepts *mut FPSmanager.
        unsafe { gfx::framerate::SDL_getFramerate(&self.raw as *const _ as *mut _) as i32 }
    }

    /// Return the current framecount.
    pub fn get_frame_count(&self) -> i32 {
        // will not get an error
        // SAFETY: SDL_getFramecount will not mutate self.raw, even though it accepts *mut FPSmanager.
        unsafe { gfx::framerate::SDL_getFramecount(&self.raw as *const _ as *mut _) as i32 }
    }

    /// Delay execution to maintain a constant framerate and calculate fps.
    pub fn delay(&mut self) -> u32 {
        unsafe { gfx::framerate::SDL_framerateDelay(&mut self.raw) as u32 }
    }
}
