#![crate_type = "bin"]

extern crate sdl2;
extern crate sdl2_image;
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
        println!("Usage: ./demo image.[png|jpg]")
    } else {
        video::main(&Path::new(args.get(1).to_string()));
    }
}
