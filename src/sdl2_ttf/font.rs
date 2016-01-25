use std::ffi::{CString, CStr};
use std::os::raw::{c_int, c_long};
use std::path::Path;
use std::error;
use std::error::Error;
use std::ffi::NulError;
use std::fmt;
use sdl2::ErrorMessage;
use sdl2::surface::Surface;
use sdl2_sys::surface::SDL_Surface;
use sdl2::get_error;
use sdl2::pixels;
use sdl2::pixels::Color;
use sdl2_sys::pixels::SDL_Color;
use sdl2::SdlResult;
use ffi;

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

#[derive(Debug, PartialEq, Clone)]
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

/// The result of an SDL2_TTF font operation.
pub type FontResult<T> = Result<T, FontError>;

/// A font-related error.
#[derive(Debug)]
pub enum FontError {
    /// A Latin-1 encoded byte string is invalid.
    InvalidLatin1Text(NulError),
    /// A SDL2-related error occured.
    SdlError(ErrorMessage),
}

impl error::Error for FontError {
    fn description(&self) -> &str {
        match self {
            &FontError::InvalidLatin1Text(ref error) => {
                error.description()
            },
            &FontError::SdlError(ref message) => {
                message.description()
            },
        }
    }

    fn cause<'a>(&'a self) -> Option<&'a error::Error> {
        match self {
            &FontError::InvalidLatin1Text(ref error) => {
                Some(error)
            },
            &FontError::SdlError(_) => {
                None
            },
        }
    }
}

impl fmt::Display for FontError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &FontError::InvalidLatin1Text(ref err) => {
                write!(f, "Invalid Latin-1 bytes: {}", err.description())
            },
            &FontError::SdlError(ref msg) => {
                write!(f, "SDL2 error: {}", msg)
            },
        }
        
    }
}

/// A renderable piece of text in the UTF8 or Latin-1 format.
enum RenderableText<'a> {
    Utf8(&'a str),
    Latin1(&'a [u8]),
}
impl<'a> RenderableText<'a> {
    /// Converts the given text to a c-style string if possible.
    fn convert(&self) -> FontResult<CString> {
        match self {
            &RenderableText::Utf8(text) => {
                Ok(CString::new(text).unwrap())
            },
            &RenderableText::Latin1(bytes) => {
                match CString::new(bytes) {
                    Err(err) => {
                        Err(FontError::InvalidLatin1Text(err))
                    },
                    Ok(cstring) => {
                        Ok(cstring)
                    }
                }
            }
        }
    }
}

/// A builder for a font rendering.
#[must_use]
pub struct PartialRender<'a> {
    text: RenderableText<'a>,
    font: &'a Font,
}

/// Converts the given raw pointer to a surface.
fn convert_to_surface<'a>(raw: *mut SDL_Surface) -> FontResult<Surface<'a>> {
    if (raw as *mut ()).is_null() {
        Err(FontError::SdlError(get_error()))
    } else {
        Ok(unsafe { 
            Surface::from_ll(raw)
        })
    }
}

