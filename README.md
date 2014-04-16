Rust-SDL2_gfx
===============

[![Build Status](https://travis-ci.org/andelf/rust-sdl2_gfx.svg?branch=master)](https://travis-ci.org/andelf/rust-sdl2_gfx)
[![Build Status](https://drone.io/github.com/andelf/rust-sdl2_gfx/status.png)](https://drone.io/github.com/andelf/rust-sdl2_gfx/latest)

Rust bindings for SDL2_gfx

Requirements
------------

* [Rust-sdl2](https://github.com/AngryLawyer/rust-sdl2) or [My Fork](https://github.com/andelf/rust-sdl2)
* sdl2_gfx development libraries
* Rust master

Installation
------------

```
# compile your rust-sdl2 somewhere
git clone https://github.com/andelf/rust-sdl2_gfx
cd rust-sdl2_gfx
rustc -L. src/sdl2_gfx/lib.rs
```

Demo
----

```
rustc -L. src/demo/main.rs -o demo
./demo
```
