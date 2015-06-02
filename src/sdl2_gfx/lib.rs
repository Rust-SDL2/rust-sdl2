/*!
A binding for SDL2_gfx.
 */

extern crate libc;
extern crate num;
extern crate sdl2;
extern crate sdl2_sys as sys;
extern crate c_vec;

// Setup linking for all targets.
#[link(name="SDL2_gfx")]
extern {}

pub mod primitives;
pub mod rotozoom;
pub mod framerate;
pub mod imagefilter;
