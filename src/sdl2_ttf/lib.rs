#![crate_id="sdl2_ttf#sdl2_ttf:0.1"]
#![crate_type = "lib"]
#![desc = "SDL2_ttf bindings and wrappers"]
#![comment = "SDL2_ttf bindings and wrappers"]
#![license = "MIT"]


extern crate sdl2;

use std::libc::{c_int, c_long};
use std::ptr;
use std::c_str::CString;
use std::num::FromPrimitive;
use sdl2::surface::Surface;
use sdl2::get_error;
use sdl2::pixels;
use sdl2::pixels::Color;
use sdl2::pixels::ll::SDL_Color;

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

#[cfg(target_os="win32")]
#[cfg(target_os="linux")]
#[cfg(target_os="freebsd")]
mod others {
    #[link(name="SDL2_ttf")]
    extern {}
}

#[allow(non_camel_case_types, dead_code)]
mod ffi;

#[deriving(Show)]
pub enum Style {
    StyleNormal = ffi::TTF_STYLE_NORMAL as int,
    StyleBold   = ffi::TTF_STYLE_BOLD   as int,
    StyleItalic = ffi::TTF_STYLE_ITALIC as int,
    StyleUnderline = ffi::TTF_STYLE_UNDERLINE as int,
    StyleStrikeThrough = ffi::TTF_STYLE_STRIKETHROUGH as int
}

#[deriving(Show, Eq, FromPrimitive)]
pub enum Hinting {
    HintingNormal = ffi::TTF_HINTING_NORMAL as int,
    HintingLight  = ffi::TTF_HINTING_LIGHT  as int,
    HintingMono   = ffi::TTF_HINTING_MONO   as int,
    HintingNone   = ffi::TTF_HINTING_NONE   as int
}

#[deriving(Eq, Show)]
pub struct GlyphMetrics {
    minx: int,
    maxx: int,
    miny: int,
    maxy: int,
    advance: int
}

#[inline]
fn color_to_c_color(color: Color) -> SDL_Color {
    match color {
        pixels::RGB(r, g, b)     => SDL_Color { r: r, g: g, b: b, a: 255 },
        pixels::RGBA(r, g, b, a) => SDL_Color { r: r, g: g, b: b, a: a   }
    }
}

pub fn init() -> bool {
    unsafe {
        if ffi::TTF_WasInit() == 1 {
            true
        } else {
            ffi::TTF_Init() == 0
        }
    }
}

pub fn was_inited() -> bool {
    unsafe {
        ffi::TTF_WasInit() == 1
    }
}


pub fn quit() {
    unsafe { ffi::TTF_Quit(); }
}

