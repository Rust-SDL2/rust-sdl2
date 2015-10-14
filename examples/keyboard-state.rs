extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let _window = video_subsystem.window("Keyboard", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut events = sdl_context.event_pump().unwrap();

    let mut prev_keys = HashSet::new();

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                _ => ()
            }
        }

        // Create a set of pressed Keys.
        let keys = events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

        // Get the difference between the new and old sets.
        let new_keys = &keys - &prev_keys;
        let old_keys = &prev_keys - &keys;

        if !new_keys.is_empty() || !old_keys.is_empty() {
            println!("{:?} -> {:?}", new_keys, old_keys);
        }

        prev_keys = keys;

        std::thread::sleep_ms(100);
    }
}
