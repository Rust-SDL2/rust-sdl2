# Rust-SDL2_gfx

[![Build Status](https://travis-ci.org/andelf/rust-sdl2_gfx.svg?branch=master)](https://travis-ci.org/andelf/rust-sdl2_gfx)
[![Build Status](https://drone.io/github.com/andelf/rust-sdl2_gfx/status.png)](https://drone.io/github.com/andelf/rust-sdl2_gfx/latest)

Rust bindings for SDL2_gfx

## Requirements

* [Rust-SDL2](https://github.com/AngryLawyer/rust-sdl2) or [My Fork](https://github.com/andelf/rust-sdl2)
* [SDL2_gfx](http://sourceforge.net/projects/sdl2gfx/) development libraries
* Rust master or nightly

## Installation

```
# compile your rust-sdl2 somewhere
git clone https://github.com/andelf/rust-sdl2_gfx
cd rust-sdl2_gfx
rustc -L. src/sdl2_gfx/lib.rs
```

    NOTE: sdl2_gfx doesn't support mac_framework.

## Demo

```
rustc -L. src/demo/gfx_demo.rs
./gfx_demo
```
