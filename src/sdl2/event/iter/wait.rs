use std::marker::PhantomData;
use std::mem;

use crate::event::Event;
use crate::get_error;

pub(in crate::event) fn wait_event() -> Event {
    let mut raw = mem::MaybeUninit::uninit();
    let success = unsafe { sys::SDL_WaitEvent(raw.as_mut_ptr()) } == 1;
    let raw = unsafe { raw.assume_init() };

    if success {
        Event::from_ll(raw)
    } else {
        panic!(get_error())
    }
}

/// An iterator that calls `EventPump::wait_event()`.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct EventWaitIterator<'a> {
    _marker: PhantomData<&'a ()>,
}

impl<'a> EventWaitIterator<'a> {
    pub(in crate::event) fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for EventWaitIterator<'a> {
    type Item = Event;
    fn next(&mut self) -> Option<Event> {
        Some(wait_event())
    }
}
