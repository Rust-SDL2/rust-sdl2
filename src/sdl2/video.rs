use libc::{c_int, c_uint, c_float, c_char};
use std::ffi::{CStr, CString, NulError};
use std::{mem, ptr, fmt};
use std::rc::Rc;
use std::error::Error;
use std::ops::{Deref, DerefMut};
use std::convert::TryFrom;

use crate::rect::Rect;
use crate::render::CanvasBuilder;
use crate::surface::SurfaceRef;
use crate::pixels::PixelFormatEnum;
use crate::VideoSubsystem;
use crate::EventPump;
use crate::common::{validate_int, IntegerOrSdlError};

use crate::get_error;

use crate::sys;

pub use crate::sys::{VkInstance, VkSurfaceKHR};


pub struct WindowSurfaceRef<'a>(&'a mut SurfaceRef, &'a Window);

impl<'a> Deref for WindowSurfaceRef<'a> {
    type Target = SurfaceRef;

    #[inline]
    fn deref(&self) -> &SurfaceRef {
        self.0
    }
}

impl<'a> DerefMut for WindowSurfaceRef<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut SurfaceRef {
        &mut self.0
    }
}

impl<'a> WindowSurfaceRef<'a> {
    /// Updates the change made to the inner Surface to the Window it was created from.
    ///
    /// This would effectively be the theoretical equivalent of `present` from a Canvas.
    pub fn update_window(&self) -> Result<(), String> {
        unsafe {
            if sys::SDL_UpdateWindowSurface(self.1.context.raw) == 0 {
                Ok(())
            } else {
                Err(get_error())
            }
        }
    }

    /// Same as `update_window`, but only update the parts included in `rects` to the Window it was
    /// created from.
    pub fn update_window_rects(&self, rects: &[Rect]) -> Result<(), String> {
        unsafe {
            if sys::SDL_UpdateWindowSurfaceRects(self.1.context.raw, Rect::raw_slice(rects), rects.len() as c_int) == 0 {
                Ok(())
            } else {
                Err(get_error())
            }
        }
    }

