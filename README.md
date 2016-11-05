Rust-SDL2_image
===============

Rust bindings for SDL2_image

> **NOTE**: The 1.0.0 and 1.1.0 version of this crate are yanked because of [#65](https://github.com/xsleonard/rust-sdl2_image/issues/65)! We are now using same ``x.y._`` version number as [sdl2](https://crates.io/crates/sdl2). See [sdl2_image/0.25.0](https://crates.io/crates/sdl2_image/0.25.0).

# Overview

Rust-SDL2_image is a library for talking to the new SDL2_image library from Rust.

Rust-SDL2_image uses the MIT license.

# Requirements

* [Rust-SDL2](https://github.com/AngryLawyer/rust-sdl2)
* [SDL_image 2.0 development libraries](https://www.libsdl.org/projects/SDL_image/)
* Rust master -- usage of Nightly is recommended

# [Documentation](https://docs.rs/sdl2_image/0.25.0/sdl2_image/)

# Installation

If you're using Cargo to manage your project, enter the following into your
Cargo.toml file:

```toml
[dependencies]
sdl2 = "0.25.0"
sdl2_image = "0.25.0"
```

Or, to reference this repository directly:

```toml
[dependencies.sdl2_image]
git = "https://github.com/xsleonard/rust-sdl2_image"
```

Otherwise, clone this repo and run:

```bash
cargo build
```

If you're not using Cargo, you can compile manually:

```bash
git clone https://github.com/xsleonard/rust-sdl2_image
cd rust-sdl2_image
rustc src/sdl2_image/lib.rs
# OR if you are using the mac framework version
rustc --cfg mac_framework src/sdl2_image/lib.rs
```

# Demo

You'll find included with the library a simple demo that loads and displays
a given image :

```bash
cargo run /path/to/some/image.(jpg|png)
```

Or:

```bash
rustc -L. src/demo/main.rs -o demo
./demo image.(png|jpg)
```
