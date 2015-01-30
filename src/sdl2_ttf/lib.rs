/*!
A binding for SDL2_ttf.
 */

#![crate_type = "lib"]
#![desc = "SDL2_ttf bindings and wrappers"]
#![comment = "SDL2_ttf bindings and wrappers"]
#![license = "MIT"]

extern crate libc;
extern crate sdl2;
extern crate "sdl2-sys" as sdl2_sys;

#[macro_use]
extern crate bitflags;

use libc::{c_int, c_long};
use std::ffi::{c_str_to_bytes, CString};
use std::num::FromPrimitive;
use sdl2::surface::Surface;
use sdl2::get_error;
use sdl2::pixels;
use sdl2::pixels::Color;
use sdl2_sys::pixels::SDL_Color;
use sdl2::rwops::RWops;
use sdl2::version::Version;
use sdl2::SdlResult;

// Setup linking for all targets.
#[cfg(target_os="macos")]
mod mac {
    #[cfg(mac_framework)]
    #[link(kind="framework", name="SDL2_ttf")]
    extern {}

    #[cfg(not(mac_framework))]
    #[link(name="SDL2_ttf")]
    extern {}
}

#[cfg(any(target_os="windows", target_os="linux", target_os="freebsd"))]
mod others {
    #[link(name="SDL2_ttf")]
    extern {}
}

#[allow(non_camel_case_types, dead_code)]
mod ffi;

#[inline]
fn color_to_c_color(color: Color) -> SDL_Color {
    match color {
        pixels::Color::RGB(r, g, b)     => SDL_Color { r: r, g: g, b: b, a: 255 },
        pixels::Color::RGBA(r, g, b, a) => SDL_Color { r: r, g: g, b: b, a: a   }
    }
}

/// Font Style
bitflags! {
    flags FontStyle : c_int {
    const StyleNormal = ffi::TTF_STYLE_NORMAL,
    const StyleBold   = ffi::TTF_STYLE_BOLD,
    const StyleItalic = ffi::TTF_STYLE_ITALIC,
    const StyleUnderline = ffi::TTF_STYLE_UNDERLINE,
    const StyleStrikeThrough = ffi::TTF_STYLE_STRIKETHROUGH,
    }
}

#[derive(Show, PartialEq, FromPrimitive)]
pub enum Hinting {
    HintingNormal = ffi::TTF_HINTING_NORMAL as isize,
    HintingLight  = ffi::TTF_HINTING_LIGHT  as isize,
    HintingMono   = ffi::TTF_HINTING_MONO   as isize,
    HintingNone   = ffi::TTF_HINTING_NONE   as isize
}

/// Glyph Metrics
#[derive(PartialEq, Clone, Show)]
pub struct GlyphMetrics {
    pub minx: isize,
    pub maxx: isize,
    pub miny: isize,
    pub maxy: isize,
    pub advance: isize
}

/// Returns the version of the dynamically linked SDL_ttf library
pub fn get_linked_version() -> Version {
    unsafe {
        Version::from_ll(ffi::TTF_Linked_Version())
    }
}

pub fn init() -> bool {
    //! Initialize the truetype font API.
    unsafe {
        if ffi::TTF_WasInit() == 1 {
            true
        } else {
            ffi::TTF_Init() == 0
        }
    }
}

pub fn was_inited() -> bool {
    //! Query the initilization status of the truetype font API.
    unsafe {
        ffi::TTF_WasInit() == 1
    }
}

pub fn quit() {
    //! Shutdown and cleanup the truetype font API.
    unsafe { ffi::TTF_Quit(); }
}

/// The opaque holder of a loaded font.
#[allow(raw_pointer_deriving)]
#[derive(PartialEq)]
pub struct Font {
    raw: *const ffi::TTF_Font,
    owned: bool
}

impl Drop for Font {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                // avoid close font after quit()
                if ffi::TTF_WasInit() == 1 {
                    ffi::TTF_CloseFont(self.raw);
                }
            }
        }
    }
}

impl Font {
    fn from_ll(raw: *const ffi::TTF_Font, owned: bool) -> Font {
        Font { raw: raw, owned: owned }
    }

