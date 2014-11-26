# Rust-SDL2_mixer


[![Build Status](https://travis-ci.org/andelf/rust-sdl2_mixer.svg?branch=master)](https://travis-ci.org/andelf/rust-sdl2_mixer)

Rust bindings for SDL2_mixer.

## Installation

Requirements

* [Rust-sdl2](https://github.com/AngryLawyer/rust-sdl2)
* sdl_mixer 2.0 development libraries
* Rust nightly
* Cargo nightly


### normal

```
git clone https://github.com/andelf/rust-sdl2_mixer
cargo build
# TODO: OR if you are using the mac framework version
rustc --cfg mac_framework src/sdl2_mixer/lib.rs
```

### cargo

```
[dependencies]
sdl2_mixer = "$version-here$"
```

or

```
[dependencies.sdl2_mixer]
git = "https://github.com/andelf/rust-sdl2_mixer"
```

## Demo

```
cargo run /path/to/wav_file.wav
```
