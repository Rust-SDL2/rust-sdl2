extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rust-sdl2 resource-manager demo", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().software().build().map_err(|e| e.to_string())?;
    let creator = canvas.texture_creator();
    let mut texture = creator.create_texture_target(PixelFormatEnum::RGBA8888, 400, 300)
        .map_err(|e| e.to_string())?;

    let mut angle = 0.0;

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }
        angle = (angle + 0.5) % 360.;
        canvas.with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.clear();
            texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
            texture_canvas.fill_rect(Rect::new(0, 0, 400, 300)).expect("could not fill rect");
        }).map_err(|e| e.to_string())?;
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        let dst = Some(Rect::new(0, 0, 400, 300));
        canvas.clear();
        canvas.copy_ex(&texture,
            None,
            dst,
            angle,
            Some(Point::new(400, 300)),
            false,
            false
        )?;
        canvas.present();
    }

    Ok(())
}
