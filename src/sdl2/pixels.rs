extern crate rand;

use num::FromPrimitive;

use sys::pixels as ll;

use get_error;

pub struct Palette {
    raw: *mut ll::SDL_Palette
}

impl Palette {
    #[inline]
    /// Creates a new, uninitialized palette
    pub fn new(mut capacity: usize) -> Result<Self, String> {
        use common::*;

        let ncolors = {
            // This is kind of a hack. We have to cast twice because
            // ncolors is a c_int, and validate_int only takes a u32.
            // FIXME: Modify validate_int to make this unnecessary
            let u32_max = u32::max_value() as usize;
            if capacity > u32_max { capacity = u32_max; }

            match validate_int(capacity as u32, "capacity") {
                Ok(len) => len,
                Err(e) => return Err(format!("{}", e)),
            }
        };

        let raw = unsafe { ll::SDL_AllocPalette(ncolors) };

        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(Palette {
                raw: raw,
            })
        }
    }

    /// Creates a palette from the provided colors
    pub fn with_colors(colors: &[Color]) -> Result<Self, String> {
        let pal = try!(Self::new(colors.len()));

        // Already validated, so don't check again
        let ncolors = colors.len() as ::libc::c_int;

        let result = unsafe {
            let mut raw_colors: Vec<ll::SDL_Color> = colors.iter()
                .map(|color| color.raw())
                .collect();

            let pal_ptr = (&mut raw_colors[0]) as *mut ll::SDL_Color;

            ll::SDL_SetPaletteColors(pal.raw, pal_ptr, 0, ncolors)
        };

        if result < 0 {
            Err(get_error())
        } else {
            Ok(pal)
        }
    }

    pub fn len(&self) -> usize {
        unsafe { (*self.raw).ncolors as usize }
    }
}

impl Drop for Palette {
    fn drop(&mut self) {
        unsafe { ll::SDL_FreePalette(self.raw); }
    }
}

impl_raw_accessors!((Palette, *mut ll::SDL_Palette));

#[test]
fn create_palette() {
    let colors: Vec<_> = (0 .. 0xff).map(|u| {
        Color::RGB(u, 0, 0xff - u)
    }).collect();

    let palette = Palette::with_colors(&colors).unwrap();

    assert!(palette.len() == 255);
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Color {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8)
}

impl Color {
    pub fn to_u32(&self, format: &PixelFormat) -> u32 {
        match self {
            &Color::RGB(r, g, b) => {
                unsafe { ll::SDL_MapRGB(format.raw, r, g, b) }
            }
            &Color::RGBA(r, g, b, a) => {
                unsafe { ll::SDL_MapRGBA(format.raw, r, g, b, a) }
            }
        }
    }

    pub fn from_u32(format: &PixelFormat, pixel: u32) -> Color {
        let (mut r, mut g, mut b, mut a) = (0, 0, 0, 0);

        unsafe {
            ll::SDL_GetRGBA(pixel, format.raw, &mut r, &mut g, &mut b, &mut a)
        };
        Color::RGBA(r, g, b, a)
    }

    pub fn rgb(&self) -> (u8, u8, u8) {
        match self {
            &Color::RGB(r, g, b) => (r, g, b),
            &Color::RGBA(r, g, b, _) => (r, g, b)
        }
    }

    pub fn rgba(&self) -> (u8, u8, u8, u8) {
        match self {
            &Color::RGB(r, g, b) => (r, g, b, 0xff),
            &Color::RGBA(r, g, b, a) => (r, g, b, a),
        }
    }

    // Implemented manually and kept private, because reasons
    unsafe fn raw(&self) -> ll::SDL_Color {
        let (r, g, b, a) = self.rgba();
        ll::SDL_Color { r: r, g: g, b: b, a: a }
    }
}

