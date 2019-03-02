fn main() {
    #[cfg(target_os="openbsd")]
    println!(r"cargo:rustc-link-search=/usr/local/lib");
}
