macro_rules! impl_raw_accessors(
    ($(($t:ty, $raw:ty)),+) => (
        $(
        impl $t {
            #[inline]
            pub unsafe fn raw(&self) -> $raw { self.raw }
        }
        )+
    )
);

macro_rules! impl_raw_constructor(
    ($(($t:ty, $te:ident ($($r:ident:$rt:ty),+))),+) => (
        $(
        impl $t {
            #[inline]
            pub unsafe fn from_ll($($r:$rt),+) -> $t {
                $te { $($r: $r),+ }
            }
        }
        )+
    )
);

/// Many SDL functions will accept `int` values, even if it doesn't make sense for the values to be negative.
/// In the cases that SDL doesn't check negativity, passing negative values could be unsafe.
/// For example, `SDL_JoystickGetButton` uses the index argument to access an array without checking if it's negative,
/// which could potentially lead to segmentation faults.
macro_rules! u32_to_int(
    ($value:expr) => (
        if $value >= 1<<31 { Err(format!("`{}` is out of bounds.", stringify!($value))) }
        else { Ok($value as ::libc::c_int) }
    )
);

macro_rules! usize_to_int(
    ($value:expr) => (
        if $value >= 1<<31 { Err(format!("`{}` is out of bounds.", stringify!($value))) }
        else { Ok($value as ::libc::c_int) }
    )
);

macro_rules! int_to_u32(
    ($value:expr) => (
        if $value < 0 { Err(format!("`{}` is out of bounds.", stringify!($value))) }
        else { Ok($value as u32) }
    )
);
