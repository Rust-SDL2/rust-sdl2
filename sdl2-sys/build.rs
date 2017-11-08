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

#[macro_use]
extern crate cfg_if;

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

#[cfg(feature = "use-pkgconfig")]
#[cfg(feature = "static-link")]
fn get_pkg_config() -> pkg_config::Library {
    pkg_config::Config::new()
        .statik(true)
        .probe("sdl2").unwrap()
}

#[cfg(feature = "use-pkgconfig")]
#[cfg(not(feature = "static-link"))]
fn get_pkg_config() -> pkg_config::Library {
    pkg_config::Config::new()
        .statik(false)
        .probe("sdl2").unwrap()
}

// returns the location of the downloaded source
#[cfg(feature = "bundled")]
fn download_sdl2() -> PathBuf {
    let out_dir = env::var("OUT_DIR").unwrap();
    
    let sdl2_archive_name = format!("SDL2-{}.tar.gz", LASTEST_SDL2_VERSION);
    let sdl2_archive_url = format!("http://libsdl.org/release/{}", sdl2_archive_name);
    
    let sdl2_archive_path = Path::new(&out_dir).join(sdl2_archive_name);
    let sdl2_build_path = Path::new(&out_dir).join(format!("SDL2-{}", LASTEST_SDL2_VERSION));

    // avoid re-downloading the archive if it already exists    
    if !sdl2_archive_path.exists() {
        let sdl2_archive = fs::File::create(&sdl2_archive_path).unwrap();
        download_to(&sdl2_archive_url, &sdl2_archive);
    }

    let reader = flate2::read::GzDecoder::new(
        fs::File::open(&sdl2_archive_path).unwrap()
    ).unwrap();
    let mut ar = tar::Archive::new(reader);
    ar.unpack(&out_dir).unwrap();

    sdl2_build_path
}

// compile a shared or static lib depending on the feature 
#[cfg(feature = "bundled")]
fn compile_sdl2(sdl2_build_path: &Path) -> PathBuf {
    let install_path = if cfg!(feature = "static-link") {
        cmake::Config::new(sdl2_build_path)
            .define("SDL_SHARED", "OFF")
            .define("SDL_STATIC", "ON")
            .build()
    } else {
        cmake::Config::new(sdl2_build_path)
            .define("SDL_SHARED", "ON")
            .define("SDL_STATIC", "OFF")
            .build()
    };

    install_path
}

#[cfg(not(feature = "bundled"))]
fn compute_include_paths() -> Vec<String> {
    let mut include_paths: Vec<String> = vec!();
    
    if let Ok(include_path) = env::var("SDL2_INCLUDE_PATH") {
        include_paths.push(format!("{}", include_path));
    };

    #[cfg(feature = "pkg-config")] {
        // don't print the "cargo:xxx" directives, we're just trying to get the include paths here
        let pkg_config_library = pkg_config::Config::new().print_system_libs(false).probe("sdl2").unwrap();
        for path in pkg_config_library.include_paths {
            include_paths.push(format!("{}", path.display()));
        };
    }

    include_paths
}

fn link_sdl2(target_os: &str) {
    #[cfg(all(feature = "use-pkgconfig", not(feature = "bundled")))] {
        // prints the appropriate linking parameters when using pkg-config
        // useless when using "bundled"
        get_pkg_config();
    }

    #[cfg(not(feature = "static-link"))] {
        if target_os == "ios" {
            // iOS requires additional linking to function properly
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

        // pkg-config automatically prints this output when probing,
        // however pkg_config isn't used with the feature "bundled"
        if cfg!(feature = "bundled") || cfg!(not(feature = "use-pkgconfig")) { 
            if cfg!(feature = "use_mac_framework") && target_os == "darwin" {
                println!("cargo:rustc-flags=-l framework=SDL2");
            } else {
                println!("cargo:rustc-flags=-l SDL2");
            }
        }
    }

    #[cfg(feature = "static-link")] {
        if cfg!(feature = "bundled") || cfg!(feature = "use-pkgconfig") == false { 
            println!("cargo:rustc-link-lib=static=SDL2main");
            println!("cargo:rustc-link-lib=static=SDL2");
        }

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
        } else if target_os.contains("linux") {
            println!("cargo:rustc-link-lib=sndio");
        } else if target_os == "darwin" {
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=Carbon");
            println!("cargo:rustc-link-lib=framework=ForceFeedback");
            println!("cargo:rustc-link-lib=framework=CoreVideo");
            println!("cargo:rustc-link-lib=framework=CoreAudio");
            println!("cargo:rustc-link-lib=framework=AudioToolbox");
            println!("cargo:rustc-link-lib=iconv");
        } else {
            // TODO: Add other platform linker options here.
        }
    }
}

