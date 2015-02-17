#[cfg(feature="pkg-config")]
extern crate "pkg-config" as pkg_config;

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
    let opts = pkg_config::default_options("sdl2");
    if pkg_config::find_library_opts("sdl2", &opts).is_err() {
        panic!("Could not find SDL2 via pkgconfig");
    }
    true
}
