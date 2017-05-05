extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;
use sdl2::rect::Rect;
use std::time::Duration;

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 600;

#[derive(Clone, Copy)]
enum Gradient {
    Red,
    Cyan,
    Green,
    Blue,
    White
}

fn set_window_gradient(window: &mut Window, event_pump: &sdl2::EventPump, gradient: Gradient) {
    let mut surface = window.surface(event_pump).unwrap();
    for i in 0 .. (WINDOW_WIDTH / 4) {
        let c : u8 = 255 - (i as u8);
        let i = i as i32;
        let color = match gradient {
            Gradient::Red => Color::RGB(c, 0, 0),
            Gradient::Cyan => Color::RGB(0, c, c),
            Gradient::Green => Color::RGB(0, c, 0),
            Gradient::Blue => Color::RGB(0, 0, c),
            Gradient::White => Color::RGB(c, c, c),
        };
        surface.fill_rect(Rect::new(i*4, 0, 4, WINDOW_HEIGHT), color).unwrap();
    }
    surface.finish().unwrap();
}

fn next_gradient(gradient: Gradient) -> Gradient {
    use Gradient::*;
    match gradient {
        Red => Cyan,
        Cyan => Green,
        Green => Blue,
        Blue => White,
        White => Red,
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let mut window = video_subsystem.window("rust-sdl2 demo: No Renderer", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut current_gradient = Gradient::Red;
    set_window_gradient(&mut window, &event_pump, current_gradient);
    'running: loop {
        let mut keypress : bool = false;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { repeat: false, .. } => {
                    keypress = true;
                },
                _ => {}
            }
        }
        if keypress {
            current_gradient = next_gradient(current_gradient);
            set_window_gradient(&mut window, &event_pump, current_gradient);
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
