use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::rc::Rc;

use crate::rect::Rect;
use crate::get_error;
use std::ptr;
use libc::c_int;
use std::convert::TryFrom;
use crate::pixels;
use crate::render::{BlendMode, Canvas};
use crate::rwops::RWops;
use std::mem::transmute;
use crate::render::{Texture, TextureCreator, TextureValueError};

use crate::sys;

/// Holds a `SDL_Surface`
///
/// When the `SurfaceContext` is dropped, it frees the `SDL_Surface`
///
/// *INTERNAL USE ONLY*
pub struct SurfaceContext<'a> {
    raw: *mut sys::SDL_Surface,
    _marker: PhantomData<&'a ()>
}

impl<'a> Drop for SurfaceContext<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe { sys::SDL_FreeSurface(self.raw); }
    }
}

/// Holds a `Rc<SurfaceContext>`.
///
/// Note: If a `Surface` goes out of scope but it cloned its context,
/// then the `SDL_Surface` will not be free'd until there are no more references to the `SurfaceContext`.
pub struct Surface<'a> {
    context: Rc<SurfaceContext<'a>>,
}

/// An unsized Surface reference.
///
/// This type is used whenever Surfaces need to be borrowed from the SDL library, without concern
/// for freeing the Surface.
pub struct SurfaceRef {
    // It's nothing! (it gets transmuted to SDL_Surface later).
    // The empty private field is need to a) make `std::mem::swap()` copy nothing instead of
    // clobbering two surfaces (SDL_Surface's size could change in the future),
    // and b) prevent user initialization of this type.
    _raw: ()
}

impl AsRef<SurfaceRef> for SurfaceRef  {
    fn as_ref(&self) -> &SurfaceRef {
        self
    }
}

#[test]
fn test_surface_ref_size() {
    // `SurfaceRef` must be 0 bytes.
    assert_eq!(::std::mem::size_of::<SurfaceRef>(), 0);
}

impl<'a> Deref for Surface<'a> {
    type Target = SurfaceRef;

    #[inline]
    fn deref(&self) -> &SurfaceRef {
        unsafe { mem::transmute(self.context.raw) }
    }
}

impl<'a> DerefMut for Surface<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut SurfaceRef {
        unsafe { mem::transmute(self.context.raw) }
    }
}

impl<'a> AsRef<SurfaceRef> for Surface<'a> {
    #[inline]
    fn as_ref(&self) -> &SurfaceRef {
        unsafe { mem::transmute(self.context.raw) }
    }
}

impl<'a> AsMut<SurfaceRef> for Surface<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut SurfaceRef {
        unsafe { mem::transmute(self.context.raw) }
    }
}


impl<'a> Surface<'a> {
    pub unsafe fn from_ll<'b>(raw: *mut sys::SDL_Surface) -> Surface<'b> {
        let context = SurfaceContext {
            raw,
            _marker: PhantomData,
        };
        Surface { context: Rc::new(context) }
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
    pub fn new(width: u32, height: u32, format: pixels::PixelFormatEnum) -> Result<Surface<'static>, String> {
        let masks = format.into_masks()?;
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
    pub fn from_pixelmasks(width: u32, height: u32, masks: pixels::PixelMasks) -> Result<Surface<'static>, String> {
        unsafe {
            if width >= (1<<31) || height >= (1<<31) {
                Err("Image is too large.".to_owned())
            } else {
                let raw = sys::SDL_CreateRGBSurface(0, width as c_int, height as c_int,
                    masks.bpp as c_int, masks.rmask, masks.gmask, masks.bmask, masks.amask);

                if raw.is_null() {
                    Err(get_error())
                } else {
                    Ok(Surface::from_ll(raw))
                }
            }
        }
    }