impl rand::Rand for Color {
    fn rand<R: rand::Rng>(rng: &mut R) -> Color {
        if rng.gen() { Color::RGBA(rng.gen(), rng.gen(), rng.gen(), rng.gen()) }
        else { Color::RGB(rng.gen(), rng.gen(), rng.gen()) }
    }
}

pub struct PixelMasks {
    /// Bits per pixel; usually 15, 16, or 32
    pub bpp: u8,
    /// The red mask
    pub rmask: u32,
    /// The green mask
    pub gmask: u32,
    /// The blue mask
    pub bmask: u32,
    /// The alpha mask
    pub amask: u32
}

pub struct PixelFormat {
    raw: *mut ll::SDL_PixelFormat
}

impl_raw_accessors!((PixelFormat, *mut ll::SDL_PixelFormat));
impl_raw_constructor!((PixelFormat, PixelFormat (raw: *mut ll::SDL_PixelFormat)));

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PixelFormatEnum {
    Unknown = ll::SDL_PIXELFORMAT_UNKNOWN as i32,
    Index1LSB = ll::SDL_PIXELFORMAT_INDEX1LSB as i32,
    Index1MSB = ll::SDL_PIXELFORMAT_INDEX1MSB as i32,
    Index4LSB = ll::SDL_PIXELFORMAT_INDEX4LSB as i32,
    Index4MSB = ll::SDL_PIXELFORMAT_INDEX4MSB as i32,
    Index8 = ll::SDL_PIXELFORMAT_INDEX8 as i32,
    RGB332 = ll::SDL_PIXELFORMAT_RGB332 as i32,
    RGB444 = ll::SDL_PIXELFORMAT_RGB444 as i32,
    RGB555 = ll::SDL_PIXELFORMAT_RGB555 as i32,
    BGR555 = ll::SDL_PIXELFORMAT_BGR555 as i32,
    ARGB4444 = ll::SDL_PIXELFORMAT_ARGB4444 as i32,
    RGBA4444 = ll::SDL_PIXELFORMAT_RGBA4444 as i32,
    ABGR4444 = ll::SDL_PIXELFORMAT_ABGR4444 as i32,
    BGRA4444 = ll::SDL_PIXELFORMAT_BGRA4444 as i32,
    ARGB1555 = ll::SDL_PIXELFORMAT_ARGB1555 as i32,
    RGBA5551 = ll::SDL_PIXELFORMAT_RGBA5551 as i32,
    ABGR1555 = ll::SDL_PIXELFORMAT_ABGR1555 as i32,
    BGRA5551 = ll::SDL_PIXELFORMAT_BGRA5551 as i32,
    RGB565 = ll::SDL_PIXELFORMAT_RGB565 as i32,
    BGR565 = ll::SDL_PIXELFORMAT_BGR565 as i32,
    RGB24 = ll::SDL_PIXELFORMAT_RGB24 as i32,
    BGR24 = ll::SDL_PIXELFORMAT_BGR24 as i32,
    RGB888 = ll::SDL_PIXELFORMAT_RGB888 as i32,
    RGBX8888 = ll::SDL_PIXELFORMAT_RGBX8888 as i32,
    BGR888 = ll::SDL_PIXELFORMAT_BGR888 as i32,
    BGRX8888 = ll::SDL_PIXELFORMAT_BGRX8888 as i32,
    ARGB8888 = ll::SDL_PIXELFORMAT_ARGB8888 as i32,
    RGBA8888 = ll::SDL_PIXELFORMAT_RGBA8888 as i32,
    ABGR8888 = ll::SDL_PIXELFORMAT_ABGR8888 as i32,
    BGRA8888 = ll::SDL_PIXELFORMAT_BGRA8888 as i32,
    ARGB2101010 = ll::SDL_PIXELFORMAT_ARGB2101010 as i32,
    YV12 = ll::SDL_PIXELFORMAT_YV12 as i32,
    IYUV = ll::SDL_PIXELFORMAT_IYUV as i32,
    YUY2 = ll::SDL_PIXELFORMAT_YUY2 as i32,
    UYVY = ll::SDL_PIXELFORMAT_UYVY as i32,
    YVYU = ll::SDL_PIXELFORMAT_YVYU as i32
}

