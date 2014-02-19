extern crate sdl2;
extern crate native;

mod video;

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

#[main]
fn main() {

	video::main();
}