    /// Creates a new surface from an existing buffer, using a pixel format.
    pub fn from_data(data: &'a mut [u8], width: u32, height: u32, pitch: u32, format: pixels::PixelFormatEnum) -> Result<Surface<'a>, String> {
        let masks = format.into_masks()?;
        Surface::from_data_pixelmasks(data, width, height, pitch, masks)
    }

    /// Creates a new surface from an existing buffer, using pixel masks.
    pub fn from_data_pixelmasks(data: &'a mut [u8], width: u32, height: u32, pitch: u32, masks: pixels::PixelMasks) -> Result<Surface<'a>, String> {
        unsafe {
            if width >= (1<<31) || height >= (1<<31) {
                Err("Image is too large.".to_owned())
            } else if pitch >= (1<<31) {
                Err("Pitch is too large.".to_owned())
            } else {
                let raw = sys::SDL_CreateRGBSurfaceFrom(
                    data.as_mut_ptr() as *mut _, width as c_int, height as c_int,
                    masks.bpp as c_int, pitch as c_int, masks.rmask, masks.gmask, masks.bmask, masks.amask);

                if raw.is_null() {
                    Err(get_error())
                } else {
                    Ok(Surface::from_ll(raw))
                }
            }
        }
    }

    /// A convenience function for [`TextureCreator::create_texture_from_surface`].
    ///
    /// ```no_run
    /// use sdl2::pixels::PixelFormatEnum;
    /// use sdl2::surface::Surface;
    /// use sdl2::render::{Canvas, Texture};
    /// use sdl2::video::Window;
    ///
    /// // We init systems.
    /// let sdl_context = sdl2::init().expect("failed to init SDL");
    /// let video_subsystem = sdl_context.video().expect("failed to get video context");
    ///
    /// // We create a window.
    /// let window = video_subsystem.window("sdl2 demo", 800, 600)
    ///     .build()
    ///     .expect("failed to build window");
    ///
    /// // We get the canvas from which we can get the `TextureCreator`.
    /// let mut canvas: Canvas<Window> = window.into_canvas()
    ///     .build()
    ///     .expect("failed to build window's canvas");
    /// let texture_creator = canvas.texture_creator();
    ///
    /// let surface = Surface::new(512, 512, PixelFormatEnum::RGB24).unwrap();
    /// let texture = surface.as_texture(&texture_creator).unwrap();
    /// ```
    #[cfg(not(feature = "unsafe_textures"))]
    pub fn as_texture<'b, T>(&self, texture_creator: &'b TextureCreator<T>) -> Result<Texture<'b>, TextureValueError> {
        texture_creator.create_texture_from_surface(self)
    }

    /// A convenience function for [`TextureCreator::create_texture_from_surface`].
    ///
    /// ```no_run
    /// use sdl2::pixels::PixelFormatEnum;
    /// use sdl2::surface::Surface;
    /// use sdl2::render::{Canvas, Texture};
    /// use sdl2::video::Window;
    ///
    /// // We init systems.
    /// let sdl_context = sdl2::init().expect("failed to init SDL");
    /// let video_subsystem = sdl_context.video().expect("failed to get video context");
    ///
    /// // We create a window.
    /// let window = video_subsystem.window("sdl2 demo", 800, 600)
    ///     .build()
    ///     .expect("failed to build window");
    ///
    /// // We get the canvas from which we can get the `TextureCreator`.
    /// let mut canvas: Canvas<Window> = window.into_canvas()
    ///     .build()
    ///     .expect("failed to build window's canvas");
    /// let texture_creator = canvas.texture_creator();
    ///
    /// let surface = Surface::new(512, 512, PixelFormatEnum::RGB24).unwrap();
    /// let texture = surface.as_texture(&texture_creator).unwrap();
    /// ```
    #[cfg(feature = "unsafe_textures")]
    pub fn as_texture<T>(&self, texture_creator: &TextureCreator<T>) -> Result<Texture, TextureValueError> {
        texture_creator.create_texture_from_surface(self)
    }

    pub fn load_bmp_rw(rwops: &mut RWops) -> Result<Surface<'static>, String> {
        let raw = unsafe {
            sys::SDL_LoadBMP_RW(rwops.raw(), 0)
        };

