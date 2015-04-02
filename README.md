# Rust-SDL2 [![Build Status][trav-ci-img]][trav-ci]

Bindings for SDL2 in Rust

# Overview

Rust-SDL2 is a library for talking to the new SDL2.0 libraries from Rust.
Low-level C components are wrapped in Rust code to make them more idiomatic and
abstract away inappropriate manual memory management.

Rust-SDL2 uses the MIT license.

If you want a library compatible with earlier versions of SDL, please see
[here][early-sdl]

## Where are SDL_image, SDL_mixer, and SDL_ttf?

These live outside of the repo.

* https://github.com/xsleonard/rust-sdl2_image
* https://github.com/andelf/rust-sdl2_ttf
* https://github.com/andelf/rust-sdl2_mixer
* https://github.com/andelf/rust-sdl2_gfx

# Requirements

## Rust

We currently compile against the *Master* branch. I'd recommend using the
Nightly installer, as that has the greatest chance of working.

## *SDL2.0  development libraries*
### Linux
Install these through your favourite package management tool, or via
http://www.libsdl.org/

Ubuntu example:
> sudo apt-get install libsdl2-dev

### Mac OS X
If you're running OSX, it's a good idea to install these via
[homebrew][homebrew]

> brew install sdl2

If you're having issues, [see here][pdev-issue].

### Windows (MinGW)
On Windows, make certain you are using the MinGW version of SDL; the native
version will crash on `sdl2::init`.

1. Download mingw development libraries from
http://www.libsdl.org/ (SDL2-devel-2.0.x-mingw.tar.gz).
2. Unpack to a folder of your choosing (You can delete it afterwards).
3. Copy all lib files from
    > SDL2-devel-2.0.x-mingw\SDL2-2.0.x\x86_64-w64-mingw32\lib

    inside
    > C:\Rust\bin\rustlib\x86_64-pc-windows-gnu\lib

4. Copy SDL2.dll from
    > SDL2-devel-2.0.x-mingw\SDL2-2.0.x\x86_64-w64-mingw32\bin

    into your cargo project, right next to your Cargo.toml.

# Installation

If you're using [cargo][crates] to manage your project, you can
download through Crates.io:

```toml
    [dependencies]
    sdl2 = "0.0.35"
```

Alternatively, pull it from GitHub

```rust
    [dependencies.sdl2]
    git = "https://github.com/AngryLawyer/rust-sdl2"
```

Otherwise, clone this repo and run [cargo][crates]

> cargo build

# Demo

We have some simple example projects included:

> cargo run --example demo

> cargo run --example audio-whitenoise

Some additional examples can be found in the
[rs-sdl2-examples][examples] repo.

# OpenGL

If you want to use OpenGL, you also need the
[gl-rs][gl-rs] package. If you're using
[cargo][crates], just add these lines to your Cargo.toml:

```toml
    [dependencies.gl]
    git = "https://github.com/bjz/gl-rs"
```

Then you need to add this to add this initialization code to establish the
bindings:

```rust
    gl::load_with(|s| unsafe {
        std::mem::transmute(sdl2::video::gl_get_proc_address(s))
    });
```

Note that these bindings are very raw, and many of the calls will require
unsafe blocks.

# When things go wrong
Rust, and Rust-SDL2, are both still heavily in development, and you may run
into teething issues when using this. Before panicking, check that you're using
the latest version of both Rust and Cargo, check that you've updated Rust-SDL2
to the latest version, and run `cargo clean`. If that fails, please let us know
on the issue tracker.

[trav-ci-img]: https://travis-ci.org/AngryLawyer/rust-sdl2.png?branch=master
[trav-ci]: https://travis-ci.org/AngryLawyer/rust-sdl2
[early-sdl]: https://github.com/brson/rust-sdl
[homebrew]: http://brew.sh/
[crates]: http://crates.io/
[examples]: https://github.com/jdeseno/rs-sdl2-examples
[gl-rs]: https://github.com/bjz/gl-rs
[pdev-issue]: https://github.com/PistonDevelopers/rust-empty/issues/175
