use std::ffi::{CStr, CString, NulError};
use std::rc::Rc;
use std::{mem, ptr};

use libc::{c_char, c_float, c_int, c_uint};

use crate::common::validate_int;
use crate::pixels::PixelFormatEnum;
use crate::render::CanvasBuilder;
use crate::surface::SurfaceRef;
use crate::sys::{VkInstance, VkSurfaceKHR};
use crate::{get_error, EventPump, IntegerOrSdlError, VideoSubsystem};

mod builder;
pub use self::builder::WindowBuilder;
mod context;
pub use self::context::WindowContext;
mod pos;
use self::pos::to_ll_windowpos;
pub use self::pos::WindowPos;
mod surface_ref;
pub use self::surface_ref::WindowSurfaceRef;

use super::{DisplayMode, FullscreenType, GLContext};

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
        Window {
            context: Rc::new(context),
        }
    }
}

impl From<Window> for CanvasBuilder {
    fn from(window: Window) -> CanvasBuilder {
        CanvasBuilder::new(window)
    }
}

impl Window {
    #[inline]
    // this can prevent introducing UB until
    // https://github.com/rust-lang/rust-clippy/issues/5953 is fixed
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn raw(&self) -> *mut sys::SDL_Window {
        self.context.raw
    }

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
    pub fn subsystem(&self) -> &VideoSubsystem {
        &self.context.subsystem
    }

    /// Initializes a new `CanvasBuilder`; a convenience method that calls `CanvasBuilder::new()`.
    pub fn into_canvas(self) -> CanvasBuilder {
        self.into()
    }

    pub fn context(&self) -> Rc<WindowContext> {
        self.context.clone()
    }

    #[doc(alias = "SDL_GetWindowID")]
    pub fn id(&self) -> u32 {
        unsafe { sys::SDL_GetWindowID(self.context.raw) }
    }

    #[doc(alias = "SDL_GL_CreateContext")]
    pub fn gl_create_context(&self) -> Result<GLContext, String> {
        let result = unsafe { sys::SDL_GL_CreateContext(self.context.raw) };
        if result.is_null() {
            Err(get_error())
        } else {
            Ok(unsafe { GLContext::from_raw(result) })
        }
    }

    /// Set the window's OpenGL context to the current context on the thread.
    #[doc(alias = "SDL_GL_GetCurrentContext")]
    pub fn gl_set_context_to_current(&self) -> Result<(), String> {
        unsafe {
            let context_raw = sys::SDL_GL_GetCurrentContext();

            if !context_raw.is_null() && sys::SDL_GL_MakeCurrent(self.context.raw, context_raw) == 0
            {
                Ok(())
            } else {
                Err(get_error())
            }
        }
    }

    #[doc(alias = "SDL_GL_MakeCurrent")]
    pub fn gl_make_current(&self, context: &GLContext) -> Result<(), String> {
        unsafe {
            if sys::SDL_GL_MakeCurrent(self.context.raw, context.raw()) == 0 {
                Ok(())
            } else {
                Err(get_error())
            }
        }
    }

    #[doc(alias = "SDL_GL_SwapWindow")]
    pub fn gl_swap_window(&self) {
        unsafe { sys::SDL_GL_SwapWindow(self.context.raw) }
    }

