use std::rand;
use std::rand::RngUtil;

pub mod ll {
    use std::libc::{c_int, uint8_t, uint32_t};

    //SDL_pixels.h
    pub struct SDL_Color {
        r: uint8_t,
        g: uint8_t,
        b: uint8_t,
        a: uint8_t,
    }

    pub struct SDL_Pallette {
        ncolors: c_int,
        colors: *SDL_Color,
        version: uint32_t,
        refcount: c_int
    }

    pub struct SDL_PixelFormat {
        format: uint32_t,
        palette: *SDL_Pallette,
        BitsPerPixel: uint8_t,
        BytesPerPixel: uint8_t,
        padding: [uint8_t, ..2],
        Rmask: uint8_t,
        Gmask: uint8_t,
        Bmask: uint8_t,
        Amask: uint8_t,
        Rloss: uint8_t,
        Gloss: uint8_t,
        Bloss: uint8_t,
        Aloss: uint8_t,
        Rshift: uint8_t,
        Gshift: uint8_t,
        Bshift: uint8_t,
        Ashift: uint8_t,
        refcount: c_int,
        next: *SDL_PixelFormat
    }
}

#[deriving(Eq)]
pub enum Color {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8)
}

impl rand::Rand for Color {
    fn rand<R: rand::Rng>(rng: &mut R) -> Color {
        if rng.gen() { RGBA(rng.gen(), rng.gen(), rng.gen(), rng.gen()) }
        else { RGB(rng.gen(), rng.gen(), rng.gen()) }
    }
}
