extern crate sdl2;
extern crate sdl2_sys as sdl2_sys;

use libc::{c_int, c_char, c_long, c_void, uint16_t, uint32_t};
use sdl2_sys::surface::SDL_Surface;
use sdl2_sys::pixels::SDL_Color;
use sdl2_sys::rwops::SDL_RWops;
use sdl2_sys::version::SDL_version;


pub const TTF_STYLE_NORMAL        : c_int = 0x00;
pub const TTF_STYLE_BOLD          : c_int = 0x01;
pub const TTF_STYLE_ITALIC        : c_int = 0x02;
pub const TTF_STYLE_UNDERLINE     : c_int = 0x04;
pub const TTF_STYLE_STRIKETHROUGH : c_int = 0x08;

pub const TTF_HINTING_NORMAL    : c_int = 0;
pub const TTF_HINTING_LIGHT     : c_int = 1;
pub const TTF_HINTING_MONO      : c_int = 2;
pub const TTF_HINTING_NONE      : c_int = 3;

pub type TTF_Font = c_void;
extern "C" {
    pub fn TTF_Linked_Version() -> *const SDL_version;
    pub fn TTF_ByteSwappedUNICODE(swapped: c_int);
    pub fn TTF_Init() -> c_int;
    pub fn TTF_OpenFont(file: *const c_char, ptsize: c_int) -> *const TTF_Font;
    pub fn TTF_OpenFontIndex(file: *const c_char, ptsize: c_int, index: c_long) ->
     *const TTF_Font;
    pub fn TTF_OpenFontRW(src: *const SDL_RWops, freesrc: c_int, ptsize: c_int)
     -> *const TTF_Font;
    pub fn TTF_OpenFontIndexRW(src: *const SDL_RWops, freesrc: c_int,
                               ptsize: c_int, index: c_long) -> *const TTF_Font;
    pub fn TTF_GetFontStyle(font: *const TTF_Font) -> c_int;
    pub fn TTF_SetFontStyle(font: *const TTF_Font, style: c_int);
    pub fn TTF_GetFontOutline(font: *const TTF_Font) -> c_int;
    pub fn TTF_SetFontOutline(font: *const TTF_Font, outline: c_int);
    pub fn TTF_GetFontHinting(font: *const TTF_Font) -> c_int;
    pub fn TTF_SetFontHinting(font: *const TTF_Font, hinting: c_int);
    pub fn TTF_FontHeight(font: *const TTF_Font) -> c_int;
    pub fn TTF_FontAscent(font: *const TTF_Font) -> c_int;
    pub fn TTF_FontDescent(font: *const TTF_Font) -> c_int;
    pub fn TTF_FontLineSkip(font: *const TTF_Font) -> c_int;
    pub fn TTF_GetFontKerning(font: *const TTF_Font) -> c_int;
    pub fn TTF_SetFontKerning(font: *const TTF_Font, allowed: c_int);
    pub fn TTF_FontFaces(font: *const TTF_Font) -> c_long;
    pub fn TTF_FontFaceIsFixedWidth(font: *const TTF_Font) -> c_int;
    pub fn TTF_FontFaceFamilyName(font: *const TTF_Font) -> *const c_char;
    pub fn TTF_FontFaceStyleName(font: *const TTF_Font) -> *const c_char;
    pub fn TTF_GlyphIsProvided(font: *const TTF_Font, ch: uint16_t) -> c_int;
    pub fn TTF_GlyphMetrics(font: *const TTF_Font, ch: uint16_t, minx: *const c_int,
                            maxx: *const c_int, miny: *const c_int,
                            maxy: *const c_int, advance: *const c_int) -> c_int;
    pub fn TTF_SizeText(font: *const TTF_Font, text: *const c_char, w: *const c_int,
                        h: *const c_int) -> c_int;
    pub fn TTF_SizeUTF8(font: *const TTF_Font, text: *const c_char, w: *const c_int,
                        h: *const c_int) -> c_int;
    pub fn TTF_SizeUNICODE(font: *const TTF_Font, text: *const uint16_t, w: *const c_int,
                           h: *const c_int) -> c_int;
    pub fn TTF_RenderText_Solid(font: *const TTF_Font, text: *const c_char,
                                fg: SDL_Color) -> *mut SDL_Surface;
    pub fn TTF_RenderUTF8_Solid(font: *const TTF_Font, text: *const c_char,
                                fg: SDL_Color) -> *mut SDL_Surface;
    pub fn TTF_RenderUNICODE_Solid(font: *const TTF_Font, text: *const uint16_t,
                                   fg: SDL_Color) -> *mut SDL_Surface;
    pub fn TTF_RenderGlyph_Solid(font: *const TTF_Font, ch: uint16_t,
                                 fg: SDL_Color) -> *mut SDL_Surface;
    pub fn TTF_RenderText_Shaded(font: *const TTF_Font, text: *const c_char,
                                 fg: SDL_Color, bg: SDL_Color) ->
     *mut SDL_Surface;
    pub fn TTF_RenderUTF8_Shaded(font: *const TTF_Font, text: *const c_char,
                                 fg: SDL_Color, bg: SDL_Color) ->
     *mut SDL_Surface;
    pub fn TTF_RenderUNICODE_Shaded(font: *const TTF_Font, text: *const uint16_t,
                                    fg: SDL_Color, bg: SDL_Color) ->
     *mut SDL_Surface;
    pub fn TTF_RenderGlyph_Shaded(font: *const TTF_Font, ch: uint16_t,
                                  fg: SDL_Color, bg: SDL_Color) ->
     *mut SDL_Surface;
    pub fn TTF_RenderText_Blended(font: *const TTF_Font, text: *const c_char,
                                  fg: SDL_Color) -> *mut SDL_Surface;
    pub fn TTF_RenderUTF8_Blended(font: *const TTF_Font, text: *const c_char,
                                  fg: SDL_Color) -> *mut SDL_Surface;
    pub fn TTF_RenderUNICODE_Blended(font: *const TTF_Font, text: *const uint16_t,
                                     fg: SDL_Color) -> *mut SDL_Surface;
    pub fn TTF_RenderText_Blended_Wrapped(font: *const TTF_Font, text: *const c_char,
                                          fg: SDL_Color, wrapLength: uint32_t)
     -> *mut SDL_Surface;
    pub fn TTF_RenderUTF8_Blended_Wrapped(font: *const TTF_Font, text: *const c_char,
                                          fg: SDL_Color, wrapLength: uint32_t)
     -> *mut SDL_Surface;
    pub fn TTF_RenderUNICODE_Blended_Wrapped(font: *const TTF_Font,
                                             text: *const uint16_t, fg: SDL_Color,
                                             wrapLength: uint32_t) ->
     *const SDL_Surface;
    pub fn TTF_RenderGlyph_Blended(font: *const TTF_Font, ch: uint16_t,
                                   fg: SDL_Color) -> *mut SDL_Surface;
    pub fn TTF_CloseFont(font: *const TTF_Font);
    pub fn TTF_Quit();
    pub fn TTF_WasInit() -> c_int;
    pub fn TTF_GetFontKerningSize(font: *const TTF_Font, prev_index: c_int,
                                  index: c_int) -> c_int;
}
