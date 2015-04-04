#![cfg_attr(feature = "no_std", feature(no_std, core))]
#![cfg_attr(feature = "no_std", no_std)]
#![allow(non_camel_case_types)]

extern crate libc;
#[cfg(feature = "no_std")]
extern crate core;

pub mod scancode;
pub mod keycode;

pub mod audio;
pub mod clipboard;
pub mod controller;
pub mod cpuinfo;
pub mod event;
pub mod filesystem;
pub mod haptic;
pub mod gesture;
pub mod joystick;
pub mod keyboard;
pub mod messagebox;
pub mod rect;
pub mod pixels;
pub mod render;
pub mod rwops;
pub mod surface;
pub mod touch;
pub mod video;
pub mod mouse;
pub mod sdl;
pub mod timer;
pub mod version;
