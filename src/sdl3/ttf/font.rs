use get_error;
use pixels::Color;
use rwops::RWops;
use std::error;
use std::error::Error;
use std::ffi::NulError;
use std::ffi::{CStr, CString};
use std::fmt;
use std::marker::PhantomData;
use std::os::raw::{c_int, c_long, c_uint};
use std::path::Path;
use surface::Surface;
use sys::ttf;
use sys::SDL_Surface;

bitflags! {
    /// The styling of a font.
    pub struct FontStyle: i32 {
        const NORMAL        = ttf::TTF_STYLE_NORMAL as i32;
        const BOLD          = ttf::TTF_STYLE_BOLD as i32;
        const ITALIC        = ttf::TTF_STYLE_ITALIC as i32;
        const UNDERLINE     = ttf::TTF_STYLE_UNDERLINE as i32;
        const STRIKETHROUGH = ttf::TTF_STYLE_STRIKETHROUGH as i32;
    }
}

/// Information about the hinting of a font.
/// See [wikipedia](https://en.wikipedia.org/wiki/Font_hinting)
#[repr(i32)]
#[derive(Debug, PartialEq, Clone)]
pub enum Hinting {
    Normal = ttf::TTF_HINTING_NORMAL as i32,
    Light = ttf::TTF_HINTING_LIGHT as i32,
    Mono = ttf::TTF_HINTING_MONO as i32,
    None = ttf::TTF_HINTING_NONE as i32,
}

/// Information about a specific glyph (character) in a font face.
#[derive(Debug, PartialEq, Clone)]
pub struct GlyphMetrics {
    pub minx: i32,
    pub maxx: i32,
    pub miny: i32,
    pub maxy: i32,
    pub advance: i32,
}

/// The result of an `SDL2_TTF` font operation.
pub type FontResult<T> = Result<T, FontError>;

/// A font-related error.
#[derive(Debug, Clone)]
pub enum FontError {
    /// A Latin-1 encoded byte string is invalid.
    InvalidLatin1Text(NulError),
    /// A SDL2-related error occured.
    SdlError(String),
}

impl error::Error for FontError {
    fn description(&self) -> &str {
        match *self {
            FontError::InvalidLatin1Text(ref error) => error.description(),
            FontError::SdlError(ref message) => message,
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            FontError::InvalidLatin1Text(ref error) => Some(error),
            FontError::SdlError(_) => None,
        }
    }
}

impl fmt::Display for FontError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            FontError::InvalidLatin1Text(ref err) => {
                write!(f, "Invalid Latin-1 bytes: {}", err.description())
            }
            FontError::SdlError(ref msg) => {
                write!(f, "SDL2 error: {}", msg)
            }
        }
    }
}