impl PixelFormatEnum {
    pub fn from_masks(masks: PixelMasks) -> PixelFormatEnum {
        unsafe {
            let format = ll::SDL_MasksToPixelFormatEnum(masks.bpp as i32, masks.rmask, masks.gmask, masks.bmask, masks.amask);
            PixelFormatEnum::from_u64(format as u64).unwrap()
        }
    }

    pub fn into_masks(self) -> Result<PixelMasks, String> {
        let format: u32 = self as u32;
        let mut bpp = 0;
        let mut rmask = 0;
        let mut gmask = 0;
        let mut bmask = 0;
        let mut amask = 0;
        let result = unsafe {
            ll::SDL_PixelFormatEnumToMasks(format, &mut bpp, &mut rmask, &mut gmask, &mut bmask, &mut amask)
        };
        if result == 0 {
            // SDL_FALSE
            Err(get_error())
        } else {
            Ok(PixelMasks {
                bpp: bpp as u8,
                rmask: rmask,
                gmask: gmask,
                bmask: bmask,
                amask: amask
            })
        }
    }

    /// Calculates the total byte size of an image buffer, given its pitch
    /// and height.
    pub fn byte_size_from_pitch_and_height(&self, pitch: usize, height: usize) -> usize {
        match *self {
            PixelFormatEnum::YV12 | PixelFormatEnum::IYUV => {
                // YUV is 4:2:0.
                // `pitch` is the width of the Y component, and
                // `height` is the height of the Y component.
                // U and V have half the width and height of Y.
                pitch * height + 2 * (pitch / 2 * height / 2)
            },
            _ => pitch * height
        }
    }

    pub fn byte_size_of_pixels(&self, num_of_pixels: usize) -> usize {
        match *self {
            PixelFormatEnum::RGB332
                => num_of_pixels * 1,
            PixelFormatEnum::RGB444 | PixelFormatEnum::RGB555 |
            PixelFormatEnum::BGR555 | PixelFormatEnum::ARGB4444 |
            PixelFormatEnum::RGBA4444 | PixelFormatEnum::ABGR4444 |
            PixelFormatEnum::BGRA4444 | PixelFormatEnum::ARGB1555 |
            PixelFormatEnum::RGBA5551 | PixelFormatEnum::ABGR1555 |
            PixelFormatEnum::BGRA5551 | PixelFormatEnum::RGB565 |
            PixelFormatEnum::BGR565
                => num_of_pixels * 2,
            PixelFormatEnum::RGB24 | PixelFormatEnum::BGR24
                => num_of_pixels * 3,
            PixelFormatEnum::RGB888 | PixelFormatEnum::RGBX8888 |
            PixelFormatEnum::BGR888 | PixelFormatEnum::BGRX8888 |
            PixelFormatEnum::ARGB8888 | PixelFormatEnum::RGBA8888 |
            PixelFormatEnum::ABGR8888 | PixelFormatEnum::BGRA8888 |
            PixelFormatEnum::ARGB2101010
                => num_of_pixels * 4,
            // YUV formats
            // FIXME: rounding error here?
            PixelFormatEnum::YV12 | PixelFormatEnum::IYUV
                => num_of_pixels / 2 * 3,
            PixelFormatEnum::YUY2 | PixelFormatEnum::UYVY |
            PixelFormatEnum::YVYU
                => num_of_pixels * 2,
            // Unsupported formats
            PixelFormatEnum::Index8
                => num_of_pixels * 1,
            PixelFormatEnum::Unknown | PixelFormatEnum::Index1LSB |
            PixelFormatEnum::Index1MSB | PixelFormatEnum::Index4LSB |
            PixelFormatEnum::Index4MSB
                => panic!("not supported format: {:?}", *self),
        }
    }

