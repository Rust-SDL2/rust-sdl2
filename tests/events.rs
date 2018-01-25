extern crate sdl2;

use sdl2::event;

#[test]
fn test_events() {
    let sdl = sdl2::init().unwrap();
    let ev = sdl.event().unwrap();
    let mut ep = sdl.event_pump().unwrap();

    test1(&ev);
    test2(&ev, &mut ep);

    test3(&ev);
    test4(&ev, &mut ep);
}

fn test1(ev: &sdl2::EventSubsystem) {
    let user_event1_id = unsafe { ev.register_event().unwrap() };
    let user_event2_id = unsafe { ev.register_event().unwrap() };
    assert_ne!(user_event1_id, user_event2_id);
}

fn test2(ev: &sdl2::EventSubsystem, ep: &mut sdl2::EventPump) {
    let user_event_id = unsafe { ev.register_event().unwrap() };

    let event = event::Event::User {
        timestamp: 0,
        window_id: 0,
        type_: user_event_id,
        code: 456,
        data1: 0x12_34 as *mut libc::c_void,
        data2: 0x56_78 as *mut libc::c_void,
    };

    let (w1, t1, c1, a1, a2) = match event {
        event::Event::User {
            window_id: w1,
            type_: t1,
            code: c1,
            data1: a1,
            data2: a2,
            ..
        } => (w1, t1, c1, a1, a2),
        _ => panic!("expected user event"),
    };
    ev.push_event(event.clone()).unwrap();
    let received = ep.poll_event().unwrap();
    // Do not check for timestamp here because it is always modified by
    // SDL_PushEvent.
    match &received {
        &event::Event::User {
            window_id: w2,
            type_: t2,
            code: c2,
            data1: b1,
            data2: b2,
            ..
        } => {
            assert_eq!(w1, w2);
            assert_eq!(t1, t2);
            assert_eq!(c1, c2);
            assert_eq!(a1, b1);
            assert_eq!(a2, b2);
        }
        other => panic!("Received non User event: {:?}", other),
    }
}

#[allow(unused)]
struct SomeEventTypeTest3 {
    a: u32
}

#[allow(unused)]
struct SomeOtherEventTypeTest3 {
    b: u32
}

fn test3(ev: &sdl2::EventSubsystem) {
    ev.register_custom_event::<SomeEventTypeTest3>().unwrap();
    ev.register_custom_event::<SomeOtherEventTypeTest3>().unwrap();

    assert!(ev.register_custom_event::<SomeEventTypeTest3>().is_err());
}

struct SomeEventTypeTest4 {
    a: u32
}

fn test4(ev: &sdl2::EventSubsystem, ep: &mut sdl2::EventPump) {
    ev.register_custom_event::<SomeEventTypeTest4>().unwrap();
    let event = SomeEventTypeTest4 { a: 42 };
    ev.push_custom_event(event).unwrap();

    let received = ep.poll_event().unwrap();
    if received.is_user_event() {
        let e2 = received.as_user_event_type::<SomeEventTypeTest4>().unwrap();
        assert_eq!(e2.a, 42);
    }
}

#[test]
fn test_event_sender_no_subsystem() {
    let sdl = sdl2::init().unwrap();
    let ev = sdl.event().unwrap();
    let tx = ev.event_sender();

    assert!(tx.push_event(sdl2::event::Event::Window {
        timestamp: 0,
        window_id: 0,
        win_event: sdl2::event::WindowEvent::Shown,
    }).is_ok());

    drop(ev);

    // Should return an error now the evet subsystem has been shut down
    assert!(tx.push_event(sdl2::event::Event::Window {
        timestamp: 0,
        window_id: 0,
        win_event: sdl2::event::WindowEvent::Hidden,
    }).is_err());
}