/// A renderable piece of text in the UTF8 or Latin-1 format.
enum RenderableText<'a> {
    Utf8(&'a str),
    Latin1(&'a [u8]),
    Char(String),
}
impl<'a> RenderableText<'a> {
    /// Converts the given text to a c-style string if possible.
    fn convert(&self) -> FontResult<CString> {
        match *self {
            RenderableText::Utf8(text) => Ok(CString::new(text).unwrap()),
            RenderableText::Latin1(bytes) => match CString::new(bytes) {
                Err(err) => Err(FontError::InvalidLatin1Text(err)),
                Ok(cstring) => Ok(cstring),
            },
            RenderableText::Char(ref string) => Ok(CString::new(string.as_bytes()).unwrap()),
        }
    }
}

/// A builder for a font rendering.
#[must_use]
pub struct PartialRendering<'f, 'text> {
    text: RenderableText<'text>,
    font: &'f Font<'f, 'f>,
}

/// Converts the given raw pointer to a surface.
fn convert_to_surface<'a>(raw: *mut SDL_Surface) -> FontResult<Surface<'a>> {
    if (raw as *mut ()).is_null() {
        Err(FontError::SdlError(get_error()))
    } else {
        Ok(unsafe { Surface::from_ll(raw) })
    }
}

impl<'f, 'text> PartialRendering<'f, 'text> {
    /// Renders the text in *solid* mode.
    /// See [the SDL2_TTF docs](https://www.libsdl.org/projects/SDL_ttf/docs/SDL_ttf.html#SEC42)
    /// for an explanation.
    pub fn solid<'b, T>(self, color: T) -> FontResult<Surface<'b>>
    where
        T: Into<Color>,
    {
        let source = self.text.convert()?;
        let color = color.into().into();
        let raw = unsafe {
            match self.text {
                RenderableText::Utf8(_) | RenderableText::Char(_) => {
                    ttf::TTF_RenderUTF8_Solid(self.font.raw(), source.as_ptr(), color)
                }
                RenderableText::Latin1(_) => {
                    ttf::TTF_RenderText_Solid(self.font.raw(), source.as_ptr(), color)
                }
            }
        };
        convert_to_surface(raw)
    }

    /// Renders the text in *shaded* mode.
    /// See [the SDL2_TTF docs](https://www.libsdl.org/projects/SDL_ttf/docs/SDL_ttf.html#SEC42)
    /// for an explanation.
    pub fn shaded<'b, T>(self, color: T, background: T) -> FontResult<Surface<'b>>
    where
        T: Into<Color>,
    {
        let source = self.text.convert()?;
        let foreground = color.into().into();
        let background = background.into().into();
        let raw = unsafe {
            match self.text {
                RenderableText::Utf8(_) | RenderableText::Char(_) => ttf::TTF_RenderUTF8_Shaded(
                    self.font.raw(),
                    source.as_ptr(),
                    foreground,
                    background,
                ),
                RenderableText::Latin1(_) => ttf::TTF_RenderText_Shaded(
                    self.font.raw(),
                    source.as_ptr(),
                    foreground,
                    background,
                ),
            }
        };
        convert_to_surface(raw)
    }

    /// Renders the text in *blended* mode.
    /// See [the SDL2_TTF docs](https://www.libsdl.org/projects/SDL_ttf/docs/SDL_ttf.html#SEC42)
    /// for an explanation.
    pub fn blended<'b, T>(self, color: T) -> FontResult<Surface<'b>>
    where
        T: Into<Color>,
    {
        let source = self.text.convert()?;
        let color = color.into().into();
        let raw = unsafe {
            match self.text {
                RenderableText::Utf8(_) | RenderableText::Char(_) => {
                    ttf::TTF_RenderUTF8_Blended(self.font.raw(), source.as_ptr(), color)
                }
                RenderableText::Latin1(_) => {
                    ttf::TTF_RenderText_Blended(self.font.raw(), source.as_ptr(), color)
                }
            }
        };
        convert_to_surface(raw)
    }

    /// Renders the text in *blended* mode but wrapping the words if the width
    /// exceeds the given maximum width.
    /// See [the SDL2_TTF docs](https://www.libsdl.org/projects/SDL_ttf/docs/SDL_ttf.html#SEC42)
    /// for an explanation of the mode.
    pub fn blended_wrapped<'b, T>(self, color: T, wrap_max_width: u32) -> FontResult<Surface<'b>>
    where
        T: Into<Color>,
    {
        let source = self.text.convert()?;
        let color = color.into().into();
        let raw = unsafe {
            match self.text {
                RenderableText::Utf8(_) | RenderableText::Char(_) => {
                    ttf::TTF_RenderUTF8_Blended_Wrapped(
                        self.font.raw(),
                        source.as_ptr(),
                        color,
                        wrap_max_width,
                    )
                }
                RenderableText::Latin1(_) => ttf::TTF_RenderText_Blended_Wrapped(
                    self.font.raw(),
                    source.as_ptr(),
                    color,
                    wrap_max_width,
                ),
            }
        };
        convert_to_surface(raw)
    }
}

/// A loaded TTF font.
pub struct Font<'ttf_module, 'rwops> {
    raw: *mut ttf::TTF_Font,
    // RWops is only stored here because it must not outlive
    // the Font struct, and this RWops should not be used by
    // anything else
    // None means that the RWops is handled by SDL itself,
    // and Some(rwops) means that the RWops is handled by the Rust
    // side
    #[allow(dead_code)]
    rwops: Option<RWops<'rwops>>,
    #[allow(dead_code)]
    _marker: PhantomData<&'ttf_module ()>,
}

impl<'ttf, 'r> Drop for Font<'ttf, 'r> {
    fn drop(&mut self) {
        unsafe {
            // avoid close font after quit()
            if ttf::TTF_WasInit() == 1 {
                ttf::TTF_CloseFont(self.raw);
            }
        }
    }
}

/// Internally used to load a font (for internal visibility).
pub fn internal_load_font<'ttf, P: AsRef<Path>>(
    path: P,
    ptsize: u16,
) -> Result<Font<'ttf, 'static>, String> {
    unsafe {
        let cstring = CString::new(path.as_ref().to_str().unwrap()).unwrap();
        let raw = ttf::TTF_OpenFont(cstring.as_ptr(), ptsize as c_int);
        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(Font {
                raw: raw,
                rwops: None,
                _marker: PhantomData,
            })
        }
    }
}

