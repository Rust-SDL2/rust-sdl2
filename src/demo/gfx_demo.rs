#![feature(macro_rules)]

extern crate rand;
extern crate sdl2;
extern crate sdl2_gfx;

use rand::Rand;
use sdl2::{event, pixels};
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


fn mainloop() -> Result<(), ~str> {
    sdl2::init([sdl2::InitVideo]);

    let window = try!(sdl2::video::Window::new(
        "rust-sdl2_gfx: draw line & FPSManager", sdl2::video::PosCentered,
        sdl2::video::PosCentered, SCREEN_WIDTH, SCREEN_HEIGHT,
        [sdl2::video::OpenGL]));

    let renderer = try!(sdl2::render::Renderer::from_window(
            window, sdl2::render::DriverAuto, [sdl2::render::Accelerated]));

    try!(renderer.set_draw_color(pixels::RGB(0, 0, 0)));

    try!(renderer.clear());

    renderer.present();

    let mut rng = rand::StdRng::new().unwrap();
    let (mut lastx, mut lasty) = (0, 0);

    let mut fpsm = FPSManager::new();
    // default 30 is not good
    try!(fpsm.set_framerate(100));

    'main : loop {
        'event : loop {
            // this will avoid program to run 100% CPU
            fpsm.framerate_delay();

            match event::poll_event() {
                event::QuitEvent(_) => break 'main,
                event::KeyDownEvent(_, _, key, _, _) => {
                    if key == sdl2::keycode::EscapeKey {
                        break 'main
                    } else if key == sdl2::keycode::SpaceKey {
                        for i in range(0, 400) {
                            try!(renderer.pixel(i as i16, i as i16, 0xFF000FF));
                        }
                        renderer.present();

                    }
                }
                event::MouseButtonDownEvent(_, _, _, _, x, y) => {
                    let color : pixels::Color = Rand::rand(&mut rng);
                    println!("color => {:?}", color);
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
