extern crate sdl2;

use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::poll_event;
use sdl2::event::Event::{Quit, KeyDown};
use sdl2::keycode::KeyCode;

pub fn main() {
    sdl2::init(sdl2::INIT_VIDEO);

    let window = match Window::new("rust-sdl2 demo: Renderer + Texture", WindowPos::PosCentered, WindowPos::PosCentered, 800, 600, OPENGL) {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    let renderer = match Renderer::from_window(window, RenderDriverIndex::Auto, ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    let mut texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, (256, 256)).unwrap();
    // Create a red-green gradient
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for y in (0us..256) {
            for x in (0us..256) {
                let offset = y*pitch + x*3;
                buffer[offset + 0] = x as u8;
                buffer[offset + 1] = y as u8;
                buffer[offset + 2] = 0;
            }
        }
    }).unwrap();

    let mut drawer = renderer.drawer();
    drawer.clear();
    drawer.copy(&texture, None, Some(Rect::new(100, 100, 256, 256)));
    drawer.copy_ex(&texture, None, Some(Rect::new(450, 100, 256, 256)), 30.0, None, (false, false));
    drawer.present();

    loop {
        match poll_event() {
            Quit{..} => break,
            KeyDown { keycode: key, .. } => {
                if key == KeyCode::Escape {
                    break;
                }
            }
            _ => {}
        }
    }

    sdl2::quit();
}
