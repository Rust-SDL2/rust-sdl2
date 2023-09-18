#![allow(unused_imports, dead_code, unused_variables)]

#[cfg(feature = "bindgen")]
extern crate bindgen;
#[macro_use]
extern crate cfg_if;
#[cfg(feature = "bundled")]
extern crate cmake;
#[cfg(feature = "pkg-config")]
extern crate pkg_config;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs, io};

#[cfg(feature = "bindgen")]
macro_rules! add_msvc_includes_to_bindings {
    ($bindings:expr) => {
        $bindings = $bindings.clang_arg(format!(
            "-IC:/Program Files (x86)/Windows Kits/8.1/Include/shared"
        ));
        $bindings = $bindings.clang_arg(format!("-IC:/Program Files/LLVM/lib/clang/5.0.0/include"));
        $bindings = $bindings.clang_arg(format!(
            "-IC:/Program Files (x86)/Windows Kits/10/Include/10.0.10240.0/ucrt"
        ));
        $bindings = $bindings.clang_arg(format!(
            "-IC:/Program Files (x86)/Microsoft Visual Studio 14.0/VC/include"
        ));
        $bindings = $bindings.clang_arg(format!(
            "-IC:/Program Files (x86)/Windows Kits/8.1/Include/um"
        ));
    };
}

fn init_submodule(sdl_path: &Path) {
    if !sdl_path.join("CMakeLists.txt").exists() {
        Command::new("git")
            .args(&["submodule", "update", "--init"])
            .current_dir(sdl_path.clone())
            .status()
            .expect("Git is needed to retrieve the SDL source files");
    }
}

#[cfg(feature = "use-pkgconfig")]
fn pkg_config_print(statik: bool, lib_name: &str) {
    pkg_config::Config::new()
        .statik(statik)
        .probe(lib_name)
        .unwrap();
}

