extern crate sdl2;

use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::pixels::Color;
use sdl2::event::poll_event;
use sdl2::event::Event::{Quit, KeyDown};
use sdl2::keycode::KeyCode;

pub fn main() {
    sdl2::init(sdl2::INIT_VIDEO);

    let window = match Window::new("rust-sdl2 demo: Video", WindowPos::PosCentered, WindowPos::PosCentered, 800, 600, OPENGL) {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    let renderer = match Renderer::from_window(window, RenderDriverIndex::Auto, ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    let _ = renderer.set_draw_color(Color::RGB(255, 0, 0));
    let _ = renderer.clear();
    renderer.present();

    loop {
        match poll_event() {
            Quit(_) => break,
            KeyDown(_, _, key, _, _, _) => {
                if key == KeyCode::Escape {
                    break;
                }
            }
            _ => {}
        }
    }

    sdl2::quit();
}

