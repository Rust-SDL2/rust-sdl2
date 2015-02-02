use std::marker::ContravariantLifetime;
use std::mem;
use std::ops::{Deref, DerefMut};

/// An unowned shared value.
///
/// This type is used by rust-sdl2 when obtaining a handle owned by a parent
/// resource.
///
/// This does not free the underlying value when dropped.
/// Instead, `std::mem::forget()` is called on the value when dropped.
pub struct Unowned<'a, T> {
    /// The reason `Option` is used is because `std::mem::forget` consumes
    /// a value. The only way to obtain the value from drop() is to swap this
    /// field with a non-dropping variant such as None.
    value: Option<T>,
    _marker: ContravariantLifetime<'a>
}

#[unsafe_destructor]
impl<'a, T> Drop for Unowned<'a, T> {
    fn drop(&mut self) {
        unsafe { mem::forget(mem::replace(&mut self.value, None)) };
    }
}

impl<'a, T> Deref for Unowned<'a, T> {
    type Target = T;

    fn deref(&self) -> &T { self.value.as_ref().unwrap() }
}

impl<'a, T> Unowned<'a, T> {
    pub unsafe fn new<'l>(value: T) -> Unowned<'l, T> {
        Unowned {
            value: Some(value),
            _marker: ContravariantLifetime
        }
    }
}

/// An unowned mutable value.
///
/// This type is used by rust-sdl2 when obtaining a handle owned by a parent
/// resource.
///
/// This does not free the underlying value when dropped.
/// Instead, `std::mem::forget()` is called on the value when dropped.
pub struct UnownedMut<'a, T> {
    value: Option<T>,
    _marker: ContravariantLifetime<'a>
}

#[unsafe_destructor]
impl<'a, T> Drop for UnownedMut<'a, T> {
    fn drop(&mut self) {
        unsafe { mem::forget(mem::replace(&mut self.value, None)) };
    }
}

impl<'a, T> Deref for UnownedMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &T { self.value.as_ref().unwrap() }
}

impl<'a, T> DerefMut for UnownedMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T { self.value.as_mut().unwrap() }
}

impl<'a, T> UnownedMut<'a, T> {
    pub unsafe fn new<'l>(value: T) -> UnownedMut<'l, T> {
        UnownedMut {
            value: Some(value),
            _marker: ContravariantLifetime
        }
    }
}
