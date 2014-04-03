extern crate sdl2;

use std::libc::{c_int, c_char, uint8_t};
use sdl2::surface::ll::SDL_Surface;
use sdl2::rwops::ll::SDL_RWops;
use sdl2::render::ll::SDL_Texture;
use sdl2::render::ll::SDL_Renderer;

pub type IMG_InitFlags = c_int;
pub static IMG_INIT_JPG: IMG_InitFlags = 0x00000001;
pub static IMG_INIT_PNG: IMG_InitFlags = 0x00000002;
pub static IMG_INIT_TIF: IMG_InitFlags = 0x00000004;
pub static IMG_INIT_WEBP: IMG_InitFlags = 0x00000008;

pub struct SDL_version {
    pub major: uint8_t,
    pub minor: uint8_t,
    pub patch: uint8_t,
}

extern "C" {

// This function gets the version of the dynamically linked SDL_image library.
pub fn IMG_Linked_Version() -> *SDL_version;

// Loads dynamic libraries and prepares them for use.  Flags should be
// one or more flags from IMG_InitFlags OR'd together.
// It returns the flags successfully initialized, or 0 on failure.
pub fn IMG_Init(flags: c_int) -> c_int;

// Unloads libraries loaded with IMG_Init
pub fn IMG_Quit();

// Load an image from an SDL data source.
// The 'type' may be one of: "BMP", "GIF", "PNG", etc.
// If the image format supports a transparent pixel, SDL will set the
// colorkey for the surface.  You can enable RLE acceleration on the
// surface afterwards by calling:
//  SDL_SetColorKey(image, SDL_RLEACCEL, image->format->colorkey);
pub fn IMG_LoadTyped_RW(src: *SDL_RWops, freesrc: c_int,
                        fmt: *c_char) -> *SDL_Surface;

// Convenience functions
pub fn IMG_Load(file: *c_char) -> *SDL_Surface;
pub fn IMG_Load_RW(src: *SDL_RWops, freesrc: c_int) -> *SDL_Surface;

// Load an image directly into a render texture.
// Requires SDL2
pub fn IMG_LoadTexture(renderer: *SDL_Renderer,
                       file: *c_char) -> *SDL_Texture;
pub fn IMG_LoadTexture_RW(renderer: *SDL_Renderer, src: *SDL_RWops,
                          freesrc: c_int) -> *SDL_Texture;
pub fn IMG_LoadTextureTyped_RW(renderer: *SDL_Renderer, src: *SDL_RWops,
                               freesrc: c_int, fmt: *c_char) -> *SDL_Texture;

// Functions to detect a file type, given a seekable source
pub fn IMG_isICO(src: *SDL_RWops) -> c_int;
pub fn IMG_isCUR(src: *SDL_RWops) -> c_int;
pub fn IMG_isBMP(src: *SDL_RWops) -> c_int;
pub fn IMG_isGIF(src: *SDL_RWops) -> c_int;
pub fn IMG_isJPG(src: *SDL_RWops) -> c_int;
pub fn IMG_isLBM(src: *SDL_RWops) -> c_int;
pub fn IMG_isPCX(src: *SDL_RWops) -> c_int;
pub fn IMG_isPNG(src: *SDL_RWops) -> c_int;
pub fn IMG_isPNM(src: *SDL_RWops) -> c_int;
pub fn IMG_isTIF(src: *SDL_RWops) -> c_int;
pub fn IMG_isXCF(src: *SDL_RWops) -> c_int;
pub fn IMG_isXPM(src: *SDL_RWops) -> c_int;
pub fn IMG_isXV(src: *SDL_RWops) -> c_int;
pub fn IMG_isWEBP(src: *SDL_RWops) -> c_int;

// Individual loading functions
pub fn IMG_LoadICO_RW(src: *SDL_RWops) -> *SDL_Surface;
pub fn IMG_LoadCUR_RW(src: *SDL_RWops) -> *SDL_Surface;
pub fn IMG_LoadBMP_RW(src: *SDL_RWops) -> *SDL_Surface;
pub fn IMG_LoadGIF_RW(src: *SDL_RWops) -> *SDL_Surface;
pub fn IMG_LoadJPG_RW(src: *SDL_RWops) -> *SDL_Surface;
pub fn IMG_LoadLBM_RW(src: *SDL_RWops) -> *SDL_Surface;
pub fn IMG_LoadPCX_RW(src: *SDL_RWops) -> *SDL_Surface;
pub fn IMG_LoadPNG_RW(src: *SDL_RWops) -> *SDL_Surface;
pub fn IMG_LoadPNM_RW(src: *SDL_RWops) -> *SDL_Surface;
pub fn IMG_LoadTGA_RW(src: *SDL_RWops) -> *SDL_Surface;
pub fn IMG_LoadTIF_RW(src: *SDL_RWops) -> *SDL_Surface;
pub fn IMG_LoadXCF_RW(src: *SDL_RWops) -> *SDL_Surface;
pub fn IMG_LoadXPM_RW(src: *SDL_RWops) -> *SDL_Surface;
pub fn IMG_LoadXV_RW(src: *SDL_RWops) -> *SDL_Surface;
pub fn IMG_LoadWEBP_RW(src: *SDL_RWops) -> *SDL_Surface;
pub fn IMG_ReadXPMFromArray(xpm: **c_char) -> *SDL_Surface;

// Individual saving functions
pub fn IMG_SavePNG(surface: *SDL_Surface, file: *c_char) -> c_int;
pub fn IMG_SavePNG_RW(surface: *SDL_Surface, dst: *SDL_RWops,
                      freedst: c_int) -> c_int;

}   // extern "C"
