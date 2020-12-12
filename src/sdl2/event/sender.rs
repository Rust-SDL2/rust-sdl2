use libc::c_void;

use crate::get_error;

use super::event_type::custom;
use super::Event;

/// A sendible type that can push events to the event queue.
pub struct EventSender {
    _priv: (),
}

impl EventSender {
    pub(in crate::event) fn new() -> Self {
        Self { _priv: () }
    }

    /// Pushes an event to the event queue.
    #[doc(alias = "SDL_PushEvent")]
    pub fn push_event(&self, event: Event) -> Result<(), String> {
        match event.to_ll() {
            Some(mut raw_event) => {
                let ok = unsafe { sys::SDL_PushEvent(&mut raw_event) == 1 };
                if ok {
                    Ok(())
                } else {
                    Err(get_error())
                }
            }
            None => Err("Cannot push unsupported event type to the queue".to_owned()),
        }
    }

    /// Push a custom event
    ///
    /// If the event type ``T`` was not registered using
    /// [EventSubsystem::register_custom_event]
    /// (../struct.EventSubsystem.html#method.register_custom_event),
    /// this method will panic.
    ///
    /// # Example: pushing and receiving a custom event
    /// ```
    /// struct SomeCustomEvent {
    ///     a: i32
    /// }
    ///
    /// let sdl = sdl2::init().unwrap();
    /// let ev = sdl.event().unwrap();
    /// let mut ep = sdl.event_pump().unwrap();
    ///
    /// ev.register_custom_event::<SomeCustomEvent>().unwrap();
    ///
    /// let event = SomeCustomEvent { a: 42 };
    ///
    /// ev.push_custom_event(event);
    ///
    /// let received = ep.poll_event().unwrap(); // or within a for event in ep.poll_iter()
    /// if received.is_user_event() {
    ///     let e2 = received.as_user_event_type::<SomeCustomEvent>().unwrap();
    ///     assert_eq!(e2.a, 42);
    /// }
    /// ```
    pub fn push_custom_event<T: ::std::any::Any>(&self, event: T) -> Result<(), String> {
        use std::any::TypeId;
        let cet = custom::lock().unwrap();
        let type_id = TypeId::of::<Box<T>>();

        let user_event_id = *match cet.type_id_to_sdl_id.get(&type_id) {
            Some(id) => id,
            None => {
                return Err("Type is not registered as a custom event type!".to_owned());
            }
        };

        let event_box = Box::new(event);
        let event = Event::User {
            timestamp: 0,
            window_id: 0,
            type_: user_event_id,
            code: 0,
            data1: Box::into_raw(event_box) as *mut c_void,
            data2: ::std::ptr::null_mut(),
        };

        self.push_event(event)?;

        Ok(())
    }
}
