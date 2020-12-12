#![cfg(test)]

use super::super::controller::{Axis, Button};
use super::super::joystick::HatState;
use super::super::keyboard::{Keycode, Mod, Scancode};
use super::super::mouse::{MouseButton, MouseState, MouseWheelDirection};
use super::Event;
use super::WindowEvent;

// Tests a round-trip conversion from an Event type to
// the SDL event type and back, to make sure it's sane.
#[test]
fn test_to_from_ll() {
    {
        let e = Event::Quit { timestamp: 0 };
        let e2 = Event::from_ll(e.clone().to_ll().unwrap());
        assert_eq!(e, e2);
    }
    {
        let e = Event::Window {
            timestamp: 0,
            window_id: 0,
            win_event: WindowEvent::Resized(1, 2),
        };
        let e2 = Event::from_ll(e.clone().to_ll().unwrap());
        assert_eq!(e, e2);
    }
    {
        let e = Event::KeyDown {
            timestamp: 0,
            window_id: 1,
            keycode: None,
            scancode: Some(Scancode::Q),
            keymod: Mod::all(),
            repeat: false,
        };
        let e2 = Event::from_ll(e.clone().to_ll().unwrap());
        assert_eq!(e, e2);
    }
    {
        let e = Event::KeyUp {
            timestamp: 123,
            window_id: 0,
            keycode: Some(Keycode::R),
            scancode: Some(Scancode::R),
            keymod: Mod::empty(),
            repeat: true,
        };
        let e2 = Event::from_ll(e.clone().to_ll().unwrap());
        assert_eq!(e, e2);
    }
    {
        let e = Event::MouseMotion {
            timestamp: 0,
            window_id: 0,
            which: 1,
            mousestate: MouseState::from_sdl_state(1),
            x: 3,
            y: 91,
            xrel: -1,
            yrel: 43,
        };
        let e2 = Event::from_ll(e.clone().to_ll().unwrap());
        assert_eq!(e, e2);
    }
    {
        let e = Event::MouseButtonDown {
            timestamp: 5634,
            window_id: 2,
            which: 0,
            mouse_btn: MouseButton::Left,
            clicks: 1,
            x: 543,
            y: 345,
        };
        let e2 = Event::from_ll(e.clone().to_ll().unwrap());
        assert_eq!(e, e2);
    }
    {
        let e = Event::MouseButtonUp {
            timestamp: 0,
            window_id: 2,
            which: 0,
            mouse_btn: MouseButton::Left,
            clicks: 1,
            x: 543,
            y: 345,
        };
        let e2 = Event::from_ll(e.clone().to_ll().unwrap());
        assert_eq!(e, e2);
    }
    {
        let e = Event::MouseWheel {
            timestamp: 1,
            window_id: 0,
            which: 32,
            x: 23,
            y: 91,
            direction: MouseWheelDirection::Flipped,
        };
        let e2 = Event::from_ll(e.clone().to_ll().unwrap());
        assert_eq!(e, e2);
    }
    {
        let e = Event::JoyAxisMotion {
            timestamp: 0,
            which: 1,
            axis_idx: 1,
            value: 12,
        };
        let e2 = Event::from_ll(e.clone().to_ll().unwrap());
        assert_eq!(e, e2);
    }
    {
        let e = Event::JoyBallMotion {
            timestamp: 0,
            which: 0,
            ball_idx: 1,
            xrel: 123,
            yrel: 321,
        };
        let e2 = Event::from_ll(e.clone().to_ll().unwrap());
        assert_eq!(e, e2);
    }
    {
        let e = Event::JoyHatMotion {
            timestamp: 0,
            which: 3,
            hat_idx: 1,
            state: HatState::Left,
        };
        let e2 = Event::from_ll(e.clone().to_ll().unwrap());
        assert_eq!(e, e2);
    }
    {
        let e = Event::JoyButtonDown {
            timestamp: 0,
            which: 0,
            button_idx: 3,
        };
        let e2 = Event::from_ll(e.clone().to_ll().unwrap());
        assert_eq!(e, e2);
    }
    {
        let e = Event::JoyButtonUp {
            timestamp: 9876,
            which: 1,
            button_idx: 2,
        };
        let e2 = Event::from_ll(e.clone().to_ll().unwrap());
        assert_eq!(e, e2);
    }
    {
        let e = Event::JoyDeviceAdded {
            timestamp: 0,
            which: 1,
        };
        let e2 = Event::from_ll(e.clone().to_ll().unwrap());
        assert_eq!(e, e2);
    }
    {
        let e = Event::JoyDeviceRemoved {
            timestamp: 0,
            which: 2,
        };
        let e2 = Event::from_ll(e.clone().to_ll().unwrap());
        assert_eq!(e, e2);
    }
    {
        let e = Event::ControllerAxisMotion {
            timestamp: 53,
            which: 0,
            axis: Axis::LeftX,
            value: 3,
        };
        let e2 = Event::from_ll(e.clone().to_ll().unwrap());
        assert_eq!(e, e2);
    }
    {
        let e = Event::ControllerButtonDown {
            timestamp: 0,
            which: 1,
            button: Button::Guide,
        };
        let e2 = Event::from_ll(e.clone().to_ll().unwrap());
        assert_eq!(e, e2);
    }
    {
        let e = Event::ControllerButtonUp {
            timestamp: 654214,
            which: 0,
            button: Button::DPadRight,
        };
        let e2 = Event::from_ll(e.clone().to_ll().unwrap());
        assert_eq!(e, e2);
    }
    {
        let e = Event::ControllerDeviceAdded {
            timestamp: 543,
            which: 3,
        };
        let e2 = Event::from_ll(e.clone().to_ll().unwrap());
        assert_eq!(e, e2);
    }
    {
        let e = Event::ControllerDeviceRemoved {
            timestamp: 555,
            which: 3,
        };
        let e2 = Event::from_ll(e.clone().to_ll().unwrap());
        assert_eq!(e, e2);
    }
    {
        let e = Event::ControllerDeviceRemapped {
            timestamp: 654,
            which: 0,
        };
        let e2 = Event::from_ll(e.clone().to_ll().unwrap());
        assert_eq!(e, e2);
    }
}

#[test]
fn test_from_ll_keymod_keydown_unknown_bits() {
    let mut raw_event = Event::KeyDown {
        timestamp: 0,
        window_id: 1,
        keycode: None,
        scancode: Some(Scancode::Q),
        keymod: Mod::empty(),
        repeat: false,
    }
    .to_ll()
    .unwrap();

    // Simulate SDL setting bits unknown to us, see PR #780
    raw_event.key.keysym.mod_ = 0xffff;

    if let Event::KeyDown { keymod, .. } = Event::from_ll(raw_event) {
        assert_eq!(keymod, Mod::all());
    } else {
        panic!()
    }
}

#[test]
fn test_from_ll_keymod_keyup_unknown_bits() {
    let mut raw_event = Event::KeyUp {
        timestamp: 0,
        window_id: 1,
        keycode: None,
        scancode: Some(Scancode::Q),
        keymod: Mod::empty(),
        repeat: false,
    }
    .to_ll()
    .unwrap();

    // Simulate SDL setting bits unknown to us, see PR #780
    raw_event.key.keysym.mod_ = 0xffff;

    if let Event::KeyUp { keymod, .. } = Event::from_ll(raw_event) {
        assert_eq!(keymod, Mod::all());
    } else {
        panic!()
    }
}
