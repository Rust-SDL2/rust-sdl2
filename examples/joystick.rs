extern crate sdl2;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let joystick_subsystem = sdl_context.joystick().unwrap();

    let available =
        match joystick_subsystem.num_joysticks() {
            Ok(n)  => n,
            Err(e) => panic!("can't enumerate joysticks: {}", e),
        };

    println!("{} joysticks available", available);

    let mut joystick = None;

    // Iterate over all available joysticks and stop once we manage to
    // open one.
    for id in 0..available {
        match joystick_subsystem.open(id) {
            Ok(c) => {
                println!("Success: opened \"{}\"", c.name());
                joystick = Some(c);
                break;
            },
            Err(e) => println!("failed: {:?}", e),
        }
    }

    // Print the joystick's power level, if a joystick was found.
    let mut joystick = match joystick {
        Some(j) => {
            println!("\"{}\" power level: {:?}", j.name(), j.power_level().unwrap());
            j
        },
        None => panic!("Couldn't open any joystick"),
    };

    let (mut lo_freq, mut hi_freq) = (0, 0);

    for event in sdl_context.event_pump().unwrap().wait_iter() {
        use sdl2::event::Event;

        match event {
            Event::JoyAxisMotion{ axis_idx, value: val, .. } => {
                // Axis motion is an absolute value in the range
                // [-32768, 32767]. Let's simulate a very rough dead
                // zone to ignore spurious events.
                let dead_zone = 10_000;
                if val > dead_zone || val < -dead_zone {
                    println!("Axis {} moved to {}", axis_idx, val);
                }
            }
            Event::JoyButtonDown{ button_idx, .. } => {
                println!("Button {} down", button_idx);
                if button_idx == 0 {
                    lo_freq = 65535;
                } else if button_idx == 1 {
                    hi_freq = 65535;
                }
                if button_idx < 2 {
                    match joystick.set_rumble(lo_freq, hi_freq, 15000) {
                        Ok(()) => println!("Set rumble to ({}, {})", lo_freq, hi_freq),
                        Err(e) => println!("Error setting rumble to ({}, {}): {:?}", lo_freq, hi_freq, e),
                    }
                }
            }
            Event::JoyButtonUp{ button_idx, .. } => {
                println!("Button {} up", button_idx);
                if button_idx == 0 {
                    lo_freq = 0;
                } else if button_idx == 1 {
                    hi_freq = 0;
                }
                if button_idx < 2 {
                    match joystick.set_rumble(lo_freq, hi_freq, 15000) {
                        Ok(()) => println!("Set rumble to ({}, {})", lo_freq, hi_freq),
                        Err(e) => println!("Error setting rumble to ({}, {}): {:?}", lo_freq, hi_freq, e),
                    }
                }
            }
            Event::JoyHatMotion{ hat_idx, state, .. } =>
                println!("Hat {} moved to {:?}", hat_idx, state),
            Event::Quit{..} => break,
            _ => (),
        }
    }
}
