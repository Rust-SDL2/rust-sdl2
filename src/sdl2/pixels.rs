extern crate rand;

#[allow(non_camel_case_types)]
pub mod ll {
    use libc::{c_int, uint8_t, uint32_t};

    //SDL_pixels.h
    pub struct SDL_Color {
        pub r: uint8_t,
        pub g: uint8_t,
        pub b: uint8_t,
        pub a: uint8_t,
    }

    pub struct SDL_Palette {
        pub ncolors: c_int,
        pub colors: *const SDL_Color,
        pub version: uint32_t,
        pub refcount: c_int
    }

    #[allow(uppercase_variables)]
    pub struct SDL_PixelFormat {
        pub format: SDL_PixelFormatFlag,
        pub palette: *const SDL_Palette,
        pub BitsPerPixel: uint8_t,
        pub BytesPerPixel: uint8_t,
        pub padding: [uint8_t, ..2],
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

    pub type SDL_PixelFormatFlag = uint32_t;
    pub static SDL_PIXELFORMAT_UNKNOWN: SDL_PixelFormatFlag = 0x0;
    pub static SDL_PIXELFORMAT_INDEX1LSB: SDL_PixelFormatFlag = 0x11100100;
    pub static SDL_PIXELFORMAT_INDEX1MSB: SDL_PixelFormatFlag = 0x11200100;
    pub static SDL_PIXELFORMAT_INDEX4LSB: SDL_PixelFormatFlag = 0x12100400;
    pub static SDL_PIXELFORMAT_INDEX4MSB: SDL_PixelFormatFlag = 0x12200400;
    pub static SDL_PIXELFORMAT_INDEX8: SDL_PixelFormatFlag = 0x13000801;
    pub static SDL_PIXELFORMAT_RGB332: SDL_PixelFormatFlag = 0x14110801;
    pub static SDL_PIXELFORMAT_RGB444: SDL_PixelFormatFlag = 0x15120c02;
    pub static SDL_PIXELFORMAT_RGB555: SDL_PixelFormatFlag = 0x15130f02;
    pub static SDL_PIXELFORMAT_BGR555: SDL_PixelFormatFlag = 0x15530f02;
    pub static SDL_PIXELFORMAT_ARGB4444: SDL_PixelFormatFlag = 0x15321002;
    pub static SDL_PIXELFORMAT_RGBA4444: SDL_PixelFormatFlag = 0x15421002;
    pub static SDL_PIXELFORMAT_ABGR4444: SDL_PixelFormatFlag = 0x15721002;
    pub static SDL_PIXELFORMAT_BGRA4444: SDL_PixelFormatFlag = 0x15821002;
    pub static SDL_PIXELFORMAT_ARGB1555: SDL_PixelFormatFlag = 0x15331002;
    pub static SDL_PIXELFORMAT_RGBA5551: SDL_PixelFormatFlag = 0x15441002;
    pub static SDL_PIXELFORMAT_ABGR1555: SDL_PixelFormatFlag = 0x15731002;
    pub static SDL_PIXELFORMAT_BGRA5551: SDL_PixelFormatFlag = 0x15841002;
    pub static SDL_PIXELFORMAT_RGB565: SDL_PixelFormatFlag = 0x15151002;
    pub static SDL_PIXELFORMAT_BGR565: SDL_PixelFormatFlag = 0x15551002;
    pub static SDL_PIXELFORMAT_RGB24: SDL_PixelFormatFlag = 0x17101803;
    pub static SDL_PIXELFORMAT_BGR24: SDL_PixelFormatFlag = 0x17401803;
    pub static SDL_PIXELFORMAT_RGB888: SDL_PixelFormatFlag = 0x16161804;
    pub static SDL_PIXELFORMAT_RGBX8888: SDL_PixelFormatFlag = 0x16261804;
    pub static SDL_PIXELFORMAT_BGR888: SDL_PixelFormatFlag = 0x16561804;
    pub static SDL_PIXELFORMAT_BGRX8888: SDL_PixelFormatFlag = 0x16661804;
    pub static SDL_PIXELFORMAT_ARGB8888: SDL_PixelFormatFlag = 0x16362004;
    pub static SDL_PIXELFORMAT_RGBA8888: SDL_PixelFormatFlag = 0x16462004;
    pub static SDL_PIXELFORMAT_ABGR8888: SDL_PixelFormatFlag = 0x16762004;
    pub static SDL_PIXELFORMAT_BGRA8888: SDL_PixelFormatFlag = 0x16862004;
    pub static SDL_PIXELFORMAT_ARGB2101010: SDL_PixelFormatFlag = 0x16372004;
    pub static SDL_PIXELFORMAT_YV12: SDL_PixelFormatFlag = 0x32315659;
    pub static SDL_PIXELFORMAT_IYUV: SDL_PixelFormatFlag = 0x56555949;
    pub static SDL_PIXELFORMAT_YUY2: SDL_PixelFormatFlag = 0x32595559;
    pub static SDL_PIXELFORMAT_UYVY: SDL_PixelFormatFlag = 0x59565955;
    pub static SDL_PIXELFORMAT_YVYU: SDL_PixelFormatFlag = 0x55595659;

