Rust-SDL2_image
===============

Rust bindings for SDL2_image

Requirements
------------

* [Rust-sdl2](https://github.com/AngryLawyer/rust-sdl2)
* sdl_image 2.0 development libraries
* Rust master

Installation
------------

```
git clone https://github.com/xsleonard/rust-sdl2_image
cd rust-sdl2_image
rustc src/sdl2_image/lib.rs
# OR if you are using the mac framework version
rustc --cfg mac_framework src/sdl2_image/lib.rs
```

Demo
----

```
rustc -L. src/demo/main.rs -o demo
./demo image.[png|jpg]
```
