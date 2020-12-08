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

mod context;
mod font;

pub use self::context::{
    get_linked_version, has_been_initialized, init, InitError, Sdl2TtfContext,
};
pub use self::font::{
    Font, FontError, FontResult, FontStyle, GlyphMetrics, Hinting, PartialRendering,
};
