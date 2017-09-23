#![allow(unused_imports, dead_code, unused_variables)]

#[cfg(feature = "pkg-config")]
extern crate pkg_config;
#[cfg(feature = "bindgen")]
extern crate bindgen;

use std::path::PathBuf;
use std::env;
use std::fs;

const SDL2_BUNDLED_VERSION: &str = "2.0.6";

fn main() {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let host = env::var("HOST").expect("Cargo build scripts always have HOST");

    prepare_bindings(&target, &host);

    if get_os_from_triple(&target).unwrap() == "ios" {
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

#[cfg(not(feature = "pkg-config"))]
fn build_pkgconfig() -> bool {
    false
}

#[cfg(feature = "pkg-config")]
fn build_pkgconfig() -> bool {
    pkg_config::probe_library("sdl2").is_ok()
}

#[cfg(not(feature = "bindgen"))]
fn prepare_bindings(target: &str, host: &str) {
    add_explicit_linker_flags(get_os_from_triple(target).unwrap());
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    fs::copy(crate_path.join("pregenerated_bindings.rs"), out_path.join("bindings.rs"))
        .expect("Couldn't find pregenerated bindings!");
}

#[cfg(feature = "bindgen")]
fn prepare_bindings(target: &str, host: &str) {
    let mut bindings = bindgen::Builder::default();

    // Set correct target triple when cross-compiling
    if target != host {
        bindings = bindings.clang_arg("-target");
        bindings = bindings.clang_arg(target.clone());
    }

    if let Ok(include_path) = env::var("SDL2_INCLUDE_PATH") {
        bindings = bindings.clang_arg(String::from("-I") + &include_path);
        add_explicit_linker_flags(get_os_from_triple(target).unwrap());
    } else if build_pkgconfig() {
        #[cfg(feature = "pkg-config")]
        for path in &pkg_config::find_library("sdl2").unwrap().include_paths {
            bindings = bindings.clang_arg(String::from("-I") +
                                          &path.clone().into_os_string().into_string().unwrap());
        }
    } else {
        let mut include_path: PathBuf = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        include_path.push(String::from("SDL2-") + SDL2_BUNDLED_VERSION);
        include_path.push("include");
        bindings = bindings.clang_arg(String::from("-I") +
                                      &include_path.into_os_string().into_string().unwrap());
        add_explicit_linker_flags(get_os_from_triple(target).unwrap());
    }

    let bindings = bindings
        .header("wrapper.h")
        .hide_type("_")
        .hide_type("FP_NAN")
        .hide_type("FP_INFINITE")
        .hide_type("FP_ZERO")
        .hide_type("FP_SUBNORMAL")
        .hide_type("FP_NORMAL") // Until https://github.com/rust-lang-nursery/rust-bindgen/issues/687 gets fixed
        .hide_type("max_align_t") // Until https://github.com/rust-lang-nursery/rust-bindgen/issues/550 gets fixed
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn get_os_from_triple(triple: &str) -> Option<&str>
{
    triple.split("-").nth(2)
}

fn add_explicit_linker_flags(target_os: &str) {
    if cfg!(feature = "use_mac_framework") && target_os == "darwin" {
        println!("cargo:rustc-flags=-l framework=SDL2");
    } else {
        println!("cargo:rustc-flags=-l SDL2");
    }
}
