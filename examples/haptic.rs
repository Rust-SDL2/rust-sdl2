extern crate sdl2;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let joystick_subsystem = sdl_context.joystick().unwrap();
    let haptic_subsystem = sdl_context.haptic().unwrap();

    let available =
        match joystick_subsystem.num_joysticks() {
            Ok(n)  => n,
            Err(e) => panic!("can't enumerate joysticks: {}", e),
        };

    println!("{} joysticks available", available);

    let mut joystick_index = None;

    // Iterate over all available joysticks and stop once we manage to
    // open one.
    for id in 0..available {
        match joystick_subsystem.open(id) {
            Ok(c) => {
                println!("Success: opened \"{}\"", c.name());
                joystick_index = Some(id);
                break;
            },
            Err(e) => println!("failed: {:?}", e),
        }
    }

    if joystick_index.is_none() {
        panic!("Couldn't open any joystick");
    };

    let mut haptic = haptic_subsystem.open_from_joystick_id(joystick_index.unwrap()).unwrap();

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
            Event::JoyButtonDown{ button_idx, .. } =>{
                println!("Button {} down", button_idx);
                haptic.rumble_play(0.5, 500);
            },
            Event::JoyButtonUp{ button_idx, .. } =>
                println!("Button {} up", button_idx),
            Event::JoyHatMotion{ hat_idx, state, .. } =>
                println!("Hat {} moved to {:?}", hat_idx, state),
            Event::Quit{..} => break,
            _ => (),
        }
    }
}
