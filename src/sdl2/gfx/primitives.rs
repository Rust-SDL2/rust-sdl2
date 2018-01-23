//! Graphic Primitives

use std::mem;
use std::ptr;
use std::ffi::CString;
use num::traits::ToPrimitive;
use libc::{c_void, c_int, c_char};
use render::Canvas;
use surface::Surface;
use pixels;
use get_error;
use super::ffi as ll;

/// generic Color type
pub trait ToColor {
    fn as_rgba(&self) -> (u8, u8, u8, u8);

    #[inline]
    fn as_u32(&self) -> u32 {
        unsafe { mem::transmute(self.as_rgba()) }
    }
}

impl ToColor for pixels::Color {
    #[inline]
    fn as_rgba(&self) -> (u8, u8, u8, u8) {
        self.rgba()
    }
}

impl ToColor for (u8, u8, u8, u8) {
    #[inline]
    fn as_rgba(&self) -> (u8, u8, u8, u8) {
        *self
    }

    #[inline]
    fn as_u32(&self) -> u32 {
        unsafe { mem::transmute(*self) }
    }
}

impl ToColor for u32 {
    #[inline]
    fn as_rgba(&self) -> (u8, u8, u8, u8) {
        unsafe { mem::transmute(*self) }
    }

    #[inline]
    fn as_u32(&self) -> u32 {
        *self
    }
}

// for 0xXXXXXXXX
impl ToColor for isize {
    #[inline]
    fn as_rgba(&self) -> (u8, u8, u8, u8) {
        unsafe { mem::transmute(self.to_u32().expect("Can't convert to Color Type")) }
    }

    #[inline]
    fn as_u32(&self) -> u32 {
        self.to_u32().expect("Can't convert to Color Type")
    }
}

