extern crate "pkg-config" as pkg_config;

fn main() {
    if std::os::getenv("CARGO_FEATURE_USE_PKGCONFIG").is_some() {
      if build_pkgconfig() { return; }
      panic!("Could not find SDL2 via pkgconfig");
    } else {
      println!("cargo:rustc-flags=-l SDL2");
    }
}

fn build_pkgconfig() -> bool {
    let opts = pkg_config::default_options("sdl2");
    pkg_config::find_library_opts("sdl2", &opts).is_ok()
}
