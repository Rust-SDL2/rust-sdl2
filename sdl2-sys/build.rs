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
const SDL2_HEADERS_BUNDLED_VERSION: &str = "2.0.8";

// means the lastest stable version that can be downloaded from SDL2's source
const LASTEST_SDL2_VERSION: &str = "2.0.8";
        
macro_rules! add_msvc_includes_to_bindings {
    ($bindings:expr) => {
        $bindings = $bindings.clang_arg(format!("-IC:/Program Files (x86)/Windows Kits/8.1/Include/shared"));
        $bindings = $bindings.clang_arg(format!("-IC:/Program Files/LLVM/lib/clang/5.0.0/include"));
        $bindings = $bindings.clang_arg(format!("-IC:/Program Files (x86)/Windows Kits/10/Include/10.0.10240.0/ucrt"));
        $bindings = $bindings.clang_arg(format!("-IC:/Program Files (x86)/Microsoft Visual Studio 14.0/VC/include"));
        $bindings = $bindings.clang_arg(format!("-IC:/Program Files (x86)/Windows Kits/8.1/Include/um"));
    };
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

#[cfg(feature = "use-pkgconfig")]
fn pkg_config_print(statik: bool, lib_name: &str) {
    pkg_config::Config::new()
        .statik(statik)
        .probe(lib_name).unwrap();
}

#[cfg(feature = "use-pkgconfig")]
fn get_pkg_config() {
    let statik: bool = if cfg!(feature = "static-link") { true } else { false };

    pkg_config_print(statik, "sdl2");
    if cfg!(feature = "image") {
        pkg_config_print(statik, "SDL2_image");
    }
    if cfg!(feature = "ttf") {
        pkg_config_print(statik, "SDL2_ttf");
    }
    if cfg!(feature = "mixer") {
        pkg_config_print(statik, "SDL2_mixer");
    }
    if cfg!(feature = "gfx") {
        pkg_config_print(statik, "SDL2_gfx");
    }
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
fn compile_sdl2(sdl2_build_path: &Path, target_os: &str) -> PathBuf {
    let mut cfg = cmake::Config::new(sdl2_build_path);

    if target_os == "windows-gnu" {
        cfg.define("VIDEO_OPENGLES", "OFF");
    }

    if cfg!(feature = "static-link") {
        cfg.define("SDL_SHARED", "OFF");
        cfg.define("SDL_STATIC", "ON");
    } else {
        cfg.define("SDL_SHARED", "ON");
        cfg.define("SDL_STATIC", "OFF");
    }

    cfg.build()
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
            println!("cargo:rustc-link-lib=static=SDL2maind");
            println!("cargo:rustc-link-lib=static=SDL2d");
        }

        // Also linked to any required libraries for each supported platform
        if target_os.contains("windows") {
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
    // SDL libraries seem to not be packed with pkgconfig file on all distros,
    // and in the same distros (fedora at least) a symlink is also missing.
    //
    // Linking directly with file is not possible with cargo since the
    // ':filename' syntax is used for renaming of libraries, which basically
    // leaves it up to the user to make a symlink to the shared object so
    // -lSDL2_mixer can find it.
    #[cfg(all(not(feature = "use-pkgconfig"), not(feature = "static-link")))] {
        if cfg!(feature = "mixer") {
            if cfg!(any(target_os="linux", target_os="freebsd")) {
                println!("cargo:rustc-flags=-l SDL2_mixer");
            } else if cfg!(target_os="windows") {
                println!("cargo:rustc-flags=-l SDL2_mixer");
            } else if cfg!(target_os="macos") {
                if cfg!(any(mac_framework, feature="use_mac_framework")) {
                    println!("cargo:rustc-flags=-l framework=SDL2_mixer");
                } else {
                    println!("cargo:rustc-flags=-l SDL2_mixer");
                }
            }
        }
        if cfg!(feature = "image") {
            if cfg!(any(target_os="linux", target_os="freebsd")) {
                println!("cargo:rustc-flags=-l SDL2_image");
            } else if cfg!(target_os="windows") {
                println!("cargo:rustc-flags=-l SDL2_image");
            } else if cfg!(target_os="macos") {
                if cfg!(any(mac_framework, feature="use_mac_framework")) {
                    println!("cargo:rustc-flags=-l framework=SDL2_image");
                } else {
                    println!("cargo:rustc-flags=-l SDL2_image");
                }
            }
        }
        if cfg!(feature = "ttf") {
            if cfg!(any(target_os="linux", target_os="freebsd")) {
                println!("cargo:rustc-flags=-l SDL2_ttf");
            } else if cfg!(target_os="windows") {
                println!("cargo:rustc-flags=-l SDL2_ttf");
            } else if cfg!(target_os="macos") {
                if cfg!(any(mac_framework, feature="use_mac_framework")) {
                    println!("cargo:rustc-flags=-l framework=SDL2_ttf");
                } else {
                    println!("cargo:rustc-flags=-l SDL2_ttf");
                }
            }
        }
        if cfg!(feature = "gfx") {
            if cfg!(any(target_os="linux", target_os="freebsd")) {
                println!("cargo:rustc-flags=-l SDL2_gfx");
            } else if cfg!(target_os="windows") {
                println!("cargo:rustc-flags=-l SDL2_gfx");
            } else if cfg!(target_os="macos") {
                if cfg!(any(mac_framework, feature="use_mac_framework")) {
                    println!("cargo:rustc-flags=-l framework=SDL2_gfx");
                } else {
                    println!("cargo:rustc-flags=-l SDL2_gfx");
                }
            }
        }
    }
}

fn main() {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let host = env::var("HOST").expect("Cargo build scripts always have HOST");
    let target_os = get_os_from_triple(target.as_str()).unwrap();

    #[cfg(feature = "bundled")] {
        let sdl2_source_path = download_sdl2();
        let sdl2_compiled_path = compile_sdl2(sdl2_source_path.as_path(), target_os);

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
    fs::copy(crate_path.join("sdl_bindings.rs"), out_path.join("sdl_bindings.rs"))
        .expect("Couldn't find pregenerated bindings!");

    if cfg!(feature = "image") {
        fs::copy(crate_path.join("sdl_image_bindings.rs"), out_path.join("sdl_image_bindings.rs"))
            .expect("Couldn't find pregenerated SDL_image bindings!");
    }
    if cfg!(feature = "ttf") {
        fs::copy(crate_path.join("sdl_ttf_bindings.rs"), out_path.join("sdl_ttf_bindings.rs"))
            .expect("Couldn't find pregenerated SDL_ttf bindings!");
    }
    if cfg!(feature = "mixer") {
        fs::copy(crate_path.join("sdl_mixer_bindings.rs"), out_path.join("sdl_mixer_bindings.rs"))
            .expect("Couldn't find pregenerated SDL_mixer bindings!");
    }

    if cfg!(feature = "gfx") {
        fs::copy(crate_path.join("sdl_gfx_framerate_bindings.rs"), out_path.join("sdl_gfx_framerate_bindings.rs"))
            .expect("Couldn't find pregenerated SDL_gfx framerate bindings!");

        fs::copy(crate_path.join("sdl_gfx_primitives_bindings.rs"), out_path.join("sdl_gfx_primitives_bindings.rs"))
            .expect("Couldn't find pregenerated SDL_gfx primitives bindings!");

        fs::copy(crate_path.join("sdl_gfx_imagefilter_bindings.rs"), out_path.join("sdl_gfx_imagefilter_bindings.rs"))
            .expect("Couldn't find pregenerated SDL_gfx imagefilter bindings!");

        fs::copy(crate_path.join("sdl_gfx_rotozoom_bindings.rs"), out_path.join("sdl_gfx_rotozoom_bindings.rs"))
            .expect("Couldn't find pregenerated SDL_gfx rotozoom bindings!");
    }
}

#[cfg(feature = "bindgen")]
// headers_path is a list of directories where the SDL2 headers are expected
// to be found by bindgen (should point to the include/ directories)
fn generate_bindings<S: AsRef<str> + ::std::fmt::Debug>(target: &str, host: &str, headers_paths: &[S]) {
    let target_os = get_os_from_triple(target).unwrap();
    let mut bindings = bindgen::Builder::default();

    let mut image_bindings = bindgen::Builder::default();

    let mut ttf_bindings = bindgen::Builder::default();

    let mut mixer_bindings = bindgen::Builder::default();

    let mut gfx_framerate_bindings = bindgen::Builder::default();
    let mut gfx_primitives_bindings = bindgen::Builder::default();
    let mut gfx_imagefilter_bindings = bindgen::Builder::default();
    let mut gfx_rotozoom_bindings = bindgen::Builder::default();

    // Set correct target triple for bindgen when cross-compiling
    if target != host {
        bindings = bindings.clang_arg("-target");
        bindings = bindings.clang_arg(target.clone());

        if cfg!(feature = "image") {
            image_bindings = image_bindings.clang_arg("-target");
            image_bindings = image_bindings.clang_arg(target.clone());
        }

        if cfg!(feature = "ttf") {
            ttf_bindings = ttf_bindings.clang_arg("-target");
            ttf_bindings = ttf_bindings.clang_arg(target.clone());
        }

        if cfg!(feature = "mixer") {
            mixer_bindings = mixer_bindings.clang_arg("-target");
            mixer_bindings = mixer_bindings.clang_arg(target.clone());
        }

        if cfg!(feature = "gfx") {
            gfx_framerate_bindings = gfx_framerate_bindings.clang_arg("-target");
            gfx_framerate_bindings = gfx_framerate_bindings.clang_arg(target.clone());

            gfx_primitives_bindings = gfx_primitives_bindings.clang_arg("-target");
            gfx_primitives_bindings = gfx_primitives_bindings.clang_arg(target.clone());

            gfx_imagefilter_bindings = gfx_imagefilter_bindings.clang_arg("-target");
            gfx_imagefilter_bindings = gfx_imagefilter_bindings.clang_arg(target.clone());

            gfx_rotozoom_bindings = gfx_rotozoom_bindings.clang_arg("-target");
            gfx_rotozoom_bindings = gfx_rotozoom_bindings.clang_arg(target.clone());
        }
    }

    if headers_paths.len() == 0 {
        // if no paths are being provided, fall back to the headers included in this repo
        let mut include_path: PathBuf = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        include_path.push(format!("SDL2-{}", SDL2_HEADERS_BUNDLED_VERSION));
        include_path.push("include");
        bindings = bindings.clang_arg(format!("-I{}", include_path.display()));
        if cfg!(feature = "image") {
            image_bindings = image_bindings.clang_arg(format!("-I{}", include_path.display()));
        }
        if cfg!(feature = "ttf") {
            ttf_bindings = ttf_bindings.clang_arg(format!("-I{}", include_path.display()));
        }
        if cfg!(feature = "mixer") {
            mixer_bindings = mixer_bindings.clang_arg(format!("-I{}", include_path.display()));
        }
        if cfg!(feature = "gfx") {
            gfx_framerate_bindings = gfx_framerate_bindings.clang_arg(format!("-I{}", include_path.display()));
            gfx_primitives_bindings = gfx_primitives_bindings.clang_arg(format!("-I{}", include_path.display()));
            gfx_imagefilter_bindings = gfx_imagefilter_bindings.clang_arg(format!("-I{}", include_path.display()));
            gfx_rotozoom_bindings = gfx_rotozoom_bindings.clang_arg(format!("-I{}", include_path.display()));
        }
    } else {
        // if paths are included, use them for bindgen. Bindgen should use the first one.
        for headers_path in headers_paths {
            bindings = bindings.clang_arg(format!("-I{}", headers_path.as_ref()));
            if cfg!(feature = "image") {
                image_bindings = image_bindings.clang_arg(format!("-I{}", headers_path.as_ref()));
            }
            if cfg!(feature = "ttf") {
                ttf_bindings = ttf_bindings.clang_arg(format!("-I{}", headers_path.as_ref()));
            }
            if cfg!(feature = "mixer") {
                mixer_bindings = mixer_bindings.clang_arg(format!("-I{}", headers_path.as_ref()));
            }
            if cfg!(feature = "gfx") {
                gfx_framerate_bindings = gfx_framerate_bindings.clang_arg(format!("-I{}", headers_path.as_ref()));
                gfx_primitives_bindings = gfx_primitives_bindings.clang_arg(format!("-I{}", headers_path.as_ref()));
                gfx_imagefilter_bindings = gfx_imagefilter_bindings.clang_arg(format!("-I{}", headers_path.as_ref()));
                gfx_rotozoom_bindings = gfx_rotozoom_bindings.clang_arg(format!("-I{}", headers_path.as_ref()));
            }
        }
    }

    if target_os == "windows-msvc" {

        add_msvc_includes_to_bindings!(bindings);
        if cfg!(feature = "image") {
            add_msvc_includes_to_bindings!(image_bindings);
        }
        if cfg!(feature = "ttf") {
            add_msvc_includes_to_bindings!(ttf_bindings);
        }
        if cfg!(feature = "mixer") {
            add_msvc_includes_to_bindings!(mixer_bindings);
        }
        if cfg!(feature = "gfx") {
            add_msvc_includes_to_bindings!(gfx_framerate_bindings);
            add_msvc_includes_to_bindings!(gfx_primitives_bindings);
            add_msvc_includes_to_bindings!(gfx_imagefilter_bindings);
            add_msvc_includes_to_bindings!(gfx_rotozoom_bindings);
        }
    };

    // SDL2 hasn't a default configuration for Linux
    if target_os == "linux-gnu" {
        bindings = bindings.clang_arg("-DSDL_VIDEO_DRIVER_X11");
        bindings = bindings.clang_arg("-DSDL_VIDEO_DRIVER_WAYLAND");
        if cfg!(feature = "image") {
            image_bindings = image_bindings.clang_arg("-DSDL_VIDEO_DRIVER_X11");
            image_bindings = image_bindings.clang_arg("-DSDL_VIDEO_DRIVER_WAYLAND");
        }
        if cfg!(feature = "ttf") {
            ttf_bindings = ttf_bindings.clang_arg("-DSDL_VIDEO_DRIVER_X11");
            ttf_bindings = ttf_bindings.clang_arg("-DSDL_VIDEO_DRIVER_WAYLAND");
        }
        if cfg!(feature = "mixer") {
            mixer_bindings = mixer_bindings.clang_arg("-DSDL_VIDEO_DRIVER_X11");
            mixer_bindings = mixer_bindings.clang_arg("-DSDL_VIDEO_DRIVER_WAYLAND");
        }
        if cfg!(feature = "gfx") {
            gfx_framerate_bindings = gfx_framerate_bindings.clang_arg("-DSDL_VIDEO_DRIVER_X11");
            gfx_framerate_bindings = gfx_framerate_bindings.clang_arg("-DSDL_VIDEO_DRIVER_WAYLAND");
            gfx_primitives_bindings = gfx_primitives_bindings.clang_arg("-DSDL_VIDEO_DRIVER_X11");
            gfx_primitives_bindings = gfx_primitives_bindings.clang_arg("-DSDL_VIDEO_DRIVER_WAYLAND");
            gfx_imagefilter_bindings = gfx_imagefilter_bindings.clang_arg("-DSDL_VIDEO_DRIVER_X11");
            gfx_imagefilter_bindings = gfx_imagefilter_bindings.clang_arg("-DSDL_VIDEO_DRIVER_WAYLAND");
            gfx_rotozoom_bindings = gfx_rotozoom_bindings.clang_arg("-DSDL_VIDEO_DRIVER_X11");
            gfx_rotozoom_bindings = gfx_rotozoom_bindings.clang_arg("-DSDL_VIDEO_DRIVER_WAYLAND");
        }
    }

    let bindings = bindings
        .header("wrapper.h")
        .rustified_enum(".*")
        .blacklist_type("FP_NAN")
        .blacklist_type("FP_INFINITE")
        .blacklist_type("FP_ZERO")
        .blacklist_type("FP_SUBNORMAL")
        .blacklist_type("FP_NORMAL")
        .blacklist_type("max_align_t") // Until https://github.com/rust-lang-nursery/rust-bindgen/issues/550 gets fixed
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("sdl_bindings.rs"))
        .expect("Couldn't write bindings!");

    if cfg!(feature = "image") {
        let image_bindings = image_bindings
            .header("wrapper_image.h")
            .blacklist_type("FP_NAN")
            .blacklist_type("FP_INFINITE")
            .blacklist_type("FP_ZERO")
            .blacklist_type("FP_SUBNORMAL")
            .blacklist_type("FP_NORMAL")
            .blacklist_type("max_align_t") // Until https://github.com/rust-lang-nursery/rust-bindgen/issues/550 gets fixed
            .whitelist_type("IMG.*")
            .whitelist_function("IMG.*")
            .whitelist_var("IMG.*")
            .blacklist_type("SDL_.*")
            .blacklist_type("_IO.*|FILE")
            .generate()
            .expect("Unable to generate image_bindings!");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

        image_bindings
            .write_to_file(out_path.join("sdl_image_bindings.rs"))
            .expect("Couldn't write image_bindings!");
    }

    if cfg!(feature = "ttf") {
        let ttf_bindings = ttf_bindings
            .header("wrapper_ttf.h")
            .blacklist_type("FP_NAN")
            .blacklist_type("FP_INFINITE")
            .blacklist_type("FP_ZERO")
            .blacklist_type("FP_SUBNORMAL")
            .blacklist_type("FP_NORMAL")
            .whitelist_type("TTF.*")
            .whitelist_function("TTF.*")
            .whitelist_var("TTF.*")
            .blacklist_type("SDL_.*")
            .blacklist_type("_IO.*|FILE")
            .generate()
            .expect("Unable to generate ttf_bindings!");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

        ttf_bindings
            .write_to_file(out_path.join("sdl_ttf_bindings.rs"))
            .expect("Couldn't write ttf_bindings!");
    }

    if cfg!(feature = "mixer") {
        let mixer_bindings = mixer_bindings
            .header("wrapper_mixer.h")
            .blacklist_type("FP_NAN")
            .blacklist_type("FP_INFINITE")
            .blacklist_type("FP_ZERO")
            .blacklist_type("FP_SUBNORMAL")
            .blacklist_type("FP_NORMAL")
            .whitelist_type("MIX.*")
            .whitelist_type("Mix.*")
            .whitelist_type("MUS.*")
            .whitelist_function("Mix.*")
            .whitelist_var("MIX.*")
            .whitelist_var("MUS.*")
            .blacklist_type("SDL_.*")
            .blacklist_type("_IO.*|FILE")
            .generate()
            .expect("Unable to generate mixer_bindings!");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

        mixer_bindings
            .write_to_file(out_path.join("sdl_mixer_bindings.rs"))
            .expect("Couldn't write mixer_bindings!");
    }

    if cfg!(feature = "gfx") {
        let gfx_framerate_bindings = gfx_framerate_bindings
            .header("wrapper_gfx_framerate.h")
            .blacklist_type("FP_NAN")
            .blacklist_type("FP_INFINITE")
            .blacklist_type("FP_ZERO")
            .blacklist_type("FP_SUBNORMAL")
            .blacklist_type("FP_NORMAL")
            .whitelist_type("FPS.*")
            .whitelist_function("SDL_.*rame.*")
            .whitelist_var("FPS.*")
            .blacklist_type("_IO.*|FILE")
            .generate()
            .expect("Unable to generate gfx_framerate_bindings!");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

        gfx_framerate_bindings
            .write_to_file(out_path.join("sdl_gfx_framerate_bindings.rs"))
            .expect("Couldn't write gfx_framerate_bindings!");

        let gfx_primitives_bindings = gfx_primitives_bindings
            .header("wrapper_gfx_primitives.h")
            .blacklist_type("FP_NAN")
            .blacklist_type("FP_INFINITE")
            .blacklist_type("FP_ZERO")
            .blacklist_type("FP_SUBNORMAL")
            .blacklist_type("FP_NORMAL")
            .blacklist_type("SDL_.*")
            .whitelist_function("pixel.*")
            .whitelist_function("rectangle.*")
            .whitelist_function("rounded.*")
            .whitelist_function("box.*")
            .whitelist_function(".*line(Color|RGBA).*")
            .whitelist_function("thick.*")
            .whitelist_function(".*circle.*")
            .whitelist_function("arc.*")
            .whitelist_function("filled.*")
            .whitelist_function(".*ellipse.*")
            .whitelist_function("pie.*")
            .whitelist_function(".*trigon.*")
            .whitelist_function(".*polygon.*")
            .whitelist_function("textured.*")
            .whitelist_function("bezier.*")
            .whitelist_function("character.*")
            .whitelist_function("string.*")
            .whitelist_function("gfx.*")
            .blacklist_type("_IO.*|FILE")
            .generate()
            .expect("Unable to generate gfx_primitives_bindings!");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

        gfx_primitives_bindings
            .write_to_file(out_path.join("sdl_gfx_primitives_bindings.rs"))
            .expect("Couldn't write gfx_primitives_bindings!");

        let gfx_imagefilter_bindings = gfx_imagefilter_bindings
            .header("wrapper_gfx_imagefilter.h")
            .whitelist_function("SDL_image.*")
            .blacklist_type("FP_NAN")
            .blacklist_type("FP_INFINITE")
            .blacklist_type("FP_ZERO")
            .blacklist_type("FP_SUBNORMAL")
            .blacklist_type("FP_NORMAL")
            .blacklist_type("_IO.*|FILE")
            .generate()
            .expect("Unable to generate gfx_imagefilter_bindings!");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

        gfx_imagefilter_bindings
            .write_to_file(out_path.join("sdl_gfx_imagefilter_bindings.rs"))
            .expect("Couldn't write gfx_imagefilter_bindings!");

        let gfx_rotozoom_bindings = gfx_rotozoom_bindings
            .header("wrapper_gfx_rotozoom.h")
            .blacklist_type("SDL_.*")
            .whitelist_function("rotozoom.*")
            .whitelist_function("zoom.*")
            .whitelist_function("shrink.*")
            .whitelist_function("rotate.*")
            .blacklist_type("FP_NAN")
            .blacklist_type("FP_INFINITE")
            .blacklist_type("FP_ZERO")
            .blacklist_type("FP_SUBNORMAL")
            .blacklist_type("FP_NORMAL")
            .blacklist_type("_IO.*|FILE")
            .generate()
            .expect("Unable to generate gfx_rotozoom_bindings!");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

        gfx_rotozoom_bindings
            .write_to_file(out_path.join("sdl_gfx_rotozoom_bindings.rs"))
            .expect("Couldn't write gfx_rotozoom_bindings!");
    }
}

fn get_os_from_triple(triple: &str) -> Option<&str>
{
    triple.splitn(3, "-").nth(2)
}