        if raw.is_null() {
            Err(get_error())
        } else {
            Ok( unsafe{ Surface::from_ll(raw) } )
        }
    }

    pub fn load_bmp<P: AsRef<Path>>(path: P) -> Result<Surface<'static>, String> {
        let mut file = RWops::from_file(path, "rb")?;
        Surface::load_bmp_rw(&mut file)
    }

    /// Creates a Software Canvas to allow rendering in the Surface itself. This `Canvas` will
    /// never be accelerated materially, so there is no performance change between `Surface` and
    /// `Canvas` coming from a `Surface`.
    ///
    /// The only change is this case is that `Canvas` has a
    /// better API to draw stuff in the `Surface` in that case, but don't expect any performance
    /// changes, there will be none.
    pub fn into_canvas(self) -> Result<Canvas<Surface<'a>>, String> {
        Canvas::from_surface(self)
    }

    pub fn context(&self) -> Rc<SurfaceContext<'a>> {
        self.context.clone()
    }
}

impl SurfaceRef {
    #[inline]
    pub unsafe fn from_ll<'a>(raw: *const sys::SDL_Surface) -> &'a SurfaceRef {
        &*(raw as *const () as *const SurfaceRef)
    }

    #[inline]
    pub unsafe fn from_ll_mut<'a>(raw: *mut sys::SDL_Surface) -> &'a mut SurfaceRef {
        &mut *(raw as *mut () as *mut SurfaceRef)
    }

    #[inline]
    pub fn raw(&self) -> *mut sys::SDL_Surface {
        self as *const SurfaceRef as *mut SurfaceRef as *mut () as *mut sys::SDL_Surface
    }

    #[inline]
    fn raw_ref(&self) -> &sys::SDL_Surface {
        unsafe {
            &*(self as *const _ as *const () as *const sys::SDL_Surface)
        }
    }

    pub fn width(&self) -> u32 {
        self.raw_ref().w as u32
    }

    pub fn height(&self) -> u32 {
        self.raw_ref().h as u32
    }

    pub fn pitch(&self) -> u32 {
        self.raw_ref().pitch as u32
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    pub fn rect(&self) -> Rect {
        Rect::new(0, 0, self.width(), self.height())
    }

    pub fn pixel_format(&self) -> pixels::PixelFormat {
        unsafe {
            pixels::PixelFormat::from_ll(self.raw_ref().format)
        }
    }

    pub fn pixel_format_enum(&self) -> pixels::PixelFormatEnum {
        pixels::PixelFormatEnum::from(self.pixel_format())
    }

    /// Locks a surface so that the pixels can be directly accessed safely.
    pub fn with_lock<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
        unsafe {
            if sys::SDL_LockSurface(self.raw()) != 0 { panic!("could not lock surface"); }

            let raw_pixels = self.raw_ref().pixels as *const _;
            let len = self.raw_ref().pitch as usize * (self.raw_ref().h as usize);
            let pixels = ::std::slice::from_raw_parts(raw_pixels, len);
            let rv = f(pixels);
            sys::SDL_UnlockSurface(self.raw());
            rv
        }
    }

    /// Locks a surface so that the pixels can be directly accessed safely.
    pub fn with_lock_mut<R, F: FnOnce(&mut [u8]) -> R>(&mut self, f: F) -> R {
        unsafe {
            if sys::SDL_LockSurface(self.raw()) != 0 { panic!("could not lock surface"); }

            let raw_pixels = self.raw_ref().pixels as *mut _;
            let len = self.raw_ref().pitch as usize * (self.raw_ref().h as usize);
            let pixels = ::std::slice::from_raw_parts_mut(raw_pixels, len);
            let rv = f(pixels);
            sys::SDL_UnlockSurface(self.raw());
            rv
        }
    }

    /// Returns the Surface's pixel buffer if the Surface doesn't require locking
    /// (e.g. it's a software surface).
    pub fn without_lock(&self) -> Option<&[u8]> {
        if self.must_lock() {
            None
        } else {
            unsafe {
                let raw_pixels = self.raw_ref().pixels as *const _;
                let len = self.raw_ref().pitch as usize * (self.raw_ref().h as usize);

                Some(::std::slice::from_raw_parts(raw_pixels, len))
            }
        }
    }

    /// Returns the Surface's pixel buffer if the Surface doesn't require locking
    /// (e.g. it's a software surface).
    pub fn without_lock_mut(&mut self) -> Option<&mut [u8]> {
        if self.must_lock() {
            None
        } else {
            unsafe {
                let raw_pixels = self.raw_ref().pixels as *mut _;
                let len = self.raw_ref().pitch as usize * (self.raw_ref().h as usize);

                Some(::std::slice::from_raw_parts_mut(raw_pixels, len))
            }
        }
    }

