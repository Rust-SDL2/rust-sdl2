Rust-SDL2_image
===============

Rust bindings for SDL2_image

# Overview

Rust-SDL2_image is a library for talking to the new SDL2_image library from Rust.

Rust-SDL2_image uses the MIT license.

# Requirements

* [Rust-SDL2](https://github.com/AngryLawyer/rust-sdl2)
* SDL_image 2.0 development libraries
* Rust master -- usage of Nightly is recommended

# Installation

If you're using Cargo to manage your project, enter the following into your
Cargo.toml file:

```toml
[dependencies]
sdl2_image = "0.0.33"
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