    extern "C" {
        pub fn SDL_GetRGB(pixel: uint32_t, format: *const SDL_PixelFormat, r: *const uint8_t, g: *const uint8_t, b: *const uint8_t);
        pub fn SDL_GetRGBA(pixel: uint32_t, format: *const SDL_PixelFormat, r: *const uint8_t, g: *const uint8_t, b: *const uint8_t, a: *const uint8_t);
        pub fn SDL_MapRGB(format: *const SDL_PixelFormat, r: uint8_t, g: uint8_t, b: uint8_t) -> uint32_t;
        pub fn SDL_MapRGBA(format: *const SDL_PixelFormat, r: uint8_t, g: uint8_t, b: uint8_t, a: uint8_t) -> uint32_t;
    }
}
#[deriving(PartialEq)] #[allow(raw_pointer_deriving)]
pub struct Palette {
    raw: *const ll::SDL_Palette
}

impl_raw_accessors!(Palette, *const ll::SDL_Palette)

#[deriving(PartialEq)]
pub enum Color {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8)
}

impl Color {
    pub fn to_u32(&self, format: &PixelFormat) -> u32 {
        match self {
            &RGB(r, g, b) => {
                unsafe { ll::SDL_MapRGB(format.raw, r, g, b) }
            }
            &RGBA(r, g, b, a) => {
                unsafe { ll::SDL_MapRGBA(format.raw, r, g, b, a) }
            }
        }
    }

    pub fn from_u32(format: &PixelFormat, pixel: u32) -> Color {
        let r: u8 = 0;
        let g: u8 = 0;
        let b: u8 = 0;
        let a: u8 = 0;

        unsafe {
            ll::SDL_GetRGBA(pixel, format.raw, &r, &g, &b, &a)
        };
        RGBA(r, g, b, a)
    }
}

impl rand::Rand for Color {
    fn rand<R: rand::Rng>(rng: &mut R) -> Color {
        if rng.gen() { RGBA(rng.gen(), rng.gen(), rng.gen(), rng.gen()) }
        else { RGB(rng.gen(), rng.gen(), rng.gen()) }
    }
}

#[deriving(PartialEq)] #[allow(raw_pointer_deriving)]
pub struct PixelFormat {
    raw: *const ll::SDL_PixelFormat
}

impl_raw_accessors!(PixelFormat, *const ll::SDL_PixelFormat)
impl_raw_constructor!(PixelFormat -> PixelFormat (raw: *const ll::SDL_PixelFormat))

