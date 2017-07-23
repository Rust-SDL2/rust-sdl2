#[cfg(feature="pkg-config")]
extern crate pkg_config;

fn main() {
    let target = ::std::env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let target_os = target.splitn(3, "-").nth(2).unwrap();

    if !build_pkgconfig() {
        if cfg!(feature="use_mac_framework") && target_os == "darwin" {
            println!("cargo:rustc-flags=-l framework=SDL2");
        } else {
            println!("cargo:rustc-flags=-l SDL2");
        }
    }

    if target_os == "ios" {
        println!("cargo:rustc-flags=-l framework=AVFoundation");
        println!("cargo:rustc-flags=-l framework=AudioToolbox");
        println!("cargo:rustc-flags=-l framework=CoreAudio");
        println!("cargo:rustc-flags=-l framework=CoreGraphics");
        println!("cargo:rustc-flags=-l framework=CoreMotion");
        println!("cargo:rustc-flags=-l framework=Foundation");
        println!("cargo:rustc-flags=-l framework=GameController");
        println!("cargo:rustc-flags=-l framework=OpenGLES");
        println!("cargo:rustc-flags=-l framework=QuartzCore");
        println!("cargo:rustc-flags=-l framework=UIKit");
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