    /// Gives up this WindowSurfaceRef, allowing to use the window freely again. Before being
    /// destroyed, calls `update_window` one last time.
    ///
    /// If you don't want to `update_window` one last time, simply Drop this struct. However
    /// beware, since the Surface will still be in the state you left it the next time you will
    /// call `window.surface()` again.
    pub fn finish(self) -> Result<(), String> {
        self.update_window()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GLProfile {
    /// OpenGL core profile - deprecated functions are disabled
    Core,
    /// OpenGL compatibility profile - deprecated functions are allowed
    Compatibility,
    /// OpenGL ES profile - only a subset of the base OpenGL functionality is available
    GLES,
    /// Unknown profile - SDL will tend to return 0 if you ask when no particular profile
    /// has been defined or requested.
    Unknown(i32)
}

trait GLAttrTypeUtil {
    fn to_gl_value(self) -> i32;
    fn from_gl_value(value: i32) -> Self;
}

impl GLAttrTypeUtil for u8 {
    fn to_gl_value(self) -> i32 { self as i32 }
    fn from_gl_value(value: i32) -> u8 { value as u8 }
}

impl GLAttrTypeUtil for bool {
    fn to_gl_value(self) -> i32 { if self { 1 } else { 0 } }
    fn from_gl_value(value: i32) -> bool { value != 0 }
}

impl GLAttrTypeUtil for GLProfile {
    fn to_gl_value(self) -> i32 {
        use self::GLProfile::*;

        match self {
            Unknown(i) => i,
            Core => 1,
            Compatibility => 2,
            GLES => 4,
        }
    }
    fn from_gl_value(value: i32) -> GLProfile {
        use self::GLProfile::*;

        match value {
            1 => Core,
            2 => Compatibility,
            4 => GLES,
            i => Unknown(i),
        }
    }
}

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

/// OpenGL context getters and setters
///
/// # Example
/// ```no_run
/// use sdl2::video::GLProfile;
///
/// let sdl_context = sdl2::init().unwrap();
/// let video_subsystem = sdl_context.video().unwrap();
/// let gl_attr = video_subsystem.gl_attr();
///
/// // Don't use deprecated OpenGL functions
/// gl_attr.set_context_profile(GLProfile::Core);
///
/// // Set the context into debug mode
/// gl_attr.set_context_flags().debug().set();
///
/// // Set the OpenGL context version (OpenGL 3.2)
/// gl_attr.set_context_version(3, 2);
///
/// // Enable anti-aliasing
/// gl_attr.set_multisample_buffers(1);
/// gl_attr.set_multisample_samples(4);
///
/// let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600).opengl().build().unwrap();
///
/// // Yes, we're still using the Core profile
/// assert_eq!(gl_attr.context_profile(), GLProfile::Core);
/// // ... and we're still using OpenGL 3.2
/// assert_eq!(gl_attr.context_version(), (3, 2));
/// ```
pub mod gl_attr {
    use crate::sys;
    use crate::get_error;
    use std::marker::PhantomData;
    use super::{GLProfile, GLAttrTypeUtil};

    /// OpenGL context getters and setters. Obtain with `VideoSubsystem::gl_attr()`.
    pub struct GLAttr<'a> {
        _marker: PhantomData<&'a crate::VideoSubsystem>
    }

    impl crate::VideoSubsystem {
        /// Obtains access to the OpenGL window attributes.
        pub fn gl_attr(&self) -> GLAttr {
            GLAttr {
                _marker: PhantomData
            }
        }
    }

    macro_rules! gl_set_attribute {
        ($attr:ident, $value:expr) => ({
            let result = unsafe {
                sys::SDL_GL_SetAttribute(sys::SDL_GLattr::$attr, $value)
            };

            if result != 0 {
                // Panic and print the attribute that failed.
                panic!("couldn't set attribute {}: {}", stringify!($attr), get_error());
            }
        })
    }

    macro_rules! gl_get_attribute {
        ($attr:ident) => ({
            let mut value = 0;
            let result = unsafe {
                sys::SDL_GL_GetAttribute(sys::SDL_GLattr::$attr, &mut value)
            };
            if result != 0 {
                // Panic and print the attribute that failed.
                panic!("couldn't get attribute {}: {}", stringify!($attr), get_error());
            }
            value
        })
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
        _marker: PhantomData<&'a crate::VideoSubsystem>
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
        flags: i32
    }

    impl ContextFlags {
        #[inline]
        pub const fn has_debug(&self) -> bool { self.flags & 0x0001 != 0 }

        #[inline]
        pub const fn has_forward_compatible(&self) -> bool { self.flags & 0x0002 != 0 }

        #[inline]
        pub const fn has_robust_access(&self) -> bool { self.flags & 0x0004 != 0 }

        #[inline]
        pub const fn has_reset_isolation(&self) -> bool { self.flags & 0x0008 != 0 }
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
            _marker: PhantomData
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

        ContextFlags {
            flags
        }
    }

    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DisplayMode {
    pub format: PixelFormatEnum,
    pub w: i32,
    pub h: i32,
    pub refresh_rate: i32
}

impl DisplayMode {
    pub fn new(format: PixelFormatEnum, w: i32, h: i32, refresh_rate: i32) -> DisplayMode {
        DisplayMode {
            format,
            w,
            h,
            refresh_rate
        }
    }

    pub fn from_ll(raw: &sys::SDL_DisplayMode) -> DisplayMode {
        DisplayMode::new(
            PixelFormatEnum::try_from(raw.format as u32).unwrap_or(PixelFormatEnum::Unknown),
            raw.w as i32,
            raw.h as i32,
            raw.refresh_rate as i32
        )
    }

    pub fn to_ll(&self) -> sys::SDL_DisplayMode {
        sys::SDL_DisplayMode {
            format: self.format as u32,
            w: self.w as c_int,
            h: self.h as c_int,
            refresh_rate: self.refresh_rate as c_int,
            driverdata: ptr::null_mut()
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum FullscreenType {
    Off = 0,
    True = 0x00_00_00_01,
    Desktop = 0x00_00_10_01,
}

impl FullscreenType {
    pub fn from_window_flags(window_flags:u32) -> FullscreenType {
        if window_flags & FullscreenType::Desktop as u32 == FullscreenType::Desktop as u32 {
            FullscreenType::Desktop
        } else if window_flags & FullscreenType::True as u32 == FullscreenType::True as u32  {
            FullscreenType::True
        } else {
            FullscreenType::Off
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum WindowPos {
    Undefined,
    Centered,
    Positioned(i32)
}

impl From<i32> for WindowPos {
    fn from(pos: i32) -> Self {
        WindowPos::Positioned(pos)
    }
}

fn to_ll_windowpos (pos: WindowPos) -> c_int {
    match pos {
        WindowPos::Undefined => sys::SDL_WINDOWPOS_UNDEFINED_MASK as c_int,
        WindowPos::Centered => sys::SDL_WINDOWPOS_CENTERED_MASK as c_int,
        WindowPos::Positioned(x) => x as c_int
    }
}

pub struct GLContext {
    raw: sys::SDL_GLContext
}

impl Drop for GLContext {
    fn drop(&mut self) {
        unsafe {
            sys::SDL_GL_DeleteContext(self.raw)
        }
    }
}

impl GLContext {
    /// Returns true if the OpenGL context is the current one in the thread.
    pub fn is_current(&self) -> bool {
        let current_raw = unsafe { sys::SDL_GL_GetCurrentContext() };
        self.raw == current_raw
    }
}

/// Holds a `SDL_Window`
///
/// When the `WindowContext` is dropped, it destroys the `SDL_Window`
pub struct WindowContext {
    subsystem: VideoSubsystem,
    raw: *mut sys::SDL_Window,
}

impl Drop for WindowContext {
    #[inline]
    fn drop(&mut self) {
        unsafe { sys::SDL_DestroyWindow(self.raw) };
    }
}

impl WindowContext {
    #[inline]
    /// Unsafe if the `*mut SDL_Window` is used after the `WindowContext` is dropped
    pub unsafe fn from_ll(subsystem: VideoSubsystem, raw: *mut sys::SDL_Window) -> WindowContext {
        WindowContext {
            subsystem: subsystem.clone(),
            raw,
        }
    }
}

/// Represents a setting for vsync/swap interval.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(i32)]
pub enum SwapInterval {
    Immediate = 0,
    VSync = 1,
    LateSwapTearing = -1,
}

impl From<i32> for SwapInterval {
    fn from(i: i32) -> Self {
        match i {
            -1 => SwapInterval::LateSwapTearing,
            0  => SwapInterval::Immediate,
            1  => SwapInterval::VSync,
            other => panic!("Invalid value for SwapInterval: {}; valid values are -1, 0, 1", other),
        }
    }
}

/// Represents the "shell" of a `Window`.
///
/// You can set get and set many of the `SDL_Window` properties (i.e., border, size, `PixelFormat`, etc)
///
/// However, you cannot directly access the pixels of the `Window`.
/// It needs to be converted to a `Canvas` to access the rendering functions.
///
/// Note: If a `Window` goes out of scope but it cloned its context,
/// then the `SDL_Window` will not be destroyed until there are no more references to the `WindowContext`.
/// This may happen when a `TextureCreator<Window>` outlives the `Canvas<Window>`
pub struct Window {
    context: Rc<WindowContext>,
}

impl From<WindowContext> for Window {
    fn from(context: WindowContext) -> Window {
        Window { context: Rc::new(context) }
    }
}

impl_raw_accessors!(
    (GLContext, sys::SDL_GLContext)
);

impl VideoSubsystem {
    /// Initializes a new `WindowBuilder`; a convenience method that calls `WindowBuilder::new()`.
    pub fn window(&self, title: &str, width: u32, height: u32) -> WindowBuilder {
        WindowBuilder::new(self, title, width, height)
    }

    pub fn current_video_driver(&self) -> &'static str {
        use std::str;

        unsafe {
            let buf = sys::SDL_GetCurrentVideoDriver();
            assert!(!buf.is_null());

            str::from_utf8(CStr::from_ptr(buf as *const _).to_bytes()).unwrap()
        }
    }

    pub fn num_video_displays(&self) -> Result<i32, String> {
        let result = unsafe { sys::SDL_GetNumVideoDisplays() };
        if result < 0 {
            Err(get_error())
        } else {
            Ok(result as i32)
        }
    }

    /// Get the name of the display at the index `display_name`.
    ///
    /// Will return an error if the index is out of bounds or if SDL experienced a failure; inspect
    /// the returned string for further info.
    pub fn display_name(&self, display_index: i32) -> Result<String, String> {
        unsafe {
            let display = sys::SDL_GetDisplayName(display_index as c_int);
            if display.is_null() {
                Err(get_error())
            } else {
                Ok(CStr::from_ptr(display as *const _).to_str().unwrap().to_owned())
            }
        }
    }

    pub fn display_bounds(&self, display_index: i32) -> Result<Rect, String> {
        let mut out = mem::MaybeUninit::uninit();
        let result = unsafe { sys::SDL_GetDisplayBounds(display_index as c_int, out.as_mut_ptr()) == 0 };

        if result {
            let out = unsafe { out.assume_init() };
            Ok(Rect::from_ll(out))
        } else {
            Err(get_error())
        }
    }

    pub fn num_display_modes(&self, display_index: i32) -> Result<i32, String> {
        let result = unsafe { sys::SDL_GetNumDisplayModes(display_index as c_int) };
        if result < 0 {
            Err(get_error())
        } else {
            Ok(result as i32)
        }
    }

    pub fn display_mode(&self, display_index: i32, mode_index: i32) -> Result<DisplayMode, String> {
        let mut dm = mem::MaybeUninit::uninit();
        let result = unsafe { sys::SDL_GetDisplayMode(display_index as c_int, mode_index as c_int, dm.as_mut_ptr()) == 0};

        if result {
            let dm = unsafe { dm.assume_init() };
            Ok(DisplayMode::from_ll(&dm))
        } else {
            Err(get_error())
        }
    }

    pub fn desktop_display_mode(&self, display_index: i32) -> Result<DisplayMode, String> {
        let mut dm = mem::MaybeUninit::uninit();
        let result = unsafe { sys::SDL_GetDesktopDisplayMode(display_index as c_int, dm.as_mut_ptr()) == 0};

        if result {
            let dm = unsafe { dm.assume_init() };
            Ok(DisplayMode::from_ll(&dm))
        } else {
            Err(get_error())
        }
    }

    pub fn current_display_mode(&self, display_index: i32) -> Result<DisplayMode, String> {
        let mut dm = mem::MaybeUninit::uninit();
        let result = unsafe { sys::SDL_GetCurrentDisplayMode(display_index as c_int, dm.as_mut_ptr()) == 0};

        if result {
            let dm = unsafe { dm.assume_init() };
            Ok(DisplayMode::from_ll(&dm))
        } else {
            Err(get_error())
        }
    }

    pub fn closest_display_mode(&self, display_index: i32, mode: &DisplayMode) -> Result<DisplayMode, String> {
        let input = mode.to_ll();
        let mut dm = mem::MaybeUninit::uninit();

        let result = unsafe { sys::SDL_GetClosestDisplayMode(display_index as c_int, &input, dm.as_mut_ptr()) };

        if result.is_null() {
            Err(get_error())
        } else {
            let dm = unsafe { dm.assume_init() };
            Ok(DisplayMode::from_ll(&dm))
        }
    }

    /// Return a triplet `(ddpi, hdpi, vdpi)` containing the diagonal, horizontal and vertical
    /// dots/pixels-per-inch of a display
    pub fn display_dpi(&self, display_index: i32) -> Result<(f32, f32, f32), String> {
        let mut ddpi = 0.0;
        let mut hdpi = 0.0;
        let mut vdpi = 0.0;
        let result = unsafe { sys::SDL_GetDisplayDPI(display_index as c_int, &mut ddpi, &mut hdpi, &mut vdpi) };
        if result < 0 {
            Err(get_error())
        } else {
            Ok((ddpi, hdpi, vdpi))
        }
    }

    pub fn is_screen_saver_enabled(&self) -> bool {
        unsafe { sys::SDL_IsScreenSaverEnabled() == sys::SDL_bool::SDL_TRUE }
    }

    pub fn enable_screen_saver(&self) {
        unsafe { sys::SDL_EnableScreenSaver() }
    }

    pub fn disable_screen_saver(&self) {
        unsafe { sys::SDL_DisableScreenSaver() }
    }

    /// Loads the default OpenGL library.
    ///
    /// This should be done after initializing the video driver, but before creating any OpenGL windows.
    /// If no OpenGL library is loaded, the default library will be loaded upon creation of the first OpenGL window.
    ///
    /// If a different library is already loaded, this function will return an error.
    pub fn gl_load_library_default(&self) -> Result<(), String> {
        unsafe {
            if sys::SDL_GL_LoadLibrary(ptr::null()) == 0 {
                Ok(())
            } else {
                Err(get_error())
            }
        }
    }

    /// Loads the OpenGL library using a platform-dependent OpenGL library name (usually a file path).
    ///
    /// This should be done after initializing the video driver, but before creating any OpenGL windows.
    /// If no OpenGL library is loaded, the default library will be loaded upon creation of the first OpenGL window.
    ///
    /// If a different library is already loaded, this function will return an error.
    pub fn gl_load_library<P: AsRef<::std::path::Path>>(&self, path: P) -> Result<(), String> {
        unsafe {
            // TODO: use OsStr::to_cstring() once it's stable
            let path = CString::new(path.as_ref().to_str().unwrap()).unwrap();
            if sys::SDL_GL_LoadLibrary(path.as_ptr() as *const c_char) == 0 {
                Ok(())
            } else {
                Err(get_error())
            }
        }
    }

    /// Unloads the current OpenGL library.
    ///
    /// To completely unload the library, this should be called for every successful load of the
    /// OpenGL library.
    pub fn gl_unload_library(&self) {
        unsafe { sys::SDL_GL_UnloadLibrary(); }
    }

    /// Gets the pointer to the named OpenGL function.
    ///
    /// This is useful for OpenGL wrappers such as [`gl-rs`](https://github.com/bjz/gl-rs).
    pub fn gl_get_proc_address(&self, procname: &str) -> *const () {
        match CString::new(procname) {
            Ok(procname) => unsafe { sys::SDL_GL_GetProcAddress(procname.as_ptr() as *const c_char) as *const () },
            // string contains a nul byte - it won't match anything.
            Err(_) => ptr::null()
        }
    }

    pub fn gl_extension_supported(&self, extension: &str) -> bool {
        match CString::new(extension) {
            Ok(extension) => unsafe { sys::SDL_GL_ExtensionSupported(extension.as_ptr() as *const c_char) != sys::SDL_bool::SDL_FALSE },
            // string contains a nul byte - it won't match anything.
            Err(_) => false
        }
    }

    pub fn gl_get_current_window_id(&self) -> Result<u32, String> {
        let raw = unsafe { sys::SDL_GL_GetCurrentWindow() };
        if raw.is_null() {
            Err(get_error())
        } else {
            let id = unsafe { sys::SDL_GetWindowID(raw) };
            Ok(id)
        }
    }

    /// Releases the thread's current OpenGL context, i.e. sets the current OpenGL context to nothing.
    pub fn gl_release_current_context(&self) -> Result<(), String> {
        let result = unsafe { sys::SDL_GL_MakeCurrent(ptr::null_mut(), ptr::null_mut()) };

        if result == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }

    pub fn gl_set_swap_interval<S: Into<SwapInterval>>(&self, interval: S) -> Result<(), String> {
        let result = unsafe { sys::SDL_GL_SetSwapInterval(interval.into() as c_int) };
        if result == 0{
            Ok(())
        } else {
            Err(get_error())
        }
    }

    pub fn gl_get_swap_interval(&self) -> SwapInterval {
        unsafe {
            let interval = sys::SDL_GL_GetSwapInterval() as i32;
            assert!(interval == -1 || interval == 0 || interval == 1);
            mem::transmute(interval)
        }
    }

    /// Loads the default Vulkan library.
    ///
    /// This should be done after initializing the video driver, but before creating any Vulkan windows.
    /// If no Vulkan library is loaded, the default library will be loaded upon creation of the first Vulkan window.
    ///
    /// If a different library is already loaded, this function will return an error.
    pub fn vulkan_load_library_default(&self) -> Result<(), String> {
        unsafe {
            if sys::SDL_Vulkan_LoadLibrary(ptr::null()) == 0 {
                Ok(())
            } else {
                Err(get_error())
            }
        }
    }

    /// Loads the Vulkan library using a platform-dependent Vulkan library name (usually a file path).
    ///
    /// This should be done after initializing the video driver, but before creating any Vulkan windows.
    /// If no Vulkan library is loaded, the default library will be loaded upon creation of the first Vulkan window.
    ///
    /// If a different library is already loaded, this function will return an error.
    pub fn vulkan_load_library<P: AsRef<::std::path::Path>>(&self, path: P) -> Result<(), String> {
        unsafe {
            // TODO: use OsStr::to_cstring() once it's stable
            let path = CString::new(path.as_ref().to_str().unwrap()).unwrap();
            if sys::SDL_Vulkan_LoadLibrary(path.as_ptr() as *const c_char) == 0 {
                Ok(())
            } else {
                Err(get_error())
            }
        }
    }

    /// Unloads the current Vulkan library.
    ///
    /// To completely unload the library, this should be called for every successful load of the
    /// Vulkan library.
    pub fn vulkan_unload_library(&self) {
        unsafe { sys::SDL_Vulkan_UnloadLibrary(); }
    }

    /// Gets the pointer to the
    /// [`vkGetInstanceProcAddr`](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetInstanceProcAddr.html)
    /// Vulkan function. This function can be called to retrieve the address of other Vulkan
    /// functions.
    pub fn vulkan_get_proc_address_function(&self) -> Result<*const (), String> {
        let result = unsafe { sys::SDL_Vulkan_GetVkGetInstanceProcAddr() as *const () };
        if result.is_null() {
            Err(get_error())
        } else {
            Ok(result)
        }
    }
}

#[derive(Debug)]
pub enum WindowBuildError {
    HeightOverflows(u32),
    WidthOverflows(u32),
    InvalidTitle(NulError),
    SdlError(String),
}

impl fmt::Display for WindowBuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::WindowBuildError::*;

        match *self {
            HeightOverflows(h) => write!(f, "Window height ({}) is too high.", h),
            WidthOverflows(w) => write!(f, "Window width ({}) is too high.", w),
            InvalidTitle(ref e) => write!(f, "Invalid window title: {}", e),
            SdlError(ref e) => write!(f, "SDL error: {}", e),
        }
    }
}

impl Error for WindowBuildError {
    fn description(&self) -> &str {
        use self::WindowBuildError::*;

        match *self {
            HeightOverflows(_) => "window height overflow",
            WidthOverflows(_) => "window width overflow",
            InvalidTitle(_) => "invalid window title",
            SdlError(ref e) => e,
        }
    }
}

/// The type that allows you to build windows.
#[derive(Debug)]
pub struct WindowBuilder {
    title: String,
    width: u32,
    height: u32,
    x: WindowPos,
    y: WindowPos,
    window_flags: u32,
    /// The window builder cannot be built on a non-main thread, so prevent cross-threaded moves and references.
    /// `!Send` and `!Sync`,
    subsystem: VideoSubsystem
}

impl WindowBuilder {
    /// Initializes a new `WindowBuilder`.
    pub fn new(v: &VideoSubsystem, title: &str, width: u32, height: u32) -> WindowBuilder {
        WindowBuilder {
            title: title.to_owned(),
            width,
            height,
            x: WindowPos::Undefined,
            y: WindowPos::Undefined,
            window_flags: 0,
            subsystem: v.clone()
        }
    }

    /// Builds the window.
    pub fn build(&self) -> Result<Window, WindowBuildError> {
        use self::WindowBuildError::*;
        let title = match CString::new(self.title.clone()) {
            Ok(t) => t,
            Err(err) => return Err(InvalidTitle(err)),
        };
        if self.width >= (1 << 31) {
            return Err(WidthOverflows(self.width));
        }
        if self.height >= (1 << 31) {
            return Err(HeightOverflows(self.width));
        }

        let raw_width = self.width as c_int;
        let raw_height = self.height as c_int;
        unsafe {
            let raw = sys::SDL_CreateWindow(
                title.as_ptr() as *const c_char,
                to_ll_windowpos(self.x),
                to_ll_windowpos(self.y),
                raw_width,
                raw_height,
                self.window_flags
            );

            if raw.is_null() {
                Err(SdlError(get_error()))
            } else {
                Ok(Window::from_ll(self.subsystem.clone(), raw))
            }
        }
    }

    /// Gets the underlying window flags.
    pub fn window_flags(&self) -> u32 { self.window_flags }

    /// Sets the underlying window flags.
    /// This will effectively undo any previous build operations, excluding window size and position.
    pub fn set_window_flags(&mut self, flags: u32) -> &mut WindowBuilder {
        self.window_flags = flags;
        self
    }

    /// Sets the window position.
    pub fn position(&mut self, x: i32, y: i32) -> &mut WindowBuilder {
        self.x = WindowPos::Positioned(x);
        self.y = WindowPos::Positioned(y);
        self
    }

    /// Centers the window.
    pub fn position_centered(&mut self) -> &mut WindowBuilder {
        self.x = WindowPos::Centered;
        self.y = WindowPos::Centered;
        self
    }

    /// Sets the window to fullscreen.
    pub fn fullscreen(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sys::SDL_WindowFlags::SDL_WINDOW_FULLSCREEN as u32;
        self
    }

    /// Sets the window to fullscreen at the current desktop resolution.
    pub fn fullscreen_desktop(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sys::SDL_WindowFlags::SDL_WINDOW_FULLSCREEN_DESKTOP as u32;
        self
    }

    /// Sets the window to be usable with an OpenGL context
    pub fn opengl(&mut self) -> & mut WindowBuilder {
        self.window_flags |= sys::SDL_WindowFlags::SDL_WINDOW_OPENGL as u32;
        self
    }

    /// Sets the window to be usable with a Vulkan instance
    pub fn vulkan(&mut self) -> & mut WindowBuilder {
        self.window_flags |= sys::SDL_WindowFlags::SDL_WINDOW_VULKAN as u32;
        self
    }

    /// Hides the window.
    pub fn hidden(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sys::SDL_WindowFlags::SDL_WINDOW_HIDDEN as u32;
        self
    }

    /// Removes the window decoration.
    pub fn borderless(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sys::SDL_WindowFlags::SDL_WINDOW_BORDERLESS as u32;
        self
    }

    /// Sets the window to be resizable.
    pub fn resizable(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sys::SDL_WindowFlags::SDL_WINDOW_RESIZABLE as u32;
        self
    }

    /// Minimizes the window.
    pub fn minimized(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sys::SDL_WindowFlags::SDL_WINDOW_MINIMIZED as u32;
        self
    }

    /// Maximizes the window.
    pub fn maximized(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sys::SDL_WindowFlags::SDL_WINDOW_MAXIMIZED as u32;
        self
    }

    /// Sets the window to have grabbed input focus.
    pub fn input_grabbed(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sys::SDL_WindowFlags::SDL_WINDOW_INPUT_GRABBED as u32;
        self
    }

    /// Creates the window in high-DPI mode if supported (>= SDL 2.0.1)
    pub fn allow_highdpi(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sys::SDL_WindowFlags::SDL_WINDOW_ALLOW_HIGHDPI as u32;
        self
    }
}

impl From<Window> for CanvasBuilder {
    fn from(window: Window) -> CanvasBuilder {
        CanvasBuilder::new(window)
    }
}

impl Window {
    #[inline]
    pub fn raw(&self) -> *mut sys::SDL_Window { self.context.raw }

    #[inline]
    pub unsafe fn from_ll(subsystem: VideoSubsystem, raw: *mut sys::SDL_Window) -> Window {
        let context = WindowContext::from_ll(subsystem, raw);
        context.into()
    }

    #[inline]
    /// Create a new `Window` without taking ownership of the `WindowContext`
    pub const unsafe fn from_ref(context: Rc<WindowContext>) -> Window {
        Window { context }
    }

    #[inline]
    pub fn subsystem(&self) -> &VideoSubsystem { &self.context.subsystem }

    /// Initializes a new `CanvasBuilder`; a convenience method that calls `CanvasBuilder::new()`.
    pub fn into_canvas(self) -> CanvasBuilder {
        self.into()
    }

    pub fn context(&self) -> Rc<WindowContext> {
        self.context.clone()
    }

    pub fn id(&self) -> u32 {
        unsafe { sys::SDL_GetWindowID(self.context.raw) }
    }

    pub fn gl_create_context(&self) -> Result<GLContext, String> {
        let result = unsafe { sys::SDL_GL_CreateContext(self.context.raw) };
        if result.is_null() {
            Err(get_error())
        } else {
            Ok(GLContext{ raw: result })
        }
    }

    /// Set the window's OpenGL context to the current context on the thread.
    pub fn gl_set_context_to_current(&self) -> Result<(), String> {
        unsafe {
            let context_raw = sys::SDL_GL_GetCurrentContext();

            if !context_raw.is_null() && sys::SDL_GL_MakeCurrent(self.context.raw, context_raw) == 0 {
                Ok(())
            } else {
                Err(get_error())
            }
        }
    }

    pub fn gl_make_current(&self, context: &GLContext) -> Result<(), String> {
        unsafe {
            if sys::SDL_GL_MakeCurrent(self.context.raw, context.raw) == 0 {
                Ok(())
            } else {
                Err(get_error())
            }
        }
    }

    pub fn gl_swap_window(&self) {
        unsafe { sys::SDL_GL_SwapWindow(self.context.raw) }
    }

    /// Get the names of the Vulkan instance extensions needed to create a surface with `vulkan_create_surface`.
    pub fn vulkan_instance_extensions(&self) -> Result<Vec<&'static str>, String> {
        let mut count: c_uint = 0;
        if unsafe { sys::SDL_Vulkan_GetInstanceExtensions(self.context.raw, &mut count, ptr::null_mut()) } == sys::SDL_bool::SDL_FALSE {
            return Err(get_error());
        }
        let mut names: Vec<*const c_char> = vec![ptr::null(); count as usize];
        if unsafe { sys::SDL_Vulkan_GetInstanceExtensions(self.context.raw, &mut count, names.as_mut_ptr()) } == sys::SDL_bool::SDL_FALSE {
            return Err(get_error());
        }
        Ok(names.iter().map(|&val| unsafe { CStr::from_ptr(val) }.to_str().unwrap()).collect())
    }

    /// Create a Vulkan rendering surface for a window.
    ///
    /// The `VkInstance` must be created using a prior call to the
    /// [`vkCreateInstance`](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateInstance.html)
    /// function in the Vulkan library.
    pub fn vulkan_create_surface(&self, instance: VkInstance) -> Result<VkSurfaceKHR, String> {
        let mut surface: VkSurfaceKHR = 0;
        if unsafe { sys::SDL_Vulkan_CreateSurface(self.context.raw, instance, &mut surface) } == sys::SDL_bool::SDL_FALSE {
            Err(get_error())
        } else {
            Ok(surface)
        }
    }

    pub fn display_index(&self) -> Result<i32, String> {
        let result = unsafe { sys::SDL_GetWindowDisplayIndex(self.context.raw) };
        if result < 0 {
            Err(get_error())
        } else {
            Ok(result as i32)
        }
    }

    pub fn set_display_mode<D>(&mut self, display_mode: D) -> Result<(), String>
    where D: Into<Option<DisplayMode>>
    {
        unsafe {
            let result = sys::SDL_SetWindowDisplayMode(
                self.context.raw,
                match display_mode.into() {
                    Some(ref mode) => &mode.to_ll(),
                    None => ptr::null()
                }
            );
            if result < 0 {
                Err(get_error())
            } else {
                Ok(())
            }
        }
    }

    pub fn display_mode(&self) -> Result<DisplayMode, String> {
        let mut dm = mem::MaybeUninit::uninit();

        let result = unsafe {
            sys::SDL_GetWindowDisplayMode(
                self.context.raw,
                dm.as_mut_ptr(),
            ) == 0
        };

        if result {
            let dm = unsafe { dm.assume_init() };
            Ok(DisplayMode::from_ll(&dm))
        } else {
            Err(get_error())
        }
    }

    pub fn window_pixel_format(&self) -> PixelFormatEnum {
        unsafe{ PixelFormatEnum::try_from(sys::SDL_GetWindowPixelFormat(self.context.raw) as u32).unwrap() }
    }

    pub fn window_flags(&self) -> u32 {
        unsafe {
            sys::SDL_GetWindowFlags(self.context.raw)
        }
    }

    pub fn set_title(&mut self, title: &str) -> Result<(), NulError> {
        let title = CString::new(title)?;
        unsafe {
            sys::SDL_SetWindowTitle(self.context.raw, title.as_ptr() as *const c_char);
        }
        Ok(())
    }

    pub fn title(&self) -> &str {
        unsafe {
            let buf = sys::SDL_GetWindowTitle(self.context.raw);

            // The window title must be encoded in UTF-8.
            CStr::from_ptr(buf as *const _).to_str().unwrap()
        }
    }

    /// Use this function to set the icon for a window.
    ///
    /// # Example:
    /// ```compile_fail
    /// // requires "--features 'image'"
    /// use sdl2::surface::Surface;
    ///
    /// let window_icon = Surface::from_file("/path/to/icon.png")?;
    /// window.set_icon(window_icon);
    /// ```
    pub fn set_icon<S: AsRef<SurfaceRef>>(&mut self, icon: S) {
        unsafe {
            sys::SDL_SetWindowIcon(self.context.raw, icon.as_ref().raw())
        }
    }

    //pub fn SDL_SetWindowData(window: *SDL_Window, name: *c_char, userdata: *c_void) -> *c_void; //TODO: Figure out what this does
    //pub fn SDL_GetWindowData(window: *SDL_Window, name: *c_char) -> *c_void;

    pub fn set_position(&mut self, x: WindowPos, y: WindowPos) {
        unsafe {
            sys::SDL_SetWindowPosition(
                self.context.raw, to_ll_windowpos(x), to_ll_windowpos(y)
            )
        }
    }

    pub fn position(&self) -> (i32, i32) {
        let mut x: c_int = 0;
        let mut y: c_int = 0;
        unsafe { sys::SDL_GetWindowPosition(self.context.raw, &mut x, &mut y) };
        (x as i32, y as i32)
    }

    /// Use this function to get the size of a window's borders (decorations) around the client area.
    ///
    /// # Remarks
    /// This function is only supported on X11, otherwise an error is returned.
    pub fn border_size(&self) -> Result<(u16, u16, u16, u16), String> {
        let mut top: c_int = 0;
        let mut left: c_int = 0;
        let mut bottom: c_int = 0;
        let mut right: c_int = 0;
        let result = unsafe { sys::SDL_GetWindowBordersSize(self.context.raw, &mut top, &mut left, &mut bottom, &mut right) };
        if result < 0 {
            Err(get_error())
        } else {
            Ok((top as u16, left as u16, bottom as u16, right as u16))
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32)
            -> Result<(), IntegerOrSdlError> {
        let w = validate_int(width, "width")?;
        let h = validate_int(height, "height")?;
        unsafe {
            sys::SDL_SetWindowSize(self.context.raw, w, h);
        }
        Ok(())
    }

    pub fn size(&self) -> (u32, u32) {
        let mut w: c_int = 0;
        let mut h: c_int = 0;
        unsafe { sys::SDL_GetWindowSize(self.context.raw, &mut w, &mut h) };
        (w as u32, h as u32)
    }

    pub fn drawable_size(&self) -> (u32, u32) {
        let mut w: c_int = 0;
        let mut h: c_int = 0;
        unsafe { sys::SDL_GL_GetDrawableSize(self.context.raw, &mut w, &mut h) };
        (w as u32, h as u32)
    }

    pub fn vulkan_drawable_size(&self) -> (u32, u32) {
        let mut w: c_int = 0;
        let mut h: c_int = 0;
        unsafe { sys::SDL_Vulkan_GetDrawableSize(self.context.raw, &mut w, &mut h) };
        (w as u32, h as u32)
    }

    pub fn set_minimum_size(&mut self, width: u32, height: u32)
            -> Result<(), IntegerOrSdlError> {
        let w = validate_int(width, "width")?;
        let h = validate_int(height, "height")?;
        unsafe {
            sys::SDL_SetWindowMinimumSize(self.context.raw, w, h);
        }
        Ok(())
    }

    pub fn minimum_size(&self) -> (u32, u32) {
        let mut w: c_int = 0;
        let mut h: c_int = 0;
        unsafe { sys::SDL_GetWindowMinimumSize(self.context.raw, &mut w, &mut h) };
        (w as u32, h as u32)
    }

    pub fn set_maximum_size(&mut self, width: u32, height: u32)
            -> Result<(), IntegerOrSdlError> {
        let w = validate_int(width, "width")?;
        let h = validate_int(height, "height")?;
        unsafe {
            sys::SDL_SetWindowMaximumSize(self.context.raw, w, h);
        }
        Ok(())
    }

    pub fn maximum_size(&self) -> (u32, u32) {
        let mut w: c_int = 0;
        let mut h: c_int = 0;
        unsafe {
            sys::SDL_GetWindowMaximumSize(self.context.raw, &mut w, &mut h)
        };
        (w as u32, h as u32)
    }

    pub fn set_bordered(&mut self, bordered: bool) {
        unsafe {
            sys::SDL_SetWindowBordered(
                self.context.raw,
                if bordered { sys::SDL_bool::SDL_TRUE } else { sys::SDL_bool::SDL_FALSE }
            )
        }
    }

    pub fn show(&mut self) {
        unsafe { sys::SDL_ShowWindow(self.context.raw) }
    }

    pub fn hide(&mut self) {
        unsafe { sys::SDL_HideWindow(self.context.raw) }
    }

    pub fn raise(&mut self) {
        unsafe { sys::SDL_RaiseWindow(self.context.raw) }
    }

    pub fn maximize(&mut self) {
        unsafe { sys::SDL_MaximizeWindow(self.context.raw) }
    }

    pub fn minimize(&mut self) {
        unsafe { sys::SDL_MinimizeWindow(self.context.raw) }
    }

    pub fn restore(&mut self) {
        unsafe { sys::SDL_RestoreWindow(self.context.raw) }
    }

    pub fn fullscreen_state(&self) -> FullscreenType {
        FullscreenType::from_window_flags(self.window_flags())
    }

    pub fn set_fullscreen(&mut self, fullscreen_type: FullscreenType)
            -> Result<(), String> {
        unsafe {
            let result = sys::SDL_SetWindowFullscreen(
                self.context.raw, fullscreen_type as u32
            );
            if result == 0 {
                Ok(())
            } else {
                Err(get_error())
            }
        }
    }

    /// Returns a WindowSurfaceRef, which can be used like a regular Surface. This is an
    /// alternative way to the Renderer (Canvas) way to modify pixels directly in the Window.
    ///
    /// For this to happen, simply create a `WindowSurfaceRef` via this method, use the underlying
    /// Surface however you like, and when the changes of the Surface must be applied to the
    /// screen, call `update_window` if you intend to keep using the WindowSurfaceRef afterwards,
    /// or `finish` if you don't intend to use it afterwards.
    ///
    /// The Renderer way is of course much more flexible and recommended; even though you only want
    /// to support Software Rendering (which is what using Surface is), you can still create a
    /// Renderer which renders in a Software-based manner, so try to rely on a Renderer as much as
    /// possible !
    pub fn surface<'a>(&'a self, _e: &'a EventPump) -> Result<WindowSurfaceRef<'a>, String> {
        let raw = unsafe { sys::SDL_GetWindowSurface(self.context.raw) };

        if raw.is_null() {
            Err(get_error())
        } else {
            let surface_ref = unsafe { SurfaceRef::from_ll_mut(raw) };
            Ok(WindowSurfaceRef(surface_ref, self))
        }
    }

    pub fn set_grab(&mut self, grabbed: bool) {
        unsafe { sys::SDL_SetWindowGrab(self.context.raw, if grabbed { sys::SDL_bool::SDL_TRUE } else { sys::SDL_bool::SDL_FALSE }) }
    }

    pub fn grab(&self) -> bool {
        unsafe { sys::SDL_GetWindowGrab(self.context.raw) == sys::SDL_bool::SDL_TRUE }
    }

    pub fn set_brightness(&mut self, brightness: f64) -> Result<(), String> {
        unsafe {
            if sys::SDL_SetWindowBrightness(self.context.raw, brightness as c_float) == 0 {
                Ok(())
            } else {
                Err(get_error())
            }
        }
    }

    pub fn brightness(&self) -> f64 {
        unsafe { sys::SDL_GetWindowBrightness(self.context.raw) as f64 }
    }

    pub fn set_gamma_ramp<'a, 'b, 'c, R, G, B>(&mut self, red: R, green: G, blue: B) -> Result<(), String>
    where R: Into<Option<&'a [u16; 256]>>,
          G: Into<Option<&'b [u16; 256]>>,
          B: Into<Option<&'c [u16; 256]>>,
    {
        let unwrapped_red = match red.into() {
            Some(values) => values.as_ptr(),
            None => ptr::null()
        };
        let unwrapped_green = match green.into() {
            Some(values) => values.as_ptr(),
            None => ptr::null()
        };
        let unwrapped_blue = match blue.into() {
            Some(values) => values.as_ptr(),
            None => ptr::null()
        };
        let result = unsafe {
            sys::SDL_SetWindowGammaRamp(
                self.context.raw, unwrapped_red, unwrapped_green, unwrapped_blue
            )
        };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn gamma_ramp(&self) -> Result<(Vec<u16>, Vec<u16>, Vec<u16>), String> {
        let mut red: Vec<u16> = vec![0; 256];
        let mut green: Vec<u16> = vec![0; 256];
        let mut blue: Vec<u16> = vec![0; 256];
        let result = unsafe {
            sys::SDL_GetWindowGammaRamp(
                self.context.raw, red.as_mut_ptr(), green.as_mut_ptr(),
                blue.as_mut_ptr()
            )
        };
        if result == 0 {
            Ok((red, green, blue))
        } else {
            Err(get_error())
        }
    }

    /// Set the transparency of the window. The given value will be clamped internally between
    /// `0.0` (fully transparent), and `1.0` (fully opaque).
    ///
    /// This method returns an error if opacity isn't supported by the current platform.
    pub fn set_opacity(&mut self, opacity: f32) -> Result<(), String> {
        let result = unsafe { sys::SDL_SetWindowOpacity(self.context.raw, opacity) };
        if result < 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Returns the transparency of the window, as a value between `0.0` (fully transparent), and
    /// `1.0` (fully opaque).
    ///
    /// If opacity isn't supported by the current platform, this method returns `Ok(1.0)` instead
    /// of an error.
    pub fn opacity(&self) -> Result<f32, String> {
        let mut opacity = 0.0;
        let result = unsafe { sys::SDL_GetWindowOpacity(self.context.raw, &mut opacity) };
        if result < 0 {
            Err(get_error())
        } else {
            Ok(opacity)
        }
    }
}

#[derive(Copy, Clone)]
pub struct DriverIterator {
    length: i32,
    index: i32
}

impl Iterator for DriverIterator {
    type Item = &'static str;

    #[inline]
    fn next(&mut self) -> Option<&'static str> {
        if self.index >= self.length {
            None
        } else {
            use std::str;

            unsafe {
                let buf = sys::SDL_GetVideoDriver(self.index);
                assert!(!buf.is_null());
                self.index += 1;

                Some(str::from_utf8(CStr::from_ptr(buf as *const _).to_bytes()).unwrap())
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let l = self.length as usize;
        (l, Some(l))
    }
}

impl ExactSizeIterator for DriverIterator { }

/// Gets an iterator of all video drivers compiled into the SDL2 library.
#[inline]
pub fn drivers() -> DriverIterator {
    // This function is thread-safe and doesn't require the video subsystem to be initialized.
    // The list of drivers are read-only and statically compiled into SDL2, varying by platform.

    // SDL_GetNumVideoDrivers can never return a negative value.
    DriverIterator {
        length: unsafe { sys::SDL_GetNumVideoDrivers() },
        index: 0
    }
}