#[cfg(feature = "use-pkgconfig")]
fn get_pkg_config() {
    let statik: bool = if cfg!(feature = "static-link") {
        true
    } else {
        false
    };

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

#[cfg(feature = "use-vcpkg")]
fn get_vcpkg_config() {
    vcpkg::find_package("sdl2").unwrap();
    if cfg!(feature = "image") {
        vcpkg::find_package("sdl2-image").unwrap();
    }
    if cfg!(feature = "ttf") {
        vcpkg::find_package("sdl2-ttf").unwrap();
    }
    if cfg!(feature = "mixer") {
        vcpkg::find_package("sdl2-mixer").unwrap();
    }
    if cfg!(feature = "gfx") {
        vcpkg::find_package("sdl2-gfx").unwrap();
    }
}

// compile a shared or static lib depending on the feature
#[cfg(feature = "bundled")]
fn compile_sdl2(sdl2_build_path: &Path, target_os: &str) -> PathBuf {
    let mut cfg = cmake::Config::new(sdl2_build_path);
    if let Ok(profile) = env::var("SDL2_BUILD_PROFILE") {
        cfg.profile(&profile);
    } else {
        cfg.profile("Release");
    }

    // Allow specifying custom toolchain specifically for SDL2.
    if let Ok(toolchain) = env::var("SDL2_TOOLCHAIN") {
        cfg.define("CMAKE_TOOLCHAIN_FILE", &toolchain);
    } else {
        // Override __FLTUSED__ to keep the _fltused symbol from getting defined in the static build.
        // This conflicts and fails to link properly when building statically on Windows, likely due to
        // COMDAT conflicts/breakage happening somewhere.
        #[cfg(feature = "static-link")]
        cfg.cflag("-D__FLTUSED__");

        #[cfg(target_os = "linux")]
        {
            // Add common flag for affected version and above
            use version_compare::{compare_to, Cmp};
            if let Ok(version) = std::process::Command::new("cc")
                .arg("-dumpversion")
                .output()
            {
                let affected =
                    compare_to(std::str::from_utf8(&version.stdout).unwrap(), "10", Cmp::Ge)
                        .unwrap_or(true);
                if affected {
                    cfg.cflag("-fcommon");
                }
            }
        }
    }

    if target_os == "windows-gnu" {
        cfg.define("VIDEO_OPENGLES", "OFF");
    }

    if target_os == "android" {
        cfg.define(
            "ANDROID_NDK",
            env::var("ANDROID_NDK_HOME").expect(
                "ANDROID_NDK_HOME environment variable must be set when compiling for Android",
            ),
        );
    }

    if cfg!(feature = "static-link") {
        cfg.define("SDL_SHARED", "OFF");
        cfg.define("SDL_STATIC", "ON");
        // Prevent SDL to provide it own "main" which cause a conflict when this crate linked
        // to C/C++ program.
        cfg.define("SDL_MAIN_HANDLED", "ON");
    } else {
        cfg.define("SDL_SHARED", "ON");
        cfg.define("SDL_STATIC", "OFF");
    }

    cfg.build()
}

#[cfg(not(feature = "bundled"))]
fn compute_include_paths(fallback_path: String) -> Vec<String> {
    let mut include_paths: Vec<String> = vec![];

    if let Ok(include_path) = env::var("SDL2_INCLUDE_PATH") {
        include_paths.push(include_path);
    };

    #[cfg(feature = "pkg-config")]
    {
        // don't print the "cargo:xxx" directives, we're just trying to get the include paths here
        let pkg_config_library = pkg_config::Config::new()
            .print_system_libs(false)
            .probe("sdl2")
            .unwrap();
        for path in pkg_config_library.include_paths {
            include_paths.push(format!("{}", path.display()));
        }
    }

    #[cfg(feature = "vcpkg")]
    {
        // don't print the "cargo:xxx" directives, we're just trying to get the include paths here
        let vcpkg_library = vcpkg::Config::new()
            .cargo_metadata(false)
            .probe("sdl2")
            .unwrap();
        for path in vcpkg_library.include_paths {
            include_paths.push(format!("{}", path.display()));
        }
    }

    if include_paths.is_empty() {
        include_paths.push(fallback_path);
    }

    include_paths
}

fn link_sdl2(target_os: &str) {
    #[cfg(all(feature = "use-pkgconfig", not(feature = "bundled")))]
    {
        // prints the appropriate linking parameters when using pkg-config
        // useless when using "bundled"
        get_pkg_config();
    }

    #[cfg(all(feature = "use-vcpkg", not(feature = "bundled")))]
    {
        // prints the appropriate linking parameters when using pkg-config
        // useless when using "bundled"
        get_vcpkg_config();
    }

    #[cfg(not(feature = "static-link"))]
    {
        if target_os == "ios" {
            // iOS requires additional linking to function properly
            println!("cargo:rustc-flags=-l framework=AVFoundation");
            println!("cargo:rustc-flags=-l framework=AudioToolbox");
            println!("cargo:rustc-flags=-l framework=CoreAudio");
            println!("cargo:rustc-flags=-l framework=CoreGraphics");
            println!("cargo:rustc-flags=-l framework=CoreMotion");
            println!("cargo:rustc-flags=-l framework=Foundation");
            println!("cargo:rustc-flags=-l framework=GameController");
            println!("cargo:rustc-flags=-l framework=CoreHaptics");
            println!("cargo:rustc-flags=-l framework=OpenGLES");
            println!("cargo:rustc-flags=-l framework=QuartzCore");
            println!("cargo:rustc-flags=-l framework=UIKit");
        }

        // pkg-config automatically prints this output when probing,
        // however pkg_config isn't used with the feature "bundled"
        if cfg!(feature = "bundled") || cfg!(not(feature = "use-pkgconfig")) {
            if cfg!(feature = "use_mac_framework") && target_os == "darwin" {
                println!("cargo:rustc-flags=-l framework=SDL2");
            } else if target_os != "emscripten" {
                println!("cargo:rustc-flags=-l SDL2");
            }
        }
    }

    #[cfg(feature = "static-link")]
    {
        if cfg!(feature = "bundled")
            || (cfg!(feature = "use-pkgconfig") == false && cfg!(feature = "use-vcpkg") == false)
        {
            println!("cargo:rustc-link-lib=static=SDL2main");
            if target_os.contains("windows") {
                println!("cargo:rustc-link-lib=static=SDL2-static");
            } else {
                println!("cargo:rustc-link-lib=static=SDL2");
            }
        }

        // Also linked to any required libraries for each supported platform
        if target_os.contains("windows") {
            println!("cargo:rustc-link-lib=shell32");
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
            println!("cargo:rustc-link-lib=setupapi");
        } else if target_os == "darwin" {
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=Carbon");
            println!("cargo:rustc-link-lib=framework=ForceFeedback");
            println!("cargo:rustc-link-lib=framework=GameController");
            println!("cargo:rustc-link-lib=framework=CoreHaptics");
            println!("cargo:rustc-link-lib=framework=CoreVideo");
            println!("cargo:rustc-link-lib=framework=CoreAudio");
            println!("cargo:rustc-link-lib=framework=AudioToolbox");
            println!("cargo:rustc-link-lib=framework=Metal");
            println!("cargo:rustc-link-lib=iconv");
        } else if target_os == "android" {
            println!("cargo:rustc-link-lib=android");
            println!("cargo:rustc-link-lib=dl");
            println!("cargo:rustc-link-lib=GLESv1_CM");
            println!("cargo:rustc-link-lib=GLESv2");
            println!("cargo:rustc-link-lib=hidapi");
            println!("cargo:rustc-link-lib=log");
            println!("cargo:rustc-link-lib=OpenSLES");
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
    #[cfg(all(not(feature = "use-pkgconfig"), not(feature = "static-link")))]
    {
        if cfg!(feature = "mixer") {
            if target_os.contains("linux")
                || target_os.contains("freebsd")
                || target_os.contains("openbsd")
            {
                println!("cargo:rustc-flags=-l SDL2_mixer");
            } else if target_os.contains("windows") {
                println!("cargo:rustc-flags=-l SDL2_mixer");
            } else if target_os.contains("darwin") {
                if cfg!(any(mac_framework, feature = "use_mac_framework")) {
                    println!("cargo:rustc-flags=-l framework=SDL2_mixer");
                } else {
                    println!("cargo:rustc-flags=-l SDL2_mixer");
                }
            }
        }
        if cfg!(feature = "image") {
            if target_os.contains("linux")
                || target_os.contains("freebsd")
                || target_os.contains("openbsd")
            {
                println!("cargo:rustc-flags=-l SDL2_image");
            } else if target_os.contains("windows") {
                println!("cargo:rustc-flags=-l SDL2_image");
            } else if target_os.contains("darwin") {
                if cfg!(any(mac_framework, feature = "use_mac_framework")) {
                    println!("cargo:rustc-flags=-l framework=SDL2_image");
                } else {
                    println!("cargo:rustc-flags=-l SDL2_image");
                }
            }
        }
        if cfg!(feature = "ttf") {
            if target_os.contains("linux")
                || target_os.contains("freebsd")
                || target_os.contains("openbsd")
            {
                println!("cargo:rustc-flags=-l SDL2_ttf");
            } else if target_os.contains("windows") {
                println!("cargo:rustc-flags=-l SDL2_ttf");
            } else if target_os.contains("darwin") {
                if cfg!(any(mac_framework, feature = "use_mac_framework")) {
                    println!("cargo:rustc-flags=-l framework=SDL2_ttf");
                } else {
                    println!("cargo:rustc-flags=-l SDL2_ttf");
                }
            }
        }
        if cfg!(feature = "gfx") {
            if target_os.contains("linux")
                || target_os.contains("freebsd")
                || target_os.contains("openbsd")
            {
                println!("cargo:rustc-flags=-l SDL2_gfx");
            } else if target_os.contains("windows") {
                println!("cargo:rustc-flags=-l SDL2_gfx");
            } else if target_os.contains("darwin") {
                if cfg!(any(mac_framework, feature = "use_mac_framework")) {
                    println!("cargo:rustc-flags=-l framework=SDL2_gfx");
                } else {
                    println!("cargo:rustc-flags=-l SDL2_gfx");
                }
            }
        }
    }
}

fn find_cargo_target_dir() -> PathBuf {
    // Infer the top level cargo target dir from the OUT_DIR by searching
    // upwards until we get to $CARGO_TARGET_DIR/build/ (which is always one
    // level up from the deepest directory containing our package name)
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
    let mut out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    loop {
        {
            let final_path_segment = out_dir.file_name().unwrap();
            if final_path_segment.to_string_lossy().contains(&pkg_name) {
                break;
            }
        }
        if !out_dir.pop() {
            panic!("Malformed build path: {}", out_dir.to_string_lossy());
        }
    }
    out_dir.pop();
    out_dir.pop();
    out_dir
}

#[cfg(unix)]
fn copy_library_symlink(src_path: &Path, target_path: &Path) {
    if let Ok(link_path) = fs::read_link(src_path) {
        // Copy symlinks to:
        //  * target dir: as a product ship product of the build,
        //  * deps directory: as comment example testing doesn't pick up the library search path
        //    otherwise and fails.
        let deps_path = target_path.join("deps");
        for path in &[target_path, &deps_path] {
            let dst_path = path.join(src_path.file_name().expect("Path missing filename"));
            // Silently drop errors here, in case the symlink already exists.
            let _ = std::os::unix::fs::symlink(&link_path, &dst_path);
        }
    }
}

#[cfg(not(unix))]
fn copy_library_symlink(src_path: &Path, target_path: &Path) {}

fn copy_library_file(src_path: &Path, target_path: &Path) {
    // Copy the shared libs to:
    //  * target dir: as a product ship product of the build,
    //  * deps directory: as comment example testing doesn't pick up the library search path
    //    otherwise and fails.
    let deps_path = target_path.join("deps");
    for path in &[target_path, &deps_path] {
        let dst_path = path.join(src_path.file_name().expect("Path missing filename"));

        fs::copy(&src_path, &dst_path).expect(&format!(
            "Failed to copy SDL2 dynamic library from {} to {}",
            src_path.to_string_lossy(),
            dst_path.to_string_lossy()
        ));
    }
}

fn copy_dynamic_libraries(sdl2_compiled_path: &PathBuf, target_os: &str) {
    let target_path = find_cargo_target_dir();

    // Windows binaries do not embed library search paths, so successfully
    // linking the DLL isn't sufficient to find it at runtime -- it must be
    // either on PATH or in the current working directory when we run binaries
    // linked against it. In other words, to run the test suite we need to
    // copy sdl2.dll out of its build tree and down to the top level cargo
    // binary output directory.
    if target_os.contains("windows") {
        let sdl2_dll_name = "SDL2.dll";
        let sdl2_bin_path = sdl2_compiled_path.join("bin");
        let src_dll_path = sdl2_bin_path.join(sdl2_dll_name);

        copy_library_file(&src_dll_path, &target_path);
    } else if target_os != "emscripten" {
        // Find all libraries build and copy them, symlinks included.
        let mut found = false;
        let lib_dirs = &["lib", "lib64"];
        for lib_dir in lib_dirs {
            let lib_path = sdl2_compiled_path.join(lib_dir);
            if lib_path.exists() {
                found = true;
                for entry in std::fs::read_dir(&lib_path)
                    .unwrap_or_else(|_| panic!("Couldn't readdir {}", lib_dir))
                {
                    let entry = entry.expect("Error looking at lib dir");
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_symlink() {
                            copy_library_symlink(&entry.path(), &target_path);
                        } else if file_type.is_file() {
                            copy_library_file(&entry.path(), &target_path)
                        }
                    }
                }
                break;
            }
        }
        if !found {
            panic!("Failed to find CMake output dir");
        }
    }
}

fn main() {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let host = env::var("HOST").expect("Cargo build scripts always have HOST");
    let target_os = get_os_from_triple(target.as_str()).unwrap();

    let sdl2_source_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("SDL");
    init_submodule(sdl2_source_path.as_path());

    let sdl2_compiled_path: PathBuf;
    #[cfg(feature = "bundled")]
    {
        sdl2_compiled_path = compile_sdl2(sdl2_source_path.as_path(), target_os);

        println!(
            "cargo:rustc-link-search={}",
            sdl2_compiled_path.join("lib64").display()
        );
        println!(
            "cargo:rustc-link-search={}",
            sdl2_compiled_path.join("lib").display()
        );
    }

    let sdl2_includes = sdl2_source_path
        .join("include")
        .to_str()
        .unwrap()
        .to_string();

    #[cfg(feature = "bindgen")]
    {
        let include_paths: Vec<String>;
        #[cfg(feature = "bundled")]
        {
            include_paths = vec![sdl2_includes];
        }
        #[cfg(not(feature = "bundled"))]
        {
            include_paths = compute_include_paths(sdl2_includes)
        }
        generate_bindings(target.as_str(), host.as_str(), include_paths.as_slice());
        println!("cargo:include={}", include_paths.join(":"));
    }

    #[cfg(not(feature = "bindgen"))]
    {
        copy_pregenerated_bindings();
        println!("cargo:include={}", sdl2_includes);
    }

    link_sdl2(target_os);

    // Android builds shared libhidapi.so even for static builds.
    #[cfg(all(
        feature = "bundled",
        any(not(feature = "static-link"), target_os = "android")
    ))]
    {
        copy_dynamic_libraries(&sdl2_compiled_path, target_os);
    }
}

