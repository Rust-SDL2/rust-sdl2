Rust-SDL2_mixer
===============

[![Build Status](https://travis-ci.org/andelf/rust-sdl2_mixer.svg?branch=master)](https://travis-ci.org/andelf/rust-sdl2_mixer)

Rust bindings for SDL2_mixer.

Requirements
------------

* [Rust-sdl2](https://github.com/AngryLawyer/rust-sdl2)
* sdl_mixer 2.0 development libraries
* Rust master

Installation
------------

```
git clone https://github.com/andelf/rust-sdl2_mixer
cargo build
# TODO: OR if you are using the mac framework version
rustc --cfg mac_framework src/sdl2_mixer/lib.rs
```

Demo
----

```
cargo run /path/to/wav_file.wav
```
