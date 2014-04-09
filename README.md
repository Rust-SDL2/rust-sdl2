Rust-SDL2_mixer
===============

Rust bindings for SDL2_mixer

Requirements
------------

* [Rust-sdl2](https://github.com/AngryLawyer/rust-sdl2)
* sdl_mixer 2.0 development libraries
* Rust master

Installation
------------

```
git clone https://github.com/andelf/rust-sdl2_mixer
cd rust-sdl2_mixer
rustc src/sdl2_mixer/lib.rs
# OR if you are using the mac framework version
rustc --cfg mac_framework src/sdl2_mixer/lib.rs
```

Demo
----

```
rustc -L. src/demo/demo.rs
./demo mixer.[wav|mp3|ogg]
```
