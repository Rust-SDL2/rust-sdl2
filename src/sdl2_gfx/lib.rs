/*!
A binding for SDL2_gfx.
 */

#![crate_name="sdl2_gfx"]
#![crate_type = "lib"]

#![feature(libc)]

extern crate libc;
extern crate sdl2;
extern crate "sdl2-sys" as sys;

// Setup linking for all targets.
#[link(name="SDL2_gfx")]
extern {}

pub mod primitives;
pub mod rotozoom;
pub mod framerate;
pub mod imagefilter;
