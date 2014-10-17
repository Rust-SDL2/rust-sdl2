#![crate_name = "sdl2"]
#![crate_type = "lib"]

#![desc = "SDL2 bindings"]
#![license = "MIT"]

#![feature(default_type_params, globs, macro_rules, slicing_syntax, unsafe_destructor)]

extern crate libc;
extern crate collections;

pub use sdl::*;

pub mod keycode;
pub mod scancode;

pub mod clipboard;
pub mod cpuinfo;
pub mod macros;
pub mod event;
pub mod filesystem;
pub mod gesture;
pub mod touch;
pub mod joystick;
pub mod controller;
pub mod keyboard;
pub mod mouse;
pub mod rect;
pub mod surface;
pub mod pixels;
pub mod video;
pub mod timer;
pub mod render;
pub mod rwops;
pub mod sdl;
pub mod audio;
pub mod version;
pub mod messagebox;
