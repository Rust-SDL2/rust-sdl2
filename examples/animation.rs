extern crate sdl2;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("SDL2", 640, 480)
        .position_centered().build().unwrap();

    let mut canvas = window.into_canvas()
        .accelerated().build().unwrap();
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));

    let mut timer = sdl_context.timer().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/animate.bmp")).unwrap();
    let texture = texture_creator.create_texture_from_surface(&temp_surface).unwrap();
    
    let center = Point::new(320,240);
    let mut source_rect = Rect::new(0, 0, 128, 82);
    let mut dest_rect = Rect::new(0,0, 128, 82);
    dest_rect.center_on(center);

    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                },
                _ => {}
            }
        }

        let ticks = timer.ticks();

        source_rect.set_x((128 * ((ticks / 100) % 6) ) as i32);
        canvas.clear();
        canvas.copy_ex(&texture, Some(source_rect), Some(dest_rect), 10.0, None, true, false).unwrap();
        canvas.present();

        std::thread::sleep(Duration::from_millis(100));
    }
}
