#[cfg(feature="pkg-config")]
extern crate pkg_config;

fn main() {
    if !build_pkgconfig() {
      println!("cargo:rustc-flags=-l SDL2");
    }
}

#[cfg(not(feature="pkg-config"))]
fn build_pkgconfig() -> bool {
    false
}

#[cfg(feature="pkg-config")]
fn build_pkgconfig() -> bool {
    if pkg_config::find_library("sdl2").is_err() {
        panic!("Could not find SDL2 via pkgconfig");
    }
    true
}
