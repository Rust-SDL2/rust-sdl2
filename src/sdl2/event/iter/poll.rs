use std::marker::PhantomData;
use std::mem;

use crate::event::Event;

pub(in crate::event) fn poll_event() -> Option<Event> {
    let mut raw = mem::MaybeUninit::uninit();
    let has_pending = unsafe { sys::SDL_PollEvent(raw.as_mut_ptr()) } == 1;
    let raw = unsafe { raw.assume_init() };

    if has_pending {
        Some(Event::from_ll(raw))
    } else {
        None
    }
}

/// An iterator that calls `EventPump::poll_event()`.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct EventPollIterator<'a> {
    _marker: PhantomData<&'a ()>,
}

impl<'a> EventPollIterator<'a> {
    pub(in crate::event) fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for EventPollIterator<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        poll_event()
    }
}
