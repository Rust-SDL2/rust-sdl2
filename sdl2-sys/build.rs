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
extern crate unidiff;

#[macro_use]
extern crate cfg_if;

use std::path::{Path, PathBuf};
use std::{io, fs, env};

// corresponds to the headers that we have in sdl2-sys/SDL2-{version}
const SDL2_HEADERS_BUNDLED_VERSION: &str = "2.0.10";

// means the lastest stable version that can be downloaded from SDL2's source
const LASTEST_SDL2_VERSION: &str = "2.0.10";

#[cfg(feature = "bindgen")]
macro_rules! add_msvc_includes_to_bindings {
    ($bindings:expr) => {
        $bindings = $bindings.clang_arg(format!("-IC:/Program Files (x86)/Windows Kits/8.1/Include/shared"));
        $bindings = $bindings.clang_arg(format!("-IC:/Program Files/LLVM/lib/clang/5.0.0/include"));
        $bindings = $bindings.clang_arg(format!("-IC:/Program Files (x86)/Windows Kits/10/Include/10.0.10240.0/ucrt"));
        $bindings = $bindings.clang_arg(format!("-IC:/Program Files (x86)/Microsoft Visual Studio 14.0/VC/include"));
        $bindings = $bindings.clang_arg(format!("-IC:/Program Files (x86)/Windows Kits/8.1/Include/um"));
    };
}

fn get_bundled_header_path() -> PathBuf {
    let mut include_path: PathBuf = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    include_path.push(format!("SDL2-{}", SDL2_HEADERS_BUNDLED_VERSION));
    include_path.push("include");
    include_path
}

#[cfg(feature = "bundled")]
fn run_command(cmd: &str, args: &[&str]) {
    use std::process::Command;
    match Command::new(cmd).args(args).output() {
        Ok(output) => {
            if !output.status.success() {
                let error = std::str::from_utf8(&output.stderr).unwrap();
                panic!("Command '{}' failed: {}", cmd, error);
            }
        }
        Err(error) => {
            panic!("Error running command '{}': {:#}", cmd, error);
        }
    }
}