fn main() {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let host = env::var("HOST").expect("Cargo build scripts always have HOST");
    let target_os = get_os_from_triple(target.as_str()).unwrap();

    #[cfg(feature = "bundled")] {
        let sdl2_source_path = download_sdl2();
        let sdl2_compiled_path = compile_sdl2(sdl2_source_path.as_path());

        let sdl2_downloaded_include_path = sdl2_source_path.join("include");
        let sdl2_compiled_lib_path = sdl2_compiled_path.join("lib");

        println!("cargo:rustc-link-search={}", sdl2_compiled_lib_path.display());
        
        #[cfg(feature = "bindgen")] {
            let include_paths = vec!(String::from(sdl2_downloaded_include_path.to_str().unwrap()));
            generate_bindings(target.as_str(), host.as_str(), include_paths.as_slice())
        }
    };

    #[cfg(all(not(feature = "bundled"), feature = "bindgen"))] {
        let include_paths: Vec<String> = compute_include_paths();
        generate_bindings(target.as_str(), host.as_str(), include_paths.as_slice())
    }

    #[cfg(not(feature = "bindgen"))] {
        copy_pregenerated_bindings();
    }

    link_sdl2(target_os);
}

#[cfg(not(feature = "bindgen"))]
fn copy_pregenerated_bindings() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    fs::copy(crate_path.join("pregenerated_bindings.rs"), out_path.join("bindings.rs"))
        .expect("Couldn't find pregenerated bindings!");
}

#[cfg(feature = "bindgen")]
// headers_path is a list of directories where the SDL2 headers are expected
// to be found by bindgen (should point to the include/ directories)
fn generate_bindings<S: AsRef<str> + ::std::fmt::Debug>(target: &str, host: &str, headers_paths: &[S]) {
    let target_os = get_os_from_triple(target).unwrap();
    let mut bindings = bindgen::Builder::default();

    // Set correct target triple for bindgen when cross-compiling
    if target != host {
        bindings = bindings.clang_arg("-target");
        bindings = bindings.clang_arg(target.clone());
    }

    if headers_paths.len() == 0 {
        // if no paths are being provided, fall back to the headers included in this repo
        let mut include_path: PathBuf = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        include_path.push(format!("SDL2-{}", SDL2_HEADERS_BUNDLED_VERSION));
        include_path.push("include");
        bindings = bindings.clang_arg(format!("-I{}", include_path.display()));
    } else {
        // if paths are included, use them for bindgen. Bindgen should use the first one.
        for headers_path in headers_paths {
            bindings = bindings.clang_arg(format!("-I{}", headers_path.as_ref()))
        }
    }

    if target_os == "windows-msvc" {
        bindings = bindings.clang_arg(format!("-IC:/Program Files (x86)/Windows Kits/8.1/Include/shared"));
        bindings = bindings.clang_arg(format!("-IC:/Program Files/LLVM/lib/clang/5.0.0/include"));
        bindings = bindings.clang_arg(format!("-IC:/Program Files (x86)/Windows Kits/10/Include/10.0.10240.0/ucrt"));
        bindings = bindings.clang_arg(format!("-IC:/Program Files (x86)/Microsoft Visual Studio 14.0/VC/include"));
        bindings = bindings.clang_arg(format!("-IC:/Program Files (x86)/Windows Kits/8.1/Include/um"));
    };

    // SDL2 hasn't a default configuration for Linux
    if target_os == "linux" {
        bindings = bindings.clang_arg("-DSDL_VIDEO_DRIVER_X11");
        bindings = bindings.clang_arg("-DSDL_VIDEO_DRIVER_WAYLAND");
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
    triple.splitn(3, "-").nth(2)
}
