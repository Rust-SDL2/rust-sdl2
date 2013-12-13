#[link(name = "sdl2",
       package_id = "sdl2",
       vers = "0.0.1",
       uuid = "263e35b2-0727-11e3-b3dd-29219a890b3c",
       url = "http://github.com/AngryLawyer/rust-sdl2")];

#[desc = "SDL2 bindings"];
#[license = "MIT"];

#[feature(globs)];

pub use sdl::*;
#[path = "generated/keycode.rs"]
pub mod keycode;
#[path = "generated/scancode.rs"]
pub mod scancode;

pub mod event;
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
