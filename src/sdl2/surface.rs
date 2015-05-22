use std::marker::PhantomData;
use std::mem;
use std::path::Path;
use rect::Rect;
use get_error;
use SdlResult;
use std::ptr;
use libc::{c_int, uint32_t};
use num::FromPrimitive;
use pixels;
use render::BlendMode;
use rwops::RWops;

use sys::surface as ll;

pub struct Surface<'a> {
    raw: *mut ll::SDL_Surface,
    owned: bool,
    _marker: PhantomData<&'a ()>
}

impl<'a> Drop for Surface<'a> {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ll::SDL_FreeSurface(self.raw);
            }
        }
    }
}

impl<'a> Surface<'a> {
    pub unsafe fn raw(&self) -> *mut ll::SDL_Surface { self.raw }

    pub unsafe fn owned(&self) -> bool { self.owned }

    pub unsafe fn from_ll<'b>(raw: *mut ll::SDL_Surface, owned: bool) -> Surface<'b> {
        Surface {
            raw: raw,
            owned: owned,
            _marker: PhantomData
        }
    }

    /// Creates a new surface using a pixel format.
    ///
    /// # Example
    /// ```no_run
    /// use sdl2::pixels::PixelFormatEnum;
    /// use sdl2::surface::Surface;
    ///
    /// let surface = Surface::new(512, 512, PixelFormatEnum::RGB24).unwrap();
    /// ```
    pub fn new(width: u32, height: u32, format: pixels::PixelFormatEnum) -> SdlResult<Surface<'static>> {
        let masks = try!(format.into_masks());
        Surface::from_pixelmasks(width, height, masks)
    }

    /// Creates a new surface using pixel masks.
    ///
    /// # Example
    /// ```no_run
    /// use sdl2::pixels::PixelFormatEnum;
    /// use sdl2::surface::Surface;
    ///
    /// let masks = PixelFormatEnum::RGB24.into_masks().unwrap();
    /// let surface = Surface::from_pixelmasks(512, 512, masks).unwrap();
    /// ```
    pub fn from_pixelmasks(width: u32, height: u32, masks: pixels::PixelMasks) -> SdlResult<Surface<'static>> {
        unsafe {
            if width >= (1<<31) || height >= (1<<31) {
                Err(format!("Image is too large."))
            } else {
                let raw = ll::SDL_CreateRGBSurface(0, width as c_int, height as c_int,
                    masks.bpp as c_int, masks.rmask, masks.gmask, masks.bmask, masks.amask);

                if raw.is_null() {
                    Err(get_error())
                } else {
                    Ok(Surface {
                        raw: raw,
                        owned: true,
                        _marker: PhantomData
                    })
                }
            }
        }
    }

    /// Creates a new surface from an existing buffer, using a pixel format.
    pub fn from_data(data: &'a mut [u8], width: u32, height: u32, pitch: u32, format: pixels::PixelFormatEnum) -> SdlResult<Surface<'a>> {
        let masks = try!(format.into_masks());
        Surface::from_data_pixelmasks(data, width, height, pitch, masks)
    }

    /// Creates a new surface from an existing buffer, using pixel masks.
    pub fn from_data_pixelmasks(data: &'a mut [u8], width: u32, height: u32, pitch: u32, masks: pixels::PixelMasks) -> SdlResult<Surface<'a>> {
        unsafe {
            if width >= (1<<31) || height >= (1<<31) {
                Err(format!("Image is too large."))
            } else if pitch >= (1<<31) {
                Err(format!("Pitch is too large."))
            } else {
                let raw = ll::SDL_CreateRGBSurfaceFrom(
                    data.as_mut_ptr() as *mut _, width as c_int, height as c_int,
                    masks.bpp as c_int, pitch as c_int, masks.rmask, masks.gmask, masks.bmask, masks.amask);

                if raw.is_null() {
                    Err(get_error())
                } else {
                    Ok(Surface {
                        raw: raw,
                        owned: true,
                        _marker: PhantomData
                    })
                }
            }
        }
    }

    pub fn get_width(&self) -> u32 {
        unsafe { (*self.raw).w as u32 }
    }

    pub fn get_height(&self) -> u32 {
        unsafe { (*self.raw).h as u32 }
    }

    pub fn get_pitch(&self) -> u32 {
        unsafe { (*self.raw).pitch as u32 }
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.get_width(), self.get_height())
    }

    pub fn get_rect(&self) -> Rect {
        Rect {
            x: 0,
            y: 0,
            w: self.get_width() as i32,
            h: self.get_height() as i32
        }
    }

    pub fn get_pixel_format(&self) -> pixels::PixelFormat {
        unsafe {
            pixels::PixelFormat::from_ll((*self.raw).format)
        }
    }

    pub fn lock(&self) -> bool {
        unsafe { ll::SDL_LockSurface(self.raw) == 0 }
    }

    /// Locks a surface so that the pixels can be directly accessed safely.
    pub fn with_lock<R, F: FnOnce(&mut [u8]) -> R>(&mut self, f: F) -> R {
        unsafe {
            if ll::SDL_LockSurface(self.raw) != 0 { panic!("could not lock surface"); }

            let raw_pixels = (*self.raw).pixels as *mut _;
            let len = (*self.raw).pitch as usize * ((*self.raw).h as usize);
            let pixels = ::std::slice::from_raw_parts_mut(raw_pixels, len);
            let rv = f(pixels);
            ll::SDL_UnlockSurface(self.raw);
            rv
        }
    }

    pub fn unlock(&self) {
        unsafe { ll::SDL_UnlockSurface(self.raw); }
    }

    pub fn load_bmp_rw(rwops: &mut RWops) -> SdlResult<Surface<'static>> {
        let raw = unsafe {
            ll::SDL_LoadBMP_RW(rwops.raw(), 0)
        };

        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(Surface {
                raw: raw,
                owned: true,
                _marker: PhantomData
            })
        }
    }

    pub fn save_bmp_rw(&self, rwops: &mut RWops) -> SdlResult<()> {
        let ret = unsafe {
            ll::SDL_SaveBMP_RW(self.raw, rwops.raw(), 0)
        };
        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn load_bmp<P: AsRef<Path>>(path: P) -> SdlResult<Surface<'static>> {
        let mut file = try!(RWops::from_file(path, "rb"));
        Surface::load_bmp_rw(&mut file)
    }

    pub fn save_bmp<P: AsRef<Path>>(&self, path: P) -> SdlResult<()> {
        let mut file = try!(RWops::from_file(path, "wb"));
        self.save_bmp_rw(&mut file)
    }

    pub fn set_palette(&self, palette: &pixels::Palette) -> bool {
        unsafe {
            ll::SDL_SetSurfacePalette(self.raw, palette.raw()) == 0
        }
    }

    #[allow(non_snake_case)]
    pub fn enable_RLE(&self) -> bool {
        unsafe {
            ll::SDL_SetSurfaceRLE(self.raw, 1) == 0
        }
    }

    #[allow(non_snake_case)]
    pub fn disable_RLE(&self) -> bool {
        unsafe {
            ll::SDL_SetSurfaceRLE(self.raw, 0) == 0
        }
    }

    pub fn set_color_key(&self, enable: bool, color: pixels::Color) -> SdlResult<()> {
        let key = color.to_u32(&self.get_pixel_format());
        let result = unsafe {
            ll::SDL_SetColorKey(self.raw, if enable { 1 } else { 0 }, key)
        };
        if result == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }

    pub fn get_color_key(&self) -> SdlResult<pixels::Color> {
        let mut key = 0;
        let result = unsafe {
            ll::SDL_GetColorKey(self.raw, &mut key)
        };

        if result == 0 {
            Ok(pixels::Color::from_u32(&self.get_pixel_format(), key))
        } else {
            Err(get_error())
        }
    }

    pub fn set_color_mod(&self, color: pixels::Color) -> bool {
        let (r, g, b) = match color {
            pixels::Color::RGB(r, g, b) => (r, g, b),
            pixels::Color::RGBA(r, g, b, _) => (r, g, b)
        };

        unsafe {
            ll::SDL_SetSurfaceColorMod(self.raw, r, g, b) == 0
        }
    }

    pub fn get_color_mod(&self) -> SdlResult<pixels::Color> {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        let result = unsafe {
            ll::SDL_GetSurfaceColorMod(self.raw, &mut r, &mut g, &mut b) == 0
        };

        if result {
            Ok(pixels::Color::RGB(r,g,b))
        } else {
            Err(get_error())
        }
    }

    pub fn blit( &self, src: &Surface, dstrect: Option<Rect>, srcrect: Option<Rect> ) -> bool {
        unsafe {
            let dstrect_ptr = mem::transmute( dstrect.as_ref() );
            let srcrect_ptr = mem::transmute( srcrect.as_ref() );
            ll::SDL_UpperBlit( src.raw, srcrect_ptr, self.raw, dstrect_ptr ) == 0
        }
    }

    pub fn fill_rect(&mut self, rect: Option<Rect>, color: pixels::Color) -> SdlResult<()> {
        unsafe {
            let rect_ptr = mem::transmute( rect.as_ref() );
            let format = self.get_pixel_format();
            let result = ll::SDL_FillRect( self.raw, rect_ptr, color.to_u32(&format) );
            match result {
                0 => Ok(()),
                _ => Err(get_error())
            }
        }
    }

    pub fn fill_rects(&mut self, rects: &[Option<Rect>], color: pixels::Color) -> SdlResult<()> {
        for &rect in rects.iter() {
            let result = self.fill_rect(rect, color);
            match result {
                Err(e) => return Err(e),
                _ => ()
            };
        }

        Ok(())
    }

    pub fn set_alpha_mod(&mut self, alpha: u8) -> SdlResult<()> {
        let result = unsafe {
            ll::SDL_SetSurfaceAlphaMod(self.raw, alpha)
        };

        match result {
            0 => Ok(()),
            _ => Err(get_error())
        }
    }

    pub fn get_alpha_mod(&self) -> SdlResult<u8> {
        let mut alpha = 0;
        let result = unsafe {
            ll::SDL_GetSurfaceAlphaMod(self.raw, &mut alpha)
        };

        match result {
            0 => Ok(alpha),
            _ => Err(get_error())
        }
    }

    pub fn set_blend_mode(&mut self, mode: BlendMode) -> SdlResult<()> {
        let result = unsafe {
            ll::SDL_SetSurfaceBlendMode(self.raw, mode as c_int)
        };

        match result {
            0 => Ok(()),
            _ => Err(get_error())
        }
    }

    pub fn get_blend_mode(&self) -> SdlResult<BlendMode> {
        let mut mode: ll::SDL_BlendMode = 0;
        let result = unsafe {
            ll::SDL_GetSurfaceBlendMode(self.raw, &mut mode)
        };

        match result {
            0 => Ok(FromPrimitive::from_i32(mode as i32).unwrap()),
            _ => Err(get_error())
        }
    }

    pub fn set_clip_rect(&mut self, rect: Option<Rect>) -> bool {
        unsafe {
            ll::SDL_SetClipRect(self.raw, mem::transmute(rect.as_ref())) == 1
        }
    }

    pub fn get_clip_rect(&self) -> Rect {
        let mut rect = Rect::new(0, 0, 0, 0);
        unsafe {
            ll::SDL_GetClipRect(self.raw, &mut rect)
        };
        rect
    }

    pub fn convert(&self, format: &pixels::PixelFormat) -> SdlResult<Surface<'static>> {
        // SDL_ConvertSurface takes a flag as the last parameter, which should be 0 by the docs.
        let surface_ptr = unsafe { ll::SDL_ConvertSurface(self.raw, format.raw(), 0u32) };

        if surface_ptr== ptr::null_mut() {
            Err(get_error())
        } else {
            unsafe { Ok(Surface::from_ll(surface_ptr, true)) }
        }
    }

    pub fn convert_format(&self, format: pixels::PixelFormatEnum) -> SdlResult<Surface<'static>> {
        let surface_ptr = unsafe { ll::SDL_ConvertSurfaceFormat(self.raw, format as uint32_t, 0u32) };

        if surface_ptr== ptr::null_mut() {
            Err(get_error())
        } else {
            unsafe { Ok(Surface::from_ll(surface_ptr, true)) }
        }
    }

    pub fn lower_blit(&self, src_rect: Option<Rect>,
                      dst: &mut Surface, dst_rect: Option<Rect>) -> SdlResult<()> {

        match unsafe {
            let src_rect_ptr = mem::transmute(src_rect.as_ref());
            let dst_rect_ptr = mem::transmute(dst_rect.as_ref());
            ll::SDL_LowerBlit(self.raw, src_rect_ptr, dst.raw, dst_rect_ptr)
        } {
            0 => Ok(()),
            _ => Err(get_error())
        }
    }

    pub fn soft_stretch(&self, src_rect: Option<Rect>,
                        dst: &mut Surface, dst_rect: Option<Rect>) -> SdlResult<()> {

        match unsafe {
            let src_rect_ptr = mem::transmute(src_rect.as_ref());
            let dst_rect_ptr = mem::transmute(dst_rect.as_ref());
            ll::SDL_SoftStretch(self.raw, src_rect_ptr, dst.raw, dst_rect_ptr)
        } {
            0 => Ok(()),
            _ => Err(get_error())
        }
    }

    pub fn blit_scaled(&self, src_rect: Option<Rect>,
                             dst: &mut Surface, dst_rect: Option<Rect>) -> SdlResult<()> {

        match unsafe {
            let src_rect_ptr = mem::transmute(src_rect.as_ref());
            let dst_rect_ptr = mem::transmute(dst_rect.as_ref());
            ll::SDL_UpperBlitScaled(self.raw, src_rect_ptr, dst.raw, dst_rect_ptr)
        } {
            0 => Ok(()),
            _ => Err(get_error())
        }
    }

    pub fn upper_blit_scaled(&self, src_rect: Option<Rect>,
                             dst: &mut Surface, dst_rect: Option<Rect>) -> SdlResult<()> {
        //! This function is deprecated

        match unsafe {
            let src_rect_ptr = mem::transmute(src_rect.as_ref());
            let dst_rect_ptr = mem::transmute(dst_rect.as_ref());
            ll::SDL_UpperBlitScaled(self.raw, src_rect_ptr, dst.raw, dst_rect_ptr)
        } {
            0 => Ok(()),
            _ => Err(get_error())
        }
    }

    pub fn lower_blit_scaled(&self, src_rect: Option<Rect>,
                             dst: &mut Surface, dst_rect: Option<Rect>) -> SdlResult<()> {

        match unsafe {
            let src_rect_ptr = mem::transmute(src_rect.as_ref());
            let dst_rect_ptr = mem::transmute(dst_rect.as_ref());
            ll::SDL_LowerBlitScaled(self.raw, src_rect_ptr, dst.raw, dst_rect_ptr)
        } {
            0 => Ok(()),
            _ => Err(get_error())
        }
    }

    /*
    pub fn SDL_ConvertPixels(width: c_int, height: c_int, src_format: uint32_t, src: *c_void, src_pitch: c_int, dst_format: uint32_t, dst: *c_void, dst_pitch: c_int) -> c_int;
    */
}
