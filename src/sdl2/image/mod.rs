//!
//! A binding for the library `SDL2_image`
//!
//!
//! Note that you need to build with the
//! feature `image` for this module to be enabled,
//! like so:
//!
//! ```bash
//! $ cargo build --features "image"
//! ```
//!
//! If you want to use this with from inside your own
//! crate, you will need to add this in your Cargo.toml
//!
//! ```toml
//! [dependencies.sdl2]
//! version = ...
//! default-features = false
//! features = ["image"]
//! ```

use get_error;
use render::{Texture, TextureCreator};
use rwops::RWops;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::path::Path;
use surface::Surface;
use sys;
use sys::image;
use version::Version;

bitflags! {
    /// InitFlags are passed to init() to control which subsystem
    /// functionality to load.
    pub struct InitFlag : u32 {
        const JPG  = image::IMG_InitFlags_IMG_INIT_JPG as u32;
        const PNG  = image::IMG_InitFlags_IMG_INIT_PNG as u32;
        const TIF  = image::IMG_InitFlags_IMG_INIT_TIF as u32;
        const WEBP = image::IMG_InitFlags_IMG_INIT_WEBP as u32;
    }
}

// This is used for error message for init
impl ::std::fmt::Display for InitFlag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        if self.contains(InitFlag::JPG) {
            f.write_str("INIT_JPG ")?;
        }
        if self.contains(InitFlag::PNG) {
            f.write_str("INIT_PNG ")?;
        }
        if self.contains(InitFlag::TIF) {
            f.write_str("INIT_TIF ")?;
        }
        if self.contains(InitFlag::WEBP) {
            f.write_str("INIT_WEBP ")?;
        }
        Ok(())
    }
}

/// Static method extensions for creating Surfaces
pub trait LoadSurface: Sized {
    // Self is only returned here to type hint to the compiler.
    // The syntax for type hinting in this case is not yet defined.
    // The intended return value is Result<~Surface, String>.
    fn from_file<P: AsRef<Path>>(filename: P) -> Result<Self, String>;
    fn from_xpm_array(xpm: *const *const i8) -> Result<Self, String>;
}

/// Method extensions to Surface for saving to disk
pub trait SaveSurface {
    fn save<P: AsRef<Path>>(&self, filename: P) -> Result<(), String>;
    fn save_rw(&self, dst: &mut RWops) -> Result<(), String>;
}

impl<'a> LoadSurface for Surface<'a> {
    fn from_file<P: AsRef<Path>>(filename: P) -> Result<Surface<'a>, String> {
        //! Loads an SDL Surface from a file
        unsafe {
            let c_filename = CString::new(filename.as_ref().to_str().unwrap()).unwrap();
            let raw = image::IMG_Load(c_filename.as_ptr() as *const _);
            if (raw as *mut ()).is_null() {
                Err(get_error())
            } else {
                Ok(Surface::from_ll(raw))
            }
        }
    }

    fn from_xpm_array(xpm: *const *const i8) -> Result<Surface<'a>, String> {
        //! Loads an SDL Surface from XPM data
        unsafe {
            let raw = image::IMG_ReadXPMFromArray(xpm as *mut *mut c_char);
            if (raw as *mut ()).is_null() {
                Err(get_error())
            } else {
                Ok(Surface::from_ll(raw))
            }
        }
    }
}

impl<'a> SaveSurface for Surface<'a> {
    fn save<P: AsRef<Path>>(&self, filename: P) -> Result<(), String> {
        //! Saves an SDL Surface to a file
        unsafe {
            let c_filename = CString::new(filename.as_ref().to_str().unwrap()).unwrap();
            let status = image::IMG_SavePNG(self.raw(), c_filename.as_ptr() as *const _);
            if status != 0 {
                Err(get_error())
            } else {
                Ok(())
            }
        }
    }

    fn save_rw(&self, dst: &mut RWops) -> Result<(), String> {
        //! Saves an SDL Surface to an RWops
        unsafe {
            let status = image::IMG_SavePNG_RW(self.raw(), dst.raw(), 0);

            if status != 0 {
                Err(get_error())
            } else {
                Ok(())
            }
        }
    }
}

/// Method extensions for creating Textures from a `TextureCreator`
pub trait LoadTexture {
    fn load_texture<P: AsRef<Path>>(&self, filename: P) -> Result<Texture, String>;
    fn load_texture_bytes(&self, buf: &[u8]) -> Result<Texture, String>;
}

