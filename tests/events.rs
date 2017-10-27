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
        data1: 0x12_34 as *mut ::std::os::raw::c_void,
        data2: 0x56_78 as *mut ::std::os::raw::c_void,
    };

    let (t1, a1, a2) = match event {
        event::Event::User { type_: t1, data1: a1, data2: a2, .. } => { (t1, a1, a2) }
        _ => { panic!("expected user event") }
    };
    ev.push_event(event.clone()).unwrap();
    let received = ep.poll_event().unwrap();
    assert_eq!(&event, &received);
    match &received {
        &event::Event::User { type_: t2, data1: b1, data2: b2, .. }  => {
            assert_eq!(t1, t2);
            assert_eq!(a1, b1);
            assert_eq!(a2, b2);
        }
        other => { panic!("Received non User event: {:?}", other) }
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
