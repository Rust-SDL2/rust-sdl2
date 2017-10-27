use std::os::raw::{c_int, c_char};
use sys;
use sys::{SDL_RWops, SDL_Surface, SDL_Renderer, SDL_Texture};

pub type IMG_InitFlags = c_int;
pub const IMG_INIT_JPG: IMG_InitFlags = 0x00_00_00_01;
pub const IMG_INIT_PNG: IMG_InitFlags = 0x00_00_00_02;
pub const IMG_INIT_TIF: IMG_InitFlags = 0x00_00_00_04;
pub const IMG_INIT_WEBP: IMG_InitFlags = 0x00_00_00_08;

extern "C" {

// This function gets the version of the dynamically linked SDL_image library.
pub fn IMG_Linked_Version() -> *const sys::SDL_version;

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
pub fn IMG_LoadTyped_RW(src: *const SDL_RWops, freesrc: c_int,
                        fmt: *const c_char) -> *mut SDL_Surface;

// Convenience functions
pub fn IMG_Load(file: *const c_char) -> *mut SDL_Surface;
pub fn IMG_Load_RW(src: *const SDL_RWops, freesrc: c_int) -> *mut SDL_Surface;

// Load an image directly into a render texture.
// Requires SDL2
pub fn IMG_LoadTexture(renderer: *const SDL_Renderer,
                       file: *const c_char) -> *mut SDL_Texture;
pub fn IMG_LoadTexture_RW(renderer: *const SDL_Renderer, src: *const SDL_RWops,
                          freesrc: c_int) -> *const SDL_Texture;
pub fn IMG_LoadTextureTyped_RW(renderer: *const SDL_Renderer, src: *const SDL_RWops,
                               freesrc: c_int, fmt: *const c_char) -> *const SDL_Texture;

// Functions to detect a file type, given a seekable source
pub fn IMG_isICO(src: *const SDL_RWops) -> c_int;
pub fn IMG_isCUR(src: *const SDL_RWops) -> c_int;
pub fn IMG_isBMP(src: *const SDL_RWops) -> c_int;
pub fn IMG_isGIF(src: *const SDL_RWops) -> c_int;
pub fn IMG_isJPG(src: *const SDL_RWops) -> c_int;
pub fn IMG_isLBM(src: *const SDL_RWops) -> c_int;
pub fn IMG_isPCX(src: *const SDL_RWops) -> c_int;
pub fn IMG_isPNG(src: *const SDL_RWops) -> c_int;
pub fn IMG_isPNM(src: *const SDL_RWops) -> c_int;
pub fn IMG_isTIF(src: *const SDL_RWops) -> c_int;
pub fn IMG_isXCF(src: *const SDL_RWops) -> c_int;
pub fn IMG_isXPM(src: *const SDL_RWops) -> c_int;
pub fn IMG_isXV(src: *const SDL_RWops) -> c_int;
pub fn IMG_isWEBP(src: *const SDL_RWops) -> c_int;

// Individual loading functions
pub fn IMG_LoadICO_RW(src: *const SDL_RWops) -> *mut SDL_Surface;
pub fn IMG_LoadCUR_RW(src: *const SDL_RWops) -> *mut SDL_Surface;
pub fn IMG_LoadBMP_RW(src: *const SDL_RWops) -> *mut SDL_Surface;
pub fn IMG_LoadGIF_RW(src: *const SDL_RWops) -> *mut SDL_Surface;
pub fn IMG_LoadJPG_RW(src: *const SDL_RWops) -> *mut SDL_Surface;
pub fn IMG_LoadLBM_RW(src: *const SDL_RWops) -> *mut SDL_Surface;
pub fn IMG_LoadPCX_RW(src: *const SDL_RWops) -> *mut SDL_Surface;
pub fn IMG_LoadPNG_RW(src: *const SDL_RWops) -> *mut SDL_Surface;
pub fn IMG_LoadPNM_RW(src: *const SDL_RWops) -> *mut SDL_Surface;
pub fn IMG_LoadTGA_RW(src: *const SDL_RWops) -> *mut SDL_Surface;
pub fn IMG_LoadTIF_RW(src: *const SDL_RWops) -> *mut SDL_Surface;
pub fn IMG_LoadXCF_RW(src: *const SDL_RWops) -> *mut SDL_Surface;
pub fn IMG_LoadXPM_RW(src: *const SDL_RWops) -> *mut SDL_Surface;
pub fn IMG_LoadXV_RW(src: *const SDL_RWops) -> *mut SDL_Surface;
pub fn IMG_LoadWEBP_RW(src: *const SDL_RWops) -> *mut SDL_Surface;
pub fn IMG_ReadXPMFromArray(xpm: *const *const c_char) -> *mut SDL_Surface;

// Individual saving functions
pub fn IMG_SavePNG(surface: *mut SDL_Surface, file: *const c_char) -> c_int;
pub fn IMG_SavePNG_RW(surface: *mut SDL_Surface, dst: *const SDL_RWops,
                      freedst: c_int) -> c_int;

}   // extern "C"
