extern crate sdl2;

use sdl2::joystick::{Joystick, num_joysticks};
use sdl2::event::Event;
use std::old_io::timer::sleep;
use std::time::duration::Duration;
use std::num::SignedInt;

fn main() {
    sdl2::init(sdl2::INIT_JOYSTICK);

    let available =
        match num_joysticks() {
            Ok(n)  => n,
            Err(e) => panic!("can't enumerate joysticks: {}", e),
        };

    println!("{} joysticks available", available);

    let mut joystick = None;

    // Iterate over all available joysticks and stop once we manage to
    // open one.
    for id in 0..available {
        match Joystick::open(id) {
            Ok(c) => {
                println!("Success: opened \"{}\"", c.name());
                joystick = Some(c);
                break;
            },
            Err(e) => println!("failed: {:?}", e),
        }
    }

    if joystick.is_none() { 
        panic!("Couldn't open any joystick");
    };

    loop {
        match sdl2::event::poll_event() {
            Event::JoyAxisMotion{ axis_idx, value: val, .. } => {
                // Axis motion is an absolute value in the range
                // [-32768, 32767]. Let's simulate a very rough dead
                // zone to ignore spurious events.
                if val.abs() > 10000 {
                    println!("Axis {} moved to {}", axis_idx, val);
                }
            }
            Event::JoyButtonDown{ button_idx, .. } =>
                println!("Button {} down", button_idx),
            Event::JoyButtonUp{ button_idx, .. } =>
                println!("Button {} up", button_idx),
            Event::JoyHatMotion{ hat_idx, state, .. } =>
                println!("Hat {} moved to {:?}", hat_idx, state),
            Event::Quit{..} => break,
            Event::None =>
                // Don't hog the CPU while waiting for events
                sleep(Duration::milliseconds(100)),
            _ => (),
        }
    }

    sdl2::quit();
}
