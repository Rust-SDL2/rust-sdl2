#![macro_escape]

macro_rules! impl_raw_accessors(
    ($($t:ty, $raw:ty);+) => (
        $(
        impl $t {
            #[inline]
            pub fn raw(&self) -> $raw { self.raw }
        }
        )+
    )
)

macro_rules! impl_owned_accessors(
    ($($t:ty, $owned:ident);+) => (
        $(
        impl $t {
            #[inline]
            pub fn $owned(&self) -> bool { self.$owned }
        }
        )+
    )
)

macro_rules! impl_raw_constructor(
    ($($t:ty -> $te:ident ($r:ident:$rt:ty));+) => (
        $(
        impl $t {
            #[inline]
            pub unsafe fn new_from_raw($r:$rt) -> $t {
                $te { $r: $r }
            }
        }
        )+
    );
    ($($t:ty -> $te:ident ($r:ident:$rt:ty, $o:ident:$ot:ty));+) => (
        $(
        impl $t {
            #[inline]
            pub unsafe fn new_from_raw($r:$rt, $o:$ot) -> $t {
                $te { $r: $r, $o: $o }
            }
        }
        )+
    )
)
