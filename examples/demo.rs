extern crate sdl2;
extern crate sdl2_ttf;

use std::env;
use std::path::Path;

use sdl2::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::pixels::Color;

static SCREEN_WIDTH : u32 = 800;
static SCREEN_HEIGHT : u32 = 600;

// fail when error
macro_rules! trying(
    ($e:expr) => (match $e { Ok(e) => e, Err(e) => panic!("failed: {}", e) })
);

// hadle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new_unwrap($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (SCREEN_WIDTH as i32 - w) / 2;
    let cy = (SCREEN_HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

fn run(filename: &Path) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    sdl2_ttf::init();

    let window = video_subsys.window("rust-sdl2 demo: Video", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    // Load a font
    let font = trying!(sdl2_ttf::Font::from_file(filename, 128));

    // render a surface, and convert it to a texture bound to the renderer
    let surface = trying!(font.render_str_blended("Hello Rust!", Color::RGBA(255, 0, 0, 255)));
    let mut texture = trying!(renderer.create_texture_from_surface(&surface));

    renderer.set_draw_color(Color::RGBA(195, 217, 255, 255));
    renderer.clear();

    let TextureQuery { width, height, .. } = texture.query();

    // If the example text is too big for the screen, downscale it (and center irregardless)
    let padding = 64;
    let target = get_centered_rect(width, height, SCREEN_WIDTH - padding, SCREEN_HEIGHT - padding);

    renderer.copy(&mut texture, None, Some(target));
    renderer.present();

    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit{..} => break 'mainloop,
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => break 'mainloop,
                _ => {}
            }
        }
    }

    sdl2_ttf::quit();
}

fn main() {
    let args: Vec<_> = env::args().collect();

    println!("linked sdl2_ttf: {}", sdl2_ttf::get_linked_version());

    if args.len() < 2 {
        println!("Usage: ./demo font.[ttf|ttc|fon]")
    } else {
        let path: &Path = Path::new(&args[1]);
        run(path);
    }
}
