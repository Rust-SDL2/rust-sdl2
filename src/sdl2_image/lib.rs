#![feature(macro_rules)]

#![crate_id="sdl2_image#sdl2_image:0.1"]
#![crate_type = "lib"]
#![desc = "SDL2_image bindings and wrappers"]
#![comment = "SDL2_image bindings and wrappers"]
#![license = "MIT"]


extern crate sdl2;
extern crate libc;

use libc::{c_int, c_char};
use std::ptr;
use sdl2::surface::Surface;
use sdl2::render::Texture;
use sdl2::render::Renderer;
use sdl2::rwops::RWops;
use sdl2::version::Version;
use sdl2::get_error;

// Setup linking for all targets.
#[cfg(target_os="macos")]
mod mac {
    #[cfg(mac_framework)]
    #[link(kind="framework", name="SDL2_image")]
    extern {}

    #[cfg(not(mac_framework))]
    #[link(name="SDL2_image")]
    extern {}
}

#[cfg(target_os="win32")]
#[cfg(target_os="linux")]
#[cfg(target_os="freebsd")]
mod others {
    #[link(name="SDL2_image")]
    extern {}
}

#[allow(non_camel_case_types, dead_code)]
mod ffi;
mod flag;

/// InitFlags are passed to init() to control which subsystem
/// functionality to load.
flag_type!(InitFlag : c_int {
    InitJpg = ffi::IMG_INIT_JPG,
    InitPng = ffi::IMG_INIT_PNG,
    InitTif = ffi::IMG_INIT_TIF,
    InitWebp = ffi::IMG_INIT_WEBP
})

/// Static method extensions for creating Surfaces
pub trait LoadSurface {
    // Self is only returned here to type hint to the compiler.
    // The syntax for type hinting in this case is not yet defined.
    // The intended return value is Result<~Surface, ~str>.
    fn from_file(filename: &Path) -> Result<~Self, ~str>;
    fn from_xpm_array(xpm: **i8) -> Result<~Self, ~str>;
}

/// Method extensions to Surface for saving to disk
pub trait SaveSurface {
    fn save(&self, filename: &Path) -> Result<(), ~str>;
    fn save_rw(&self, dst: &mut RWops) -> Result<(), ~str>;
}

impl LoadSurface for Surface {
    fn from_file(filename: &Path) -> Result<~Surface, ~str> {
        //! Loads an SDL Surface from a file
        unsafe {
            let raw = ffi::IMG_Load(filename.to_c_str().unwrap());
            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }

    fn from_xpm_array(xpm: **i8) -> Result<~Surface, ~str> {
        //! Loads an SDL Surface from XPM data
        unsafe {
            let raw = ffi::IMG_ReadXPMFromArray(xpm as **c_char);
            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }
}

impl SaveSurface for Surface {
    fn save(&self, filename: &Path) -> Result<(), ~str> {
        //! Saves an SDL Surface to a file
        unsafe {
            let status = ffi::IMG_SavePNG(self.raw,
                                          filename.to_c_str().unwrap());
            if status != 0 {
                Err(get_error())
            } else {
                Ok(())
            }
        }
    }

    fn save_rw(&self, dst: &mut RWops) -> Result<(), ~str> {
        //! Saves an SDL Surface to an RWops
        unsafe {
            let status = ffi::IMG_SavePNG_RW(self.raw, dst.raw, 0);

            if status != 0 {
                Err(get_error())
            } else {
                Ok(())
            }
        }
    }
}

/// Method extensions for creating Textures from a Renderer
pub trait LoadTexture {
    fn load_texture(&self, filename: &Path) -> Result<~Texture, ~str>;
}

impl LoadTexture for Renderer {
    fn load_texture(&self, filename: &Path) -> Result<~Texture, ~str> {
        //! Loads an SDL Texture from a file
        unsafe {
            let raw = ffi::IMG_LoadTexture(self.raw,
                                           filename.to_c_str().unwrap());
            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Texture{ raw: raw, owned: true })
            }
        }
    }
}

pub fn init(flags: InitFlag) -> InitFlag {
    //! Initializes SDL2_image with InitFlags and returns which
    //! InitFlags were actually used.
    let used = unsafe { ffi::IMG_Init(flags.get()) };
    InitFlag::new(used)
}

pub fn quit() {
    //! Teardown the SDL2_Image subsystem
    unsafe { ffi::IMG_Quit(); }
}

pub fn get_linked_version() -> Version {
    //! Returns the version of the dynamically linked SDL_image library
    unsafe {
        Version::from_ll(ffi::IMG_Linked_Version())
    }
}

#[inline]
fn to_surface_result(raw: *sdl2::surface::ll::SDL_Surface) -> Result<~Surface, ~str> {
    if raw == ptr::null() {
        Err(get_error())
    } else {
        Ok(~Surface { raw: raw, owned: true })
    }
}

pub trait ImageRWops {
    /// load as a surface. except TGA
    fn load(&self) -> Result<~Surface, ~str>;
    /// load as a surface. This can load all supported image formats.
    fn load_typed(&self, _type: &str) -> Result<~Surface, ~str>;

    fn load_cur(&self) -> Result<~Surface, ~str>;
    fn load_ico(&self) -> Result<~Surface, ~str>;
    fn load_bmp(&self) -> Result<~Surface, ~str>;
    fn load_pnm(&self) -> Result<~Surface, ~str>;
    fn load_xpm(&self) -> Result<~Surface, ~str>;
    fn load_xcf(&self) -> Result<~Surface, ~str>;
    fn load_pcx(&self) -> Result<~Surface, ~str>;
    fn load_gif(&self) -> Result<~Surface, ~str>;
    fn load_jpg(&self) -> Result<~Surface, ~str>;
    fn load_tif(&self) -> Result<~Surface, ~str>;
    fn load_png(&self) -> Result<~Surface, ~str>;
    fn load_tga(&self) -> Result<~Surface, ~str>;
    fn load_lbm(&self) -> Result<~Surface, ~str>;
    fn load_xv(&self)  -> Result<~Surface, ~str>;
    fn load_webp(&self) -> Result<~Surface, ~str>;

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
    fn is_xv(&self)  -> bool;
    fn is_webp(&self) -> bool;
}

