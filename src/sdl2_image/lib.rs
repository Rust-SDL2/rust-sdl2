#[crate_type = "rlib"];
#[crate_id="github.com/xsleonard/sdl2_image-rs#sdl2_image:0.1"];
#[desc = "SDL2_image bindings and wrappers"];
#[comment = "SDL2_image bindings and wrappers"];
#[license = "MIT"];

extern mod sdl2;

use std::libc::{c_int, c_char};
use std::ptr;
use std::cast;
use std::io;
use sdl2::surface::Surface;
use sdl2::render::Texture;
use sdl2::render::Renderer;
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

mod ffi;

// repr(C) "makes the size of the enum's discriminant the default
// size of enums that the C ABI for the platform uses."
#[repr(C)]
#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum InitFlag {
    InitJpg = ffi::IMG_INIT_JPG as int,
    InitPng = ffi::IMG_INIT_PNG as int,
    InitTif = ffi::IMG_INIT_TIF as int,
    InitWebp = ffi::IMG_INIT_WEBP as int,
}

#[deriving(Eq, Clone)]
pub struct SDLImageVersion {
    major: int,
    minor: int,
    patch: int,
}

impl ToStr for SDLImageVersion {
    fn to_str(&self) -> ~str {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl SDLImageVersion {
    fn from_sdl_version(sv: *ffi::SDL_version) -> SDLImageVersion {
        unsafe {
            let v = *sv;
            SDLImageVersion{ major: v.major, minor: v.minor, patch: v.patch }
        }
    }
}

pub trait LoadSurface {
    // Self is only returned here to type hint to the compiler.
    // The syntax for type hinting in this case is not yet defined.
    // The intended return value is Result<~Surface, ~str>.
    fn from_file(filename: &str) -> Result<~Self, ~str>;
    fn from_xpm_array(xpm: **i8) -> Result<~Self, ~str>;
}

pub trait SaveImage {
    fn save(&self, filename: &str) -> Result<(), ~str>;
}

impl LoadSurface for Surface {
    fn from_file(filename: &str) -> Result<~Surface, ~str> {
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

impl SaveImage for Surface {
    fn save(&self, filename: &str) -> Result<(), ~str> {
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
}

pub trait LoadTexture {
    fn load_texture(&self, filename: &str) -> Result<~Texture, ~str>;
}

impl LoadTexture for Renderer {
    fn load_texture(&self, filename: &str) -> Result<~Texture, ~str> {
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

pub fn init(flags: &[InitFlag]) -> ~[InitFlag] {
    //! Initializes SDL2_image with InitFlags and returns which
    //! InitFlags were actually used.
    let mut used = ~[];
    unsafe {
        let used_flags = ffi::IMG_Init(
            flags.iter().fold(0, |flags, &flag| {
                flags | flag as ffi::IMG_InitFlags
            })
        );
        for flag in flags.iter() {
            if used_flags & *flag as c_int != 0 {
                used.push(*flag)
            }
        }
    }
    used
}

pub fn quit() {
    //! Teardown the SDL2_Image subsystem
    unsafe { ffi::IMG_Quit(); }
}

pub fn get_linked_version() -> SDLImageVersion {
    //! Returns the version of the dynamically linked SDL_image library
    unsafe {
        SDLImageVersion::from_sdl_version(ffi::IMG_Linked_Version())
    }
}

// TODO -- this should be in rust-sdl2
// Most of the sdl2_image API relies on SDL_RWops.

// #[deriving(Eq)]
// pub struct RWops {
//     raw: *SDL_RWops;
//     owned: bool;
// }

// impl Drop for RWops {
//     fn drop(&mut self) {
//         if self.owned {
//             unsafe {
//                 // TODO -- close() returns a c_int error status.
//                 // How do we deal with errors in the destructor?
//                 // Probably either kill the task, or don't implement this
//                 // as a destructor
//                 self.raw.close()
//             }
//         }
//     }
// }
