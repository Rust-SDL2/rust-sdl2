extern crate "pkg-config" as pkg_config;

fn main() {
    if build_pkgconfig() { return; }
    panic!("Could not find SDL2 via pkgconfig");
}

fn build_pkgconfig() -> bool {
    let opts = pkg_config::default_options("sdl2");
    pkg_config::find_library_opts("sdl2", &opts).is_ok()
}