    /// Returns true if the Surface needs to be locked before accessing the Surface pixels.
    pub fn must_lock(&self) -> bool {
        // Implements the SDL_MUSTLOCK macro.
        (self.raw_ref().flags & sys::SDL_RLEACCEL) != 0
    }

    pub fn save_bmp_rw(&self, rwops: &mut RWops) -> Result<(), String> {
        let ret = unsafe {
            sys::SDL_SaveBMP_RW(self.raw(), rwops.raw(), 0)
        };
        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn save_bmp<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let mut file = RWops::from_file(path, "wb")?;
        self.save_bmp_rw(&mut file)
    }

    pub fn set_palette(&mut self, palette: &pixels::Palette) -> Result<(), String> {
        let result = unsafe { sys::SDL_SetSurfacePalette(self.raw(), palette.raw()) };

        match result {
            0 => Ok(()),
            _ => Err(get_error())
        }
    }

    #[allow(non_snake_case)]
    pub fn enable_RLE(&mut self) {
        let result = unsafe { sys::SDL_SetSurfaceRLE(self.raw(), 1) };

        if result != 0 {
            // Should only panic on a null Surface
            panic!(get_error());
        }
    }

    #[allow(non_snake_case)]
    pub fn disable_RLE(&mut self) {
        let result = unsafe { sys::SDL_SetSurfaceRLE(self.raw(), 0) };

        if result != 0 {
            // Should only panic on a null Surface
            panic!(get_error());
        }
    }

    pub fn set_color_key(&mut self, enable: bool, color: pixels::Color) -> Result<(), String> {
        let key = color.to_u32(&self.pixel_format());
        let result = unsafe {
            sys::SDL_SetColorKey(self.raw(), if enable { 1 } else { 0 }, key)
        };
        if result == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }

    /// The function will fail if the surface doesn't have color key enabled.
    pub fn color_key(&self) -> Result<pixels::Color, String> {
        let mut key = 0;

        // SDL_GetColorKey does not mutate, but requires a non-const pointer anyway.

        let result = unsafe {
            sys::SDL_GetColorKey(self.raw(), &mut key)
        };

        if result == 0 {
            Ok(pixels::Color::from_u32(&self.pixel_format(), key))
        } else {
            Err(get_error())
        }
    }

    pub fn set_color_mod(&mut self, color: pixels::Color) {
        let (r, g, b) = color.rgb();
        let result = unsafe { sys::SDL_SetSurfaceColorMod(self.raw(), r, g, b) };

        if result != 0 {
            // Should only fail on a null Surface
            panic!(get_error());
        }
    }

    pub fn color_mod(&self) -> pixels::Color {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        // SDL_GetSurfaceColorMod does not mutate, but requires a non-const pointer anyway.

        let result = unsafe {
            sys::SDL_GetSurfaceColorMod(self.raw(), &mut r, &mut g, &mut b) == 0
        };

        if result {
            pixels::Color::RGB(r, g, b)
        } else {
            // Should only fail on a null Surface
            panic!(get_error())
        }
    }

    pub fn fill_rect<R>(&mut self, rect: R, color: pixels::Color) -> Result<(), String>
    where R: Into<Option<Rect>>
    {
        unsafe {
            let rect = rect.into();
            let rect_ptr = mem::transmute(rect.as_ref()); // TODO find a better way to transform
            // Option<&...> into a *const _
            let format = self.pixel_format();
            let result = sys::SDL_FillRect(self.raw(), rect_ptr, color.to_u32(&format) );
            match result {
                0 => Ok(()),
                _ => Err(get_error())
            }
        }
    }

    #[allow(clippy::clone_on_copy)]
    pub fn fill_rects(&mut self, rects: &[Rect], color: pixels::Color) -> Result<(), String>
    {
        for rect in rects.iter() {
            if let Err(e) = self.fill_rect(rect.clone(), color)
            {
                return Err(e);
            }
        }

        Ok(())
    }

