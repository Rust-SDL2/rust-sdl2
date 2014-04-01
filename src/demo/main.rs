#![crate_type = "bin"]

#![feature(macro_rules)]
extern crate sdl2;
extern crate sdl2_ttf;
extern crate native;

use std::os;

mod video;

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

#[main]
fn main() {
    let args = os::args();
    if args.len() < 2 {
        println!("Usage: ./demo font.[ttf|ttc|fon]")
    } else {
        video::main(&Path::new(args[1]));
    }
}