impl ImageRWops for RWops {
    fn load(&self) -> Result<~Surface, ~str> {
        let raw = unsafe {
            ffi::IMG_Load_RW(self.raw, 0)
        };
        to_surface_result(raw)
    }
    fn load_typed(&self, _type: &str) -> Result<~Surface, ~str> {
        let raw = unsafe {
            ffi::IMG_LoadTyped_RW(self.raw, 0, _type.to_c_str().unwrap())
        };
        to_surface_result(raw)
    }

    fn load_cur(&self) -> Result<~Surface, ~str> {
        let raw = unsafe { ffi::IMG_LoadCUR_RW(self.raw) };
        to_surface_result(raw)
    }
    fn load_ico(&self) -> Result<~Surface, ~str> {
        let raw = unsafe { ffi::IMG_LoadICO_RW(self.raw) };
        to_surface_result(raw)
    }
    fn load_bmp(&self) -> Result<~Surface, ~str> {
        let raw = unsafe { ffi::IMG_LoadBMP_RW(self.raw) };
        to_surface_result(raw)
    }
    fn load_pnm(&self) -> Result<~Surface, ~str> {
        let raw = unsafe { ffi::IMG_LoadPNM_RW(self.raw) };
        to_surface_result(raw)
    }
    fn load_xpm(&self) -> Result<~Surface, ~str> {
        let raw = unsafe { ffi::IMG_LoadXPM_RW(self.raw) };
        to_surface_result(raw)
    }
    fn load_xcf(&self) -> Result<~Surface, ~str> {
        let raw = unsafe { ffi::IMG_LoadXCF_RW(self.raw) };
        to_surface_result(raw)
    }
    fn load_pcx(&self) -> Result<~Surface, ~str> {
        let raw = unsafe { ffi::IMG_LoadPCX_RW(self.raw) };
        to_surface_result(raw)
    }
    fn load_gif(&self) -> Result<~Surface, ~str> {
        let raw = unsafe { ffi::IMG_LoadGIF_RW(self.raw) };
        to_surface_result(raw)
    }
    fn load_jpg(&self) -> Result<~Surface, ~str> {
        let raw = unsafe { ffi::IMG_LoadJPG_RW(self.raw) };
        to_surface_result(raw)
    }
    fn load_tif(&self) -> Result<~Surface, ~str> {
        let raw = unsafe { ffi::IMG_LoadTIF_RW(self.raw) };
        to_surface_result(raw)
    }
    fn load_png(&self) -> Result<~Surface, ~str> {
        let raw = unsafe { ffi::IMG_LoadPNG_RW(self.raw) };
        to_surface_result(raw)
    }
    fn load_tga(&self) -> Result<~Surface, ~str> {
        let raw = unsafe { ffi::IMG_LoadTGA_RW(self.raw) };
        to_surface_result(raw)
    }
    fn load_lbm(&self) -> Result<~Surface, ~str> {
        let raw = unsafe { ffi::IMG_LoadLBM_RW(self.raw) };
        to_surface_result(raw)
    }
    fn load_xv(&self)  -> Result<~Surface, ~str> {
        let raw = unsafe { ffi::IMG_LoadXV_RW(self.raw) };
        to_surface_result(raw)
    }
    fn load_webp(&self)  -> Result<~Surface, ~str> {
        let raw = unsafe { ffi::IMG_LoadWEBP_RW(self.raw) };
        to_surface_result(raw)
    }

    fn is_cur(&self) -> bool {
        unsafe { ffi::IMG_isCUR(self.raw) == 1 }
    }
    fn is_ico(&self) -> bool {
        unsafe { ffi::IMG_isICO(self.raw) == 1 }
    }
    fn is_bmp(&self) -> bool {
        unsafe { ffi::IMG_isBMP(self.raw) == 1 }
    }
    fn is_pnm(&self) -> bool {
        unsafe { ffi::IMG_isPNM(self.raw) == 1 }
    }
    fn is_xpm(&self) -> bool {
        unsafe { ffi::IMG_isXPM(self.raw) == 1 }
    }
    fn is_xcf(&self) -> bool {
        unsafe { ffi::IMG_isXCF(self.raw) == 1 }
    }
    fn is_pcx(&self) -> bool {
        unsafe { ffi::IMG_isPCX(self.raw) == 1 }
    }
    fn is_gif(&self) -> bool {
        unsafe { ffi::IMG_isGIF(self.raw) == 1 }
    }
    fn is_jpg(&self) -> bool {
        unsafe { ffi::IMG_isJPG(self.raw) == 1 }
    }
    fn is_tif(&self) -> bool {
        unsafe { ffi::IMG_isTIF(self.raw) == 1 }
    }
    fn is_png(&self) -> bool {
        unsafe { ffi::IMG_isPNG(self.raw) == 1 }
    }
    fn is_lbm(&self) -> bool {
        unsafe { ffi::IMG_isLBM(self.raw) == 1 }
    }
    fn is_xv(&self)  -> bool {
        unsafe { ffi::IMG_isXV(self.raw)  == 1 }
    }
    fn is_webp(&self) -> bool {
        unsafe { ffi::IMG_isWEBP(self.raw)  == 1 }
    }
}