    pub fn set_alpha_mod(&mut self, alpha: u8) {
        let result = unsafe {
            sys::SDL_SetSurfaceAlphaMod(self.raw(), alpha)
        };

        if result != 0 {
            // Should only fail on a null Surface
            panic!(get_error());
        }
    }

    pub fn alpha_mod(&self) -> u8 {
        let mut alpha = 0;
        let result = unsafe {
            sys::SDL_GetSurfaceAlphaMod(self.raw(), &mut alpha)
        };

        match result {
            0 => alpha,
            // Should only fail on a null Surface
            _ => panic!(get_error())
        }
    }

    /// The function will fail if the blend mode is not supported by SDL.
    pub fn set_blend_mode(&mut self, mode: BlendMode) -> Result<(), String> {
        let result = unsafe {
            sys::SDL_SetSurfaceBlendMode(self.raw(), transmute(mode))
        };

        match result {
            0 => Ok(()),
            _ => Err(get_error())
        }
    }

    pub fn blend_mode(&self) -> BlendMode {
        let mut mode = sys::SDL_BlendMode::SDL_BLENDMODE_NONE;
        let result = unsafe {
            sys::SDL_GetSurfaceBlendMode(self.raw(), &mut mode)
        };

        match result {
            0 => BlendMode::try_from(mode as u32).unwrap(),
            // Should only fail on a null Surface
            _ => panic!(get_error())
        }
    }

    /// Sets the clip rectangle for the surface.
    ///
    /// If the rectangle is `None`, clipping will be disabled.
    pub fn set_clip_rect<R>(&mut self, rect: R) -> bool
    where R: Into<Option<Rect>>
    {
        let rect = rect.into();
        unsafe {
            sys::SDL_SetClipRect(self.raw(), match rect {
                Some(rect) => rect.raw(),
                None => ptr::null()
            }) == sys::SDL_bool::SDL_TRUE
        }
    }

    /// Gets the clip rectangle for the surface.
    ///
    /// Returns `None` if clipping is disabled.
    pub fn clip_rect(&self) -> Option<Rect> {
        let mut raw = mem::MaybeUninit::uninit();
        unsafe {
            sys::SDL_GetClipRect(self.raw(), raw.as_mut_ptr())
        };
        let raw = unsafe { raw.assume_init() };

        if raw.w == 0 || raw.h == 0 {
            None
        } else {
            Some(Rect::from_ll(raw))
        }
    }

    /// Copies the surface into a new one that is optimized for blitting to a surface of a specified pixel format.
    pub fn convert(&self, format: &pixels::PixelFormat) -> Result<Surface<'static>, String> {
        // SDL_ConvertSurface takes a flag as the last parameter, which should be 0 by the docs.
        let surface_ptr = unsafe { sys::SDL_ConvertSurface(self.raw(), format.raw(), 0u32) };