    pub fn byte_size_per_pixel(&self) -> usize {
        match *self {
            PixelFormatEnum::RGB332
                => 1,
            PixelFormatEnum::RGB444 | PixelFormatEnum::RGB555 |
            PixelFormatEnum::BGR555 | PixelFormatEnum::ARGB4444 |
            PixelFormatEnum::RGBA4444 | PixelFormatEnum::ABGR4444 |
            PixelFormatEnum::BGRA4444 | PixelFormatEnum::ARGB1555 |
            PixelFormatEnum::RGBA5551 | PixelFormatEnum::ABGR1555 |
            PixelFormatEnum::BGRA5551 | PixelFormatEnum::RGB565 |
            PixelFormatEnum::BGR565
                => 2,
            PixelFormatEnum::RGB24 | PixelFormatEnum::BGR24
                => 3,
            PixelFormatEnum::RGB888 | PixelFormatEnum::RGBX8888 |
            PixelFormatEnum::BGR888 | PixelFormatEnum::BGRX8888 |
            PixelFormatEnum::ARGB8888 | PixelFormatEnum::RGBA8888 |
            PixelFormatEnum::ABGR8888 | PixelFormatEnum::BGRA8888 |
            PixelFormatEnum::ARGB2101010
                => 4,
            // YUV formats
            PixelFormatEnum::YV12 | PixelFormatEnum::IYUV
                => 2,
            PixelFormatEnum::YUY2 | PixelFormatEnum::UYVY |
            PixelFormatEnum::YVYU
                => 2,
            // Unsupported formats
            PixelFormatEnum::Index8
                => 1,
            PixelFormatEnum::Unknown | PixelFormatEnum::Index1LSB |
            PixelFormatEnum::Index1MSB | PixelFormatEnum::Index4LSB |
            PixelFormatEnum::Index4MSB
                => panic!("not supported format: {:?}", *self),
        }
    }
}

impl From<PixelFormat> for PixelFormatEnum {
    fn from(pf: PixelFormat) -> PixelFormatEnum {
        unsafe {
            let ref sdl_pf = *pf.raw;
            match PixelFormatEnum::from_u64(sdl_pf.format as u64) {
                Some(pfe) => pfe,
                None => panic!("Unknown pixel format: {:?}", sdl_pf.format)
            }
        }
    }
}

impl FromPrimitive for PixelFormatEnum {
    fn from_i64(n: i64) -> Option<PixelFormatEnum> {
        use self::PixelFormatEnum::*;

        Some( match n as ll::SDL_PixelFormatEnum {
            ll::SDL_PIXELFORMAT_UNKNOWN     => Unknown,
            ll::SDL_PIXELFORMAT_INDEX1LSB   => Index1LSB,
            ll::SDL_PIXELFORMAT_INDEX1MSB   => Index1MSB,
            ll::SDL_PIXELFORMAT_INDEX4LSB   => Index4LSB,
            ll::SDL_PIXELFORMAT_INDEX4MSB   => Index4MSB,
            ll::SDL_PIXELFORMAT_INDEX8      => Index8,
            ll::SDL_PIXELFORMAT_RGB332      => RGB332,
            ll::SDL_PIXELFORMAT_RGB444      => RGB444,
            ll::SDL_PIXELFORMAT_RGB555      => RGB555,
            ll::SDL_PIXELFORMAT_BGR555      => BGR555,
            ll::SDL_PIXELFORMAT_ARGB4444    => ARGB4444,
            ll::SDL_PIXELFORMAT_RGBA4444    => RGBA4444,
            ll::SDL_PIXELFORMAT_ABGR4444    => ABGR4444,
            ll::SDL_PIXELFORMAT_BGRA4444    => BGRA4444,
            ll::SDL_PIXELFORMAT_ARGB1555    => ARGB1555,
            ll::SDL_PIXELFORMAT_RGBA5551    => RGBA5551,
            ll::SDL_PIXELFORMAT_ABGR1555    => ABGR1555,
            ll::SDL_PIXELFORMAT_BGRA5551    => BGRA5551,
            ll::SDL_PIXELFORMAT_RGB565      => RGB565,
            ll::SDL_PIXELFORMAT_BGR565      => BGR565,
            ll::SDL_PIXELFORMAT_RGB24       => RGB24,
            ll::SDL_PIXELFORMAT_BGR24       => BGR24,
            ll::SDL_PIXELFORMAT_RGB888      => RGB888,
            ll::SDL_PIXELFORMAT_RGBX8888    => RGBX8888,
            ll::SDL_PIXELFORMAT_BGR888      => BGR888,
            ll::SDL_PIXELFORMAT_BGRX8888    => BGRX8888,
            ll::SDL_PIXELFORMAT_ARGB8888    => ARGB8888,
            ll::SDL_PIXELFORMAT_RGBA8888    => RGBA8888,
            ll::SDL_PIXELFORMAT_ABGR8888    => ABGR8888,
            ll::SDL_PIXELFORMAT_BGRA8888    => BGRA8888,
            ll::SDL_PIXELFORMAT_ARGB2101010 => ARGB2101010,
            ll::SDL_PIXELFORMAT_YV12        => YV12,
            ll::SDL_PIXELFORMAT_IYUV        => IYUV,
            ll::SDL_PIXELFORMAT_YUY2        => YUY2,
            ll::SDL_PIXELFORMAT_UYVY        => UYVY,
            ll::SDL_PIXELFORMAT_YVYU        => YVYU,
            _                               => return None,
        })
    }

