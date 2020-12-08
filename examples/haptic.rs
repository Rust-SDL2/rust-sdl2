extern crate sdl2;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let joystick_subsystem = sdl_context.joystick()?;
    let haptic_subsystem = sdl_context.haptic()?;

    let available = joystick_subsystem
        .num_joysticks()
        .map_err(|e| format!("can't enumerate joysticks: {}", e))?;

    println!("{} joysticks available", available);

    // Iterate over all available joysticks and stop once we manage to open one.
    let joystick_index = (0..available)
        .find_map(|id| match joystick_subsystem.open(id) {
            Ok(c) => {
                println!("Success: opened \"{}\"", c.name());
                Some(id)
            }
            Err(e) => {
                println!("failed: {:?}", e);
                None
            }
        })
        .expect("Couldn't open any joystick");

    let mut haptic = haptic_subsystem
        .open_from_joystick_id(joystick_index)
        .map_err(|e| e.to_string())?;

    for event in sdl_context.event_pump()?.wait_iter() {
        use sdl2::event::Event;

        match event {
            Event::JoyAxisMotion {
                axis_idx,
                value: val,
                ..
            } => {
                // Axis motion is an absolute value in the range
                // [-32768, 32767]. Let's simulate a very rough dead
                // zone to ignore spurious events.
                let dead_zone = 10_000;
                if val > dead_zone || val < -dead_zone {
                    println!("Axis {} moved to {}", axis_idx, val);
                }
            }
            Event::JoyButtonDown { button_idx, .. } => {
                println!("Button {} down", button_idx);
                haptic.rumble_play(0.5, 500);
            }
            Event::JoyButtonUp { button_idx, .. } => println!("Button {} up", button_idx),
            Event::JoyHatMotion { hat_idx, state, .. } => {
                println!("Hat {} moved to {:?}", hat_idx, state)
            }
            Event::Quit { .. } => break,
            _ => (),
        }
    }

    Ok(())
}