impl<'a> PartialRender<'a> {
    /// Renders the text using the given solid color.
    pub fn solid<'b, T>(self, color: T )
            -> FontResult<Surface<'b>> where T: Into<Color> {
        let source = try!(self.text.convert());
        let color = color_to_c_color(color.into());
        let raw = unsafe {
            match self.text {
                RenderableText::Utf8(_) => {
                    ffi::TTF_RenderUTF8_Solid(self.font.raw(), 
                        source.as_ptr(), color)
                },
                RenderableText::Latin1(_) => {
                    ffi::TTF_RenderText_Solid(self.font.raw(), 
                        source.as_ptr(), color)
                },
            }
        };
        convert_to_surface(raw)
    }
    
    /// Renders the text.
    pub fn shaded<'b, T>(self, color: T, background: T) 
            -> FontResult<Surface<'b>> where T: Into<Color> {
        let source = try!(self.text.convert());
        let foreground = color_to_c_color(color.into());
        let background = color_to_c_color(background.into());
        let raw = unsafe {
            match self.text {
                RenderableText::Utf8(_) => {
                    ffi::TTF_RenderUTF8_Shaded(self.font.raw(), 
                        source.as_ptr(), foreground, background)
                },
                RenderableText::Latin1(_) => {
                    ffi::TTF_RenderText_Shaded(self.font.raw(), 
                        source.as_ptr(), foreground, background)
                },
            }
        };
        convert_to_surface(raw)
    }
    
    /// Renders the text.
    pub fn blended<'b, T>(self, color: T) 
            -> FontResult<Surface<'b>> where T: Into<Color> {
        let source = try!(self.text.convert());
        let color = color_to_c_color(color.into());
        let raw = unsafe {
            match self.text {
                RenderableText::Utf8(_) => {
                    ffi::TTF_RenderUTF8_Blended(self.font.raw(), 
                        source.as_ptr(), color)
                },
                RenderableText::Latin1(_) => {
                    ffi::TTF_RenderText_Blended(self.font.raw(), 
                        source.as_ptr(), color)
                },
            }
        };
        convert_to_surface(raw)
    }
    
    /// Renders the text blendedly but wrapping the words if the width exceeds
    /// the given maximum width.
    pub fn blended_wrapped<'b, T>(self, color: T, wrap_max_width: u32) 
            -> FontResult<Surface<'b>> where T: Into<Color> {
        let source = try!(self.text.convert());
        let color = color_to_c_color(color.into());
        let raw = unsafe {
            match self.text {
                RenderableText::Utf8(_) => {
                    ffi::TTF_RenderUTF8_Blended_Wrapped(self.font.raw(), 
                        source.as_ptr(), color, wrap_max_width)
                },
                RenderableText::Latin1(_) => {
                    ffi::TTF_RenderText_Blended_Wrapped(self.font.raw(), 
                        source.as_ptr(), color, wrap_max_width)
                },
            }
        };
        convert_to_surface(raw)
    }
}

/// The opaque holder of a loaded font.
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

pub fn internal_load_font(path: &Path, ptsize: u16) -> SdlResult<Font> {
    unsafe {
        let cstring = CString::new(path.to_str().unwrap()).unwrap();
        let raw = ffi::TTF_OpenFont(cstring.as_ptr(), ptsize as c_int);
        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(Font { raw: raw, owned: true })
        }
    }
}

pub fn internal_load_font_from_ll(raw: *const ffi::TTF_Font, owned: bool) 
        -> Font {
    Font { raw: raw, owned: owned }
}

pub fn internal_load_font_at_index(path: &Path, index: u32, ptsize: u16)
        -> SdlResult<Font> {
    unsafe {
        let cstring = CString::new(path.to_str().unwrap().as_bytes())
            .unwrap();
        let raw = ffi::TTF_OpenFontIndex(cstring.as_ptr(), 
            ptsize as c_int, index as c_long);
        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(Font { raw: raw, owned: true })
        }
    }
}

impl Font {
    /// Returns the underlying C font object.
    unsafe fn raw(&self) -> *const ffi::TTF_Font {
        self.raw
    }
    
    /// Starts specifying a render of the given UTF-8 text.
    pub fn render<'a>(&'a self, text: &'a str) -> PartialRender<'a> {
        PartialRender { 
            text: RenderableText::Utf8(text),
            font: self,
        }
    }
    
    /// Starts specifying of the given Latin-1 text.
    pub fn render_latin1<'a>(&'a self, text: &'a [u8]) -> PartialRender<'a> {
        PartialRender {
            text: RenderableText::Latin1(text),
            font: self,
        }
    }
    
    /// Returns the font's style flags.
    pub fn get_style(&self) -> FontStyle {
        unsafe {
            let raw = ffi::TTF_GetFontStyle(self.raw);
            FontStyle::from_bits_truncate(raw)
        }
    }

    /// Sets the font's style flags.
    pub fn set_style(&mut self, styles: FontStyle) {
        unsafe {
            ffi::TTF_SetFontStyle(self.raw, styles.bits())
        }
    }
    
    /// Returns the width of the font's outline.
    pub fn get_outline_width(&self) -> u16 {
        unsafe {
            ffi::TTF_GetFontOutline(self.raw) as u16
        }
    }
    
    /// Sets the width of the font's outline.
    pub fn set_outline_width(&mut self, width: u16) {
        unsafe {
            ffi::TTF_SetFontOutline(self.raw, width as c_int)
        }
    }
    
    /// Returns the font's freetype hints.
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

