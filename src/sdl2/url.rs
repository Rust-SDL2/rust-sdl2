//! Opening URLs in default system handlers

use crate::Error;

use crate::sys;

/// Opens a URL/URI in the default system-provided application.
///
/// This will most likely open a web browser for http:// and https:// links,
/// the default handler application for file:// links, but this varies
/// between platforms and is not supported on all of them.
/// It might also cause your window to lose focus, or pause your process on mobile.
///
/// There is no way to tell if the system successfully opened the provided URL,
/// an `Ok` result only means that something was launched to try to handle it.
///
/// # Examples
///
/// ```no_run
/// use sdl2::url::open_url;
///
/// open_url("https://github.com/Rust-SDL2/rust-sdl2")
///   .expect("Opening URLs not supported on this platform");
/// ```
#[doc(alias = "SDL_OpenURL")]
pub fn open_url(url: &str) -> Result<(), Error> {
    let result = unsafe { sys::SDL_OpenURL(as_cstring!(url)?.as_ptr()) };

    if result == 0 {
        Ok(())
    } else {
        Err(Error::from_sdl_error())
    }
}
