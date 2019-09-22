fn main() {
    #[cfg(any(target_os="openbsd", target_os="freebsd"))]
    println!(r"cargo:rustc-link-search=/usr/local/lib");
}
