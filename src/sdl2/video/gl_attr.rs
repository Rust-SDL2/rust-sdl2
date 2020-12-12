//! OpenGL context getters and setters
//!
//! # Example
//! ```no_run
//! use sdl2::video::GLProfile;
//!
//! let sdl_context = sdl2::init().unwrap();
//! let video_subsystem = sdl_context.video().unwrap();
//! let gl_attr = video_subsystem.gl_attr();
//!
//! // Don't use deprecated OpenGL functions
//! gl_attr.set_context_profile(GLProfile::Core);
//!
//! // Set the context into debug mode
//! gl_attr.set_context_flags().debug().set();
//!
//! // Set the OpenGL context version (OpenGL 3.2)
//! gl_attr.set_context_version(3, 2);
//!
//! // Enable anti-aliasing
//! gl_attr.set_multisample_buffers(1);
//! gl_attr.set_multisample_samples(4);
//!
//! let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600).opengl().build().unwrap();
//!
//! // Yes, we're still using the Core profile
//! assert_eq!(gl_attr.context_profile(), GLProfile::Core);
//! // ... and we're still using OpenGL 3.2
//! assert_eq!(gl_attr.context_version(), (3, 2));
//! ```

use super::GLProfile;
use crate::get_error;
use crate::sys;
use std::marker::PhantomData;

mod attr_type;
use self::attr_type::GLAttrTypeUtil;

macro_rules! attrs {
  (
      $(($attr_name:ident, $set_property:ident, $get_property:ident, $t:ty, $doc:expr)),*
  ) => (

      $(
      #[doc = "**Sets** the attribute: "]
      #[doc = $doc]
      #[inline]
      pub fn $set_property(&self, value: $t) {
          gl_set_attribute!($attr_name, value.to_gl_value());
      }

      #[doc = "**Gets** the attribute: "]
      #[doc = $doc]
      #[inline]
      pub fn $get_property(&self) -> $t {
          let value = gl_get_attribute!($attr_name);
          GLAttrTypeUtil::from_gl_value(value)
      }
      )*
  );
}

/// OpenGL context getters and setters. Obtain with `VideoSubsystem::gl_attr()`.
pub struct GLAttr<'a> {
    _marker: PhantomData<&'a crate::VideoSubsystem>,
}

impl crate::VideoSubsystem {
    /// Obtains access to the OpenGL window attributes.
    pub fn gl_attr(&self) -> GLAttr {
        GLAttr {
            _marker: PhantomData,
        }
    }
}

macro_rules! gl_set_attribute {
    ($attr:ident, $value:expr) => {{
        let result = unsafe { sys::SDL_GL_SetAttribute(sys::SDL_GLattr::$attr, $value) };

        if result != 0 {
            // Panic and print the attribute that failed.
            panic!(
                "couldn't set attribute {}: {}",
                stringify!($attr),
                get_error()
            );
        }
    }};
}

macro_rules! gl_get_attribute {
    ($attr:ident) => {{
        let mut value = 0;
        let result = unsafe { sys::SDL_GL_GetAttribute(sys::SDL_GLattr::$attr, &mut value) };
        if result != 0 {
            // Panic and print the attribute that failed.
            panic!(
                "couldn't get attribute {}: {}",
                stringify!($attr),
                get_error()
            );
        }
        value
    }};
}

impl<'a> GLAttr<'a> {
    // Note: Wish I could avoid the redundancy of set_property and property (without namespacing into new modules),
    // but Rust's `concat_idents!` macro isn't stable.
    attrs! {
        (SDL_GL_RED_SIZE, set_red_size, red_size, u8,
            "the minimum number of bits for the red channel of the color buffer; defaults to 3"),

        (SDL_GL_GREEN_SIZE, set_green_size, green_size, u8,
            "the minimum number of bits for the green channel of the color buffer; defaults to 3"),

        (SDL_GL_BLUE_SIZE, set_blue_size, blue_size, u8,
            "the minimum number of bits for the blue channel of the color buffer; defaults to 2"),

        (SDL_GL_ALPHA_SIZE, set_alpha_size, alpha_size, u8,
            "the minimum number of bits for the alpha channel of the color buffer; defaults to 0"),

        (SDL_GL_BUFFER_SIZE, set_buffer_size, buffer_size, u8,
            "the minimum number of bits for frame buffer size; defaults to 0"),

        (SDL_GL_DOUBLEBUFFER, set_double_buffer, double_buffer, bool,
            "whether the output is single or double buffered; defaults to double buffering on"),

        (SDL_GL_DEPTH_SIZE, set_depth_size, depth_size, u8,
            "the minimum number of bits in the depth buffer; defaults to 16"),

        (SDL_GL_STENCIL_SIZE, set_stencil_size, stencil_size, u8,
            "the minimum number of bits in the stencil buffer; defaults to 0"),

        (SDL_GL_ACCUM_RED_SIZE, set_accum_red_size, accum_red_size, u8,
            "the minimum number of bits for the red channel of the accumulation buffer; defaults to 0"),

        (SDL_GL_ACCUM_GREEN_SIZE, set_accum_green_size, accum_green_size, u8,
            "the minimum number of bits for the green channel of the accumulation buffer; defaults to 0"),

        (SDL_GL_ACCUM_BLUE_SIZE, set_accum_blue_size, accum_blue_size, u8,
            "the minimum number of bits for the blue channel of the accumulation buffer; defaults to 0"),

        (SDL_GL_ACCUM_ALPHA_SIZE, set_accum_alpha_size, accum_alpha_size, u8,
            "the minimum number of bits for the alpha channel of the accumulation buffer; defaults to 0"),

        (SDL_GL_STEREO, set_stereo, stereo, bool,
            "whether the output is stereo 3D; defaults to off"),

        (SDL_GL_MULTISAMPLEBUFFERS, set_multisample_buffers, multisample_buffers, u8,
            "the number of buffers used for multisample anti-aliasing; defaults to 0"),

        (SDL_GL_MULTISAMPLESAMPLES, set_multisample_samples, multisample_samples, u8,
            "the number of samples used around the current pixel used for multisample anti-aliasing; defaults to 0"),

        (SDL_GL_ACCELERATED_VISUAL, set_accelerated_visual, accelerated_visual, bool,
            "whether to require hardware acceleration; false to force software rendering; defaults to allow either"),

        (SDL_GL_CONTEXT_MAJOR_VERSION, set_context_major_version, context_major_version, u8,
            "OpenGL context major version"),

        (SDL_GL_CONTEXT_MINOR_VERSION, set_context_minor_version, context_minor_version, u8,
            "OpenGL context minor version"),

        (SDL_GL_CONTEXT_PROFILE_MASK, set_context_profile, context_profile, GLProfile,
            "type of GL context (Core, Compatibility, ES)"),

        (SDL_GL_SHARE_WITH_CURRENT_CONTEXT, set_share_with_current_context, share_with_current_context, bool,
            "OpenGL context sharing; defaults to false"),

        (SDL_GL_FRAMEBUFFER_SRGB_CAPABLE, set_framebuffer_srgb_compatible, framebuffer_srgb_compatible, bool,
            "requests sRGB capable visual; defaults to false (>= SDL 2.0.1)"),

        (SDL_GL_CONTEXT_NO_ERROR, set_context_no_error, context_no_error, bool,
            "disables OpenGL error checking; defaults to false (>= SDL 2.0.6)")
    }

