#![crate_type = "bin"]

extern crate sdl2;
extern crate sdl2_image;

use std::env;
use std::path::Path;
use sdl2_image::{self, LoadTexture, INIT_PNG, INIT_JPG};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn run(png: &Path) {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2_image::init(INIT_PNG | INIT_JPG).unwrap();
    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
      .position_centered()
      .build()
      .unwrap();

    let mut renderer = window.renderer().software().build().unwrap();
    let texture = renderer.load_texture(png).unwrap();

    renderer.copy(&texture, None, None);
    renderer.present();

    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }
    }
}


fn main() {

    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run /path/to/image.(png|jpg)")
    } else {
        run(Path::new(&args[1]));
    }
}
