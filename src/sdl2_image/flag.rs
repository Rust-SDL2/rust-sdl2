// Loosely based on zlib-licensed code from RustAllegro
//     https://github.com/SiegeLord/RustAllegro
//
// Implements efficient, type-safe flags with support for bitwise operators.
//
// Usage:
//
// flag_type!(FlagTypeName {
//     FlagName1 = FlagValue1,
//     FlagName2 = FlagValue2,
//     ...
//     FlagNameN = FlagValueN
// })
//
// fn foo(flag: FlagTypeName) {
//     let raw = (flag | FlagNameN).get();
//     bar(raw);
// }

#![macro_escape]

macro_rules! flag_type(
    ($typename:ident : $supertype:ident { $($name:ident = $value:expr),* }) => {
        pub struct $typename {
            bits: $supertype
        }

        impl $typename {
            #[inline]
            pub fn new(bits: $supertype) -> $typename {
                $typename { bits: bits }
            }

            #[inline]
            pub fn get(self) -> $supertype {
                self.bits
            }
        }

        impl ::std::default::Default for $typename {
            fn default() -> $typename {
                $typename::new(0)
            }
        }

        impl ::std::cmp::Eq for $typename {
            fn eq(&self, other: &$typename) -> bool {
                self.bits == other.bits
            }
        }

        impl ::std::cmp::TotalEq for $typename {}

        impl ::std::cmp::Ord for $typename {
            fn lt(&self, other: &$typename) -> bool {
                self.bits < other.bits
            }
        }

        impl ::std::cmp::TotalOrd for $typename {
            fn cmp(&self, other: &$typename) -> Ordering {
                self.bits.cmp(&other.bits)
            }
        }

        impl ::std::ops::Not<$typename> for $typename {
            fn not(&self) -> $typename {
                $typename { bits: !self.bits }
            }
        }

        impl ::std::ops::BitAnd<$typename, $typename> for $typename {
            fn bitand(&self, rhs: &$typename) -> $typename {
                $typename { bits: self.bits & rhs.bits }
            }
        }

        impl ::std::ops::BitOr<$typename, $typename> for $typename {
            fn bitor(&self, rhs: &$typename) -> $typename {
                $typename { bits: self.bits | rhs.bits }
            }
        }

        impl ::std::ops::BitXor<$typename, $typename> for $typename {
            fn bitxor(&self, rhs: &$typename) -> $typename {
                $typename { bits: self.bits ^ rhs.bits }
            }
        }

        impl ::std::ops::Shl<$supertype, $typename> for $typename {
            fn shl(&self, rhs: &$supertype) -> $typename {
                $typename { bits: self.bits << *rhs }
            }
        }

        impl ::std::ops::Shr<$supertype, $typename> for $typename {
            fn shr(&self, rhs: &$supertype) -> $typename {
                $typename { bits: self.bits >> *rhs }
            }
        }

        $(
            pub static $name: $typename = $typename { bits: $value as $supertype };
        )+
    };
    ($typename:ident { $($name:ident = $value:expr),* }) => {
        flag_type!($typename : u32 { $($name = $value),* })
    }
)
