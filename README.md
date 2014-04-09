# Rust-SDL2

[![Build Status](https://travis-ci.org/AngryLawyer/rust-sdl2.png?branch=master)](https://travis-ci.org/AngryLawyer/rust-sdl2)

Bindings for SDL2 in Rust

# Overview

Rust-SDL2 is a library for talking to the new SDL2.0 libraries from Rust. Low-level C components are wrapped in Rust code to make them more idiomatic and abstract away inappropriate manual memory management.

Rust-SDL2 is still in very early stages of development, and probably won't work for a little while.

Rust-SDL2 uses the MIT license.

If you want a library compatible with earlier versions of SDL, please see https://github.com/brson/rust-sdl

## Where are SDL_image, SDL_mixer, and SDL_ttf?

These live outside of the repo.

* https://github.com/xsleonard/rust-sdl2_image
* https://github.com/andelf/rust-sdl2_ttf
* https://github.com/andelf/rust-sdl2_mixer

# Requirements

* *Rust* - we currently compile against the *Master* branch. The releases on http://www.rust-lang.org tend to not work.
* *SDL2.0  development libraries* - install through your favourite package management tool, or via http://www.libsdl.org/

# Installation
Clone this repo run `make`. To see an example of the code in use, *make demo*.

# Demo

To compile the demo:

> rustpkg install demo


Then run:

> ./bin/demo

Or you could instead just use

> make demo

# When things go wrong
Rust, and Rust-SDL2, are both still heavily in development, and you may run into teething issues when using this. Before panicking, check that you're using the latest Master branch of Rust, check that you've updated Rust-SDL2 to the latest version, and run `make clean`. If that fails, please let us know on the issue tracker.
