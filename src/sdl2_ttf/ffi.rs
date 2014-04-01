extern crate sdl2;

use std::libc::{c_int, c_char, c_long, c_void, uint16_t, uint32_t};
use sdl2::surface::ll::SDL_Surface;
use sdl2::pixels::ll::SDL_Color;
use sdl2::rwops::ll::SDL_RWops;


pub static TTF_STYLE_NORMAL        : c_int = 0x00;
pub static TTF_STYLE_BOLD          : c_int = 0x01;
pub static TTF_STYLE_ITALIC        : c_int = 0x02;
pub static TTF_STYLE_UNDERLINE     : c_int = 0x04;
pub static TTF_STYLE_STRIKETHROUGH : c_int = 0x08;

pub static TTF_HINTING_NORMAL    : c_int = 0;
pub static TTF_HINTING_LIGHT     : c_int = 1;
pub static TTF_HINTING_MONO      : c_int = 2;
pub static TTF_HINTING_NONE      : c_int = 3;

pub struct SDL_version {
    major: int,
    minor: int,
    patch: int,
}

pub type TTF_Font = c_void;
extern "C" {
    pub fn TTF_Linked_Version() -> *SDL_version;
    pub fn TTF_ByteSwappedUNICODE(swapped: c_int);
    pub fn TTF_Init() -> c_int;
    pub fn TTF_OpenFont(file: *c_char, ptsize: c_int) -> *TTF_Font;
    pub fn TTF_OpenFontIndex(file: *c_char, ptsize: c_int, index: c_long) ->
     *TTF_Font;
    pub fn TTF_OpenFontRW(src: *SDL_RWops, freesrc: c_int, ptsize: c_int)
     -> *TTF_Font;
    pub fn TTF_OpenFontIndexRW(src: *SDL_RWops, freesrc: c_int,
                               ptsize: c_int, index: c_long) -> *TTF_Font;
    pub fn TTF_GetFontStyle(font: *TTF_Font) -> c_int;
    pub fn TTF_SetFontStyle(font: *TTF_Font, style: c_int);
    pub fn TTF_GetFontOutline(font: *TTF_Font) -> c_int;
    pub fn TTF_SetFontOutline(font: *TTF_Font, outline: c_int);
    pub fn TTF_GetFontHinting(font: *TTF_Font) -> c_int;
    pub fn TTF_SetFontHinting(font: *TTF_Font, hinting: c_int);
    pub fn TTF_FontHeight(font: *TTF_Font) -> c_int;
    pub fn TTF_FontAscent(font: *TTF_Font) -> c_int;
    pub fn TTF_FontDescent(font: *TTF_Font) -> c_int;
    pub fn TTF_FontLineSkip(font: *TTF_Font) -> c_int;
    pub fn TTF_GetFontKerning(font: *TTF_Font) -> c_int;
    pub fn TTF_SetFontKerning(font: *TTF_Font, allowed: c_int);
    pub fn TTF_FontFaces(font: *TTF_Font) -> c_long;
    pub fn TTF_FontFaceIsFixedWidth(font: *TTF_Font) -> c_int;
    pub fn TTF_FontFaceFamilyName(font: *TTF_Font) -> *c_char;
    pub fn TTF_FontFaceStyleName(font: *TTF_Font) -> *c_char;
    pub fn TTF_GlyphIsProvided(font: *TTF_Font, ch: uint16_t) -> c_int;
    pub fn TTF_GlyphMetrics(font: *TTF_Font, ch: uint16_t, minx: *c_int,
                            maxx: *c_int, miny: *c_int,
                            maxy: *c_int, advance: *c_int) -> c_int;
    pub fn TTF_SizeText(font: *TTF_Font, text: *c_char, w: *c_int,
                        h: *c_int) -> c_int;
    pub fn TTF_SizeUTF8(font: *TTF_Font, text: *c_char, w: *c_int,
                        h: *c_int) -> c_int;
    pub fn TTF_SizeUNICODE(font: *TTF_Font, text: *uint16_t, w: *c_int,
                           h: *c_int) -> c_int;
    pub fn TTF_RenderText_Solid(font: *TTF_Font, text: *c_char,
                                fg: SDL_Color) -> *SDL_Surface;
    pub fn TTF_RenderUTF8_Solid(font: *TTF_Font, text: *c_char,
                                fg: SDL_Color) -> *SDL_Surface;
    pub fn TTF_RenderUNICODE_Solid(font: *TTF_Font, text: *uint16_t,
                                   fg: SDL_Color) -> *SDL_Surface;
    pub fn TTF_RenderGlyph_Solid(font: *TTF_Font, ch: uint16_t,
                                 fg: SDL_Color) -> *SDL_Surface;
    pub fn TTF_RenderText_Shaded(font: *TTF_Font, text: *c_char,
                                 fg: SDL_Color, bg: SDL_Color) ->
     *SDL_Surface;
    pub fn TTF_RenderUTF8_Shaded(font: *TTF_Font, text: *c_char,
                                 fg: SDL_Color, bg: SDL_Color) ->
     *SDL_Surface;
    pub fn TTF_RenderUNICODE_Shaded(font: *TTF_Font, text: *uint16_t,
                                    fg: SDL_Color, bg: SDL_Color) ->
     *SDL_Surface;
    pub fn TTF_RenderGlyph_Shaded(font: *TTF_Font, ch: uint16_t,
                                  fg: SDL_Color, bg: SDL_Color) ->
     *SDL_Surface;
    pub fn TTF_RenderText_Blended(font: *TTF_Font, text: *c_char,
                                  fg: SDL_Color) -> *SDL_Surface;
    pub fn TTF_RenderUTF8_Blended(font: *TTF_Font, text: *c_char,
                                  fg: SDL_Color) -> *SDL_Surface;
    pub fn TTF_RenderUNICODE_Blended(font: *TTF_Font, text: *uint16_t,
                                     fg: SDL_Color) -> *SDL_Surface;
    pub fn TTF_RenderText_Blended_Wrapped(font: *TTF_Font, text: *c_char,
                                          fg: SDL_Color, wrapLength: uint32_t)
     -> *SDL_Surface;
    pub fn TTF_RenderUTF8_Blended_Wrapped(font: *TTF_Font, text: *c_char,
                                          fg: SDL_Color, wrapLength: uint32_t)
     -> *SDL_Surface;
    pub fn TTF_RenderUNICODE_Blended_Wrapped(font: *TTF_Font,
                                             text: *uint16_t, fg: SDL_Color,
                                             wrapLength: uint32_t) ->
     *SDL_Surface;
    pub fn TTF_RenderGlyph_Blended(font: *TTF_Font, ch: uint16_t,
                                   fg: SDL_Color) -> *SDL_Surface;
    pub fn TTF_CloseFont(font: *TTF_Font);
    pub fn TTF_Quit();
    pub fn TTF_WasInit() -> c_int;
    pub fn TTF_GetFontKerningSize(font: *TTF_Font, prev_index: c_int,
                                  index: c_int) -> c_int;
}
