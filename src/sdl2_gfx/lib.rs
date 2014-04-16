/*!
A binding for SDL2_gfx.
 */

#![crate_id="sdl2_gfx#sdl2_gfx:0.1"]
#![crate_type = "lib"]
#![desc = "SDL2_gfx bindings and wrappers"]
#![comment = "SDL2_gfx bindings and wrappers"]
#![license = "MIT"]

#![feature(globs, macro_rules)]

extern crate libc;
extern crate sdl2;

// Setup linking for all targets.
#[link(name="SDL2_gfx")]
extern {}


//#[allow(non_camel_case_types, dead_code)]

pub mod primitives;
pub mod rotozoom;
pub mod framerate;
pub mod imagefilter;
