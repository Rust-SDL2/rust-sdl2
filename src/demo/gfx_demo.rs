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

static SCREEN_WIDTH: i32 = 800;
static SCREEN_HEIGHT: i32 = 600;

// hadle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as i32, $h as i32)
    )
);

fn main() {

    let context = sdl2::init(sdl2::INIT_VIDEO).unwrap();

    let window = sdl2::video::Window::new(
        "rust-sdl2_gfx: draw line & FPSManager",
        sdl2::video::WindowPos::PosCentered,
        sdl2::video::WindowPos::PosCentered,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        sdl2::video::OPENGL).unwrap();

    let renderer = sdl2::render::Renderer::from_window(
        window,
        sdl2::render::RenderDriverIndex::Auto,
        sdl2::render::ACCELERATED).unwrap();

    let mut drawer = renderer.drawer();
    drawer.set_draw_color(pixels::Color::RGB(0, 0, 0));
    drawer.clear();
    drawer.present();

    let mut rng = rand::XorShiftRng::new_unseeded();
    let (mut lastx, mut lasty) = (0, 0);

    let mut fpsm = FPSManager::new();

    fpsm.set_framerate(100); // Default 30 is not good.

    let mut events = context.event_pump();

    'main: loop {

        fpsm.delay(); // This will avoid program to run 100% CPU.

        for event in events.poll_iter() {

            match event {

                Event::Quit {..} => break 'main,

                Event::KeyDown {keycode: key, ..} => {
                    if key == KeyCode::Escape {
                        break 'main
                    } else if key == KeyCode::Space {
                        for i in 0..400 {
                            renderer.pixel(i as i16, i as i16, 0xFF000FFu32);
                        }
                        drawer.present();

                    }
                }

                Event::MouseButtonDown {x: x, y: y, ..} => {
                    let color = pixels::Color::RGB(
                        Rand::rand(&mut rng),
                        Rand::rand(&mut rng),
                        Rand::rand(&mut rng));
                    renderer.line(lastx, lasty, x as i16, y as i16, color);
                    lastx = x as i16;
                    lasty = y as i16;
                    println!("mouse btn down at ({},{})", x, y);
                    drawer.present();
                }

                _ => {}
            }
        }
    }
}
