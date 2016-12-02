Rust-SDL2_ttf
=============

[![Build Status](https://travis-ci.org/andelf/rust-sdl2_ttf.svg?branch=master)](https://travis-ci.org/andelf/rust-sdl2_ttf)
[![crates.io](http://meritbadge.herokuapp.com/sdl2_ttf)](https://crates.io/crates/sdl2_ttf)

Rust bindings for SDL2_ttf.

## Overview

Rust-SDL2_ttf is a library for talking to the new SDL2_ttf library from Rust.

Rust-SDL2_ttf uses the MIT licence.

## [Documentation](https://docs.rs/sdl2_ttf/0.25.0/sdl2_ttf/)

## Requirements

* [Rust-SDL2](https://github.com/AngryLawyer/rust-sdl2)
* SDL2_ttf development libraries
* Rust master or nightly

## Installation

Place the following into your project's Cargo.toml file:

```toml
[dependencies]
sdl2_ttf = "0.25"
```

sdl2_ttf is directly compatible with the corresponding version of sdl2.
Hence sdl2_ttf v0.15 is compatible with sdl2 0.15, and so forth.
Backwards compatibility is not guaranteed by rust-sdl2, so take that into
account when creating new projects !

If you want the newest rust-sdl2_ttf, reference the repository:

```toml
[dependencies.sdl2_ttf]
git = "https://github.com/andelf/rust-sdl2_ttf"
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
cargo run --example demo /path/to/font.(ttf|ttc|fon)
```
