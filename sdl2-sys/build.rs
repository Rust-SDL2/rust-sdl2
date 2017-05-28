#[cfg(feature="pkg-config")]
extern crate pkg_config;

fn main() {
    if !build_pkgconfig() {
        let target = ::std::env::var("TARGET").expect("Cargo build scripts always have TARGET");
        let target_os = target.splitn(3, "-").nth(2).unwrap();

        if cfg!(feature="use_mac_framework") && target_os == "darwin" {
            println!("cargo:rustc-flags=-l framework=SDL2");
        } else {
            println!("cargo:rustc-flags=-l SDL2");
        }

        if target_os == "darwin" {
            println!("cargo:rustc-link-search=/usr/local/lib");
        }
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