/// For drawing with rust-sdl2 Renderer
pub trait DrawRenderer {
    fn pixel<C: ToColor>(&self, x: i16, y: i16, color: C) -> Result<(), String>;
    fn hline<C: ToColor>(&self, x1: i16, x2: i16, y: i16, color: C) -> Result<(), String>;
    fn vline<C: ToColor>(&self, x: i16, y1: i16, y2: i16, color: C) -> Result<(), String>;
    fn rectangle<C: ToColor>(&self,
                             x1: i16,
                             y1: i16,
                             x2: i16,
                             y2: i16,
                             color: C)
                             -> Result<(), String>;
    fn rounded_rectangle<C: ToColor>(&self,
                                     x1: i16,
                                     y1: i16,
                                     x2: i16,
                                     y2: i16,
                                     rad: i16,
                                     color: C)
                                     -> Result<(), String>;
    fn box_<C: ToColor>(&self, x1: i16, y1: i16, x2: i16, y2: i16, color: C) -> Result<(), String>;
    fn rounded_box<C: ToColor>(&self,
                               x1: i16,
                               y1: i16,
                               x2: i16,
                               y2: i16,
                               rad: i16,
                               color: C)
                               -> Result<(), String>;
    fn line<C: ToColor>(&self, x1: i16, y1: i16, x2: i16, y2: i16, color: C) -> Result<(), String>;
    fn aa_line<C: ToColor>(&self,
                           x1: i16,
                           y1: i16,
                           x2: i16,
                           y2: i16,
                           color: C)
                           -> Result<(), String>;
    fn thick_line<C: ToColor>(&self,
                              x1: i16,
                              y1: i16,
                              x2: i16,
                              y2: i16,
                              width: u8,
                              color: C)
                              -> Result<(), String>;
    fn circle<C: ToColor>(&self, x: i16, y: i16, rad: i16, color: C) -> Result<(), String>;
    fn aa_circle<C: ToColor>(&self, x: i16, y: i16, rad: i16, color: C) -> Result<(), String>;
    fn filled_circle<C: ToColor>(&self, x: i16, y: i16, rad: i16, color: C) -> Result<(), String>;
    fn arc<C: ToColor>(&self,
                       x: i16,
                       y: i16,
                       rad: i16,
                       start: i16,
                       end: i16,
                       color: C)
                       -> Result<(), String>;
    fn ellipse<C: ToColor>(&self,
                           x: i16,
                           y: i16,
                           rx: i16,
                           ry: i16,
                           color: C)
                           -> Result<(), String>;
    fn aa_ellipse<C: ToColor>(&self,
                              x: i16,
                              y: i16,
                              rx: i16,
                              ry: i16,
                              color: C)
                              -> Result<(), String>;
    fn filled_ellipse<C: ToColor>(&self,
                                  x: i16,
                                  y: i16,
                                  rx: i16,
                                  ry: i16,
                                  color: C)
                                  -> Result<(), String>;
    fn pie<C: ToColor>(&self,
                       x: i16,
                       y: i16,
                       rad: i16,
                       start: i16,
                       end: i16,
                       color: C)
                       -> Result<(), String>;
    fn filled_pie<C: ToColor>(&self,
                              x: i16,
                              y: i16,
                              rad: i16,
                              start: i16,
                              end: i16,
                              color: C)
                              -> Result<(), String>;
    fn trigon<C: ToColor>(&self,
                          x1: i16,
                          y1: i16,
                          x2: i16,
                          y2: i16,
                          x3: i16,
                          y3: i16,
                          color: C)
                          -> Result<(), String>;
    fn aa_trigon<C: ToColor>(&self,
                             x1: i16,
                             y1: i16,
                             x2: i16,
                             y2: i16,
                             x3: i16,
                             y3: i16,
                             color: C)
                             -> Result<(), String>;
    fn filled_trigon<C: ToColor>(&self,
                                 x1: i16,
                                 y1: i16,
                                 x2: i16,
                                 y2: i16,
                                 x3: i16,
                                 y3: i16,
                                 color: C)
                                 -> Result<(), String>;
    fn polygon<C: ToColor>(&self, vx: &[i16], vy: &[i16], color: C) -> Result<(), String>;
    fn aa_polygon<C: ToColor>(&self, vx: &[i16], vy: &[i16], color: C) -> Result<(), String>;
    fn filled_polygon<C: ToColor>(&self, vx: &[i16], vy: &[i16], color: C) -> Result<(), String>;
    fn textured_polygon<C: ToColor>(&self,
                                    vx: &[i16],
                                    vy: &[i16],
                                    texture: &Surface,
                                    texture_dx: i16,
                                    texture_dy: i16,
                                    color: C)
                                    -> Result<(), String>;
    fn bezier<C: ToColor>(&self, vx: &[i16], vy: &[i16], s: i32, color: C) -> Result<(), String>;
    fn character<C: ToColor>(&self, x: i16, y: i16, c: char, color: C) -> Result<(), String>;
    fn string<C: ToColor>(&self, x: i16, y: i16, s: &str, color: C) -> Result<(), String>;
}

