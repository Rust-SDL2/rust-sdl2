# Rust-SDL2 [![Build Status](https://travis-ci.org/AngryLawyer/rust-sdl2.png?branch=master)](https://travis-ci.org/AngryLawyer/rust-sdl2)

Bindings for SDL2 in Rust

# Overview

Rust-SDL2 is a library for talking to the new SDL2.0 libraries from Rust. Low-level C components are wrapped in Rust code to make them more idiomatic and abstract away inappropriate manual memory management.

Rust-SDL2 uses the MIT license.

If you want a library compatible with earlier versions of SDL, please see https://github.com/brson/rust-sdl

## Where are SDL_image, SDL_mixer, and SDL_ttf?

These live outside of the repo.

* https://github.com/xsleonard/rust-sdl2_image
* https://github.com/andelf/rust-sdl2_ttf
* https://github.com/andelf/rust-sdl2_mixer
* https://github.com/andelf/rust-sdl2_gfx

# Requirements

## Rust

We currently compile against the *Master* branch. I'd recommend using the Nightly installer, as that has the greatest chance of working.

## *SDL2.0  development libraries*
Install these through your favourite package management tool, or via http://www.libsdl.org/

If you're running OSX, it's a good idea to install these via [homebrew](http://brew.sh/)

> brew install sdl2

If you're having issues, [see here](https://github.com/PistonDevelopers/rust-empty/issues/175)

# Installation

Clone this repo and run [cargo](http://crates.io/)

> cargo

# Demo

To see an example of the code in use

> cargo run demo

Some additional examples can be found in the [rs-sdl2-examples](https://github.com/jdeseno/rs-sdl2-examples) repo.

# When things go wrong
Rust, and Rust-SDL2, are both still heavily in development, and you may run into teething issues when using this. Before panicking, check that you're using the latest version of both Rust and Cargo, check that you've updated Rust-SDL2 to the latest version, and run `cargo clean`. If that fails, please let us know on the issue tracker.
