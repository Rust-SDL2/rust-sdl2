#![allow(unused_imports, dead_code, unused_variables)]

#[cfg(feature = "pkg-config")]
extern crate pkg_config;
#[cfg(feature = "bindgen")]
extern crate bindgen;
#[cfg(feature="bundled")]
extern crate cmake;
#[cfg(feature="bundled")]
extern crate tar;
#[cfg(feature="bundled")]
extern crate flate2;
#[cfg(feature="bundled")]
extern crate reqwest;

use std::path::{Path, PathBuf};
use std::{io, fs, env};

// corresponds to the headers that we have in sdl2-sys/SDL2-{version}
const SDL2_HEADERS_BUNDLED_VERSION: &str = "2.0.6";

// means the lastest stable version that can be downloaded from SDL2's source
const LASTEST_SDL2_VERSION: &str = "2.0.5";

#[cfg(feature="bundled")]
fn download_to<T: io::Write>(url: &str, mut dest: T) {
    use io::BufRead;

    let resp = reqwest::get(url).expect(&format!("Failed to GET resource: {:?}", url));
    let size = resp.headers()
        .get::<reqwest::header::ContentLength>()
        .map(|ct_len| **ct_len)
        .unwrap_or(0);
    if !resp.status().is_success() { panic!("Download request failed with status: {:?}", resp.status()) }
    if size == 0 { panic!("Size of content was returned was 0") }

    let mut src = io::BufReader::new(resp);
    loop {
        let n = {
            let mut buf = src.fill_buf().unwrap();
            dest.write_all(&mut buf).unwrap();
            buf.len()
        };
        if n == 0 { break; }
        src.consume(n);
    }
}

#[cfg(feature="bundled")]
fn main() {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let host = env::var("HOST").expect("Cargo build scripts always have HOST");
    let target_os = get_os_from_triple(target.as_str()).unwrap();

    prepare_bindings(&target, &host);
    
    let sdl2_archive_name = format!("SDL2-{}.tar.gz", LASTEST_SDL2_VERSION);
    let sdl2_archive_url = format!("http://libsdl.org/release/{}", sdl2_archive_name);

    let out_dir = env::var("OUT_DIR").unwrap();

    let sdl2_archive_path = Path::new(&out_dir).join(sdl2_archive_name);
    let sdl2_build_path = Path::new(&out_dir).join(format!("SDL2-{}", LASTEST_SDL2_VERSION));
    if !sdl2_archive_path.exists() {
        let sdl2_archive = fs::File::create(&sdl2_archive_path).unwrap();
        download_to(&sdl2_archive_url, &sdl2_archive);
    }
    let reader = flate2::read::GzDecoder::new(
        fs::File::open(&sdl2_archive_path).unwrap()
    ).unwrap();
    let mut ar = tar::Archive::new(reader);
    ar.unpack(&out_dir).unwrap();

    let install_path = cmake::Config::new(sdl2_build_path)
        .define("SDL_SHARED", "OFF")
        .define("SDL_STATIC", "ON")
        .build();

    println!("cargo:rustc-link-search={}", install_path.join("lib").display());
    println!("cargo:rustc-link-lib=static=SDL2main");
    println!("cargo:rustc-link-lib=static=SDL2");

    // Also linked to any required libraries for each supported platform
    if target_os == "windows-msvc" {
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=winmm");
        println!("cargo:rustc-link-lib=imm32");
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=oleaut32");
        println!("cargo:rustc-link-lib=version");
        println!("cargo:rustc-link-lib=uuid");
        println!("cargo:rustc-link-lib=dinput8");
        println!("cargo:rustc-link-lib=dxguid");
    } else {
        // TODO: Add other platform linker options here.
    }
}

#[cfg(not(feature="bundled"))]
fn main() {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let host = env::var("HOST").expect("Cargo build scripts always have HOST");
    let target_os = get_os_from_triple(target.as_str()).unwrap();

    prepare_bindings(&target, &host);

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
#[cfg(not(feature="bundled"))]
fn build_pkgconfig() -> bool {
    false
}

#[cfg(feature="pkg-config")]
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
        // SDL2 hasn't a default configuration for Linux
        if get_os_from_triple(target).unwrap() == "linux" {
            bindings = bindings.clang_arg("-DSDL_VIDEO_DRIVER_X11");
            bindings = bindings.clang_arg("-DSDL_VIDEO_DRIVER_WAYLAND");
        }
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