        if surface_ptr.is_null() {
            Err(get_error())
        } else {
            unsafe { Ok(Surface::from_ll(surface_ptr)) }
        }
    }

    /// Copies the surface into a new one of a specified pixel format.
    pub fn convert_format(&self, format: pixels::PixelFormatEnum) -> Result<Surface<'static>, String> {
        // SDL_ConvertSurfaceFormat takes a flag as the last parameter, which should be 0 by the docs.
        let surface_ptr = unsafe { sys::SDL_ConvertSurfaceFormat(self.raw(), format as u32, 0u32) };

        if surface_ptr.is_null() {
            Err(get_error())
        } else {
            unsafe { Ok(Surface::from_ll(surface_ptr)) }
        }
    }

    /// Performs surface blitting (surface copying).
    ///
    /// Returns the final blit rectangle, if a `dst_rect` was provided.
    pub fn blit<R1, R2>(&self, src_rect: R1,
            dst: &mut SurfaceRef, dst_rect: R2)
            -> Result<Option<Rect>, String>
        where R1: Into<Option<Rect>>,
              R2: Into<Option<Rect>>,
    {
        let src_rect = src_rect.into();
        let dst_rect = dst_rect.into();

        unsafe {
            let src_rect_ptr = src_rect.as_ref().map(|r| r.raw()).unwrap_or(ptr::null());

            // Copy the rect here to make a mutable copy without requiring
            // a mutable argument
            let mut dst_rect = dst_rect;
            let dst_rect_ptr = dst_rect.as_mut().map(|r| r.raw_mut())
                .unwrap_or(ptr::null_mut());
            let result = sys::SDL_UpperBlit(
                self.raw(), src_rect_ptr, dst.raw(), dst_rect_ptr
            );

            if result == 0 {
                Ok(dst_rect)
            } else {
                Err(get_error())
            }
        }
    }

    /// Performs low-level surface blitting.
    ///
    /// Unless you know what you're doing, use `blit()` instead, which will clip the input rectangles.
    /// This function could crash if the rectangles aren't pre-clipped to the surface, and is therefore unsafe.
    pub unsafe fn lower_blit<R1, R2>(&self, src_rect: R1,
                      dst: &mut SurfaceRef, dst_rect: R2) -> Result<(), String>
    where R1: Into<Option<Rect>>,
          R2: Into<Option<Rect>>,
    {
        let src_rect = src_rect.into();
        let dst_rect = dst_rect.into();

        match {
            // The rectangles don't change, but the function requires mutable pointers.
            let src_rect_ptr = src_rect.as_ref().map(|r| r.raw())
                .unwrap_or(ptr::null()) as *mut _;
            let dst_rect_ptr = dst_rect.as_ref().map(|r| r.raw())
                .unwrap_or(ptr::null()) as *mut _;
            sys::SDL_LowerBlit(self.raw(), src_rect_ptr, dst.raw(), dst_rect_ptr)
        } {
            0 => Ok(()),
            _ => Err(get_error())
        }
    }

    /// Performs scaled surface bliting (surface copying).
    ///
    /// Returns the final blit rectangle, if a `dst_rect` was provided.
    pub fn blit_scaled<R1, R2>(&self, src_rect: R1,
                             dst: &mut SurfaceRef, dst_rect: R2) -> Result<Option<Rect>, String>
    where R1: Into<Option<Rect>>,
          R2: Into<Option<Rect>>,
    {
        let src_rect = src_rect.into();
        let dst_rect = dst_rect.into();

        match unsafe {
            let src_rect_ptr = src_rect.as_ref().map(|r| r.raw()).unwrap_or(ptr::null());

            // Copy the rect here to make a mutable copy without requiring
            // a mutable argument
            let mut dst_rect = dst_rect;
            let dst_rect_ptr = dst_rect.as_mut().map(|r| r.raw_mut())
                .unwrap_or(ptr::null_mut());
            sys::SDL_UpperBlitScaled(self.raw(), src_rect_ptr, dst.raw(), dst_rect_ptr)
        } {
            0 => Ok(dst_rect),
            _ => Err(get_error())
        }
    }

    /// Performs low-level scaled surface blitting.
    ///
    /// Unless you know what you're doing, use `blit_scaled()` instead, which will clip the input rectangles.
    /// This function could crash if the rectangles aren't pre-clipped to the surface, and is therefore unsafe.
    pub unsafe fn lower_blit_scaled<R1, R2>(&self, src_rect: R1,
                             dst: &mut SurfaceRef, dst_rect: R2) -> Result<(), String>
    where R1: Into<Option<Rect>>,
          R2: Into<Option<Rect>>
    {

        match {
            // The rectangles don't change, but the function requires mutable pointers.
            let src_rect_ptr = src_rect.into().as_ref().map(|r| r.raw())
                .unwrap_or(ptr::null()) as *mut _;
            let dst_rect_ptr = dst_rect.into().as_ref().map(|r| r.raw())
                .unwrap_or(ptr::null()) as *mut _;
            sys::SDL_LowerBlitScaled(self.raw(), src_rect_ptr, dst.raw(), dst_rect_ptr)
        } {
            0 => Ok(()),
            _ => Err(get_error())
        }
    }

    /*
    pub fn SDL_ConvertPixels(width: c_int, height: c_int, src_format: uint32_t, src: *c_void, src_pitch: c_int, dst_format: uint32_t, dst: *c_void, dst_pitch: c_int) -> c_int;
    */
}
