#![crate_name = "sdl2"]
#![crate_type = "lib"]

#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless, transmute_ptr_to_ref))]

extern crate num;
pub extern crate libc;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;
pub extern crate sdl2_sys as sys;

#[cfg(feature = "gfx")]
extern crate c_vec;

pub use sdl::*;

pub mod clipboard;
pub mod cpuinfo;
#[macro_use] pub mod macros;
pub mod event;
pub mod filesystem;
pub mod gesture;
pub mod touch;
pub mod joystick;
pub mod controller;
pub mod haptic;
pub mod keyboard;
pub mod mouse;
pub mod rect;
pub mod surface;
pub mod pixels;
pub mod video;
pub mod timer;
pub mod render;
pub mod rwops;
pub mod log;
mod sdl;
pub mod audio;
pub mod version;
pub mod messagebox;
pub mod hint;

// modules
#[cfg(feature = "ttf")]
pub mod ttf;
#[cfg(feature = "image")]
pub mod image;
#[cfg(feature = "mixer")]
pub mod mixer;
#[cfg(feature = "gfx")]
pub mod gfx;

mod common;
// Export return types and such from the common module.
pub use common::IntegerOrSdlError;
