extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use ::sdl2::messagebox::*;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    show_simple_message_box(MessageBoxFlag::ERROR,
                        "Some title",
                        "Some information inside the window",
                        canvas.window()
                    ).map_err(|e| e.to_string())?;
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
    }
    let buttons : Vec<_> = vec![
        ButtonData {
            flags:MessageBoxButtonFlag::RETURNKEY_DEFAULT,
            button_id:1,
            text:"Ok"
        },
        ButtonData {
            flags:MessageBoxButtonFlag::NOTHING,
            button_id:2,
            text:"No"
        },
        ButtonData {
            flags:MessageBoxButtonFlag::ESCAPEKEY_DEFAULT,
            button_id:3,
            text:"Cancel"
        },
    ];
    let res = show_message_box(MessageBoxFlag::WARNING,
                               buttons.as_slice(),
                               "Some warning",
                               "You forget to do something, do it anyway ?",
                               None,
                               None);
    println!("{:?}",res);

    Ok(())
}
