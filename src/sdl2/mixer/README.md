Rust-SDL2_mixer
=============

[![Build Status](https://travis-ci.org/andelf/rust-sdl2_mixer.svg?branch=master)](https://travis-ci.org/andelf/rust-sdl2_mixer)
[![crates.io](http://meritbadge.herokuapp.com/sdl2_mixer)](https://crates.io/crates/sdl2_mixer)
[![Crates.io](https://img.shields.io/crates/l/sdl2_mixer.svg)](https://github.com/andelf/rust-sdl2_mixer/blob/master/LICENSE-MIT)

Rust bindings for SDL2_mixer.

## Overview

Rust-SDL2_mixer is a library for talking to the new SDL2_mixer library from Rust.

Rust-SDL2_mixer uses the MIT licence.

## Requirements

* [Rust-SDL2](https://github.com/AngryLawyer/rust-sdl2)
* SDL2_mixer development libraries
* Latest stable Rust master

#### OSX

```bash
brew install sdl2_mixer --with-flac --with-fluid-synth --with-libmikmod --with-libmodplug --with-libvorbis --with-smpeg2
```
    
#### Windows

1. Download mingw development libraries from https://www.libsdl.org/projects/SDL_mixer/
(`SDL2_mixer-devel-2.0.x-mingw.tar.gz`)
2. Unpack to a folder of your choosing (You can delete it afterwards).
3. Copy all lib files from
    > SDL2_mixer-2.0.x\x86_64-w64-mingw32\lib\

    to (for Rust 1.6 and above) 
    > C:\Program Files\Rust\\**lib**\rustlib\x86_64-pc-windows-gnu\lib

    or to (for Rust versions 1.5 and below)
    > C:\Program Files\Rust\\**bin**\rustlib\x86_64-pc-windows-gnu\lib
    
    or to your library folder of choice, and ensure you have a system environment variable of
    > LIBRARY_PATH = C:\your\rust\library\folder

	For Multirust Users, this folder will be in
	> C:\Users\{Your Username}\AppData\Local\.multirust\toolchains\{current toolchain}\lib\rustlib\x86_64-pc-windows-gnu\lib
4. Copy `SDL2_mixer.dll` and `smpeg2.dll` from 
    > SDL2_mixer-2.0.x\x86_64-w64-mingw32\bin\

    into your cargo project, right next to your `Cargo.toml`.

## Installation

Place the following into your project's Cargo.toml file:

```toml
[dependencies]
sdl2_mixer = "0.24.0"
```

Or, to depend on the newest rust-sdl2_mixer, reference the repository:

```toml
[dependencies.sdl2_mixer]
git = "https://github.com/andelf/rust-sdl2_mixer"
```

You can also just clone and build the library yourself:

```bash
git clone https://github.com/andelf/rust-sdl2_mixer
cd rust-sdl2_mixer
cargo build
# TODO: OR if you are using the mac framework version
rustc --cfg mac_framework src/sdl2_mixer/lib.rs
```

If you're not using Cargo, you can compile the library manually:

```bash
git clone https://github.com/andelf/rust-sdl2_mixer
cd rust-sdl2_mixer
rustc src/sdl2_mixer/lib.rs
```

## Demo

A simple demo that plays out a portion of a given music file is included:

```bash
cargo run --example demo path/to/music.(mp3|flac|ogg|wav)
```

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
