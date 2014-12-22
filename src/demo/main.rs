#![crate_type = "bin"]

extern crate sdl2;
extern crate sdl2_image;

use std::os;

mod video;

#[main]
fn main() {
    let args = os::args();
    if args.len() < 2 {
        println!("Usage: ./demo image.[png|jpg]")
    } else {
        video::main(&Path::new(args[1].to_string()));
    }
}
