Rust-SDL2_ttf
===============

Rust bindings for SDL2_ttf

Requirements
------------

* [Rust-sdl2](https://github.com/AngryLawyer/rust-sdl2)
* sdl_ttf 2.0 development libraries
* Rust master

Installation
------------

```
git clone https://github.com/andelf/rust-sdl2_ttf
cd rust-sdl2_ttf
rustc src/sdl2_ttf/lib.rs
# OR if you are using the mac framework version
rustc --cfg mac_framework src/sdl2_ttf/lib.rs
```

Demo
----

```
rustc -L. src/demo/main.rs -o demo
./demo font.[ttf|ttc|fon]
```
