/*!
A binding for SDL2_ttf.
 */

extern crate libc;
extern crate sdl2;
extern crate sdl2_sys;

#[macro_use]
extern crate bitflags;

use libc::{c_int, c_long};
use std::ffi::{CString, CStr};
use std::path::Path;
use sdl2::surface::Surface;
use sdl2::get_error;
use sdl2::pixels;
use sdl2::pixels::Color;
use sdl2_sys::pixels::SDL_Color;
use sdl2::rwops::RWops;
use sdl2::version::Version;
use sdl2::SdlResult;
use std::fmt::{Display, Formatter};

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
    const STYLE_NORMAL        = ffi::TTF_STYLE_NORMAL,
    const STYLE_BOLD          = ffi::TTF_STYLE_BOLD,
    const STYLE_ITALIC        = ffi::TTF_STYLE_ITALIC,
    const STYLE_UNDERLINE     = ffi::TTF_STYLE_UNDERLINE,
    const STYLE_STRIKETHROUGH = ffi::TTF_STYLE_STRIKETHROUGH,
    }
}

#[derive(Debug, PartialEq)]
pub enum Hinting {
    Normal = ffi::TTF_HINTING_NORMAL as isize,
    Light  = ffi::TTF_HINTING_LIGHT  as isize,
    Mono   = ffi::TTF_HINTING_MONO   as isize,
    None   = ffi::TTF_HINTING_NONE   as isize
}

/// Glyph Metrics
#[derive(Debug, PartialEq, Clone)]
pub struct GlyphMetrics {
    pub minx: i32,
    pub maxx: i32,
    pub miny: i32,
    pub maxy: i32,
    pub advance: i32
}

/// A context manager for SDL2_TTF to manage C code init and quit
#[must_use]
pub struct Sdl2TtfContext;

// Clean up the context once it goes out of scope
impl Drop for Sdl2TtfContext {
    fn drop(&mut self) {
        unsafe { ffi::TTF_Quit(); }
    }
}

/// Returns the version of the dynamically linked SDL_ttf library
pub fn get_linked_version() -> Version {
    unsafe {
        Version::from_ll(*ffi::TTF_Linked_Version())
    }
}

/// An error for when sdl2_ttf is attempted initialized twice
// Necessary for context management, unless we find a way to have a singleton
#[derive(Debug)]
pub enum Error {
    InitializationError(std::io::Error),
    AlreadyInitializedError,
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::AlreadyInitializedError => "SDL2_TTF has already been initialized",
            &Error::InitializationError(ref error) => error.description(),
        }
    }

    fn cause<'a>(&'a self) -> Option<&'a std::error::Error> {
        match self {
            &Error::AlreadyInitializedError => None,
            &Error::InitializationError(ref error) => Some(error),
        }
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        formatter.write_str("SDL2_TTF has already been initialized")
    }
}

/// Initialize the truetype font API and returns a context manager which will clean up the library
/// once it goes out of scope. You can't really use it, but keep the reference alive :)
pub fn init() -> Result<Sdl2TtfContext, Error> {
    unsafe {
        if ffi::TTF_WasInit() == 1 {
            Err(Error::AlreadyInitializedError)
        } else {
            if ffi::TTF_Init() == 0 {
                Ok(Sdl2TtfContext)
            } else {
                Err(Error::InitializationError(std::io::Error::last_os_error()))
            }
        }
    }
}

/// Returns whether the underlying library has been initialized
pub fn has_been_initialized() -> bool {
    unsafe {
        ffi::TTF_WasInit() == 1
    }
}

/// The opaque holder of a loaded font.
#[allow(raw_pointer_derive)]
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

/// A renderable piece of text in the UTF8 or Latin-1 format
pub enum Text<'a> {
    Latin1(&'a [u8]),
    Utf8(&'a str),
    Char(char),
}

/// Automatically convert strs to the right format
impl <'a> From<&'a str> for Text<'a> {
    fn from(string: &'a str) -> Text<'a> {
        Text::Utf8(string)
    }
}

/// Automatically convert Strings to the right format
impl <'a> From<&'a String> for Text<'a> {
    fn from(string: &'a String) -> Text<'a> {
        Text::Utf8(string)
    }
}

/// Automatically convert chars to the right format
impl <'a> From<char> for Text<'a> {
    fn from(ch: char) -> Text<'a> {
        Text::Char(ch)
    }
}

/// Automatically convert latin-1 bytes to the right format
impl <'a> From<&'a [u8]> for Text<'a> {
    fn from(bytes: &'a [u8]) -> Text<'a> {
        Text::Latin1(bytes)
    }
}

/// The supported text rendering modes and their parameters
pub enum RenderMode {
    Solid { foreground: Color },
    Shaded { foreground: Color, background: Color },
    Blended { foreground: Color },
    BlendedWrapped { foreground: Color, wrap_length: u32 },
}