    /// Get the names of the Vulkan instance extensions needed to create a surface with `vulkan_create_surface`.
    #[doc(alias = "SDL_Vulkan_GetInstanceExtensions")]
    pub fn vulkan_instance_extensions(&self) -> Result<Vec<&'static str>, String> {
        let mut count: c_uint = 0;
        if unsafe {
            sys::SDL_Vulkan_GetInstanceExtensions(self.context.raw, &mut count, ptr::null_mut())
        } == sys::SDL_bool::SDL_FALSE
        {
            return Err(get_error());
        }
        let mut names: Vec<*const c_char> = vec![ptr::null(); count as usize];
        if unsafe {
            sys::SDL_Vulkan_GetInstanceExtensions(self.context.raw, &mut count, names.as_mut_ptr())
        } == sys::SDL_bool::SDL_FALSE
        {
            return Err(get_error());
        }
        Ok(names
            .iter()
            .map(|&val| unsafe { CStr::from_ptr(val) }.to_str().unwrap())
            .collect())
    }

    /// Create a Vulkan rendering surface for a window.
    ///
    /// The `VkInstance` must be created using a prior call to the
    /// [`vkCreateInstance`](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateInstance.html)
    /// function in the Vulkan library.
    #[doc(alias = "SDL_Vulkan_CreateSurface")]
    pub fn vulkan_create_surface(&self, instance: VkInstance) -> Result<VkSurfaceKHR, String> {
        let mut surface: VkSurfaceKHR = 0;
        if unsafe { sys::SDL_Vulkan_CreateSurface(self.context.raw, instance, &mut surface) }
            == sys::SDL_bool::SDL_FALSE
        {
            Err(get_error())
        } else {
            Ok(surface)
        }
    }

    #[doc(alias = "SDL_GetWindowDisplayIndex")]
    pub fn display_index(&self) -> Result<i32, String> {
        let result = unsafe { sys::SDL_GetWindowDisplayIndex(self.context.raw) };
        if result < 0 {
            Err(get_error())
        } else {
            Ok(result as i32)
        }
    }

    #[doc(alias = "SDL_SetWindowDisplayMode")]
    pub fn set_display_mode<D>(&mut self, display_mode: D) -> Result<(), String>
    where
        D: Into<Option<DisplayMode>>,
    {
        unsafe {
            let result = sys::SDL_SetWindowDisplayMode(
                self.context.raw,
                match display_mode.into() {
                    Some(ref mode) => &mode.to_ll(),
                    None => ptr::null(),
                },
            );
            if result < 0 {
                Err(get_error())
            } else {
                Ok(())
            }
        }
    }

    #[doc(alias = "SDL_GetWindowDisplayMode")]
    pub fn display_mode(&self) -> Result<DisplayMode, String> {
        let mut dm = mem::MaybeUninit::uninit();

        let result =
            unsafe { sys::SDL_GetWindowDisplayMode(self.context.raw, dm.as_mut_ptr()) == 0 };

        if result {
            let dm = unsafe { dm.assume_init() };
            Ok(DisplayMode::from_ll(&dm))
        } else {
            Err(get_error())
        }
    }

    #[doc(alias = "SDL_GetWindowPixelFormat")]
    pub fn window_pixel_format(&self) -> PixelFormatEnum {
        use std::convert::TryFrom;
        unsafe {
            PixelFormatEnum::try_from(sys::SDL_GetWindowPixelFormat(self.context.raw) as u32)
                .unwrap()
        }
    }

    #[doc(alias = "SDL_GetWindowFlags")]
    pub fn window_flags(&self) -> u32 {
        unsafe { sys::SDL_GetWindowFlags(self.context.raw) }
    }

    #[doc(alias = "SDL_SetWindowTitle")]
    pub fn set_title(&mut self, title: &str) -> Result<(), NulError> {
        let title = CString::new(title)?;
        unsafe {
            sys::SDL_SetWindowTitle(self.context.raw, title.as_ptr() as *const c_char);
        }
        Ok(())
    }

    #[doc(alias = "SDL_GetWindowTitle")]
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
    #[doc(alias = "SDL_SetWindowIcon")]
    pub fn set_icon<S: AsRef<SurfaceRef>>(&mut self, icon: S) {
        unsafe { sys::SDL_SetWindowIcon(self.context.raw, icon.as_ref().raw()) }
    }

    //pub fn SDL_SetWindowData(window: *SDL_Window, name: *c_char, userdata: *c_void) -> *c_void; //TODO: Figure out what this does
    //pub fn SDL_GetWindowData(window: *SDL_Window, name: *c_char) -> *c_void;

    #[doc(alias = "SDL_SetWindowPosition")]
    pub fn set_position(&mut self, x: WindowPos, y: WindowPos) {
        unsafe {
            sys::SDL_SetWindowPosition(self.context.raw, to_ll_windowpos(x), to_ll_windowpos(y))
        }
    }

    #[doc(alias = "SDL_GetWindowPosition")]
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
    #[doc(alias = "SDL_GetWindowBordersSize")]
    pub fn border_size(&self) -> Result<(u16, u16, u16, u16), String> {
        let mut top: c_int = 0;
        let mut left: c_int = 0;
        let mut bottom: c_int = 0;
        let mut right: c_int = 0;
        let result = unsafe {
            sys::SDL_GetWindowBordersSize(
                self.context.raw,
                &mut top,
                &mut left,
                &mut bottom,
                &mut right,
            )
        };
        if result < 0 {
            Err(get_error())
        } else {
            Ok((top as u16, left as u16, bottom as u16, right as u16))
        }
    }

    #[doc(alias = "SDL_SetWindowSize")]
    pub fn set_size(&mut self, width: u32, height: u32) -> Result<(), IntegerOrSdlError> {
        let w = validate_int(width, "width")?;
        let h = validate_int(height, "height")?;
        unsafe {
            sys::SDL_SetWindowSize(self.context.raw, w, h);
        }
        Ok(())
    }

    #[doc(alias = "SDL_GetWindowSize")]
    pub fn size(&self) -> (u32, u32) {
        let mut w: c_int = 0;
        let mut h: c_int = 0;
        unsafe { sys::SDL_GetWindowSize(self.context.raw, &mut w, &mut h) };
        (w as u32, h as u32)
    }

    #[doc(alias = "SDL_GL_GetDrawableSize")]
    pub fn drawable_size(&self) -> (u32, u32) {
        let mut w: c_int = 0;
        let mut h: c_int = 0;
        unsafe { sys::SDL_GL_GetDrawableSize(self.context.raw, &mut w, &mut h) };
        (w as u32, h as u32)
    }

    #[doc(alias = "SDL_Vulkan_GetDrawableSize")]
    pub fn vulkan_drawable_size(&self) -> (u32, u32) {
        let mut w: c_int = 0;
        let mut h: c_int = 0;
        unsafe { sys::SDL_Vulkan_GetDrawableSize(self.context.raw, &mut w, &mut h) };
        (w as u32, h as u32)
    }

    #[doc(alias = "SDL_SetWindowMinimumSize")]
    pub fn set_minimum_size(&mut self, width: u32, height: u32) -> Result<(), IntegerOrSdlError> {
        let w = validate_int(width, "width")?;
        let h = validate_int(height, "height")?;
        unsafe {
            sys::SDL_SetWindowMinimumSize(self.context.raw, w, h);
        }
        Ok(())
    }

    #[doc(alias = "SDL_GetWindowMinimumSize")]
    pub fn minimum_size(&self) -> (u32, u32) {
        let mut w: c_int = 0;
        let mut h: c_int = 0;
        unsafe { sys::SDL_GetWindowMinimumSize(self.context.raw, &mut w, &mut h) };
        (w as u32, h as u32)
    }

    #[doc(alias = "SDL_SetWindowMaximumSize")]
    pub fn set_maximum_size(&mut self, width: u32, height: u32) -> Result<(), IntegerOrSdlError> {
        let w = validate_int(width, "width")?;
        let h = validate_int(height, "height")?;
        unsafe {
            sys::SDL_SetWindowMaximumSize(self.context.raw, w, h);
        }
        Ok(())
    }

    #[doc(alias = "SDL_GetWindowMaximumSize")]
    pub fn maximum_size(&self) -> (u32, u32) {
        let mut w: c_int = 0;
        let mut h: c_int = 0;
        unsafe { sys::SDL_GetWindowMaximumSize(self.context.raw, &mut w, &mut h) };
        (w as u32, h as u32)
    }

    #[doc(alias = "SDL_SetWindowBordered")]
    pub fn set_bordered(&mut self, bordered: bool) {
        unsafe {
            sys::SDL_SetWindowBordered(
                self.context.raw,
                if bordered {
                    sys::SDL_bool::SDL_TRUE
                } else {
                    sys::SDL_bool::SDL_FALSE
                },
            )
        }
    }

    #[doc(alias = "SDL_ShowWindow")]
    pub fn show(&mut self) {
        unsafe { sys::SDL_ShowWindow(self.context.raw) }
    }

    #[doc(alias = "SDL_HideWindow")]
    pub fn hide(&mut self) {
        unsafe { sys::SDL_HideWindow(self.context.raw) }
    }

    #[doc(alias = "SDL_RaiseWindow")]
    pub fn raise(&mut self) {
        unsafe { sys::SDL_RaiseWindow(self.context.raw) }
    }

    #[doc(alias = "SDL_MaximizeWindow")]
    pub fn maximize(&mut self) {
        unsafe { sys::SDL_MaximizeWindow(self.context.raw) }
    }

    #[doc(alias = "SDL_MinimizeWindow")]
    pub fn minimize(&mut self) {
        unsafe { sys::SDL_MinimizeWindow(self.context.raw) }
    }

    #[doc(alias = "SDL_RestoreWindow")]
    pub fn restore(&mut self) {
        unsafe { sys::SDL_RestoreWindow(self.context.raw) }
    }

    pub fn fullscreen_state(&self) -> FullscreenType {
        FullscreenType::from_window_flags(self.window_flags())
    }

    #[doc(alias = "SDL_SetWindowFullscreen")]
    pub fn set_fullscreen(&mut self, fullscreen_type: FullscreenType) -> Result<(), String> {
        unsafe {
            let result = sys::SDL_SetWindowFullscreen(self.context.raw, fullscreen_type as u32);
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
    #[doc(alias = "SDL_GetWindowSurface")]
    pub fn surface<'a>(&'a self, _e: &'a EventPump) -> Result<WindowSurfaceRef<'a>, String> {
        let raw = unsafe { sys::SDL_GetWindowSurface(self.context.raw) };

        if raw.is_null() {
            Err(get_error())
        } else {
            let surface_ref = unsafe { SurfaceRef::from_ll_mut(raw) };
            Ok(WindowSurfaceRef::new(surface_ref, self))
        }
    }

    #[doc(alias = "SDL_SetWindowGrab")]
    pub fn set_grab(&mut self, grabbed: bool) {
        unsafe {
            sys::SDL_SetWindowGrab(
                self.context.raw,
                if grabbed {
                    sys::SDL_bool::SDL_TRUE
                } else {
                    sys::SDL_bool::SDL_FALSE
                },
            )
        }
    }

    #[doc(alias = "SDL_GetWindowGrab")]
    pub fn grab(&self) -> bool {
        unsafe { sys::SDL_GetWindowGrab(self.context.raw) == sys::SDL_bool::SDL_TRUE }
    }

    #[doc(alias = "SDL_SetWindowBrightness")]
    pub fn set_brightness(&mut self, brightness: f64) -> Result<(), String> {
        unsafe {
            if sys::SDL_SetWindowBrightness(self.context.raw, brightness as c_float) == 0 {
                Ok(())
            } else {
                Err(get_error())
            }
        }
    }

    #[doc(alias = "SDL_GetWindowBrightness")]
    pub fn brightness(&self) -> f64 {
        unsafe { sys::SDL_GetWindowBrightness(self.context.raw) as f64 }
    }

    #[doc(alias = "SDL_SetWindowGammaRamp")]
    pub fn set_gamma_ramp<'a, 'b, 'c, R, G, B>(
        &mut self,
        red: R,
        green: G,
        blue: B,
    ) -> Result<(), String>
    where
        R: Into<Option<&'a [u16; 256]>>,
        G: Into<Option<&'b [u16; 256]>>,
        B: Into<Option<&'c [u16; 256]>>,
    {
        let unwrapped_red = match red.into() {
            Some(values) => values.as_ptr(),
            None => ptr::null(),
        };
        let unwrapped_green = match green.into() {
            Some(values) => values.as_ptr(),
            None => ptr::null(),
        };
        let unwrapped_blue = match blue.into() {
            Some(values) => values.as_ptr(),
            None => ptr::null(),
        };
        let result = unsafe {
            sys::SDL_SetWindowGammaRamp(
                self.context.raw,
                unwrapped_red,
                unwrapped_green,
                unwrapped_blue,
            )
        };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    #[allow(clippy::type_complexity)]
    #[doc(alias = "SDL_GetWindowGammaRamp")]
    pub fn gamma_ramp(&self) -> Result<(Vec<u16>, Vec<u16>, Vec<u16>), String> {
        let mut red: Vec<u16> = vec![0; 256];
        let mut green: Vec<u16> = vec![0; 256];
        let mut blue: Vec<u16> = vec![0; 256];
        let result = unsafe {
            sys::SDL_GetWindowGammaRamp(
                self.context.raw,
                red.as_mut_ptr(),
                green.as_mut_ptr(),
                blue.as_mut_ptr(),
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
    #[doc(alias = "SDL_SetWindowOpacity")]
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
    #[doc(alias = "SDL_GetWindowOpacity")]
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
