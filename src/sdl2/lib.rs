#![crate_name = "sdl2"]
#![crate_type = "lib"]

extern crate num;
extern crate libc;
#[macro_use]
extern crate bitflags;
extern crate sdl2_sys as sys;

pub use sdl::*;

pub mod keycode;
pub mod scancode;

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
mod sdl;
pub mod audio;
pub mod version;
pub mod messagebox;

mod util;