/// Constructor for solid font rendering
pub fn solid<T>(foreground: T) -> RenderMode where T: Into<Color> {
    RenderMode::Solid { foreground: foreground.into() }
}

/// Constructor for blended font rendering
pub fn blended<T>(foreground: T) -> RenderMode where T: Into<Color> {
    RenderMode::Blended { foreground: foreground.into() }
}

/// Constructor for blended wrapped font rendering
pub fn blended_wrapped<T>(foreground: T, wrap_length: u32) -> RenderMode
        where T: Into<Color> {
    RenderMode::BlendedWrapped {
        foreground: foreground.into(),
        wrap_length: wrap_length,
    }
}

/// Constructor for shaded font rendering
pub fn shaded<T, U>(foreground: T, background: U) -> RenderMode
        where T: Into<Color>, U: Into<Color> {
    RenderMode::Shaded { foreground: foreground.into(), background: background.into() }
}

impl Font {
    fn from_ll(raw: *const ffi::TTF_Font, owned: bool) -> Font {
        Font { raw: raw, owned: owned }
    }

    pub fn from_file(filename: &Path, ptsize: i32) -> SdlResult<Font> {
        //! Load file for use as a font, at ptsize size.
        unsafe {
            let cstring = CString::new(filename.to_str().unwrap()).unwrap();
            let raw = ffi::TTF_OpenFont(cstring.as_ptr(), ptsize as c_int);
            if raw.is_null() {
                Err(get_error())
            } else {
                Ok(Font { raw: raw, owned: true })
            }
        }
    }

