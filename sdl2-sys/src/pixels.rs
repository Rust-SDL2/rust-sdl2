use libc::{c_int, uint8_t, uint32_t};

//SDL_pixels.h
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Color {
    pub r: uint8_t,
    pub g: uint8_t,
    pub b: uint8_t,
    pub a: uint8_t,
}

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct SDL_Palette {
    pub ncolors: c_int,
    pub colors: *const SDL_Color,
    pub version: uint32_t,
    pub refcount: c_int
}

#[allow(non_snake_case, missing_copy_implementations)]
#[repr(C)]
pub struct SDL_PixelFormat {
    pub format: SDL_PixelFormatEnum,
    pub palette: *const SDL_Palette,
    pub BitsPerPixel: uint8_t,
    pub BytesPerPixel: uint8_t,
    pub padding: [uint8_t; 2],
    pub Rmask: uint8_t,
    pub Gmask: uint8_t,
    pub Bmask: uint8_t,
    pub Amask: uint8_t,
    pub Rloss: uint8_t,
    pub Gloss: uint8_t,
    pub Bloss: uint8_t,
    pub Aloss: uint8_t,
    pub Rshift: uint8_t,
    pub Gshift: uint8_t,
    pub Bshift: uint8_t,
    pub Ashift: uint8_t,
    pub refcount: c_int,
    pub next: *const SDL_PixelFormat
}

pub type SDL_PixelFormatEnum = uint32_t;
pub const SDL_PIXELFORMAT_UNKNOWN: SDL_PixelFormatEnum = 0x0;
pub const SDL_PIXELFORMAT_INDEX1LSB: SDL_PixelFormatEnum = 0x11100100;
pub const SDL_PIXELFORMAT_INDEX1MSB: SDL_PixelFormatEnum = 0x11200100;
pub const SDL_PIXELFORMAT_INDEX4LSB: SDL_PixelFormatEnum = 0x12100400;
pub const SDL_PIXELFORMAT_INDEX4MSB: SDL_PixelFormatEnum = 0x12200400;
pub const SDL_PIXELFORMAT_INDEX8: SDL_PixelFormatEnum = 0x13000801;
pub const SDL_PIXELFORMAT_RGB332: SDL_PixelFormatEnum = 0x14110801;
pub const SDL_PIXELFORMAT_RGB444: SDL_PixelFormatEnum = 0x15120c02;
pub const SDL_PIXELFORMAT_RGB555: SDL_PixelFormatEnum = 0x15130f02;
pub const SDL_PIXELFORMAT_BGR555: SDL_PixelFormatEnum = 0x15530f02;
pub const SDL_PIXELFORMAT_ARGB4444: SDL_PixelFormatEnum = 0x15321002;
pub const SDL_PIXELFORMAT_RGBA4444: SDL_PixelFormatEnum = 0x15421002;
pub const SDL_PIXELFORMAT_ABGR4444: SDL_PixelFormatEnum = 0x15721002;
pub const SDL_PIXELFORMAT_BGRA4444: SDL_PixelFormatEnum = 0x15821002;
pub const SDL_PIXELFORMAT_ARGB1555: SDL_PixelFormatEnum = 0x15331002;
pub const SDL_PIXELFORMAT_RGBA5551: SDL_PixelFormatEnum = 0x15441002;
pub const SDL_PIXELFORMAT_ABGR1555: SDL_PixelFormatEnum = 0x15731002;
pub const SDL_PIXELFORMAT_BGRA5551: SDL_PixelFormatEnum = 0x15841002;
pub const SDL_PIXELFORMAT_RGB565: SDL_PixelFormatEnum = 0x15151002;
pub const SDL_PIXELFORMAT_BGR565: SDL_PixelFormatEnum = 0x15551002;
pub const SDL_PIXELFORMAT_RGB24: SDL_PixelFormatEnum = 0x17101803;
pub const SDL_PIXELFORMAT_BGR24: SDL_PixelFormatEnum = 0x17401803;
pub const SDL_PIXELFORMAT_RGB888: SDL_PixelFormatEnum = 0x16161804;
pub const SDL_PIXELFORMAT_RGBX8888: SDL_PixelFormatEnum = 0x16261804;
pub const SDL_PIXELFORMAT_BGR888: SDL_PixelFormatEnum = 0x16561804;
pub const SDL_PIXELFORMAT_BGRX8888: SDL_PixelFormatEnum = 0x16661804;
pub const SDL_PIXELFORMAT_ARGB8888: SDL_PixelFormatEnum = 0x16362004;
pub const SDL_PIXELFORMAT_RGBA8888: SDL_PixelFormatEnum = 0x16462004;
pub const SDL_PIXELFORMAT_ABGR8888: SDL_PixelFormatEnum = 0x16762004;
pub const SDL_PIXELFORMAT_BGRA8888: SDL_PixelFormatEnum = 0x16862004;
pub const SDL_PIXELFORMAT_ARGB2101010: SDL_PixelFormatEnum = 0x16372004;
pub const SDL_PIXELFORMAT_YV12: SDL_PixelFormatEnum = 0x32315659;
pub const SDL_PIXELFORMAT_IYUV: SDL_PixelFormatEnum = 0x56555949;
pub const SDL_PIXELFORMAT_YUY2: SDL_PixelFormatEnum = 0x32595559;
pub const SDL_PIXELFORMAT_UYVY: SDL_PixelFormatEnum = 0x59565955;
pub const SDL_PIXELFORMAT_YVYU: SDL_PixelFormatEnum = 0x55595659;

extern "C" {
    pub fn SDL_GetRGB(pixel: uint32_t, format: *const SDL_PixelFormat, r: *const uint8_t, g: *const uint8_t, b: *const uint8_t);
    pub fn SDL_GetRGBA(pixel: uint32_t, format: *const SDL_PixelFormat, r: *const uint8_t, g: *const uint8_t, b: *const uint8_t, a: *const uint8_t);
    pub fn SDL_MapRGB(format: *const SDL_PixelFormat, r: uint8_t, g: uint8_t, b: uint8_t) -> uint32_t;
    pub fn SDL_MapRGBA(format: *const SDL_PixelFormat, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> uint32_t;
}