#[cfg(not(feature = "bindgen"))]
fn copy_pregenerated_bindings() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    fs::copy(
        crate_path.join("sdl_bindings.rs"),
        out_path.join("sdl_bindings.rs"),
    )
    .expect("Couldn't find pregenerated bindings!");

    if cfg!(feature = "image") {
        fs::copy(
            crate_path.join("sdl_image_bindings.rs"),
            out_path.join("sdl_image_bindings.rs"),
        )
        .expect("Couldn't find pregenerated SDL_image bindings!");
    }
    if cfg!(feature = "ttf") {
        fs::copy(
            crate_path.join("sdl_ttf_bindings.rs"),
            out_path.join("sdl_ttf_bindings.rs"),
        )
        .expect("Couldn't find pregenerated SDL_ttf bindings!");
    }
    if cfg!(feature = "mixer") {
        fs::copy(
            crate_path.join("sdl_mixer_bindings.rs"),
            out_path.join("sdl_mixer_bindings.rs"),
        )
        .expect("Couldn't find pregenerated SDL_mixer bindings!");
    }

    if cfg!(feature = "gfx") {
        fs::copy(
            crate_path.join("sdl_gfx_framerate_bindings.rs"),
            out_path.join("sdl_gfx_framerate_bindings.rs"),
        )
        .expect("Couldn't find pregenerated SDL_gfx framerate bindings!");

        fs::copy(
            crate_path.join("sdl_gfx_primitives_bindings.rs"),
            out_path.join("sdl_gfx_primitives_bindings.rs"),
        )
        .expect("Couldn't find pregenerated SDL_gfx primitives bindings!");

        fs::copy(
            crate_path.join("sdl_gfx_imagefilter_bindings.rs"),
            out_path.join("sdl_gfx_imagefilter_bindings.rs"),
        )
        .expect("Couldn't find pregenerated SDL_gfx imagefilter bindings!");

        fs::copy(
            crate_path.join("sdl_gfx_rotozoom_bindings.rs"),
            out_path.join("sdl_gfx_rotozoom_bindings.rs"),
        )
        .expect("Couldn't find pregenerated SDL_gfx rotozoom bindings!");
    }
}