    pub fn from_file_index(filename: &Path, ptsize: i32, index: i32) -> SdlResult<Font> {
        //! Load file, face index, for use as a font, at ptsize size.
        unsafe {
            let cstring = CString::new(filename.to_str().unwrap().as_bytes()).unwrap();
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

    pub fn get_outline(&self) -> i32 {
        //! Get font outline width.
        unsafe {
            ffi::TTF_GetFontOutline(self.raw) as i32
        }
    }

    pub fn set_outline(&mut self, outline: i32) {
        //! Set font outline width.
        unsafe {
            ffi::TTF_SetFontOutline(self.raw, outline as c_int)
        }
    }

    pub fn get_hinting(&self) -> Hinting {
        //! Get freetype hinter setting.
        unsafe {
            match ffi::TTF_GetFontHinting(self.raw) as c_int {
                ffi::TTF_HINTING_NORMAL => Hinting::Normal,
                ffi::TTF_HINTING_LIGHT  => Hinting::Light,
                ffi::TTF_HINTING_MONO   => Hinting::Mono,
                ffi::TTF_HINTING_NONE   => Hinting::None,
                _                       => Hinting::None
            }
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

    pub fn height(&self) -> i32 {
        //! Get font maximum total height.
        unsafe {
            ffi::TTF_FontHeight(self.raw) as i32
        }
    }

    pub fn ascent(&self) -> i32 {
        //! Get font highest ascent (height above base).
        unsafe {
            ffi::TTF_FontAscent(self.raw) as i32
        }
    }

    pub fn descent(&self) -> i32 {
        //! Get font lowest descent (height below base).
        unsafe {
            ffi::TTF_FontDescent(self.raw) as i32
        }
    }

    pub fn line_skip(&self) -> i32 {
        //! Get font recommended line spacing.
        unsafe {
            ffi::TTF_FontLineSkip(self.raw) as i32
        }
    }

    pub fn faces(&self) -> i32 {
        //! Get the number of faces in a font.
        unsafe {
            ffi::TTF_FontFaces(self.raw) as i32
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
                Some(String::from_utf8_lossy(CStr::from_ptr(cname).to_bytes()).to_string())
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
                Some(String::from_utf8_lossy(CStr::from_ptr(cname).to_bytes()).to_string())
            }
        }
    }

    pub fn index_of_char(&self, ch: char) -> Option<i32> {
        //! Get individual font glyph availability.
        unsafe {
            let ret = ffi::TTF_GlyphIsProvided(self.raw, ch as u16);
            if ret == 0 {
                None
            } else {
                Some(ret as i32)
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
            Some(GlyphMetrics { minx: minx as i32, maxx: maxx as i32,
                                miny: miny as i32, maxy: maxy as i32,
                                advance: advance as i32 })
        }
    }

    /// Get the size of the given text piece when rendered using this font
    #[allow(unused_mut)]
    pub fn size<'a, T>(&self, text: T) -> SdlResult<(u32, u32)> where T: Into<Text<'a>> {
        let mut w = 0; // mutated by C code
        let mut h = 0; // mutated by C code
        let ctext = match text.into() {
            Text::Latin1(bytes) => CString::new(bytes).unwrap(),
            Text::Utf8(string) => CString::new(string.as_bytes()).unwrap(),
            Text::Char(ch) => {
                let mut s: String = String::new();
                s.push(ch);
                CString::new(s.as_bytes()).unwrap()
            },
        };
        let ret = unsafe {
            ffi::TTF_SizeText(self.raw, ctext.as_ptr(), &w, &h)
        };
        if ret != 0 {
            Err(get_error())
        } else {
            Ok((w as u32, h as u32))
        }
    }

    /// Attempt to render the given text to a SDL surface with the given mode.
    /// The text argument can be either a &str, a &[u8] latin-1 array or a char
    /// (This is because the Into trait has been implemented for these types)
    pub fn render<'a, T>(&self, text: T, mode: RenderMode) -> SdlResult<Surface>
            where T: Into<Text<'a>> {
        unsafe {
            let raw = match text.into() {
                // Render a latin-1 string of bytes
                Text::Latin1(bytes) => {
                    let source = CString::new(bytes).unwrap();
                    match mode {
                        RenderMode::Solid { foreground } => {
                            ffi::TTF_RenderText_Solid(self.raw, source.as_ptr(),
                                color_to_c_color(foreground))
                        },
                        RenderMode::Shaded { foreground, background } => {
                            ffi::TTF_RenderText_Shaded(self.raw, source.as_ptr(),
                                color_to_c_color(foreground), color_to_c_color(background))
                        },
                        RenderMode::Blended { foreground } => {
                            ffi::TTF_RenderText_Blended(self.raw, source.as_ptr(),
                                color_to_c_color(foreground))
                        },
                        RenderMode::BlendedWrapped { foreground, wrap_length } => {
                            ffi::TTF_RenderText_Blended_Wrapped(self.raw, source.as_ptr(),
                                color_to_c_color(foreground), wrap_length)
                        }
                    }
                },
                // Render a UTF-8 string
                Text::Utf8(string) => {
                    let source = CString::new(string.as_bytes()).unwrap();
                    match mode {
                        RenderMode::Solid { foreground } => {
                            ffi::TTF_RenderUTF8_Solid(self.raw, source.as_ptr(),
                                color_to_c_color(foreground))
                        },
                        RenderMode::Shaded { foreground, background } => {
                            ffi::TTF_RenderUTF8_Shaded(self.raw, source.as_ptr(),
                                color_to_c_color(foreground), color_to_c_color(background))
                        },
                        RenderMode::Blended { foreground } => {
                            ffi::TTF_RenderUTF8_Blended(self.raw, source.as_ptr(),
                                color_to_c_color(foreground))
                        },
                        RenderMode::BlendedWrapped { foreground, wrap_length } => {
                            ffi::TTF_RenderUTF8_Blended_Wrapped(self.raw, source.as_ptr(),
                                color_to_c_color(foreground), wrap_length)
                        }
                    }
                },
                // Render a char
                Text::Char(ch) => {
                    let source = ch as u16;
                    match mode {
                        RenderMode::Solid { foreground } => {
                            ffi::TTF_RenderGlyph_Solid(self.raw, source,
                                color_to_c_color(foreground))
                        },
                        RenderMode::Shaded { foreground, background } => {
                            ffi::TTF_RenderGlyph_Shaded(self.raw, source,
                                color_to_c_color(foreground), color_to_c_color(background))
                        },
                        RenderMode::Blended { foreground } => {
                            ffi::TTF_RenderGlyph_Blended(self.raw, source,
                                color_to_c_color(foreground))
                        },
                        RenderMode::BlendedWrapped { foreground, wrap_length: _ } => {
                            ffi::TTF_RenderGlyph_Blended(self.raw, source,
                                color_to_c_color(foreground))
                        }
                    }
                }
            };
            if (raw as *mut ()).is_null() {
                Err(get_error())
            } else {
                Ok(Surface::from_ll(raw))
            }
        }
    }
}


/// Extension trait for RWops to more easily load fonts
pub trait RWopsFontExt {
    /// Load src for use as a font.
    fn load_font(&self, ptsize: i32) -> SdlResult<Font>;
    /// Load src for use as a font.
    fn load_font_index(&self, ptsize: i32, index: i32) -> SdlResult<Font>;
}

impl<'a> RWopsFontExt for RWops<'a> {
    fn load_font(&self, ptsize: i32) -> SdlResult<Font> {
        let raw = unsafe {
            ffi::TTF_OpenFontRW(self.raw(), 0, ptsize as c_int)
        };
        if (raw as *mut ()).is_null() {
            Err(get_error())
        } else {
            Ok(Font::from_ll(raw, true))
        }
    }
    fn load_font_index(&self, ptsize: i32, index: i32) -> SdlResult<Font> {
        let raw = unsafe {
            ffi::TTF_OpenFontIndexRW(self.raw(), 0, ptsize as c_int, index as c_long)
        };
        if (raw as *mut ()).is_null() {
            Err(get_error())
        } else {
            Ok(Font::from_ll(raw, true))
        }
    }
}