impl<T> LoadTexture for TextureCreator<T> {
    fn load_texture<P: AsRef<Path>>(&self, filename: P) -> Result<Texture, String> {
        //! Loads an SDL Texture from a file
        unsafe {
            let c_filename = CString::new(filename.as_ref().to_str().unwrap()).unwrap();
            let raw = image::IMG_LoadTexture(self.raw(), c_filename.as_ptr() as *const _);
            if (raw as *mut ()).is_null() {
                Err(get_error())
            } else {
                Ok(self.raw_create_texture(raw))
            }
        }
    }

    #[doc(alias = "IMG_LoadTexture")]
    fn load_texture_bytes(&self, buf: &[u8]) -> Result<Texture, String> {
        //! Loads an SDL Texture from a buffer that the format must be something supported by SDL2_image (png, jpeg, ect, but NOT RGBA8888 bytes for instance)
        unsafe {
            let buf = sdl2_sys::SDL_RWFromMem(buf.as_ptr() as *mut libc::c_void, buf.len() as i32);
            let raw = image::IMG_LoadTexture_RW(self.raw(), buf, 1); // close(free) buff after load
            if (raw as *mut ()).is_null() {
                Err(get_error())
            } else {
                Ok(self.raw_create_texture(raw))
            }
        }
    }
}

/// Context manager for `sdl2_image` to manage quitting. Can't do much with it but
/// keep it alive while you are using it.
pub struct Sdl2ImageContext;

impl Drop for Sdl2ImageContext {
    fn drop(&mut self) {
        unsafe {
            image::IMG_Quit();
        }
    }
}

/// Initializes `SDL2_image` with `InitFlags`.
/// If not every flag is set it returns an error
pub fn init(flags: InitFlag) -> Result<Sdl2ImageContext, String> {
    let return_flags = unsafe {
        let used = image::IMG_Init(flags.bits() as c_int);
        InitFlag::from_bits_truncate(used as u32)
    };
    if !flags.intersects(return_flags) {
        // According to docs, error message text is not always set
        let mut error = get_error();
        if error.is_empty() {
            let un_init_flags = return_flags ^ flags;
            error = format!("Could not init: {}", un_init_flags);
            let _ = ::set_error(&error);
        }
        Err(error)
    } else {
        Ok(Sdl2ImageContext)
    }
}

/// Returns the version of the dynamically linked `SDL_image` library
pub fn get_linked_version() -> Version {
    unsafe { Version::from_ll(*image::IMG_Linked_Version()) }
}

#[inline]
fn to_surface_result<'a>(raw: *mut sys::SDL_Surface) -> Result<Surface<'a>, String> {
    if (raw as *mut ()).is_null() {
        Err(get_error())
    } else {
        unsafe { Ok(Surface::from_ll(raw)) }
    }
}

pub trait ImageRWops {
    /// load as a surface. except TGA
    fn load(&self) -> Result<Surface<'static>, String>;
    /// load as a surface. This can load all supported image formats.
    fn load_typed(&self, _type: &str) -> Result<Surface<'static>, String>;

    fn load_cur(&self) -> Result<Surface<'static>, String>;
    fn load_ico(&self) -> Result<Surface<'static>, String>;
    fn load_bmp(&self) -> Result<Surface<'static>, String>;
    fn load_pnm(&self) -> Result<Surface<'static>, String>;
    fn load_xpm(&self) -> Result<Surface<'static>, String>;
    fn load_xcf(&self) -> Result<Surface<'static>, String>;
    fn load_pcx(&self) -> Result<Surface<'static>, String>;
    fn load_gif(&self) -> Result<Surface<'static>, String>;
    fn load_jpg(&self) -> Result<Surface<'static>, String>;
    fn load_tif(&self) -> Result<Surface<'static>, String>;
    fn load_png(&self) -> Result<Surface<'static>, String>;
    fn load_tga(&self) -> Result<Surface<'static>, String>;
    fn load_lbm(&self) -> Result<Surface<'static>, String>;
    fn load_xv(&self) -> Result<Surface<'static>, String>;
    fn load_webp(&self) -> Result<Surface<'static>, String>;

    fn is_cur(&self) -> bool;
    fn is_ico(&self) -> bool;
    fn is_bmp(&self) -> bool;
    fn is_pnm(&self) -> bool;
    fn is_xpm(&self) -> bool;
    fn is_xcf(&self) -> bool;
    fn is_pcx(&self) -> bool;
    fn is_gif(&self) -> bool;
    fn is_jpg(&self) -> bool;
    fn is_tif(&self) -> bool;
    fn is_png(&self) -> bool;
    fn is_lbm(&self) -> bool;
    fn is_xv(&self) -> bool;
    fn is_webp(&self) -> bool;
}

