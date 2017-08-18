#[cfg(feature="pkg-config")]
extern crate pkg_config;
#[cfg(feature="bundled")]
extern crate cmake;
#[cfg(feature="bundled")]
extern crate tar;
#[cfg(feature="bundled")]
extern crate flate2;
#[cfg(feature="bundled")]
extern crate reqwest;

use std::{io, env};
use std::path::Path;
use std::fs;

struct TargetInfo {
    _triple: String,
    os: String,
}

impl TargetInfo {
    fn init() -> TargetInfo {
        let triple = ::std::env::var("TARGET").expect("Cargo build scripts always have TARGET");
        let os = triple.splitn(3, "-").nth(2).unwrap().to_string();
        TargetInfo {
            _triple: triple,
            os: os,
        }
    }
}

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
    const SDL_VERSION: &'static str = "2.0.5";
    let target_info = TargetInfo::init();
    let sdl2_archive_name = format!("SDL2-{}.tar.gz", SDL_VERSION);
    let sdl2_archive_url = format!("http://libsdl.org/release/{}", sdl2_archive_name);

    let out_dir = env::var("OUT_DIR").unwrap();

    let sdl2_archive_path = Path::new(&out_dir).join(sdl2_archive_name);
    let sdl2_build_path = Path::new(&out_dir).join(format!("SDL2-{}", SDL_VERSION));
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
    if target_info.os == "windows-msvc" {
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
    } else if target_info.os.contains("linux") {
        println!("cargo:rustc-link-lib=sndio");
    } else {
        // TODO: Add other platform linker options here.
    }
}

#[cfg(not(feature="bundled"))]
fn main() {
    let target_info = TargetInfo::init();

    if !build_pkgconfig() {
        if cfg!(feature="use_mac_framework") && target_info.os == "darwin" {
            println!("cargo:rustc-flags=-l framework=SDL2");
        } else {
            println!("cargo:rustc-flags=-l SDL2");
        }
    }

    if target_info.os == "ios" {
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
    if pkg_config::find_library("sdl2").is_err() {
        panic!("Could not find SDL2 via pkgconfig");
    }
    true
}