/// Internally used to load a font (for internal visibility).
pub fn internal_load_font_from_ll<'ttf, 'r, R>(raw: *mut ttf::TTF_Font, rwops: R) -> Font<'ttf, 'r>
where
    R: Into<Option<RWops<'r>>>,
{
    Font {
        raw: raw,
        rwops: rwops.into(),
        _marker: PhantomData,
    }
}

/// Internally used to load a font (for internal visibility).
pub fn internal_load_font_at_index<'ttf, P: AsRef<Path>>(
    path: P,
    index: u32,
    ptsize: u16,
) -> Result<Font<'ttf, 'static>, String> {
    unsafe {
        let cstring = CString::new(path.as_ref().to_str().unwrap().as_bytes()).unwrap();
        let raw = ttf::TTF_OpenFontIndex(cstring.as_ptr(), ptsize as c_int, index as c_long);
        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(Font {
                raw: raw,
                rwops: None,
                _marker: PhantomData,
            })
        }
    }
}

impl<'ttf, 'r> Font<'ttf, 'r> {
    /// Returns the underlying C font object.
    // this can prevent introducing UB until
    // https://github.com/rust-lang/rust-clippy/issues/5953 is fixed
    #[allow(clippy::trivially_copy_pass_by_ref)]
    unsafe fn raw(&self) -> *mut ttf::TTF_Font {
        self.raw
    }

    /// Starts specifying a rendering of the given UTF-8-encoded text.
    pub fn render<'a, 'b>(&'a self, text: &'b str) -> PartialRendering<'a, 'b> {
        PartialRendering {
            text: RenderableText::Utf8(text),
            font: self,
        }
    }

    /// Starts specifying a rendering of the given Latin-1-encoded text.
    pub fn render_latin1<'a, 'b>(&'a self, text: &'b [u8]) -> PartialRendering<'a, 'b> {
        PartialRendering {
            text: RenderableText::Latin1(text),
            font: self,
        }
    }

