use std::mem;
use rect::Rect;
use get_error;
use SdlResult;
use std::ptr;
use libc::{c_int, uint32_t};
use std::num::FromPrimitive;
use pixels;
use render::BlendMode;
use rwops;

pub use sys::surface as ll;

bitflags! {
    flags SurfaceFlag: u32 {
        const SWSURFACE = ll::SDL_SWSURFACE as u32,
        const PREALLOC = ll::SDL_PREALLOC as u32,
        const RLEACCEL = ll::SDL_RLEACCEL as u32,
        const DONTFREE = ll::SDL_DONTFREE as u32
    }
}

#[derive(PartialEq)]
#[allow(raw_pointer_derive, missing_copy_implementations)]
pub struct Surface {
    raw: *const ll::SDL_Surface,
    owned: bool
}

impl Drop for Surface {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ll::SDL_FreeSurface(self.raw);
            }
        }
    }
}

impl_raw_accessors!(Surface, *const ll::SDL_Surface);
impl_owned_accessors!(Surface, owned);
impl_raw_constructor!(Surface -> Surface (raw: *const ll::SDL_Surface, owned: bool));

impl Surface {
    pub fn new(surface_flags: SurfaceFlag, width: int, height: int, bpp: int,
               rmask: u32, gmask: u32, bmask: u32, amask: u32) -> SdlResult<Surface> {
        unsafe {
            let raw = ll::SDL_CreateRGBSurface(surface_flags.bits(), width as c_int, height as c_int, bpp as c_int,
                                               rmask, gmask, bmask, amask);

            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(Surface { raw: raw, owned: true })
            }
        }
    }

    pub fn from_data(data: &mut [u8], width: int, height: int, bpp: int, pitch: int,
                     rmask: u32, gmask: u32, bmask: u32, amask: u32) -> SdlResult<Surface> {

        unsafe {
            let raw = ll::SDL_CreateRGBSurfaceFrom(
                data.as_ptr() as *const _, width as c_int, height as c_int,
                bpp as c_int, pitch as c_int, rmask, gmask, bmask, amask);

            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(Surface { raw: raw, owned: true })
            }
        }
    }

    pub fn get_width(&self) -> int {
        unsafe { (*self.raw).w as int }
    }

    pub fn get_height(&self) -> int {
        unsafe { (*self.raw).h as int }
    }

    pub fn get_pitch(&self) -> int {
        unsafe { (*self.raw).pitch as int }
    }

    pub fn get_size(&self) -> (int, int) {
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
            let len = (*self.raw).pitch as uint * ((*self.raw).h as uint);
            let pixels = ::std::slice::from_raw_mut_buf(&raw_pixels, len);
            let rv = f(pixels);
            ll::SDL_UnlockSurface(self.raw);
            rv
        }
    }

    pub fn unlock(&self) {
        unsafe { ll::SDL_UnlockSurface(self.raw); }
    }

    pub fn from_bmp(path: &Path) -> SdlResult<Surface> {
        let raw = unsafe {
            ll::SDL_LoadBMP_RW(try!(rwops::RWops::from_file(path, "rb")).raw(), 0)
        };

        if raw.is_null() { Err(get_error()) }
        else { Ok(Surface{raw: raw, owned: true}) }
    }

    pub fn save_bmp(&self, path: &Path) -> SdlResult<()> {
	let ret = unsafe {
            ll::SDL_SaveBMP_RW(self.raw, try!(rwops::RWops::from_file(path, "rb")).raw(), 0)
	};
        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
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
        let key: u32 = 0;
        let result = unsafe {
            ll::SDL_GetColorKey(self.raw, &key)
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
        let r: u8 = 0;
        let g: u8 = 0;
        let b: u8 = 0;

        let result = unsafe {
            ll::SDL_GetSurfaceColorMod(self.raw, &r, &g, &b) == 0
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
        let alpha = 0u8;
        let result = unsafe {
            ll::SDL_GetSurfaceAlphaMod(self.raw, &alpha)
        };

        match result {
            0 => Ok(alpha),
            _ => Err(get_error())
        }
    }

    pub fn set_blend_mode(&mut self, mode: BlendMode) -> SdlResult<()> {
        let result = unsafe {
            ll::SDL_SetSurfaceBlendMode(self.raw, FromPrimitive::from_int(mode as int).unwrap())
        };

        match result {
            0 => Ok(()),
            _ => Err(get_error())
        }
    }

    pub fn get_blend_mode(&self) -> SdlResult<BlendMode> {
        let mode: ll::SDL_BlendMode = FromPrimitive::from_int(0).unwrap();
        let result = unsafe {
            ll::SDL_GetSurfaceBlendMode(self.raw, &mode)
        };

        match result {
            0 => Ok(FromPrimitive::from_int(mode as int).unwrap()),
            _ => Err(get_error())
        }
    }

    pub fn set_clip_rect(&mut self, rect: Option<Rect>) -> bool {
        unsafe {
            ll::SDL_SetClipRect(self.raw, mem::transmute(rect.as_ref())) == 1
        }
    }

    pub fn get_clip_rect(&self) -> Rect {
        let rect = Rect::new(0, 0, 0, 0);
        unsafe {
            ll::SDL_GetClipRect(self.raw, &rect)
        };
        rect
    }

    pub fn convert(&self, format: &pixels::PixelFormat) -> SdlResult<Surface> {
        // SDL_ConvertSurface takes a flag as the last parameter, which should be 0 by the docs.
        let surface_ptr = unsafe { ll::SDL_ConvertSurface(self.raw, format.raw(), 0u32) };

        if surface_ptr == ptr::null() {
            Err(get_error())
        } else {
            unsafe { Ok(Surface::from_ll(surface_ptr, true)) }
        }
    }

    pub fn convert_format(&self, format: pixels::PixelFormatFlag) -> SdlResult<Surface> {
        let surface_ptr = unsafe { ll::SDL_ConvertSurfaceFormat(self.raw, format as uint32_t, 0u32) };

        if surface_ptr == ptr::null() {
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

    pub fn upper_blit_scaled(&self, src_rect: Option<Rect>,
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