#[cfg(feature = "bundled")]
fn download_to(url: &str, dest: &str) {
    if cfg!(windows) {
        run_command("powershell", &[
            "-NoProfile", "-NonInteractive",
            "-Command", &format!("& {{
                $client = New-Object System.Net.WebClient
                $client.DownloadFile(\"{0}\", \"{1}\")
                if (!$?) {{ Exit 1 }}
            }}", url, dest).as_str()
        ]);
    } else {
        run_command("curl", &[url, "-o", dest]);
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

// returns the location of the downloaded source
#[cfg(feature = "bundled")]
fn download_sdl2() -> PathBuf {
    let out_dir = env::var("OUT_DIR").unwrap();
    
    let sdl2_archive_name = format!("SDL2-{}.tar.gz", LASTEST_SDL2_VERSION);
    let sdl2_archive_url = format!("https://libsdl.org/release/{}", sdl2_archive_name);

    let sdl2_archive_path = Path::new(&out_dir).join(sdl2_archive_name);
    let sdl2_build_path = Path::new(&out_dir).join(format!("SDL2-{}", LASTEST_SDL2_VERSION));

    // avoid re-downloading the archive if it already exists    
    if !sdl2_archive_path.exists() {
        download_to(&sdl2_archive_url, sdl2_archive_path.to_str().unwrap());
    }

    let reader = flate2::read::GzDecoder::new(
        fs::File::open(&sdl2_archive_path).unwrap()
    );
    let mut ar = tar::Archive::new(reader);
    ar.unpack(&out_dir).unwrap();

    sdl2_build_path
}

// apply patches to sdl2 source
#[cfg(feature = "bundled")]
fn patch_sdl2(sdl2_source_path: &Path) {
    // vector of <(patch_file_name, patch_file_contents)>
    let patches: Vec<(&str, &'static str)> = vec![
        // No patches at this time. If needed, add them like this:
        // ("SDL-2.x.y-filename.patch", include_str!("patches/SDL-2.x.y-filename.patch")),
        ("SDL2-2.0.10-CMakeLists.txt.patch", include_str!("patches/SDL2-2.0.10-CMakeLists.txt.patch")),
        // https://bugzilla.libsdl.org/show_bug.cgi?id=5105
        ("SDL2-2.0.10-sndio-shared-linux.patch", include_str!("patches/SDL2-2.0.10-sndio-shared-linux.patch")),
    ];
    let sdl_version = format!("SDL2-{}", LASTEST_SDL2_VERSION);

    for patch in &patches {
        // Only apply patches whose file name is prefixed with the currently
        // targeted version of SDL2.
        if !patch.0.starts_with(&sdl_version) {
            continue;
        }
        let mut patch_set = unidiff::PatchSet::new();
        patch_set.parse(patch.1).expect("Error parsing diff");

        // For every modified file, copy the existing file to <file_name>_old,
        // open a new copy of <file_name>. and fill the new file with a
        // combination of the unmodified contents, and the patched sections.
        // TOOD: This code is untested (save for the immediate application), and
        // probably belongs in the unidiff (or similar) package.
        for modified_file in patch_set.modified_files() {
            use std::io::{Write, BufRead};

            let file_path = sdl2_source_path.join(modified_file.path());
            let old_path = sdl2_source_path.join(format!("{}_old", modified_file.path()));
            fs::rename(&file_path, &old_path)
                .expect(&format!(
                    "Rename of {} to {} failed",
                    file_path.to_string_lossy(),
                    old_path.to_string_lossy()));

            let     dst_file = fs::File::create(file_path).unwrap();
            let mut dst_buf  = io::BufWriter::new(dst_file);
            let     old_file = fs::File::open(old_path).unwrap();
            let mut old_buf  = io::BufReader::new(old_file);
            let mut cursor = 0;

            for (i, hunk) in modified_file.into_iter().enumerate() {
                // Write old lines from cursor to the start of this hunk.
                let num_lines = hunk.source_start - cursor - 1;
                for _ in 0..num_lines {
                    let mut line = String::new();
                    old_buf.read_line(&mut line).unwrap();
                    dst_buf.write_all(line.as_bytes()).unwrap();
                }
                cursor += num_lines;

                // Skip lines in old_file, and verify that what we expect to
                // replace is present in the old_file.
                for expected_line in hunk.source_lines() {
                    let mut actual_line = String::new();
                    old_buf.read_line(&mut actual_line).unwrap();
                    actual_line.pop(); // Remove the trailing newline.
                    if expected_line.value.trim_end() != actual_line {
                        panic!("Can't apply patch; mismatch between expected and actual in hunk {}", i);
                    }
                }
                cursor += hunk.source_length;

                // Write the new lines into the destination.
                for line in hunk.target_lines() {
                    dst_buf.write_all(line.value.as_bytes()).unwrap();
                    dst_buf.write_all(b"\n").unwrap();
                }
            }

            // Write all remaining lines from the old file into the new.
            for line in old_buf.lines() {
                dst_buf.write_all(&line.unwrap().into_bytes()).unwrap();
                dst_buf.write_all(b"\n").unwrap();
            }
        }
        // For every removed file, simply delete the original.
        // TODO: This is entirely untested code. There are likely bugs here, and
        // this really should be part of the unidiff library, not a function
        // defined here. Hopefully this gets moved somewhere else before it
        // bites someone.
        for removed_file in patch_set.removed_files() {
            fs::remove_file(sdl2_source_path.join(removed_file.path()))
                .expect(
                    &format!("Failed to remove file {} from {}",
                        removed_file.path(),
                        sdl2_source_path.to_string_lossy()));
        }
        // For every new file, copy the entire contents of the patched file into
        // a newly created <file_name>.
        // TODO: This is entirely untested code. There are likely bugs here, and
        // this really should be part of the unidiff library, not a function
        // defined here. Hopefully this gets moved somewhere else before it
        // bites someone.
        for added_file in patch_set.added_files() {
            use std::io::Write;

            // This should be superfluous. I don't know how a new file would
            // ever have more than one hunk.
            assert!(added_file.len() == 1);
            let file_path = sdl2_source_path.join(added_file.path());
            let dst_file = fs::File::create(&file_path)
                .expect(&format!(
                    "Failed to create file {}",
                    file_path.to_string_lossy()));
            let mut dst_buf = io::BufWriter::new(&dst_file);

            for line in added_file.into_iter().nth(0).unwrap().target_lines() {
                dst_buf.write_all(line.value.as_bytes()).unwrap();
                dst_buf.write_all(b"\n").unwrap();
            }
        }
    }
}

// compile a shared or static lib depending on the feature 
#[cfg(feature = "bundled")]
fn compile_sdl2(sdl2_build_path: &Path, target_os: &str) -> PathBuf {
    let mut cfg = cmake::Config::new(sdl2_build_path);
    cfg.profile("release");

    #[cfg(target_os = "linux")]
    {
        use version_compare::Version;
        if let Ok(version) = std::process::Command::new("cc").arg("-dumpversion").output() {
            let local_ver = Version::from(std::str::from_utf8(&version.stdout).unwrap()).unwrap();
            let affected_ver = Version::from("10").unwrap();

            if local_ver >= affected_ver {
                cfg.cflag("-fcommon");
            }
        }
    }

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

    #[cfg(feature = "vcpkg")] {
        // don't print the "cargo:xxx" directives, we're just trying to get the include paths here
        let vcpkg_library = vcpkg::Config::new().cargo_metadata(false).probe("sdl2").unwrap();
        for path in vcpkg_library.include_paths {
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
    
    #[cfg(all(feature = "use-vcpkg", not(feature = "bundled")))] {
        // prints the appropriate linking parameters when using pkg-config
        // useless when using "bundled"
        get_vcpkg_config();
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
            } else if target_os != "emscripten" {
                println!("cargo:rustc-flags=-l SDL2");
            }
        }
    }

    #[cfg(feature = "static-link")] {
        if cfg!(feature = "bundled") || (cfg!(feature = "use-pkgconfig") == false && cfg!(feature = "use-vcpkg") == false) { 
            println!("cargo:rustc-link-lib=static=SDL2main");
            println!("cargo:rustc-link-lib=static=SDL2");
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
            if target_os.contains("linux") || target_os.contains("freebsd") || target_os.contains("openbsd") {
                println!("cargo:rustc-flags=-l SDL2_mixer");
            } else if target_os.contains("windows") {
                println!("cargo:rustc-flags=-l SDL2_mixer");
            } else if target_os.contains("darwin") {
                if cfg!(any(mac_framework, feature="use_mac_framework")) {
                    println!("cargo:rustc-flags=-l framework=SDL2_mixer");
                } else {
                    println!("cargo:rustc-flags=-l SDL2_mixer");
                }
            }
        }
        if cfg!(feature = "image") {
            if target_os.contains("linux") || target_os.contains("freebsd") || target_os.contains("openbsd") {
                println!("cargo:rustc-flags=-l SDL2_image");
            } else if target_os.contains("windows") {
                println!("cargo:rustc-flags=-l SDL2_image");
            } else if target_os.contains("darwin") {
                if cfg!(any(mac_framework, feature="use_mac_framework")) {
                    println!("cargo:rustc-flags=-l framework=SDL2_image");
                } else {
                    println!("cargo:rustc-flags=-l SDL2_image");
                }
            }
        }
        if cfg!(feature = "ttf") {
            if target_os.contains("linux") || target_os.contains("freebsd") || target_os.contains("openbsd") {
                println!("cargo:rustc-flags=-l SDL2_ttf");
            } else if target_os.contains("windows") {
                println!("cargo:rustc-flags=-l SDL2_ttf");
            } else if target_os.contains("darwin") {
                if cfg!(any(mac_framework, feature="use_mac_framework")) {
                    println!("cargo:rustc-flags=-l framework=SDL2_ttf");
                } else {
                    println!("cargo:rustc-flags=-l SDL2_ttf");
                }
            }
        }
        if cfg!(feature = "gfx") {
            if target_os.contains("linux") || target_os.contains("freebsd") || target_os.contains("openbsd") {
                println!("cargo:rustc-flags=-l SDL2_gfx");
            } else if target_os.contains("windows") {
                println!("cargo:rustc-flags=-l SDL2_gfx");
            } else if target_os.contains("darwin") {
                if cfg!(any(mac_framework, feature="use_mac_framework")) {
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

fn copy_dynamic_libraries(sdl2_compiled_path: &PathBuf, target_os: &str) {
    // Windows binaries do not embed library search paths, so successfully
    // linking the DLL isn't sufficient to find it at runtime -- it must be
    // either on PATH or in the current working directory when we run binaries
    // linked against it. In other words, to run the test suite we need to
    // copy sdl2.dll out of its build tree and down to the top level cargo
    // binary output directory.
    if target_os.contains("windows") {
        let sdl2_dll_name = "SDL2.dll";
        let sdl2_bin_path = sdl2_compiled_path.join("bin");
        let target_path = find_cargo_target_dir();

        let src_dll_path = sdl2_bin_path.join(sdl2_dll_name);
        let dst_dll_path = target_path.join(sdl2_dll_name);

        fs::copy(&src_dll_path, &dst_dll_path)
            .expect(&format!("Failed to copy SDL2 dynamic library from {} to {}",
                             src_dll_path.to_string_lossy(),
                             dst_dll_path.to_string_lossy()));
    }
}

fn main() {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let host = env::var("HOST").expect("Cargo build scripts always have HOST");
    let target_os = get_os_from_triple(target.as_str()).unwrap();

    let sdl2_compiled_path: PathBuf;
    #[cfg(feature = "bundled")] {
        let sdl2_source_path = download_sdl2();
        patch_sdl2(sdl2_source_path.as_path());
        sdl2_compiled_path = compile_sdl2(sdl2_source_path.as_path(), target_os);

        let sdl2_downloaded_include_path = sdl2_source_path.join("include");
        let sdl2_compiled_lib_path = sdl2_compiled_path.join("lib");

        println!("cargo:rustc-link-search={}", sdl2_compiled_lib_path.display());
        
        #[cfg(feature = "bindgen")] {
            let include_paths = vec!(String::from(sdl2_downloaded_include_path.to_str().unwrap()));
            println!("cargo:include={}", include_paths.join(":"));
            generate_bindings(target.as_str(), host.as_str(), include_paths.as_slice())
        }
        #[cfg(not(feature = "bindgen"))] {
            println!("cargo:include={}", sdl2_downloaded_include_path.display());
        }
    };

    #[cfg(all(not(feature = "bundled"), feature = "bindgen"))] {
        let include_paths: Vec<String> = compute_include_paths();
        generate_bindings(target.as_str(), host.as_str(), include_paths.as_slice())
    }

    #[cfg(not(feature = "bindgen"))] {
        copy_pregenerated_bindings();
        println!("cargo:include={}", get_bundled_header_path().display());
    }

    link_sdl2(target_os);

    #[cfg(all(feature = "bundled", not(feature = "static-link")))] {
        copy_dynamic_libraries(&sdl2_compiled_path, target_os);
    }
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
fn generate_bindings(target: &str, host: &str, headers_paths: &[String]) {
    let target_os = get_os_from_triple(target).unwrap();
    let mut bindings = bindgen::Builder::default()
        // enable no_std-friendly output by only using core definitions
        .use_core()
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
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

    let mut gfx_framerate_bindings = bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("libc");
    let mut gfx_primitives_bindings = bindgen::Builder::default()
        .use_core()
        .raw_line("use crate::*;")
        .ctypes_prefix("libc");
    let mut gfx_imagefilter_bindings = bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("libc");
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

    if headers_paths.len() == 0 {
        // if no paths are being provided, fall back to the headers included in this repo
        let include_path = get_bundled_header_path();
        println!("cargo:include={}", include_path.display());

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
        println!("cargo:include={}", headers_paths.join(":"));
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
                gfx_framerate_bindings = gfx_framerate_bindings.clang_arg(format!("-I{}", headers_path));
                gfx_primitives_bindings = gfx_primitives_bindings.clang_arg(format!("-I{}", headers_path));
                gfx_imagefilter_bindings = gfx_imagefilter_bindings.clang_arg(format!("-I{}", headers_path));
                gfx_rotozoom_bindings = gfx_rotozoom_bindings.clang_arg(format!("-I{}", headers_path));
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

fn get_os_from_triple(triple: &str) -> Option<&str>
{
    triple.splitn(3, "-").nth(2)
}
