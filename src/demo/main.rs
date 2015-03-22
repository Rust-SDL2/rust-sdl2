#![crate_type = "bin"]

extern crate sdl2;
extern crate sdl2_image;

use std::env;
use std::path::Path;

mod video;

fn main() {

    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./demo image.[png|jpg]")
    } else {
        let path: &Path = Path::new(&args[1]);
        video::main(path);
    }
}