    /// Starts specifying a rendering of the given UTF-8-encoded character.
    pub fn render_char<'a>(&'a self, ch: char) -> PartialRendering<'a, 'static> {
        let mut s = String::new();
        s.push(ch);
        PartialRendering {
            text: RenderableText::Char(s),
            font: self,
        }
    }

    /// Returns the width and height of the given text when rendered using this
    /// font.
    pub fn size_of(&self, text: &str) -> FontResult<(u32, u32)> {
        let c_string = RenderableText::Utf8(text).convert()?;
        let (res, size) = unsafe {
            let mut w = 0; // mutated by C code
            let mut h = 0; // mutated by C code
            let ret = ttf::TTF_SizeUTF8(self.raw, c_string.as_ptr(), &mut w, &mut h);
            (ret, (w as u32, h as u32))
        };
        if res == 0 {
            Ok(size)
        } else {
            Err(FontError::SdlError(get_error()))
        }
    }

    /// Returns the width and height of the given text when rendered using this
    /// font.
    #[allow(unused_mut)]
    pub fn size_of_latin1(&self, text: &[u8]) -> FontResult<(u32, u32)> {
        let c_string = RenderableText::Latin1(text).convert()?;
        let (res, size) = unsafe {
            let mut w: i32 = 0; // mutated by C code
            let mut h: i32 = 0; // mutated by C code
            let ret = ttf::TTF_SizeText(
                self.raw,
                c_string.as_ptr(),
                &w as *const _ as *mut i32,
                &h as *const _ as *mut i32,
            );
            (ret, (w as u32, h as u32))
        };
        if res == 0 {
            Ok(size)
        } else {
            Err(FontError::SdlError(get_error()))
        }
    }

    /// Returns the width and height of the given text when rendered using this
    /// font.
    pub fn size_of_char(&self, ch: char) -> FontResult<(u32, u32)> {
        let mut s = String::new();
        s.push(ch);
        self.size_of(&s)
    }

    /// Returns the font's style flags.
    pub fn get_style(&self) -> FontStyle {
        unsafe {
            let raw = ttf::TTF_GetFontStyle(self.raw);
            FontStyle::from_bits_truncate(raw as i32)
        }
    }

    /// Sets the font's style flags.
    pub fn set_style(&mut self, styles: FontStyle) {
        unsafe { ttf::TTF_SetFontStyle(self.raw, styles.bits() as c_int) }
    }

    /// Returns the width of the font's outline.
    pub fn get_outline_width(&self) -> u16 {
        unsafe { ttf::TTF_GetFontOutline(self.raw) as u16 }
    }

    /// Sets the width of the font's outline.
    pub fn set_outline_width(&mut self, width: u16) {
        unsafe { ttf::TTF_SetFontOutline(self.raw, width as c_int) }
    }

    /// Returns the font's freetype hints.
    pub fn get_hinting(&self) -> Hinting {
        unsafe {
            match ttf::TTF_GetFontHinting(self.raw) as c_uint {
                ttf::TTF_HINTING_NORMAL => Hinting::Normal,
                ttf::TTF_HINTING_LIGHT => Hinting::Light,
                ttf::TTF_HINTING_MONO => Hinting::Mono,
                ttf::TTF_HINTING_NONE | _ => Hinting::None,
            }
        }
    }

    /// Sets the font's freetype hints.
    pub fn set_hinting(&mut self, hinting: Hinting) {
        unsafe { ttf::TTF_SetFontHinting(self.raw, hinting as c_int) }
    }

    /// Returns whether the font is kerning.
    pub fn get_kerning(&self) -> bool {
        unsafe { ttf::TTF_GetFontKerning(self.raw) != 0 }
    }

    /// Sets whether the font should use kerning.
    pub fn set_kerning(&mut self, kerning: bool) {
        unsafe { ttf::TTF_SetFontKerning(self.raw, kerning as c_int) }
    }

    pub fn height(&self) -> i32 {
        //! Get font maximum total height.
        unsafe { ttf::TTF_FontHeight(self.raw) as i32 }
    }

    /// Returns the font's highest ascent (height above base).
    pub fn ascent(&self) -> i32 {
        unsafe { ttf::TTF_FontAscent(self.raw) as i32 }
    }

    /// Returns the font's lowest descent (height below base).
    /// This is a negative number.
    pub fn descent(&self) -> i32 {
        unsafe { ttf::TTF_FontDescent(self.raw) as i32 }
    }

    /// Returns the recommended line spacing for text rendered with this font.
    pub fn recommended_line_spacing(&self) -> i32 {
        unsafe { ttf::TTF_FontLineSkip(self.raw) as i32 }
    }

    /// Returns the number of faces in this font.
    pub fn face_count(&self) -> u16 {
        unsafe { ttf::TTF_FontFaces(self.raw) as u16 }
    }

    /// Returns whether the font is monospaced.
    pub fn face_is_fixed_width(&self) -> bool {
        unsafe { ttf::TTF_FontFaceIsFixedWidth(self.raw) != 0 }
    }

    /// Returns the family name of the current font face.
    pub fn face_family_name(&self) -> Option<String> {
        unsafe {
            // not owns buffer
            let cname = ttf::TTF_FontFaceFamilyName(self.raw);
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
            let cname = ttf::TTF_FontFaceStyleName(self.raw);
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
            let ret = ttf::TTF_GlyphIsProvided(self.raw, ch as u16);
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
            ttf::TTF_GlyphMetrics(
                self.raw,
                ch as u16,
                &minx as *const _ as *mut _,
                &maxx as *const _ as *mut _,
                &miny as *const _ as *mut _,
                &maxy as *const _ as *mut _,
                &advance as *const _ as *mut _,
            )
        };
        if ret == 0 {
            Some(GlyphMetrics {
                minx: minx as i32,
                maxx: maxx as i32,
                miny: miny as i32,
                maxy: maxy as i32,
                advance: advance as i32,
            })
        } else {
            None
        }
    }
}