    pub fn from_file(filename: &Path, ptsize: isize) -> SdlResult<Font> {
        //! Load file for use as a font, at ptsize size.
        unsafe {
            let cstring = CString::from_slice(filename.as_str().unwrap().as_bytes());
            let raw = ffi::TTF_OpenFont(cstring.as_ptr(), ptsize as c_int);
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(Font { raw: raw, owned: true })
            }
        }
    }

    pub fn from_file_index(filename: &Path, ptsize: isize, index: isize) -> SdlResult<Font> {
        //! Load file, face index, for use as a font, at ptsize size.
        unsafe {
            let cstring = CString::from_slice(filename.as_str().unwrap().as_bytes());
            let raw = ffi::TTF_OpenFontIndex(cstring.as_ptr(), ptsize as c_int, index as c_long);
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(Font { raw: raw, owned: true })
            }
        }
    }

    pub fn get_style(&self) -> FontStyle {
        //! Get font render style
        unsafe {
            let raw = ffi::TTF_GetFontStyle(self.raw);
            FontStyle::from_bits_truncate(raw)
        }
    }

    pub fn set_style(&mut self, styles: FontStyle) {
        //! Set font render style.
        unsafe {
            ffi::TTF_SetFontStyle(self.raw, styles.bits())
        }
    }

    pub fn get_outline(&self) -> isize {
        //! Get font outline width.
        unsafe {
            ffi::TTF_GetFontOutline(self.raw) as isize
        }
    }

    pub fn set_outline(&mut self, outline: isize) {
        //! Set font outline width.
        unsafe {
            ffi::TTF_SetFontOutline(self.raw, outline as c_int)
        }
    }

    pub fn get_hinting(&self) -> Hinting {
        //! Get freetype hinter setting.
        unsafe {
            FromPrimitive::from_i32(ffi::TTF_GetFontHinting(self.raw)).unwrap()
        }
    }

    pub fn set_hinting(&mut self, hinting: Hinting) {
        //! Set freetype hinter setting.
        unsafe {
            ffi::TTF_SetFontHinting(self.raw, hinting as c_int)
        }
    }

    pub fn get_kerning(&self) -> bool {
        //! Get freetype kerning setting.
        unsafe {
            ffi::TTF_GetFontKerning(self.raw) != 0
        }
    }

    pub fn set_kerning(&mut self, kerning: bool) {
        //! Set freetype kerning setting.
        unsafe {
            ffi::TTF_SetFontKerning(self.raw, kerning as c_int)
        }
    }

    pub fn height(&self) -> isize {
        //! Get font maximum total height.
        unsafe {
            ffi::TTF_FontHeight(self.raw) as isize
        }
    }

    pub fn ascent(&self) -> isize {
        //! Get font highest ascent (height above base).
        unsafe {
            ffi::TTF_FontAscent(self.raw) as isize
        }
    }

    pub fn descent(&self) -> isize {
        //! Get font lowest descent (height below base).
        unsafe {
            ffi::TTF_FontDescent(self.raw) as isize
        }
    }

    pub fn line_skip(&self) -> isize {
        //! Get font recommended line spacing.
        unsafe {
            ffi::TTF_FontLineSkip(self.raw) as isize
        }
    }

    pub fn faces(&self) -> isize {
        //! Get the number of faces in a font.
        unsafe {
            ffi::TTF_FontFaces(self.raw) as isize
        }
    }

    pub fn face_is_fixed_width(&self) -> bool {
        //! Get whether font is monospaced or not.
        unsafe {
            ffi::TTF_FontFaceIsFixedWidth(self.raw) != 0
        }
    }

    pub fn face_family_name(&self) -> Option<String> {
        //! Get current font face family name string.
        unsafe {
            // not owns buffer
            let cname = ffi::TTF_FontFaceFamilyName(self.raw);
            if cname.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(c_str_to_bytes(&cname)).to_string())
            }
        }
    }

    pub fn face_style_name(&self) -> Option<String> {
        //! Get current font face style name string.
        unsafe {
            let cname = ffi::TTF_FontFaceStyleName(self.raw);
            if cname.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(c_str_to_bytes(&cname)).to_string())
            }
        }
    }

    pub fn index_of_char(&self, ch: char) -> Option<usize> {
        //! Get individual font glyph availability.
        unsafe {
            let ret = ffi::TTF_GlyphIsProvided(self.raw, ch as u16);
            if ret == 0 {
                None
            } else {
                Some(ret as usize)
            }
        }
    }

    pub fn metrics_of_char(&self, ch: char) -> Option<GlyphMetrics> {
        //! Get individual font glyph metrics.
        let minx = 0;
        let maxx = 0;
        let miny = 0;
        let maxy = 0;
        let advance = 0;
        let ret = unsafe {
            ffi::TTF_GlyphMetrics(self.raw, ch as u16,
                                  &minx, &maxx, &miny, &maxy, &advance)
        };
        if ret != 0 {
            None
        } else {
            Some(GlyphMetrics { minx: minx as isize, maxx: maxx as isize,
                                miny: miny as isize, maxy: maxy as isize,
                                advance: advance as isize })
        }
    }

    pub fn size_of_bytes(&self, text: &[u8]) -> SdlResult<(isize, isize)> {
        //! Get size of LATIN1 text string as would be rendered.
        let w = 0;
        let h = 0;
        let ret = unsafe {
            let ctext = CString::from_slice(text).as_ptr();
            ffi::TTF_SizeText(self.raw, ctext, &w, &h)
        };
        if ret != 0 {
            Err(get_error())
        } else {
            Ok((w as isize, h as isize))
        }
    }

    pub fn size_of_str(&self, text: &str) -> SdlResult<(isize, isize)> {
        //! Get size of UTF8 text string as would be rendered.
        let w = 0;
        let h = 0;
        let ret = unsafe {
            let ctext = CString::from_slice(text.as_bytes());
            ffi::TTF_SizeUTF8(self.raw, ctext.as_ptr(), &w, &h)
        };
        if ret != 0 {
            Err(get_error())
        } else {
            Ok((w as isize, h as isize))
        }
    }

    pub fn render_bytes_solid(&self, text: &[u8], fg: Color) -> SdlResult<Surface> {
        //! Draw LATIN1 text in solid mode.
        unsafe {
            let ctext = CString::from_slice(text).as_ptr();
            let raw = ffi::TTF_RenderText_Solid(self.raw, ctext, color_to_c_color(fg));
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(Surface::from_ll(raw, true))
            }
        }
    }

    pub fn render_str_solid(&self, text: &str, fg: Color) -> SdlResult<Surface> {
        //! Draw UTF8 text in solid mode.
        unsafe {
            let ctext = CString::from_slice(text.as_bytes());
            let raw = ffi::TTF_RenderUTF8_Solid(self.raw, ctext.as_ptr(), color_to_c_color(fg));
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(Surface::from_ll(raw, true))
            }
        }
    }

    pub fn render_char_solid(&self, ch: char, fg: Color) -> SdlResult<Surface> {
        //! Draw a UNICODE glyph in solid mode.
        unsafe {
            let raw = ffi::TTF_RenderGlyph_Solid(self.raw, ch as u16, color_to_c_color(fg));
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(Surface::from_ll(raw, true))
            }
        }
    }

    pub fn render_bytes_shaded(&self, text: &[u8], fg: Color, bg: Color) -> SdlResult<Surface> {
        //! Draw LATIN1 text in shaded mode.
        unsafe {
            let ctext = CString::from_slice(text).as_ptr();
            let raw = ffi::TTF_RenderText_Shaded(self.raw, ctext, color_to_c_color(fg), color_to_c_color(bg));
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(Surface::from_ll(raw, true))
            }
        }
    }

    pub fn render_str_shaded(&self, text: &str, fg: Color, bg: Color) -> SdlResult<Surface> {
        //! Draw UTF8 text in shaded mode.
        unsafe {
            let ctext = CString::from_slice(text.as_bytes());
            let raw = ffi::TTF_RenderUTF8_Shaded(self.raw, ctext.as_ptr(), color_to_c_color(fg), color_to_c_color(bg));
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(Surface::from_ll(raw, true))
            }
        }
    }

    pub fn render_char_shaded(&self, ch: char, fg: Color, bg: Color) -> SdlResult<Surface> {
        //! Draw a UNICODE glyph in shaded mode.
        unsafe {
            let raw = ffi::TTF_RenderGlyph_Shaded(self.raw, ch as u16, color_to_c_color(fg), color_to_c_color(bg));
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(Surface::from_ll(raw, true))
            }
        }
    }

    pub fn render_bytes_blended(&self, text: &[u8], fg: Color) -> SdlResult<Surface> {
        //! Draw LATIN1 text in blended mode.
        unsafe {
            let ctext = CString::from_slice(text).as_ptr();
            let raw = ffi::TTF_RenderText_Blended(self.raw, ctext, color_to_c_color(fg));
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(Surface::from_ll(raw, true))
            }
        }
    }

    pub fn render_str_blended(&self, text: &str, fg: Color) -> SdlResult<Surface> {
        //! Draw UTF8 text in blended mode.
        unsafe {
            let ctext = CString::from_slice(text.as_bytes());
            let raw = ffi::TTF_RenderUTF8_Blended(self.raw, ctext.as_ptr(), color_to_c_color(fg));
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(Surface::from_ll(raw, true))
            }
        }
    }

    pub fn render_char_blended(&self, ch: char, fg: Color) -> SdlResult<Surface> {
        //! Draw a UNICODE glyph in blended mode.
        unsafe {
            let raw = ffi::TTF_RenderGlyph_Blended(self.raw, ch as u16, color_to_c_color(fg));
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(Surface::from_ll(raw, true))
            }
        }
    }
}


/// Loader trait for RWops
pub trait LoaderRWops {
    /// Load src for use as a font.
    fn load_font(&self, ptsize: isize) -> SdlResult<Font>;
    /// Load src for use as a font.
    fn load_font_index(&self, ptsize: isize, index: isize) -> SdlResult<Font>;
}

impl LoaderRWops for RWops {
    fn load_font(&self, ptsize: isize) -> SdlResult<Font> {
        let raw = unsafe {
            ffi::TTF_OpenFontRW(self.raw(), 0, ptsize as c_int)
        };
        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(Font::from_ll(raw, true))
        }
    }
    fn load_font_index(&self, ptsize: isize, index: isize) -> SdlResult<Font> {
        let raw = unsafe {
            ffi::TTF_OpenFontIndexRW(self.raw(), 0, ptsize as c_int, index as c_long)
        };
        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(Font::from_ll(raw, true))
        }
    }
}
