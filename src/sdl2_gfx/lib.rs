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

use libc::{c_int, c_long};
use std::ptr;
use std::fmt;
use std::c_str::CString;
use std::num::FromPrimitive;
use sdl2::surface::Surface;
use sdl2::get_error;
use sdl2::pixels;
use sdl2::pixels::Color;
use sdl2::pixels::ll::SDL_Color;
use sdl2::rwops::RWops;

// Setup linking for all targets.
#[link(kind="framework", name="SDL2_gfx")]
extern {}


//#[allow(non_camel_case_types, dead_code)]

pub mod primitives;
pub mod rotozoom;
pub mod framerate;
pub mod imagefilter;
