//! Surface Rotozoomer

use get_error;
use libc::c_int;
pub use std::f64::consts::PI;
use surface::Surface;
use sys::gfx::rotozoom;

/// `RotozoomSurface` for work with rust-sdl2 Surface type
pub trait RotozoomSurface {
    /// Rotates and zooms a surface and optional anti-aliasing.
    fn rotozoom(&self, angle: f64, zoom: f64, smooth: bool) -> Result<Surface, String>;
    /// Rotates and zooms a surface with different horizontal and vertical scaling factors and optional anti-aliasing.
    fn rotozoom_xy(
        &self,
        angle: f64,
        zoomx: f64,
        zoomy: f64,
        smooth: bool,
    ) -> Result<Surface, String>;
    /// Zoom a surface by independent horizontal and vertical factors with optional smoothing.
    fn zoom(&self, zoomx: f64, zoomy: f64, smooth: bool) -> Result<Surface, String>;
    /// Shrink a surface by an integer ratio using averaging.
    fn shrink(&self, factorx: i32, factory: i32) -> Result<Surface, String>;
    /// Rotates a 8/16/24/32 bit surface in increments of 90 degrees.
    fn rotate_90deg(&self, turns: i32) -> Result<Surface, String>;
}

impl<'a> RotozoomSurface for Surface<'a> {
    fn rotozoom(&self, angle: f64, zoom: f64, smooth: bool) -> Result<Surface, String> {
        let raw = unsafe { rotozoom::rotozoomSurface(self.raw(), angle, zoom, smooth as c_int) };
        if (raw as *mut ()).is_null() {
            Err(get_error())
        } else {
            unsafe { Ok(Surface::from_ll(raw)) }
        }
    }
    fn rotozoom_xy(
        &self,
        angle: f64,
        zoomx: f64,
        zoomy: f64,
        smooth: bool,
    ) -> Result<Surface, String> {
        let raw = unsafe {
            rotozoom::rotozoomSurfaceXY(self.raw(), angle, zoomx, zoomy, smooth as c_int)
        };
        if (raw as *mut ()).is_null() {
            Err(get_error())
        } else {
            unsafe { Ok(Surface::from_ll(raw)) }
        }
    }
    fn zoom(&self, zoomx: f64, zoomy: f64, smooth: bool) -> Result<Surface, String> {
        let raw = unsafe { rotozoom::zoomSurface(self.raw(), zoomx, zoomy, smooth as c_int) };
        if (raw as *mut ()).is_null() {
            Err(get_error())
        } else {
            unsafe { Ok(Surface::from_ll(raw)) }
        }
    }
    fn shrink(&self, factorx: i32, factory: i32) -> Result<Surface, String> {
        let raw =
            unsafe { rotozoom::shrinkSurface(self.raw(), factorx as c_int, factory as c_int) };
        if (raw as *mut ()).is_null() {
            Err(get_error())
        } else {
            unsafe { Ok(Surface::from_ll(raw)) }
        }
    }
    fn rotate_90deg(&self, turns: i32) -> Result<Surface, String> {
        let raw = unsafe { rotozoom::rotateSurface90Degrees(self.raw(), turns as c_int) };
        if (raw as *mut ()).is_null() {
            Err(get_error())
        } else {
            unsafe { Ok(Surface::from_ll(raw)) }
        }
    }
}

pub fn get_zoom_size(width: i32, height: i32, zoomx: f64, zoomy: f64) -> (i32, i32) {
    let mut w: c_int = 0;
    let mut h: c_int = 0;
    unsafe {
        rotozoom::zoomSurfaceSize(
            width as c_int,
            height as c_int,
            zoomx,
            zoomy,
            &mut w,
            &mut h,
        )
    }
    (w as i32, h as i32)
}

pub fn get_rotozoom_size(width: i32, height: i32, angle: f64, zoom: f64) -> (i32, i32) {
    let mut w: c_int = 0;
    let mut h: c_int = 0;
    unsafe {
        rotozoom::rotozoomSurfaceSize(width as c_int, height as c_int, angle, zoom, &mut w, &mut h)
    }
    (w as i32, h as i32)
}

pub fn get_rotozoom_xy_size(
    width: i32,
    height: i32,
    angle: f64,
    zoomx: f64,
    zoomy: f64,
) -> (i32, i32) {
    let mut w: c_int = 0;
    let mut h: c_int = 0;
    unsafe {
        rotozoom::rotozoomSurfaceSizeXY(
            width as c_int,
            height as c_int,
            angle,
            zoomx,
            zoomy,
            &mut w,
            &mut h,
        )
    }
    (w as i32, h as i32)
}
