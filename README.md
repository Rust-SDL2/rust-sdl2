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

If you're using [cargo](http://crates.io/) to manage your project, you can download through Crates.io:

```toml
    [dependencies]
    sdl2 = "0.0.15"
```

Alternatively, pull it from GitHub

```rust
    [dependencies.sdl2]
    git = "https://github.com/AngryLawyer/rust-sdl2"
```

Otherwise, clone this repo and run [cargo](http://crates.io/)

> cargo build

# Demo

We have some simple example projects included:

> cargo run --example demo

> cargo run --example audio-whitenoise

Some additional examples can be found in the [rs-sdl2-examples](https://github.com/jdeseno/rs-sdl2-examples) repo.

# OpenGL

If you want to use OpenGL, you also need the [gl-rs](https://github.com/bjz/gl-rs) package. If you're using [cargo](http://crates.io/), just add these lines to your Cargo.toml:

```toml
    [dependencies.gl]
    git = "https://github.com/bjz/gl-rs"
```

Then you need to add this to add this initialization code to establish the bindings:

```rust
    gl::load_with(|s| unsafe {
        std::mem::transmute(sdl2::video::gl_get_proc_address(s))
    });
```

Note that these bindings are very raw, and many of the calls will require unsafe blocks.

# When things go wrong
Rust, and Rust-SDL2, are both still heavily in development, and you may run into teething issues when using this. Before panicking, check that you're using the latest version of both Rust and Cargo, check that you've updated Rust-SDL2 to the latest version, and run `cargo clean`. If that fails, please let us know on the issue tracker.
