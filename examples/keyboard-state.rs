extern crate sdl2;

use sdl2::keycode::KeyCode;
use std::collections::HashSet;

pub fn main() {
    let mut sdl_context = sdl2::init().video().unwrap();

    let _window = sdl_context.window("Keyboard", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut running = true;

    let mut prev_keys = HashSet::new();

    while running {
        for event in sdl_context.event_pump().poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit {..} => running = false,
                _ => ()
            }
        }

        // Create a set of pressed Keys.
        let keys = sdl_context.keyboard_state().pressed_scancodes().filter_map(KeyCode::from_scancode).collect();

        // Get the difference between the new and old sets.
        let new_keys = &keys - &prev_keys;
        let old_keys = &prev_keys - &keys;

        if !new_keys.is_empty() || !old_keys.is_empty() {
            println!("{:?} -> {:?}", new_keys, old_keys);
        }

        prev_keys = keys;

        sdl2::timer::delay(100);
    }
}
