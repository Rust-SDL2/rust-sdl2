Rust-SDL2_ttf
=============

[![Build Status](https://travis-ci.org/andelf/rust-sdl2_ttf.svg?branch=master)](https://travis-ci.org/andelf/rust-sdl2_ttf)

## Overview

Rust-SDL2_ttf is a library for talking to the new SDL2_ttf library from Rust.

Rust-SDL2_ttf uses the MIT licence.

## Requirements

* [Rust-SDL2](https://github.com/AngryLawyer/rust-sdl2)
* SDL2_ttf development libraries
* Rust master or nightly

## Installation

Place the following into your project's Cargo.toml file:

```toml
[dependencies]
sdl2_ttf = "0.0.31" # Doesn't work yet, needs to be published.
```

Or, to depend on the newest rust-sdl2_ttf, reference the repository:

```toml
[dependencies.sdl2_ttf]
git = "https://github.com/andelf/rust-sdl2-ttf"
```

You can also just clone and build the library yourself:

```bash
git clone https://github.com/andelf/rust-sdl2_ttf
cd rust-sdl2_ttf
cargo build
# TODO: OR if you are using the mac framework version
rustc -L. --cfg mac_framework src/sdl2_ttf/lib.rs
```

If you're not using Cargo, you can compile the library manually:

```bash
git clone https://github.com/andelf/rust-sdl2_ttf
cd rust-sdl2_ttf
rustc src/sdl2_ttf/lib.rs
```

## Demo

A simple demo that prints out a string given a font is included:

```bash
cargo run path/to/font.(ttf|ttc|fon)
```

Or:

```bash
rustc -L. src/demo/main.rs -o demo
./demo path/to/font.(ttf|ttc|fon)
```
