#![crate_type = "bin"]

extern crate sdl2;
extern crate sdl2_image;

use std::env;
use std::path::Path;

mod video;

fn main() {

    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run /path/to/image.(png|jpg)")
    } else {
        video::main(Path::new(&args[1]));
    }
}
