macro_rules! impl_raw_accessors(
    ($($t:ty, $raw:ty);+) => (
        $(
        impl $t {
            #[inline]
            pub fn raw(&self) -> $raw { self.raw }
        }
        )+
    )
);

macro_rules! impl_owned_accessors(
    ($($t:ty, $owned:ident);+) => (
        $(
        impl $t {
            #[inline]
            pub fn $owned(&self) -> bool { self.$owned }
        }
        )+
    )
);

macro_rules! impl_raw_constructor(
    ($($t:ty -> $te:ident ($($r:ident:$rt:ty),+));+) => (
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
