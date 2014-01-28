#[crate_type = "bin"];

extern mod sdl2;
extern mod sdl2_image;
extern mod native;

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
        video::main(&Path::new(args[1]));
    }
}
