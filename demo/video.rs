use std::i16;
use std::rand::RngUtil;
use std::rand;

use sdl2;

pub fn main() {
    sdl2::init([sdl2::InitVideo]);

    let window = match sdl2::video::Window::new("rust-sdl2 demo: Video", 0, 0, 800, 600, [sdl2::video::OpenGL]) {
        Ok(window) => window,
        Err(err) => fail!(fmt!("failed to create window: %s", err))
    };
    sdl2::quit();
}
