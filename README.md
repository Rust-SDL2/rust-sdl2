# Rust-SDL2 [![Build Status][trav-ci-img]][trav-ci] [![crates.io badge][crates-io-badge]][crates-io-url]

Bindings for SDL2 in Rust

### [Changelog for 0.31](changelog.md#v031)

# Overview

Rust-SDL2 is a library for talking to the new SDL2.0 libraries from Rust.
Low-level C components are wrapped in Rust code to make them more idiomatic and
abstract away inappropriate manual memory management.

Rust-SDL2 uses the MIT license.

If you want a library compatible with earlier versions of SDL, please see
[here][early-sdl]

# Documentation

* [crates.io documentation](https://docs.rs/sdl2/), with no features at all.
* [master documentation](https://rust-sdl2.github.io/rust-sdl2/sdl2/) with the following features:
  * gfx
  * image
  * mixer
  * ttf

# Requirements

## Rust

We currently target the latest stable release of Rust.

## *SDL2.0 development libraries*

### "Bundled"

Since 0.31, this crate supports a feature named "bundled" which downloads SDL2 from source, compiles it and links it automatically. While this should work for any architecture, you **will** need a C compiler (like `gcc`, `clang`, or MS's own compiler) to use this feature properly.

### Linux
Install these through your favourite package management tool, or via
http://www.libsdl.org/

Ubuntu example:
> sudo apt-get install libsdl2-dev

Fedora example:
> sudo dnf install SDL2-devel

You might also need a C compiler (`gcc`).

### Mac OS X
#### If you are using homebrew
On OSX, it's a good idea to install these via
[homebrew][homebrew].

> brew install sdl2

Then add the following to your `~/.bash_profile` if not already present.

> export LIBRARY\_PATH="$LIBRARY\_PATH:/usr/local/lib"

##### Otherwise if you are using macports
You can also get sdl2 via `macports`.

> sudo port install libsdl2

Then add the following to your `~/.bash_profile` if not already present.

> export LIBRARY\_PATH="$LIBRARY\_PATH:/opt/local/lib/"

If you're having issues with either homebrew or macports, [see here][pdev-issue].

#### If you are using the SDL2 framework

You can download and install the SDL2 Mac OS X framework from:
https://www.libsdl.org/download-2.0.php

To make the `sdl2` crate link with the SDL2 framework, you will need to enable
the `use_mac_framework` feature.  To build and test the `sdl2` crate with this
feature, use:

> cargo test --features use\_mac\_framework

To depend on the `sdl2` crate with this feature enabled, put the following in
your project's `Cargo.toml` file:

```toml
[dependencies.sdl2]
features = ["use_mac_framework"]
version = ...  # Whichever version you are using
```

Alternatively, you can re-export the feature in your package by putting the
following in your `Cargo.toml` file:

```toml
[features]
default = []
use_sdl2_mac_framework = ["sdl2/use_mac_framework"]
```

### Windows with build script

1. Download mingw and msvc development libraries from
http://www.libsdl.org/ (SDL2-devel-2.0.x-mingw.tar.gz & SDL2-devel-2.0.x-VC.zip).
2. Unpack to folders of your choosing (You can delete it afterwards).
3. Create the following folder structure in the same folder as your Cargo.toml:

```
gnu-mingw\dll\32
gnu-mingw\dll\64
gnu-mingw\lib\32
gnu-mingw\lib\64
msvc\dll\32
msvc\dll\64
msvc\lib\32
msvc\lib\64
```

4. Copy the lib and dll files from the source archive to the directories we created in step 3 like so:
```
SDL2-devel-2.0.x-mingw.tar.gz\SDL2-2.0.x\i686-w64-mingw32\bin 		-> 	gnu-mingw\dll\32
SDL2-devel-2.0.x-mingw.tar.gz\SDL2-2.0.x\x86_64-w64-mingw32\bin 	-> 	gnu-mingw\dll\64
SDL2-devel-2.0.x-mingw.tar.gz\SDL2-2.0.x\i686-w64-mingw32\lib 		-> 	gnu-mingw\lib\32
SDL2-devel-2.0.x-mingw.tar.gz\SDL2-2.0.x\x86_64-w64-mingw32\lib 	-> 	gnu-mingw\lib\64
SDL2-devel-2.0.5-VC.zip\SDL2-2.0.x\lib\x86\*.dll	 		-> 	msvc\dll\32
SDL2-devel-2.0.5-VC.zip\SDL2-2.0.x\lib\x64\*.dll 			-> 	msvc\dll\64
SDL2-devel-2.0.5-VC.zip\SDL2-2.0.x\lib\x86\*.lib	 		-> 	msvc\lib\32
SDL2-devel-2.0.5-VC.zip\SDL2-2.0.x\lib\x64\*.lib	 		-> 	msvc\lib\64
```

5. Create a build script, if you don't already have one put this in your Cargo.toml under `[package]`:
> build = "build.rs"

6. Create a file in the same directory as Cargo.toml called build.rs (if you didn't already have a build script) and paste this into it:

```Rust
use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("pc-windows") {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let mut lib_dir = manifest_dir.clone();
        let mut dll_dir = manifest_dir.clone();
        if target.contains("msvc") {
            lib_dir.push("msvc");
            dll_dir.push("msvc");
        }
        else {
            lib_dir.push("gnu-mingw");
            dll_dir.push("gnu-mingw");
        }
        lib_dir.push("lib");
        dll_dir.push("dll");
        if target.contains("x86_64") {
            lib_dir.push("64");
            dll_dir.push("64");
        }
        else {
            lib_dir.push("32");
            dll_dir.push("32");
        }
        println!("cargo:rustc-link-search=all={}", lib_dir.display());
        for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir")  {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();
            let mut new_file_path = manifest_dir.clone();
            if let Some(file_name) = file_name_result {
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".dll") {
                    new_file_path.push(file_name);
                    std::fs::copy(&entry_path, new_file_path.as_path()).expect("Can't copy from DLL dir");
                }
            }
        }
    }
}
```

7. On build the build script will copy the needed DLLs into the same directory as your Cargo.toml, you probably don't want to commit these to any Git repositories though so add the following line to your .gitignore file

`/*.dll`

8. When you're shipping your game make sure to copy the corresponding SDL2.dll to the same directory that your compiled exe is in, otherwise the game won't launch.

And now your project should build and run on any Windows computer!

### Windows (MinGW)

1. Download mingw development libraries from
http://www.libsdl.org/ (SDL2-devel-2.0.x-mingw.tar.gz).
2. Unpack to a folder of your choosing (You can delete it afterwards).
3. Copy all lib files from
    > SDL2-devel-2.0.x-mingw\SDL2-2.0.x\x86_64-w64-mingw32\lib

    to (for Rust 1.6 and above)
    > C:\Program Files\Rust\\**lib**\rustlib\x86_64-pc-windows-gnu\lib

    or to (for Rust versions 1.5 and below)
    > C:\Program Files\Rust\\**bin**\rustlib\x86_64-pc-windows-gnu\lib

    or to your library folder of choice, and ensure you have a system environment variable of
    > LIBRARY_PATH = C:\your\rust\library\folder

	For Rustup users, this folder will be in
	> C:\Users\\{Your Username}\.multirust\toolchains\\{current toolchain}\lib\rustlib\\{current toolchain}\lib

  Where current toolchain is likely `stable-x86_64-pc-windows-gnu`.

4. Copy SDL2.dll from
    > SDL2-devel-2.0.x-mingw\SDL2-2.0.x\x86_64-w64-mingw32\bin

    into your cargo project, right next to your Cargo.toml.

5. When you're shipping your game make sure to copy SDL2.dll to the same directory that your compiled exe is in, otherwise the game won't launch.

### Windows (MSVC)

1. Download MSVC development libraries from http://www.libsdl.org/ (SDL2-devel-2.0.x-VC.zip).
2. Unpack SDL2-devel-2.0.x-VC.zip to a folder of your choosing (You can delete it afterwards).
3. Copy all lib files from
    > SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64\

    to (for Rust 1.6 and above)
    > C:\Program Files\Rust\\**lib**\rustlib\x86_64-pc-windows-msvc\lib

    or to (for Rust versions 1.5 and below)
    > C:\Program Files\Rust\\**bin**\rustlib\x86_64-pc-windows-msvc\lib

    or to your library folder of choice, and ensure you have a system environment variable of
    > LIB = C:\your\rust\library\folder

	For Rustup users, this folder will be in
	> C:\Users\\{Your Username}\.multirust\toolchains\\{current toolchain}\lib\rustlib\\{current toolchain}\lib

  Where current toolchain is likely `stable-x86_64-pc-windows-msvc`.

4. Copy SDL2.dll from
    > SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64\

    into your cargo project, right next to your Cargo.toml.

 5. When you're shipping your game make sure to copy SDL2.dll to the same directory that your compiled exe is in, otherwise the game won't launch.

# Installation

If you're using [cargo][crates] to manage your project, you can
download through Crates.io:

```toml
    [dependencies]
    sdl2 = "0.30"
```

Alternatively, pull it from GitHub to obtain the latest version from master

```toml
    [dependencies.sdl2]
    git = "https://github.com/AngryLawyer/rust-sdl2"
```

Otherwise, clone this repo and run [cargo][crates]

```
cargo build
```

You can enable features such as ttf, image, gfx and mixer by
adding this instead:

```toml
    [dependencies.sdl2]
    version = "0.30"
    default-features = false
    features = ["ttf","image","gfx","mixer"]
```

Those features need their respective libraries, which
can be found at these locations : (the install process
is the same as SDL2)

* [image, ttf, mixer](https://www.libsdl.org/projects/)
* [gfx](http://sourceforge.net/projects/sdl2gfx/)

## What about sdl2\_net ?

As of now, sdl2\_net is meaningless compared to what other crates
such as `serde` and `bincode` can offer.
We highly recommend using those to develop anything UDP or TCP
related (along with futures or TCP/UDP from the standard library).

If you still want an implementation of sdl2\_net, you can try to
add it in this repo as a feature via a Pull Request. A somewhat
outdated version of this binding can be found
[here](https://github.com/Limvot/rust-sdl2_net)

# Demo

We have several simple example projects included:

> cargo run --example demo

You can see the full list in the `examples/` folder. Some examples require some features, you can enable them like so:

> cargo run --example gfx-demo --features "gfx"

Replace "gfx" by the feature(s) needed for the example you want.

# About the `unsafe_textures` feature

In the `sdl2::render` module, `Texture` has by default lifetimes to prevent it from out-living its parent `TextureCreator`.
These lifetimes are sometimes too hard to deal with in Rust, and so you have the option to enable the `unsafe_textures` feature.

This removes the lifetimes on the `Texture`s, at the cost of optional manual memory management. If you want to manually destroy
the `Texture`s you use, you can call the `destroy` method of your `Texture`s, but beware that *it should not* be called if none of
the parents (`Canvas` or `TextureCreator`) are alive. If you do not call this method, the memory will simply be freed when
the last `Canvas` or the last `TextureCreator` will be freed.

There is no online documentation for this feature, however you can build it yourself in your project by enabling the feature in your
Cargo.toml, running `cargo doc` and accessing `target/doc/sdl2/index.html` via a browser.

# Generating sdl2-sys with bindgen

The sdl2-sys that was generated for this crate is very generic and can be used on a lot of platforms with very few limitations. However,
you may sometimes face trouble when using platform-specific features of SDL2, for instance the WindowManager category.

The feature "use-bindgen" allows you to avoid this limitation by generating the proper bindings depending on your target. It will take
the headers based on what `pkg-config` outputs (if you enabled the feature "use-pkg-config") and generate bindings based on them.
If you don't have pkg-config or disabled the feature, it will try to get the headers in `SDL-2.0.6/include` of this crate instead.

If somehow you have your own headers that you want to use (use a beta version, an older version, ...),
you can set the environment variable "SDL2_INCLUDE_PATH" and those headers will be used by bindgen instead.

# OpenGL

If you want to use OpenGL, you also need the
[gl-rs][gl-rs] package. If you're using
[cargo][crates], just add these lines to your Cargo.toml:

```toml
    [dependencies.gl]
    git = "https://github.com/bjz/gl-rs"
```

You have two options to use OpenGL with sdl2:

* Use OpenGL with Canvas and use sdl2::render 
* Use OpenGL directly on the Window "shell" and use manual OpenGL calls to render something

## Use sdl2::render

Then you need to add some initialization code to establish the bindings.

First, find the OpenGL driver from SDL:

```rust
fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}
```

Next, initialize the SDL2 subsystems, and create your window with the OpenGL canvas:

```rust

let sdl_context = sdl2::init().unwrap();
let video_subsystem = sdl_context.video().unwrap();
let window = video_subsystem.window("Window", 800, 600)
    .opengl()
    .build()
    .unwrap();
let canvas = window.into_canvas()
    .index(find_sdl_gl_driver().unwrap())
    .build()
    .unwrap();

gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
canvas.window().gl_set_context_to_current();

unsafe {
    gl::ClearColor(0.6, 0.0, 0.8, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
}

canvas.present();

// opengl code here
```

Be wary though, sdl2 has its own internal state which you should avoid messing with.
Avoid using manual OpenGL in the middle of SDL2 calls, or change the state in between.

**You cannot override the OpenGL version with this method unless via changing the gl state**

## Use OpenGL calls manually

```rust
extern crate sdl2;
extern crate gl;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem.window("Window", 800, 600)
        .opengl()
        .build()
        .unwrap();

    let ctx = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    
    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (3, 3));

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        unsafe {
            gl::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
```

This method is useful when you don't care about sdl2's render capabilities, but you do care about
its audio, controller and other neat features that sdl2 has.

# When things go wrong
Rust, and Rust-SDL2, are both still heavily in development, and you may run
into teething issues when using this. Before panicking, check that you're using
the latest version of both Rust and Cargo, check that you've updated Rust-SDL2
to the latest version, and run `cargo clean`. If that fails, please let us know
on the issue tracker.

# Contributing

Any Pull Request is welcome, however small your contribution may be ! There are, however, conditions to contribute:

* New features must be properly documented, be it via examples or inline documentation (via `cargo doc`). Documentation must be for the end user as well as your next fellow contributor.
* Breaking changes must have a proper argumentation with it. While the pre-1.0 state of this crate allows us to be somewhat unstable, **useless breaking changes will be denied**.
* Minor changes, breaking changes and new features added via Pull Request must be added in the [changelog][changelog] file. It is now **mandatory** to log your changes in the changelog. A short description with a link to your commit/pull request within GitHub is fine. Internal, documentation or meta-changes (travis build change, README instructions updates, ...) don't have to be added in the changelog.

[changelog]: ./changelog.md
[crates-io-badge]: https://img.shields.io/crates/v/sdl2.svg
[crates-io-url]: https://crates.io/crates/sdl2
[trav-ci-img]: https://travis-ci.org/Rust-SDL2/rust-sdl2.png?branch=master
[trav-ci]: https://travis-ci.org/Rust-SDL2/rust-sdl2
[early-sdl]: https://github.com/brson/rust-sdl
[homebrew]: http://brew.sh/
[crates]: http://crates.io/
[examples]: https://github.com/jdeseno/rs-sdl2-examples
[gl-rs]: https://github.com/bjz/gl-rs
[pdev-issue]: https://github.com/PistonDevelopers/rust-empty/issues/175
