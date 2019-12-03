macro_rules! impl_raw_accessors(
    ($(($t:ty, $raw:ty)),+) => (
        $(
        impl $t {
            #[inline]
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