#[allow(raw_pointer_deriving)]
#[deriving(Eq)]
pub struct Font {
    raw: *ffi::TTF_Font,
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

fn wrap_font_styles(bitflags: u32) -> Vec<Style> {
    let styles = [StyleBold, StyleItalic, StyleUnderline, StyleStrikeThrough];
    styles.iter().filter_map(|&flag| {
        if bitflags & (flag as u32) != 0 { Some(flag) }
        else { None }
    }).collect()
}

impl Font {
    pub fn from_file(filename: &Path, ptsize: int) -> Result<~Font, ~str> {
        unsafe {
            let raw = ffi::TTF_OpenFont(filename.to_c_str().unwrap(), ptsize as c_int);
            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Font { raw: raw, owned: true })
            }
        }
    }

    pub fn from_file_index(filename: &Path, ptsize: int, index: int) -> Result<~Font, ~str> {
        unsafe {
            let raw = ffi::TTF_OpenFontIndex(filename.to_c_str().unwrap(), ptsize as c_int, index as c_long);
            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Font { raw: raw, owned: true })
            }
        }
    }

    pub fn get_style(&self) -> Vec<Style> {
        let raw = unsafe { ffi::TTF_GetFontStyle(self.raw) };
        wrap_font_styles(raw as u32)
    }

    pub fn set_style(&self, styles: &[Style]) {
        let flags = styles.iter().fold(0i32, |flags, flag| { flags | *flag as i32 });
        unsafe {
            ffi::TTF_SetFontStyle(self.raw, flags)
        }
    }

    pub fn get_outline(&self) -> int {
        unsafe {
            ffi::TTF_GetFontOutline(self.raw) as int
        }
    }

    pub fn set_outline(&self, outline: int) {
        unsafe {
            ffi::TTF_SetFontOutline(self.raw, outline as c_int)
        }
    }

    pub fn get_hinting(&self) -> Hinting {
        unsafe {
            FromPrimitive::from_i32(ffi::TTF_GetFontHinting(self.raw)).unwrap()
        }
    }

    pub fn set_hinting(&self, hinting: Hinting) {
        unsafe {
            ffi::TTF_SetFontHinting(self.raw, hinting as c_int)
        }
    }

    pub fn get_kerning(&self) -> bool {
        unsafe {
            ffi::TTF_GetFontKerning(self.raw) != 0
        }
    }

    pub fn set_kerning(&self, kerning: bool) {
        unsafe {
            ffi::TTF_SetFontKerning(self.raw, kerning as c_int)
        }
    }

    pub fn height(&self) -> int {
        unsafe {
            ffi::TTF_FontHeight(self.raw) as int
        }
    }

    pub fn ascent(&self) -> int {
        unsafe {
            ffi::TTF_FontAscent(self.raw) as int
        }
    }

    pub fn descent(&self) -> int {
        unsafe {
            ffi::TTF_FontDescent(self.raw) as int
        }
    }

    pub fn line_skip(&self) -> int {
        unsafe {
            ffi::TTF_FontLineSkip(self.raw) as int
        }
    }

    pub fn faces(&self) -> int {
        unsafe {
            ffi::TTF_FontFaces(self.raw) as int
        }
    }

    pub fn face_is_fixed_width(&self) -> bool {
        unsafe {
            ffi::TTF_FontFaceIsFixedWidth(self.raw) != 0
        }
    }

    pub fn face_family_name(&self) -> Option<~str> {
        unsafe {
            // not owns buffer
            let cname = ffi::TTF_FontFaceFamilyName(self.raw);
            if cname == ptr::null() {
                None
            } else {
                Some(CString::new(cname, false).as_str().unwrap().into_owned())
            }
        }
    }

    pub fn face_style_name(&self) -> Option<~str> {
        unsafe {
            let cname = ffi::TTF_FontFaceStyleName(self.raw);
            if cname == ptr::null() {
                None
            } else {
                Some(CString::new(cname, false).as_str().unwrap().into_owned())
            }
        }
    }

    // FIXME: bug
    pub fn glyph_is_provided(&self, ch: char) -> Option<uint> {
        unsafe {
            let ret = ffi::TTF_GlyphIsProvided(self.raw, ch as u16);
            if ret == 0 {
                None
            } else {
                Some(ret as uint)
            }
        }
    }

    pub fn glyph_metrics(&self, ch: char) -> Option<GlyphMetrics> {
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
            Some(GlyphMetrics { minx: minx as int, maxx: maxx as int,
                                miny: miny as int, maxy: maxy as int,
                                advance: advance as int })
        }
    }

    pub fn size_bytes(&self, text: &[u8]) -> Result<(int, int), ~str> {
        let w = 0;
        let h = 0;
        let ret = unsafe {
            text.with_c_str(|ctext| {
                    ffi::TTF_SizeText(self.raw, ctext, &w, &h)
                })
        };
        if ret != 0 {
            Err(get_error())
        } else {
            Ok((w as int, h as int))
        }
    }

    pub fn size_str(&self, text: &str) -> Result<(int, int), ~str> {
        let w = 0;
        let h = 0;
        let ret = unsafe {
            text.with_c_str(|ctext| {
                    ffi::TTF_SizeUTF8(self.raw, ctext, &w, &h)
                })
        };
        if ret != 0 {
            Err(get_error())
        } else {
            Ok((w as int, h as int))
        }
    }

    // FIXME: represent UNICODE str in Rust
    pub fn size_unicode(&self, _text: &[u16]) -> Result<(int, int), ~str> {
        unimplemented!()
        // let w = 0;
        // let h = 0;
        // let ret = unsafe {
        //     ffi::TTF_SizeUNICODE(self.raw, cast::transmute(&ctext[0]), &w, &h)
        // };
        // if ret != 0 {
        //     Err(get_error())
        // } else {
        //     Ok((w as int, h as int))
        // }
    }

    pub fn render_bytes_solid(&self, text: &[u8], fg: Color) -> Result<~Surface, ~str> {
        unsafe {
            let raw = text.with_c_str(|ctext| {
                    ffi::TTF_RenderText_Solid(self.raw, ctext, color_to_c_color(fg))
                });
            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }

    pub fn render_str_solid(&self, text: &str, fg: Color) -> Result<~Surface, ~str> {
        unsafe {
            let raw = text.with_c_str(|ctext| {
                    ffi::TTF_RenderUTF8_Solid(self.raw, ctext, color_to_c_color(fg))
                });
            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }

    pub fn render_char_solid(&self, ch: char, fg: Color) -> Result<~Surface, ~str> {
        unsafe {
            let raw = ffi::TTF_RenderGlyph_Solid(self.raw, ch as u16, color_to_c_color(fg));
            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }

        pub fn render_bytes_shaded(&self, text: &[u8], fg: Color, bg: Color) -> Result<~Surface, ~str> {
        unsafe {
            let raw = text.with_c_str(|ctext| {
                    ffi::TTF_RenderText_Shaded(self.raw, ctext, color_to_c_color(fg), color_to_c_color(bg))
                });
            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }

    pub fn render_str_shaded(&self, text: &str, fg: Color, bg: Color) -> Result<~Surface, ~str> {
        unsafe {
            let raw = text.with_c_str(|ctext| {
                    ffi::TTF_RenderUTF8_Shaded(self.raw, ctext, color_to_c_color(fg), color_to_c_color(bg))
                });
            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }

    pub fn render_char_shaded(&self, ch: char, fg: Color, bg: Color) -> Result<~Surface, ~str> {
        unsafe {
            let raw = ffi::TTF_RenderGlyph_Shaded(self.raw, ch as u16, color_to_c_color(fg), color_to_c_color(bg));
            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }

    pub fn render_bytes_blended(&self, text: &[u8], fg: Color) -> Result<~Surface, ~str> {
        unsafe {
            let raw = text.with_c_str(|ctext| {
                    ffi::TTF_RenderText_Blended(self.raw, ctext, color_to_c_color(fg))
                });
            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }

    pub fn render_str_blended(&self, text: &str, fg: Color) -> Result<~Surface, ~str> {
        unsafe {
            let raw = text.with_c_str(|ctext| {
                    ffi::TTF_RenderUTF8_Blended(self.raw, ctext, color_to_c_color(fg))
                });
            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }

    pub fn render_char_blended(&self, ch: char, fg: Color) -> Result<~Surface, ~str> {
        unsafe {
            let raw = ffi::TTF_RenderGlyph_Blended(self.raw, ch as u16, color_to_c_color(fg));
            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }


}