#[deriving(PartialEq, Show, FromPrimitive)]
pub enum PixelFormatFlag {
    Unknown = ll::SDL_PIXELFORMAT_UNKNOWN as int,
    Index1LSB = ll::SDL_PIXELFORMAT_INDEX1LSB as int,
    Index1MSB = ll::SDL_PIXELFORMAT_INDEX1MSB as int,
    Index4LSB = ll::SDL_PIXELFORMAT_INDEX4LSB as int,
    Index4MSB = ll::SDL_PIXELFORMAT_INDEX4MSB as int,
    Index8 = ll::SDL_PIXELFORMAT_INDEX8 as int,
    RGB332 = ll::SDL_PIXELFORMAT_RGB332 as int,
    RGB444 = ll::SDL_PIXELFORMAT_RGB444 as int,
    RGB555 = ll::SDL_PIXELFORMAT_RGB555 as int,
    BGR555 = ll::SDL_PIXELFORMAT_BGR555 as int,
    ARGB4444 = ll::SDL_PIXELFORMAT_ARGB4444 as int,
    RGBA4444 = ll::SDL_PIXELFORMAT_RGBA4444 as int,
    ABGR4444 = ll::SDL_PIXELFORMAT_ABGR4444 as int,
    BGRA4444 = ll::SDL_PIXELFORMAT_BGRA4444 as int,
    ARGB1555 = ll::SDL_PIXELFORMAT_ARGB1555 as int,
    RGBA5551 = ll::SDL_PIXELFORMAT_RGBA5551 as int,
    ABGR1555 = ll::SDL_PIXELFORMAT_ABGR1555 as int,
    BGRA5551 = ll::SDL_PIXELFORMAT_BGRA5551 as int,
    RGB565 = ll::SDL_PIXELFORMAT_RGB565 as int,
    BGR565 = ll::SDL_PIXELFORMAT_BGR565 as int,
    RGB24 = ll::SDL_PIXELFORMAT_RGB24 as int,
    BGR24 = ll::SDL_PIXELFORMAT_BGR24 as int,
    RGB888 = ll::SDL_PIXELFORMAT_RGB888 as int,
    RGBX8888 = ll::SDL_PIXELFORMAT_RGBX8888 as int,
    BGR888 = ll::SDL_PIXELFORMAT_BGR888 as int,
    BGRX8888 = ll::SDL_PIXELFORMAT_BGRX8888 as int,
    ARGB8888 = ll::SDL_PIXELFORMAT_ARGB8888 as int,
    RGBA8888 = ll::SDL_PIXELFORMAT_RGBA8888 as int,
    ABGR8888 = ll::SDL_PIXELFORMAT_ABGR8888 as int,
    BGRA8888 = ll::SDL_PIXELFORMAT_BGRA8888 as int,
    ARGB2101010 = ll::SDL_PIXELFORMAT_ARGB2101010 as int,
    YV12 = ll::SDL_PIXELFORMAT_YV12 as int,
    IYUV = ll::SDL_PIXELFORMAT_IYUV as int,
    YUY2 = ll::SDL_PIXELFORMAT_YUY2 as int,
    UYVY = ll::SDL_PIXELFORMAT_UYVY as int,
    YVYU = ll::SDL_PIXELFORMAT_YVYU as int
}

impl PixelFormatFlag {
    pub fn byte_size_of_pixels(&self, num_of_pixels: uint) -> uint {
        match *self {
            RGB332
                => num_of_pixels * 1,
            RGB444 | RGB555 | BGR555 | ARGB4444 | RGBA4444 | ABGR4444 |
            BGRA4444 | ARGB1555 | RGBA5551 | ABGR1555 | BGRA5551 | RGB565 |
            BGR565
                => num_of_pixels * 2,
            RGB24 | BGR24
                => num_of_pixels * 3,
            RGB888 | RGBX8888 | BGR888 | BGRX8888 | ARGB8888 | RGBA8888 |
            ABGR8888 | BGRA8888 | ARGB2101010
                => num_of_pixels * 4,
            // YUV formats
            // FIXME: rounding error here?
            YV12 | IYUV
                => num_of_pixels / 2 * 3,
            YUY2 | UYVY | YVYU
                => num_of_pixels * 2,
            // Unsupported formats
            Index8
                => num_of_pixels * 1,
            Unknown | Index1LSB | Index1MSB | Index4LSB | Index4MSB
                => fail!("not supported format: {}", *self),
        }
    }

    pub fn byte_size_per_pixel(&self) -> uint {
        match *self {
            RGB332
                => 1,
            RGB444 | RGB555 | BGR555 | ARGB4444 | RGBA4444 | ABGR4444 |
            BGRA4444 | ARGB1555 | RGBA5551 | ABGR1555 | BGRA5551 | RGB565 |
            BGR565
                => 2,
            RGB24 | BGR24
                => 3,
            RGB888 | RGBX8888 | BGR888 | BGRX8888 | ARGB8888 | RGBA8888 |
            ABGR8888 | BGRA8888 | ARGB2101010
                => 4,
            // YUV formats
            YV12 | IYUV
                => 2,
            YUY2 | UYVY | YVYU
                => 2,
            // Unsupported formats
            Index8
                => 1,
            Unknown | Index1LSB | Index1MSB | Index4LSB | Index4MSB
                => fail!("not supported format: {}", *self),
        }
    }
}