    /// **Sets** the OpenGL context major and minor versions.
    #[inline]
    pub fn set_context_version(&self, major: u8, minor: u8) {
        self.set_context_major_version(major);
        self.set_context_minor_version(minor);
    }

    /// **Gets** the OpenGL context major and minor versions as a tuple.
    #[inline]
    pub fn context_version(&self) -> (u8, u8) {
        (self.context_major_version(), self.context_minor_version())
    }
}

/// The type that allows you to build a OpenGL context configuration.
pub struct ContextFlagsBuilder<'a> {
    flags: i32,
    _marker: PhantomData<&'a crate::VideoSubsystem>,
}

impl<'a> ContextFlagsBuilder<'a> {
    /// Finishes the builder and applies the GL context flags to the GL context.
    #[inline]
    pub fn set(&self) {
        gl_set_attribute!(SDL_GL_CONTEXT_FLAGS, self.flags);
    }

    /// Sets the context into "debug" mode.
    #[inline]
    pub fn debug(&mut self) -> &mut ContextFlagsBuilder<'a> {
        self.flags |= 0x0001;
        self
    }

    /// Sets the context into "forward compatible" mode.
    #[inline]
    pub fn forward_compatible(&mut self) -> &mut ContextFlagsBuilder<'a> {
        self.flags |= 0x0002;
        self
    }

    #[inline]
    pub fn robust_access(&mut self) -> &mut ContextFlagsBuilder<'a> {
        self.flags |= 0x0004;
        self
    }

    #[inline]
    pub fn reset_isolation(&mut self) -> &mut ContextFlagsBuilder<'a> {
        self.flags |= 0x0008;
        self
    }
}

pub struct ContextFlags {
    flags: i32,
}

impl ContextFlags {
    #[inline]
    pub const fn has_debug(&self) -> bool {
        self.flags & 0x0001 != 0
    }

    #[inline]
    pub const fn has_forward_compatible(&self) -> bool {
        self.flags & 0x0002 != 0
    }

    #[inline]
    pub const fn has_robust_access(&self) -> bool {
        self.flags & 0x0004 != 0
    }

    #[inline]
    pub const fn has_reset_isolation(&self) -> bool {
        self.flags & 0x0008 != 0
    }
}

impl<'a> GLAttr<'a> {
    /// **Sets** any combination of OpenGL context configuration flags.
    ///
    /// Note that calling this will reset any existing context flags.
    ///
    /// # Example
    /// ```no_run
    /// let sdl_context = sdl2::init().unwrap();
    /// let video_subsystem = sdl_context.video().unwrap();
    /// let gl_attr = video_subsystem.gl_attr();
    ///
    /// // Sets the GL context into debug mode.
    /// gl_attr.set_context_flags().debug().set();
    /// ```
    pub fn set_context_flags(&self) -> ContextFlagsBuilder {
        ContextFlagsBuilder {
            flags: 0,
            _marker: PhantomData,
        }
    }

    /// **Gets** the applied OpenGL context configuration flags.
    ///
    /// # Example
    /// ```no_run
    /// let sdl_context = sdl2::init().unwrap();
    /// let video_subsystem = sdl_context.video().unwrap();
    /// let gl_attr = video_subsystem.gl_attr();
    ///
    /// // Is the GL context in debug mode?
    /// if gl_attr.context_flags().has_debug() {
    ///     println!("Debug mode");
    /// }
    /// ```
    pub fn context_flags(&self) -> ContextFlags {
        let flags = gl_get_attribute!(SDL_GL_CONTEXT_FLAGS);

        ContextFlags { flags }
    }
}
