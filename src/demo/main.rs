#![crate_type = "bin"]

extern crate sdl2;
extern crate sdl2_ttf;

use std::env;
use std::path::Path;

mod video;

fn main() {

    let args: Vec<_> = env::args().collect();

    println!("linked sdl2_ttf: {}", sdl2_ttf::get_linked_version());

    if args.len() < 2 {
        println!("Usage: ./demo font.[ttf|ttc|fon]")
    } else {
        let path: &Path = Path::new(&args[1]);
        video::main(path);
    }
}
