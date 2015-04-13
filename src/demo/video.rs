use std::path::Path;
use sdl2::{self, INIT_VIDEO};
use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::render::{Renderer, RenderDriverIndex, ACCELERATED};
use sdl2_image::{self, LoadTexture, INIT_PNG, INIT_JPG};
use sdl2::event::Event;
use sdl2::keycode::KeyCode;

pub fn main(png: &Path) {

    let context = sdl2::init(INIT_VIDEO).unwrap();
    sdl2_image::init(INIT_PNG | INIT_JPG);

    let window = Window::new(
          &context,
          "rust-sdl2 demo: Video",
          WindowPos::PosCentered,
          WindowPos::PosCentered,
          800,
          600,
          OPENGL).unwrap();

    let mut renderer = Renderer::from_window(
          window,
          RenderDriverIndex::Auto,
          ACCELERATED).unwrap();

    let mut texture = renderer.load_texture(png).unwrap();

    // Draws and shows the loaded texture.
    let mut drawer = renderer.drawer();
    drawer.copy(&mut texture, None, None);
    drawer.present();

    let mut event_pump = context.event_pump();
    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: KeyCode::Escape, ..} =>
                    break 'mainloop,
                _ => {}
            }
        }
    }

    sdl2_image::quit();
}
