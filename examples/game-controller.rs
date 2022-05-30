extern crate sdl2;

fn main() -> Result<(), String> {
    // This is required for certain controllers to work on Windows without the
    // video subsystem enabled:
    sdl2::hint::set("SDL_JOYSTICK_THREAD", "1");

    let sdl_context = sdl2::init()?;
    let game_controller_subsystem = sdl_context.game_controller()?;

    let available = game_controller_subsystem
        .num_joysticks()
        .map_err(|e| format!("can't enumerate joysticks: {}", e))?;

    println!("{} joysticks available", available);

    // Iterate over all available joysticks and look for game controllers.
    let mut controller = (0..available)
        .find_map(|id| {
            if !game_controller_subsystem.is_game_controller(id) {
                println!("{} is not a game controller", id);
                return None;
            }

            println!("Attempting to open controller {}", id);

            match game_controller_subsystem.open(id) {
                Ok(c) => {
                    // We managed to find and open a game controller,
                    // exit the loop
                    println!("Success: opened \"{}\"", c.name());
                    Some(c)
                }
                Err(e) => {
                    println!("failed: {:?}", e);
                    None
                }
            }
        })
        .expect("Couldn't open any controller");

    println!("Controller mapping: {}", controller.mapping());

    let (mut lo_freq, mut hi_freq) = (0, 0);

    for event in sdl_context.event_pump()?.wait_iter() {
        use sdl2::controller::Axis;
        use sdl2::event::Event;

        match event {
            Event::ControllerAxisMotion {
                axis: Axis::TriggerLeft,
                value: val,
                ..
            } => {
                // Trigger axes go from 0 to 32767, so this should be okay
                lo_freq = (val as u16) * 2;
                match controller.set_rumble(lo_freq, hi_freq, 15000) {
                    Ok(()) => println!("Set rumble to ({}, {})", lo_freq, hi_freq),
                    Err(e) => println!(
                        "Error setting rumble to ({}, {}): {:?}",
                        lo_freq, hi_freq, e
                    ),
                }
            }
            Event::ControllerAxisMotion {
                axis: Axis::TriggerRight,
                value: val,
                ..
            } => {
                // Trigger axes go from 0 to 32767, so this should be okay
                hi_freq = (val as u16) * 2;
                match controller.set_rumble(lo_freq, hi_freq, 15000) {
                    Ok(()) => println!("Set rumble to ({}, {})", lo_freq, hi_freq),
                    Err(e) => println!(
                        "Error setting rumble to ({}, {}): {:?}",
                        lo_freq, hi_freq, e
                    ),
                }
            }
            Event::ControllerAxisMotion {
                axis, value: val, ..
            } => {
                // Axis motion is an absolute value in the range
                // [-32768, 32767]. Let's simulate a very rough dead
                // zone to ignore spurious events.
                let dead_zone = 10_000;
                if val > dead_zone || val < -dead_zone {
                    println!("Axis {:?} moved to {}", axis, val);
                }
            }
            Event::ControllerButtonDown { button, .. } => println!("Button {:?} down", button),
            Event::ControllerButtonUp { button, .. } => println!("Button {:?} up", button),
            Event::ControllerTouchpadDown {
                touchpad,
                finger,
                x,
                y,
                ..
            } => println!("Touchpad {touchpad} down finger:{finger} x:{x} y:{y}"),
            Event::ControllerTouchpadMotion {
                touchpad,
                finger,
                x,
                y,
                ..
            } => println!("Touchpad {touchpad} move finger:{finger} x:{x} y:{y}"),
            Event::ControllerTouchpadUp {
                touchpad,
                finger,
                x,
                y,
                ..
            } => println!("Touchpad {touchpad} up   finger:{finger} x:{x} y:{y}"),
            Event::Quit { .. } => break,
            _ => (),
        }
    }

    Ok(())
}
