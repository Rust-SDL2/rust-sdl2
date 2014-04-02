#[allow(non_camel_case_types)]
pub mod ll {
    use std::libc::{c_uchar, uint8_t, uint32_t, c_schar};

    struct SDL_RWops_Anon {
        data: [c_uchar, ..24],
    }

    pub struct SDL_RWops {
        pub size: *uint8_t,
        pub seek: *uint8_t,
        pub read: *uint8_t,
        pub write: *uint8_t,
        pub close: *uint8_t,
        pub _type: uint32_t,
        hidden: SDL_RWops_Anon
    }

    extern "C" {
        pub fn SDL_RWFromFile(file: *c_schar, mode: *c_schar) -> *SDL_RWops;
    }
}
