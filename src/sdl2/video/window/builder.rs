use std::ffi::CString;

use libc::{c_char, c_int};

use crate::{get_error, VideoSubsystem};

use super::pos::to_ll_windowpos;
use super::{Window, WindowPos};

mod error;
pub use self::error::WindowBuildError;

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
    subsystem: VideoSubsystem,
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
            subsystem: v.clone(),
        }
    }

    /// Builds the window.
    #[doc(alias = "SDL_CreateWindow")]
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
                self.window_flags,
            );

            if raw.is_null() {
                Err(SdlError(get_error()))
            } else {
                Ok(Window::from_ll(self.subsystem.clone(), raw))
            }
        }
    }

    /// Gets the underlying window flags.
    pub fn window_flags(&self) -> u32 {
        self.window_flags
    }

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
    pub fn opengl(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sys::SDL_WindowFlags::SDL_WINDOW_OPENGL as u32;
        self
    }

    /// Sets the window to be usable with a Vulkan instance
    pub fn vulkan(&mut self) -> &mut WindowBuilder {
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
