use std::mem;
use std::ops::{Deref, DerefMut};

/// An unowned value.
///
/// This type is used by rust-sdl2 when obtaining a handle owned by a parent
/// resource.
///
/// This does not free the underlying value when dropped.
/// Instead, `std::mem::forget()` is called on the value when dropped.
pub struct Unowned<T> {
    /// The reason `Option` is used is because `std::mem::forget` consumes
    /// a value. The only way to obtain the value from drop() is to swap this
    /// field with a non-dropping variant such as None.
    value: Option<T>
}

#[unsafe_destructor]
impl<T> Drop for Unowned<T> {
    fn drop(&mut self) {
        unsafe { mem::forget(mem::replace(&mut self.value, None)) };
    }
}

impl<T> Deref for Unowned<T> {
    type Target = T;

    fn deref(&self) -> &T { self.value.as_ref().unwrap() }
}

impl<T> DerefMut for Unowned<T> {
    fn deref_mut(&mut self) -> &mut T { self.value.as_mut().unwrap() }
}

impl<T> Unowned<T> {
    pub unsafe fn new(value: T) -> Unowned<T> {
        Unowned { value: Some(value) }
    }
}