impl<T> DrawRenderer for Canvas<T> where T: ::render::RenderTarget {
    fn pixel<C: ToColor>(&self, x: i16, y: i16, color: C) -> Result<(), String> {
        let ret = unsafe { ll::pixelColor(self.raw(), x, y, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    fn hline<C: ToColor>(&self, x1: i16, x2: i16, y: i16, color: C) -> Result<(), String> {
        let ret = unsafe { ll::hlineColor(self.raw(), x1, x2, y, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    fn vline<C: ToColor>(&self, x: i16, y1: i16, y2: i16, color: C) -> Result<(), String> {
        let ret = unsafe { ll::vlineColor(self.raw(), x, y1, y2, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    fn rectangle<C: ToColor>(&self,
                             x1: i16,
                             y1: i16,
                             x2: i16,
                             y2: i16,
                             color: C)
                             -> Result<(), String> {
        let ret = unsafe { ll::rectangleColor(self.raw(), x1, y1, x2, y2, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    fn rounded_rectangle<C: ToColor>(&self,
                                     x1: i16,
                                     y1: i16,
                                     x2: i16,
                                     y2: i16,
                                     rad: i16,
                                     color: C)
                                     -> Result<(), String> {
        let ret =
            unsafe { ll::roundedRectangleColor(self.raw(), x1, y1, x2, y2, rad, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    fn box_<C: ToColor>(&self, x1: i16, y1: i16, x2: i16, y2: i16, color: C) -> Result<(), String> {
        let ret = unsafe { ll::boxColor(self.raw(), x1, y1, x2, y2, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    fn rounded_box<C: ToColor>(&self,
                               x1: i16,
                               y1: i16,
                               x2: i16,
                               y2: i16,
                               rad: i16,
                               color: C)
                               -> Result<(), String> {
        let ret = unsafe { ll::roundedBoxColor(self.raw(), x1, y1, x2, y2, rad, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    fn line<C: ToColor>(&self, x1: i16, y1: i16, x2: i16, y2: i16, color: C) -> Result<(), String> {
        let ret = unsafe { ll::lineColor(self.raw(), x1, y1, x2, y2, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    fn aa_line<C: ToColor>(&self,
                           x1: i16,
                           y1: i16,
                           x2: i16,
                           y2: i16,
                           color: C)
                           -> Result<(), String> {
        let ret = unsafe { ll::aalineColor(self.raw(), x1, y1, x2, y2, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    fn thick_line<C: ToColor>(&self,
                              x1: i16,
                              y1: i16,
                              x2: i16,
                              y2: i16,
                              width: u8,
                              color: C)
                              -> Result<(), String> {
        let ret = unsafe { ll::thickLineColor(self.raw(), x1, y1, x2, y2, width, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    fn circle<C: ToColor>(&self, x: i16, y: i16, rad: i16, color: C) -> Result<(), String> {
        let ret = unsafe { ll::circleColor(self.raw(), x, y, rad, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    fn aa_circle<C: ToColor>(&self, x: i16, y: i16, rad: i16, color: C) -> Result<(), String> {
        let ret = unsafe { ll::aacircleColor(self.raw(), x, y, rad, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    fn filled_circle<C: ToColor>(&self, x: i16, y: i16, rad: i16, color: C) -> Result<(), String> {
        let ret = unsafe { ll::filledCircleColor(self.raw(), x, y, rad, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    fn arc<C: ToColor>(&self,
                       x: i16,
                       y: i16,
                       rad: i16,
                       start: i16,
                       end: i16,
                       color: C)
                       -> Result<(), String> {
        let ret = unsafe { ll::arcColor(self.raw(), x, y, rad, start, end, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    fn ellipse<C: ToColor>(&self,
                           x: i16,
                           y: i16,
                           rx: i16,
                           ry: i16,
                           color: C)
                           -> Result<(), String> {
        let ret = unsafe { ll::ellipseColor(self.raw(), x, y, rx, ry, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    fn aa_ellipse<C: ToColor>(&self,
                              x: i16,
                              y: i16,
                              rx: i16,
                              ry: i16,
                              color: C)
                              -> Result<(), String> {
        let ret = unsafe { ll::aaellipseColor(self.raw(), x, y, rx, ry, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    fn filled_ellipse<C: ToColor>(&self,
                                  x: i16,
                                  y: i16,
                                  rx: i16,
                                  ry: i16,
                                  color: C)
                                  -> Result<(), String> {
        let ret = unsafe { ll::filledEllipseColor(self.raw(), x, y, rx, ry, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    fn pie<C: ToColor>(&self,
                       x: i16,
                       y: i16,
                       rad: i16,
                       start: i16,
                       end: i16,
                       color: C)
                       -> Result<(), String> {
        let ret = unsafe { ll::pieColor(self.raw(), x, y, rad, start, end, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    fn filled_pie<C: ToColor>(&self,
                              x: i16,
                              y: i16,
                              rad: i16,
                              start: i16,
                              end: i16,
                              color: C)
                              -> Result<(), String> {
        let ret = unsafe { ll::filledPieColor(self.raw(), x, y, rad, start, end, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    fn trigon<C: ToColor>(&self,
                          x1: i16,
                          y1: i16,
                          x2: i16,
                          y2: i16,
                          x3: i16,
                          y3: i16,
                          color: C)
                          -> Result<(), String> {
        let ret = unsafe { ll::trigonColor(self.raw(), x1, y1, x2, y2, x3, y3, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    fn aa_trigon<C: ToColor>(&self,
                             x1: i16,
                             y1: i16,
                             x2: i16,
                             y2: i16,
                             x3: i16,
                             y3: i16,
                             color: C)
                             -> Result<(), String> {
        let ret = unsafe { ll::aatrigonColor(self.raw(), x1, y1, x2, y2, x3, y3, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    fn filled_trigon<C: ToColor>(&self,
                                 x1: i16,
                                 y1: i16,
                                 x2: i16,
                                 y2: i16,
                                 x3: i16,
                                 y3: i16,
                                 color: C)
                                 -> Result<(), String> {
        let ret =
            unsafe { ll::filledTrigonColor(self.raw(), x1, y1, x2, y2, x3, y3, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    // FIXME: may we use pointer tuple?
    fn polygon<C: ToColor>(&self, vx: &[i16], vy: &[i16], color: C) -> Result<(), String> {
        assert_eq!(vx.len(), vy.len());
        let n = vx.len() as c_int;
        let ret =
            unsafe { ll::polygonColor(self.raw(), vx.as_ptr(), vy.as_ptr(), n, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }

    fn aa_polygon<C: ToColor>(&self, vx: &[i16], vy: &[i16], color: C) -> Result<(), String> {
        assert_eq!(vx.len(), vy.len());
        let n = vx.len() as c_int;
        let ret =
            unsafe { ll::aapolygonColor(self.raw(), vx.as_ptr(), vy.as_ptr(), n, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }

    fn filled_polygon<C: ToColor>(&self, vx: &[i16], vy: &[i16], color: C) -> Result<(), String> {
        assert_eq!(vx.len(), vy.len());
        let n = vx.len() as c_int;
        let ret = unsafe {
            ll::filledPolygonColor(self.raw(), vx.as_ptr(), vy.as_ptr(), n, color.as_u32())
        };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
    #[allow(unused_variables)]
    fn textured_polygon<C: ToColor>(&self,
                                    vx: &[i16],
                                    vy: &[i16],
                                    texture: &Surface,
                                    texture_dx: i16,
                                    texture_dy: i16,
                                    color: C)
                                    -> Result<(), String> {
        unimplemented!()
    }

    fn bezier<C: ToColor>(&self, vx: &[i16], vy: &[i16], s: i32, color: C) -> Result<(), String> {
        assert_eq!(vx.len(), vy.len());
        let n = vx.len() as c_int;
        let ret = unsafe {
            ll::bezierColor(self.raw(),
                            vx.as_ptr(),
                            vy.as_ptr(),
                            n,
                            s as c_int,
                            color.as_u32())
        };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }

    fn character<C: ToColor>(&self, x: i16, y: i16, c: char, color: C) -> Result<(), String> {
        let ret = unsafe { ll::characterColor(self.raw(), x, y, c as c_char, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }

    fn string<C: ToColor>(&self, x: i16, y: i16, s: &str, color: C) -> Result<(), String> {
        let ret = unsafe {
            let cstring = CString::new(s).unwrap();
            let buf = cstring.as_bytes().as_ptr();
            ll::stringColor(self.raw(), x, y, buf as *mut c_char, color.as_u32())
        };
        if ret == 0 { Ok(()) } else { Err(get_error()) }
    }
}

/// Sets or resets the current global font data.
pub fn set_font<'b, F>(fontdata: F, cw: u32, ch: u32)
    where F: Into<Option<&'b [u8]>>
{
    let actual_fontdata = match fontdata.into() {
        None => ptr::null(),
        Some(v) => v.as_ptr(),
    };
    unsafe { ll::gfxPrimitivesSetFont(actual_fontdata as *const c_void, cw, ch) }
}

/// Sets current global font character rotation steps.
pub fn set_font_rotation(rotation: u32) {
    unsafe { ll::gfxPrimitivesSetFontRotation(rotation as u32) }
}
