//!
//! A binding for the library `SDL2_ttf`
//!
//! 
//! Note that you need to build with the
//! feature `ttf` for this module to be enabled,
//! like so:
//!
//! ```bash
//! $ cargo build --features "ttf"
//! ```
//!
//! If you want to use this with from inside your own
//! crate, you will need to add this in your Cargo.toml
//!
//! ```toml
//! [dependencies.sdl2]
//! version = ...
//! default-features = false
//! features = ["ttf"]
//! ```

mod font;
mod context;

pub use self::context::{
    init, has_been_initialized, get_linked_version, Sdl2TtfContext, InitError,
};
pub use self::font::{
    Font, FontStyle, Hinting, GlyphMetrics, PartialRendering, FontError,
    FontResult
};
