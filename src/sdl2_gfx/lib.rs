/*!
A binding for SDL2_gfx.
 */

#![feature(globs, macro_rules)]

extern crate libc;
extern crate sdl2;

// Setup linking for all targets.
#[link(name="SDL2_gfx")]
extern {}

pub mod primitives;
pub mod rotozoom;
pub mod framerate;
pub mod imagefilter;