    /// Sets the font's freetype hints.
    pub fn set_hinting(&mut self, hinting: Hinting) {
        unsafe {
            ffi::TTF_SetFontHinting(self.raw, hinting as c_int)
        }
    }
    
    /// Returns whether the font is kerning.
    pub fn get_kerning(&self) -> bool {
        unsafe {
            ffi::TTF_GetFontKerning(self.raw) != 0
        }
    }
    
    /// Sets whether the font should use kerning.
    pub fn set_kerning(&mut self, kerning: bool) {
        unsafe {
            ffi::TTF_SetFontKerning(self.raw, kerning as c_int)
        }
    }

    pub fn height(&self) -> u16 {
        //! Get font maximum total height.
        unsafe {
            ffi::TTF_FontHeight(self.raw) as u16
        }
    }

    /// Returns the font's highest ascent (height above base).
    pub fn ascent(&self) -> u16 {
        unsafe {
            ffi::TTF_FontAscent(self.raw) as u16
        }
    }

    /// Returns the font's lowest descent (height below base).
    pub fn descent(&self) -> u16 {
        unsafe {
            ffi::TTF_FontDescent(self.raw) as u16
        }
    }
    
    /// Returns the recommended line spacing for text rendered with this font.
    pub fn recommended_line_spacing(&self) -> u16 {
        unsafe {
            ffi::TTF_FontLineSkip(self.raw) as u16
        }
    }
    
    /// Returns the number of faces in this font.
    pub fn face_count(&self) -> u16 {
        unsafe {
            ffi::TTF_FontFaces(self.raw) as u16
        }
    }

    /// Returns whether the font is monospaced.
    pub fn face_is_fixed_width(&self) -> bool {
        unsafe {
            ffi::TTF_FontFaceIsFixedWidth(self.raw) != 0
        }
    }
    
    /// Returns the family name of the current font face.
    pub fn face_family_name(&self) -> Option<String> {
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
    
    /// Returns the name of the current font face.
    pub fn face_style_name(&self) -> Option<String> {
        unsafe {
            let cname = ffi::TTF_FontFaceStyleName(self.raw);
            if cname.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(CStr::from_ptr(cname).to_bytes()).to_string())
            }
        }
    }
    
    /// Returns the index of the given character in this font face.
    pub fn find_glyph(&self, ch: char) -> Option<u16> {
        unsafe {
            let ret = ffi::TTF_GlyphIsProvided(self.raw, ch as u16);
            if ret == 0 {
                None
            } else {
                Some(ret as u16)
            }
        }
    }
    
    /// Returns the glyph metrics of the given character in this font face.
    pub fn find_glyph_metrics(&self, ch: char) -> Option<GlyphMetrics> {
        let minx = 0;
        let maxx = 0;
        let miny = 0;
        let maxy = 0;
        let advance = 0;
        let ret = unsafe {
            ffi::TTF_GlyphMetrics(
                self.raw, ch as u16, &minx, &maxx, &miny, &maxy, &advance
            )
        };
        if ret != 0 {
            None
        } else {
            Some(GlyphMetrics { 
                minx: minx as i32, maxx: maxx as i32, miny: miny as i32,
                maxy: maxy as i32, advance: advance as i32
            } )
        }
    }
    
    /// Returns the surface size of a c-style string when rendered using this
    /// font.
    #[allow(unused_mut)]
    fn size_of_c_string(&self, text: &CString) -> FontResult<(u32, u32)> {
        let (res, size) = unsafe {
            let mut w = 0; // mutated by C code
            let mut h = 0; // mutated by C code
            let ret = ffi::TTF_SizeText(self.raw, text.as_ptr(), &w, &h);
            (ret, (w as u32, h as u32))
        };
        if res != 0 {
            Err(FontError::SdlError(get_error()))
        } else {
            Ok(size)
        }
    }

    /// Returns the width and height of the given text when rendered using this
    /// font.
    pub fn size_of(&self, text: &str) -> FontResult<(u32, u32)> {
        let c_string = try!(RenderableText::Utf8(text).convert());
        self.size_of_c_string(&c_string)
    }
    
    /// Returns the width and height of the given text when rendered using this
    /// font.
    pub fn size_of_latin1(&self, text: &[u8]) 
        -> FontResult<(u32, u32)> {
        let c_string = try!(RenderableText::Latin1(text).convert());
        self.size_of_c_string(&c_string)
    }
}