    fn from_u64(n: u64) -> Option<PixelFormatEnum> { FromPrimitive::from_i64(n as i64) }
}


// Just test a round-trip conversion from PixelFormat to
// PixelFormatEnum and back.
#[test]
fn test_pixel_format_enum() {
    let pixel_formats = vec![
        PixelFormatEnum::RGB332,
        PixelFormatEnum::RGB444, PixelFormatEnum::RGB555,
        PixelFormatEnum::BGR555, PixelFormatEnum::ARGB4444,
        PixelFormatEnum::RGBA4444, PixelFormatEnum::ABGR4444,
        PixelFormatEnum::BGRA4444, PixelFormatEnum::ARGB1555,
        PixelFormatEnum::RGBA5551, PixelFormatEnum::ABGR1555,
        PixelFormatEnum::BGRA5551, PixelFormatEnum::RGB565,
        PixelFormatEnum::BGR565,
        PixelFormatEnum::RGB24, PixelFormatEnum::BGR24,
        PixelFormatEnum::RGB888, PixelFormatEnum::RGBX8888,
        PixelFormatEnum::BGR888, PixelFormatEnum::BGRX8888,
        PixelFormatEnum::ARGB8888, PixelFormatEnum::RGBA8888,
        PixelFormatEnum::ABGR8888, PixelFormatEnum::BGRA8888,
        PixelFormatEnum::ARGB2101010,
        PixelFormatEnum::YV12, PixelFormatEnum::IYUV,
        PixelFormatEnum::YUY2, PixelFormatEnum::UYVY,
        PixelFormatEnum::YVYU,
        PixelFormatEnum::Index8,
        // These don't seem to be supported;
        // the round-trip 
        //PixelFormatEnum::Unknown, PixelFormatEnum::Index1LSB,
        //PixelFormatEnum::Index1MSB, PixelFormatEnum::Index4LSB,
        //PixelFormatEnum::Index4MSB
    ];

    
    let _sdl_context = ::sdl::init().unwrap();
    for format in pixel_formats {
        // If we don't support making a surface of a specific format,
        // that's fine, just keep going the best we can.
        if let Ok(surf) = super::surface::Surface::new(1, 1, format) {
            let surf_format = surf.pixel_format();
            assert_eq!(PixelFormatEnum::from(surf_format), format);
        }
    }
}