impl<'a> ImageRWops for RWops<'a> {
    fn load(&self) -> Result<Surface<'static>, String> {
        let raw = unsafe { image::IMG_Load_RW(self.raw(), 0) };
        to_surface_result(raw)
    }
    fn load_typed(&self, _type: &str) -> Result<Surface<'static>, String> {
        let raw = unsafe {
            let c_type = CString::new(_type.as_bytes()).unwrap();
            image::IMG_LoadTyped_RW(self.raw(), 0, c_type.as_ptr() as *const _)
        };
        to_surface_result(raw)
    }

    fn load_cur(&self) -> Result<Surface<'static>, String> {
        let raw = unsafe { image::IMG_LoadCUR_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_ico(&self) -> Result<Surface<'static>, String> {
        let raw = unsafe { image::IMG_LoadICO_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_bmp(&self) -> Result<Surface<'static>, String> {
        let raw = unsafe { image::IMG_LoadBMP_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_pnm(&self) -> Result<Surface<'static>, String> {
        let raw = unsafe { image::IMG_LoadPNM_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_xpm(&self) -> Result<Surface<'static>, String> {
        let raw = unsafe { image::IMG_LoadXPM_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_xcf(&self) -> Result<Surface<'static>, String> {
        let raw = unsafe { image::IMG_LoadXCF_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_pcx(&self) -> Result<Surface<'static>, String> {
        let raw = unsafe { image::IMG_LoadPCX_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_gif(&self) -> Result<Surface<'static>, String> {
        let raw = unsafe { image::IMG_LoadGIF_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_jpg(&self) -> Result<Surface<'static>, String> {
        let raw = unsafe { image::IMG_LoadJPG_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_tif(&self) -> Result<Surface<'static>, String> {
        let raw = unsafe { image::IMG_LoadTIF_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_png(&self) -> Result<Surface<'static>, String> {
        let raw = unsafe { image::IMG_LoadPNG_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_tga(&self) -> Result<Surface<'static>, String> {
        let raw = unsafe { image::IMG_LoadTGA_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_lbm(&self) -> Result<Surface<'static>, String> {
        let raw = unsafe { image::IMG_LoadLBM_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_xv(&self) -> Result<Surface<'static>, String> {
        let raw = unsafe { image::IMG_LoadXV_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_webp(&self) -> Result<Surface<'static>, String> {
        let raw = unsafe { image::IMG_LoadWEBP_RW(self.raw()) };
        to_surface_result(raw)
    }

    fn is_cur(&self) -> bool {
        unsafe { image::IMG_isCUR(self.raw()) == 1 }
    }
    fn is_ico(&self) -> bool {
        unsafe { image::IMG_isICO(self.raw()) == 1 }
    }
    fn is_bmp(&self) -> bool {
        unsafe { image::IMG_isBMP(self.raw()) == 1 }
    }
    fn is_pnm(&self) -> bool {
        unsafe { image::IMG_isPNM(self.raw()) == 1 }
    }
    fn is_xpm(&self) -> bool {
        unsafe { image::IMG_isXPM(self.raw()) == 1 }
    }
    fn is_xcf(&self) -> bool {
        unsafe { image::IMG_isXCF(self.raw()) == 1 }
    }
    fn is_pcx(&self) -> bool {
        unsafe { image::IMG_isPCX(self.raw()) == 1 }
    }
    fn is_gif(&self) -> bool {
        unsafe { image::IMG_isGIF(self.raw()) == 1 }
    }
    fn is_jpg(&self) -> bool {
        unsafe { image::IMG_isJPG(self.raw()) == 1 }
    }
    fn is_tif(&self) -> bool {
        unsafe { image::IMG_isTIF(self.raw()) == 1 }
    }
    fn is_png(&self) -> bool {
        unsafe { image::IMG_isPNG(self.raw()) == 1 }
    }
    fn is_lbm(&self) -> bool {
        unsafe { image::IMG_isLBM(self.raw()) == 1 }
    }
    fn is_xv(&self) -> bool {
        unsafe { image::IMG_isXV(self.raw()) == 1 }
    }
    fn is_webp(&self) -> bool {
        unsafe { image::IMG_isWEBP(self.raw()) == 1 }
    }
}
