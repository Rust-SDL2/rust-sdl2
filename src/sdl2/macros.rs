macro_rules! impl_raw_accessors(
    ($(($t:ty, $raw:ty)),+) => (
        $(
        impl $t {
            #[inline]
            // can prevent introducing UB until
            // https://github.com/rust-lang/rust-clippy/issues/5953 is fixed
            #[allow(clippy::trivially_copy_pass_by_ref)]
            pub const unsafe fn raw(&self) -> $raw { self.raw }
        }
        )+
    )
);

macro_rules! impl_raw_constructor(
    ($(($t:ty, $te:ident ($($r:ident:$rt:ty),+))),+) => (
        $(
        impl $t {
            #[inline]
            pub const unsafe fn from_ll($($r:$rt),+) -> $t {
                $te { $($r: $r),+ }
            }
        }
        )+
    )
);
