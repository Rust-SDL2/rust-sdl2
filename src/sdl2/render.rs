//! 2D accelerated rendering
//!
//! Official C documentation: https://wiki.libsdl.org/CategoryRender
//! # Introduction
//!
//! This module contains functions for 2D accelerated rendering.
//!
//! This API supports the following features:
//!
//! * single pixel points
//! * single pixel lines
//! * filled rectangles
//! * texture images
//! * All of these may be drawn in opaque, blended, or additive modes.
//!
//! The texture images can have an additional color tint or alpha modulation
//! applied to them, and may also be stretched with linear interpolation,
//! rotated or flipped/mirrored.
//!
//! For advanced functionality like particle effects or actual 3D you should use
//! SDL's OpenGL/Direct3D support or one of the many available 3D engines.
//!
//! This API is not designed to be used from multiple threads, see
//! [this bug](http://bugzilla.libsdl.org/show_bug.cgi?id=1995) for details.
//!
//! ---
//!
//! None of the draw methods in `Canvas` are expected to fail.
//! If they do, a panic is raised and the program is aborted.

mod blend_mode;
pub use self::blend_mode::BlendMode;
mod canvas;
pub use self::canvas::Canvas;
mod canvas_builder;
pub use self::canvas_builder::CanvasBuilder;
mod context;
pub use self::context::RendererContext;
mod driver;
pub use self::driver::*;
mod error;
pub use self::error::*;
mod renderer_info;
pub use self::renderer_info::RendererInfo;
mod target;
pub use self::target::*;
mod texture;
pub use self::texture::error::*;
pub use self::texture::{Texture, TextureQuery};
mod texture_access;
pub use self::texture_access::TextureAccess;
mod texture_creator;
pub use self::texture_creator::TextureCreator;
pub use self::texture_creator::TextureValueError;
