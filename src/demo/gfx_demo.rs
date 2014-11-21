#![feature(macro_rules)]

extern crate rand;
extern crate sdl2;
extern crate sdl2_gfx;

use rand::Rand;
use sdl2::event;
use sdl2::event::Event;
use sdl2::pixels;
use sdl2::keycode::KeyCode;


use sdl2_gfx::primitives::DrawRenderer;
use sdl2_gfx::framerate::FPSManager;

static SCREEN_WIDTH : int = 800;
static SCREEN_HEIGHT : int = 600;

// hadle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as i32, $h as i32)
    )
)

fn main() {
    match mainloop() {
        Ok(_) => (),
        Err(e) => println!("error while running mainloop: {}", e),
    }
}


fn mainloop() -> Result<(), String> {
    sdl2::init(sdl2::INIT_VIDEO);

    let window = try!(sdl2::video::Window::new(
        "rust-sdl2_gfx: draw line & FPSManager", sdl2::video::PosCentered,
        sdl2::video::PosCentered, SCREEN_WIDTH, SCREEN_HEIGHT,
        sdl2::video::OPENGL));

    let renderer = try!(sdl2::render::Renderer::from_window(
            window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED));

    try!(renderer.set_draw_color(pixels::RGB(0, 0, 0)));

    try!(renderer.clear());

    renderer.present();

    let mut rng = rand::XorShiftRng::new_unseeded();
    let (mut lastx, mut lasty) = (0, 0);

    let mut fpsm = FPSManager::new();
    // default 30 is not good
    try!(fpsm.set_framerate(100));

    'main : loop {
        'event : loop {
            // this will avoid program to run 100% CPU
            fpsm.delay();

            match event::poll_event() {
                Event::Quit(_) => break 'main,
                Event::KeyDown(_, _, key, _, _, _) => {
                    if key == KeyCode::Escape {
                        break 'main
                    } else if key == KeyCode::Space {
                        for i in range(0u, 400) {
                            try!(renderer.pixel(i as i16, i as i16, 0xFF000FFu32));
                        }
                        renderer.present();

                    }
                }
                Event::MouseButtonDown(_, _, _, _, x, y) => {
                    let color : pixels::Color = Rand::rand(&mut rng);
                    // println!("color => {:}", color);
                    try!(renderer.line(lastx, lasty, x as i16, y as i16, color));
                    lastx = x as i16;
                    lasty = y as i16;
                    println!("mouse btn down at ({},{})", x, y);
                    renderer.present();
                }

                _ => {}
            }
        }
    }
    sdl2::quit();
    Ok(())
}
