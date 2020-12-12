use std::marker::PhantomData;
use std::mem;

use libc::c_int;

use crate::event::Event;

pub(in crate::event) fn wait_event_timeout(timeout: u32) -> Option<Event> {
    let mut raw = mem::MaybeUninit::uninit();
    let success = unsafe { sys::SDL_WaitEventTimeout(raw.as_mut_ptr(), timeout as c_int) } == 1;
    let raw = unsafe { raw.assume_init() };

    if success {
        Some(Event::from_ll(raw))
    } else {
        None
    }
}

/// An iterator that calls `EventPump::wait_event_timeout()`.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct EventWaitTimeoutIterator<'a> {
    _marker: PhantomData<&'a ()>,
    timeout: u32,
}

impl<'a> EventWaitTimeoutIterator<'a> {
    pub(in crate::event) fn new(timeout: u32) -> Self {
        Self {
            _marker: PhantomData,
            timeout,
        }
    }
}

impl<'a> Iterator for EventWaitTimeoutIterator<'a> {
    type Item = Event;
    fn next(&mut self) -> Option<Event> {
        wait_event_timeout(self.timeout)
    }
}