#[cfg(feature = "bindgen")]
// headers_path is a list of directories where the SDL2 headers are expected
// to be found by bindgen (should point to the include/ directories)
fn generate_bindings(target: &str, host: &str, headers_paths: &[String]) {
    let target_os = get_os_from_triple(target).unwrap();
    let mut bindings = bindgen::Builder::default()
        // enable no_std-friendly output by only using core definitions
        .use_core()
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .ctypes_prefix("libc");

    let mut image_bindings = bindgen::Builder::default()
        .use_core()
        .raw_line("use crate::*;")
        .ctypes_prefix("libc");

    let mut ttf_bindings = bindgen::Builder::default()
        .use_core()
        .raw_line("use crate::*;")
        .ctypes_prefix("libc");

    let mut mixer_bindings = bindgen::Builder::default()
        .use_core()
        .raw_line("use crate::*;")
        .ctypes_prefix("libc");

    let mut gfx_framerate_bindings = bindgen::Builder::default().use_core().ctypes_prefix("libc");
    let mut gfx_primitives_bindings = bindgen::Builder::default()
        .use_core()
        .raw_line("use crate::*;")
        .ctypes_prefix("libc");
    let mut gfx_imagefilter_bindings = bindgen::Builder::default().use_core().ctypes_prefix("libc");
    let mut gfx_rotozoom_bindings = bindgen::Builder::default()
        .use_core()
        .raw_line("use crate::*;")
        .ctypes_prefix("libc");

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

    for headers_path in headers_paths {
        bindings = bindings.clang_arg(format!("-I{}", headers_path));
        if cfg!(feature = "image") {
            image_bindings = image_bindings.clang_arg(format!("-I{}", headers_path));
        }
        if cfg!(feature = "ttf") {
            ttf_bindings = ttf_bindings.clang_arg(format!("-I{}", headers_path));
        }
        if cfg!(feature = "mixer") {
            mixer_bindings = mixer_bindings.clang_arg(format!("-I{}", headers_path));
        }
        if cfg!(feature = "gfx") {
            gfx_framerate_bindings =
                gfx_framerate_bindings.clang_arg(format!("-I{}", headers_path));
            gfx_primitives_bindings =
                gfx_primitives_bindings.clang_arg(format!("-I{}", headers_path));
            gfx_imagefilter_bindings =
                gfx_imagefilter_bindings.clang_arg(format!("-I{}", headers_path));
            gfx_rotozoom_bindings = gfx_rotozoom_bindings.clang_arg(format!("-I{}", headers_path));
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
            gfx_primitives_bindings =
                gfx_primitives_bindings.clang_arg("-DSDL_VIDEO_DRIVER_WAYLAND");
            gfx_imagefilter_bindings = gfx_imagefilter_bindings.clang_arg("-DSDL_VIDEO_DRIVER_X11");
            gfx_imagefilter_bindings =
                gfx_imagefilter_bindings.clang_arg("-DSDL_VIDEO_DRIVER_WAYLAND");
            gfx_rotozoom_bindings = gfx_rotozoom_bindings.clang_arg("-DSDL_VIDEO_DRIVER_X11");
            gfx_rotozoom_bindings = gfx_rotozoom_bindings.clang_arg("-DSDL_VIDEO_DRIVER_WAYLAND");
        }
    }

    let bindings = bindings
        .header("wrapper.h")
        .blacklist_type("FP_NAN")
        .blacklist_type("FP_INFINITE")
        .blacklist_type("FP_ZERO")
        .blacklist_type("FP_SUBNORMAL")
        .blacklist_type("FP_NORMAL")
        .derive_debug(false)
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

fn get_os_from_triple(triple: &str) -> Option<&str> {
    triple.splitn(3, "-").nth(2)
}
