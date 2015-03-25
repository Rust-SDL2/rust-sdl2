Rust-SDL2_mixer
=============

[![Build Status](https://travis-ci.org/andelf/rust-sdl2_mixer.svg?branch=master)](https://travis-ci.org/andelf/rust-sdl2_mixer)

Rust bindings for SDL2_mixer.

## Overview

Rust-SDL2_mixer is a library for talking to the new SDL2_mixer library from Rust.

Rust-SDL2_mixer uses the MIT licence.

## Requirements

* [Rust-SDL2](https://github.com/AngryLawyer/rust-sdl2)
* SDL2_mixer development libraries
* Rust master or nightly

## Installation

Place the following into your project's Cargo.toml file:

```toml
[dependencies]
sdl2_mixer = "0.0.31" # Doesn't work yet, needs to be published.
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
cargo run path/to/music.(mp3|flac|ogg|wav)
```

Or:

```bash
rustc -L. src/demo/main.rs -o demo
./demo path/to/music.(mp3|flac|ogg|wav)
```
