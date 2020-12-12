use libc::{c_char, c_int};
use std::ffi::{CStr, CString};
use std::{mem, ptr};

use crate::rect::Rect;
use crate::VideoSubsystem;

use crate::get_error;

use crate::sys;

pub use crate::sys::{VkInstance, VkSurfaceKHR};

mod display_mode;
pub use self::display_mode::DisplayMode;
mod driver;
pub use self::driver::*;
mod fullscreen_type;
pub use self::fullscreen_type::FullscreenType;
pub mod gl_attr;
mod gl_context;
pub use self::gl_context::GLContext;
mod gl_profile;
pub use self::gl_profile::GLProfile;
mod swap_interval;
pub use self::swap_interval::SwapInterval;
mod window;
pub use self::window::*;

impl VideoSubsystem {
    /// Initializes a new `WindowBuilder`; a convenience method that calls `WindowBuilder::new()`.
    pub fn window(&self, title: &str, width: u32, height: u32) -> WindowBuilder {
        WindowBuilder::new(self, title, width, height)
    }

    #[doc(alias = "SDL_GetCurrentVideoDriver")]
    pub fn current_video_driver(&self) -> &'static str {
        use std::str;

        unsafe {
            let buf = sys::SDL_GetCurrentVideoDriver();
            assert!(!buf.is_null());

            str::from_utf8(CStr::from_ptr(buf as *const _).to_bytes()).unwrap()
        }
    }

    #[doc(alias = "SDL_GetNumVideoDisplays")]
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
    #[doc(alias = "SDL_GetDisplayName")]
    pub fn display_name(&self, display_index: i32) -> Result<String, String> {
        unsafe {
            let display = sys::SDL_GetDisplayName(display_index as c_int);
            if display.is_null() {
                Err(get_error())
            } else {
                Ok(CStr::from_ptr(display as *const _)
                    .to_str()
                    .unwrap()
                    .to_owned())
            }
        }
    }

    #[doc(alias = "SDL_GetDisplayBounds")]
    pub fn display_bounds(&self, display_index: i32) -> Result<Rect, String> {
        let mut out = mem::MaybeUninit::uninit();
        let result =
            unsafe { sys::SDL_GetDisplayBounds(display_index as c_int, out.as_mut_ptr()) == 0 };

        if result {
            let out = unsafe { out.assume_init() };
            Ok(Rect::from_ll(out))
        } else {
            Err(get_error())
        }
    }

    #[doc(alias = "SDL_GetNumDisplayModes")]
    pub fn num_display_modes(&self, display_index: i32) -> Result<i32, String> {
        let result = unsafe { sys::SDL_GetNumDisplayModes(display_index as c_int) };
        if result < 0 {
            Err(get_error())
        } else {
            Ok(result as i32)
        }
    }

    #[doc(alias = "SDL_GetDisplayMode")]
    pub fn display_mode(&self, display_index: i32, mode_index: i32) -> Result<DisplayMode, String> {
        let mut dm = mem::MaybeUninit::uninit();
        let result = unsafe {
            sys::SDL_GetDisplayMode(display_index as c_int, mode_index as c_int, dm.as_mut_ptr())
                == 0
        };

        if result {
            let dm = unsafe { dm.assume_init() };
            Ok(DisplayMode::from_ll(&dm))
        } else {
            Err(get_error())
        }
    }

    #[doc(alias = "SDL_GetDesktopDisplayMode")]
    pub fn desktop_display_mode(&self, display_index: i32) -> Result<DisplayMode, String> {
        let mut dm = mem::MaybeUninit::uninit();
        let result =
            unsafe { sys::SDL_GetDesktopDisplayMode(display_index as c_int, dm.as_mut_ptr()) == 0 };

        if result {
            let dm = unsafe { dm.assume_init() };
            Ok(DisplayMode::from_ll(&dm))
        } else {
            Err(get_error())
        }
    }

    #[doc(alias = "SDL_GetCurrentDisplayMode")]
    pub fn current_display_mode(&self, display_index: i32) -> Result<DisplayMode, String> {
        let mut dm = mem::MaybeUninit::uninit();
        let result =
            unsafe { sys::SDL_GetCurrentDisplayMode(display_index as c_int, dm.as_mut_ptr()) == 0 };

        if result {
            let dm = unsafe { dm.assume_init() };
            Ok(DisplayMode::from_ll(&dm))
        } else {
            Err(get_error())
        }
    }

    #[doc(alias = "SDL_GetClosestDisplayMode")]
    pub fn closest_display_mode(
        &self,
        display_index: i32,
        mode: &DisplayMode,
    ) -> Result<DisplayMode, String> {
        let input = mode.to_ll();
        let mut dm = mem::MaybeUninit::uninit();

        let result = unsafe {
            sys::SDL_GetClosestDisplayMode(display_index as c_int, &input, dm.as_mut_ptr())
        };

        if result.is_null() {
            Err(get_error())
        } else {
            let dm = unsafe { dm.assume_init() };
            Ok(DisplayMode::from_ll(&dm))
        }
    }

    /// Return a triplet `(ddpi, hdpi, vdpi)` containing the diagonal, horizontal and vertical
    /// dots/pixels-per-inch of a display
    #[doc(alias = "SDL_GetDisplayDPI")]
    pub fn display_dpi(&self, display_index: i32) -> Result<(f32, f32, f32), String> {
        let mut ddpi = 0.0;
        let mut hdpi = 0.0;
        let mut vdpi = 0.0;
        let result = unsafe {
            sys::SDL_GetDisplayDPI(display_index as c_int, &mut ddpi, &mut hdpi, &mut vdpi)
        };
        if result < 0 {
            Err(get_error())
        } else {
            Ok((ddpi, hdpi, vdpi))
        }
    }

    #[doc(alias = "SDL_IsScreenSaverEnabled")]
    pub fn is_screen_saver_enabled(&self) -> bool {
        unsafe { sys::SDL_IsScreenSaverEnabled() == sys::SDL_bool::SDL_TRUE }
    }

    #[doc(alias = "SDL_EnableScreenSaver")]
    pub fn enable_screen_saver(&self) {
        unsafe { sys::SDL_EnableScreenSaver() }
    }

    #[doc(alias = "SDL_DisableScreenSaver")]
    pub fn disable_screen_saver(&self) {
        unsafe { sys::SDL_DisableScreenSaver() }
    }

    /// Loads the default OpenGL library.
    ///
    /// This should be done after initializing the video driver, but before creating any OpenGL windows.
    /// If no OpenGL library is loaded, the default library will be loaded upon creation of the first OpenGL window.
    ///
    /// If a different library is already loaded, this function will return an error.
    #[doc(alias = "SDL_GL_LoadLibrary")]
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
    #[doc(alias = "SDL_GL_LoadLibrary")]
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
    #[doc(alias = "SDL_GL_UnloadLibrary")]
    pub fn gl_unload_library(&self) {
        unsafe {
            sys::SDL_GL_UnloadLibrary();
        }
    }

    /// Gets the pointer to the named OpenGL function.
    ///
    /// This is useful for OpenGL wrappers such as [`gl-rs`](https://github.com/bjz/gl-rs).
    #[doc(alias = "SDL_GL_GetProcAddress")]
    pub fn gl_get_proc_address(&self, procname: &str) -> *const () {
        match CString::new(procname) {
            Ok(procname) => unsafe {
                sys::SDL_GL_GetProcAddress(procname.as_ptr() as *const c_char) as *const ()
            },
            // string contains a nul byte - it won't match anything.
            Err(_) => ptr::null(),
        }
    }

    #[doc(alias = "SDL_GL_ExtensionSupported")]
    pub fn gl_extension_supported(&self, extension: &str) -> bool {
        match CString::new(extension) {
            Ok(extension) => unsafe {
                sys::SDL_GL_ExtensionSupported(extension.as_ptr() as *const c_char)
                    != sys::SDL_bool::SDL_FALSE
            },
            // string contains a nul byte - it won't match anything.
            Err(_) => false,
        }
    }

    #[doc(alias = "SDL_GL_GetCurrentWindow")]
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
    #[doc(alias = "SDL_GL_MakeCurrent")]
    pub fn gl_release_current_context(&self) -> Result<(), String> {
        let result = unsafe { sys::SDL_GL_MakeCurrent(ptr::null_mut(), ptr::null_mut()) };

        if result == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }

    #[doc(alias = "SDL_GL_SetSwapInterval")]
    pub fn gl_set_swap_interval<S: Into<SwapInterval>>(&self, interval: S) -> Result<(), String> {
        let result = unsafe { sys::SDL_GL_SetSwapInterval(interval.into() as c_int) };
        if result == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }

    #[doc(alias = "SDL_GL_GetSwapInterval")]
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
    #[doc(alias = "SDL_Vulkan_LoadLibrary")]
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
    #[doc(alias = "SDL_Vulkan_LoadLibrary")]
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
    #[doc(alias = "SDL_Vulkan_UnloadLibrary")]
    pub fn vulkan_unload_library(&self) {
        unsafe {
            sys::SDL_Vulkan_UnloadLibrary();
        }
    }

    /// Gets the pointer to the
    /// [`vkGetInstanceProcAddr`](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetInstanceProcAddr.html)
    /// Vulkan function. This function can be called to retrieve the address of other Vulkan
    /// functions.
    #[doc(alias = "SDL_Vulkan_GetVkGetInstanceProcAddr")]
    pub fn vulkan_get_proc_address_function(&self) -> Result<*const (), String> {
        let result = unsafe { sys::SDL_Vulkan_GetVkGetInstanceProcAddr() as *const () };
        if result.is_null() {
            Err(get_error())
        } else {
            Ok(result)
        }
    }
}
