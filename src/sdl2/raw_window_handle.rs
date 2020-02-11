extern crate raw_window_handle;

use self::raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use crate::{sys::SDL_Window, sys::SDL_bool, video::Window};

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut wm_info: sys::SDL_SysWMinfo = unsafe { std::mem::zeroed() };
        unsafe { sys::SDL_GetVersion(&mut wm_info.version) }
        if unsafe { sys::SDL_GetWindowWMInfo(self.raw(), &mut wm_info) } == SDL_bool::SDL_FALSE {
            panic!("Couldn't get SDL window info: {}", crate::get_error());
        }
        match wm_info.subsystem {
            #[cfg(target_os = "windows")]
            SDL_SYSWM_WINDOWS => {
                use self::raw_window_handle::windows::WindowsHandle;
                RawWindowHandle::Windows(WindowsHandle {
                    hwnd: unsafe { wm_info.info.win }.window as *mut libc::c_void,
                    ..WindowsHandle::empty()
                })
            },
            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
            ))]
            SDL_SYSWM_WAYLAND => {
                use self::raw_window_handle::unix::WaylandHandle;
                RawWindowHandle::Wayland(WaylandHandle {
                    surface: unsafe { wm_info.info.wl }.surface as *mut libc::c_void,
                    display: unsafe { wm_info.info.wl }.display as *mut libc::c_void,
                    ..WaylandHandle::empty()
                })
            },
            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
            ))]
            SDL_SYSWM_X11 => {
                use self::raw_window_handle::unix::XlibHandle;
                RawWindowHandle::Xlib(XlibHandle {
                    window: unsafe { wm_info.info.x11 }.window,
                    display: unsafe { wm_info.info.x11 }.display as *mut libc::c_void,
                    ..XlibHandle::empty()
                })
            },
            #[cfg(target_os = "macos")]
            SDL_SYSWM_COCOA => {
                use self::raw_window_handle::macos::MacOSHandle;
                RawWindowHandle::MacOS(MacOSHandle {
                    ns_window: unsafe { wm_info.info.cocoa }.window as *mut libc::c_void,
                    ns_view: 0 as *mut libc::c_void, // consumer of RawWindowHandle should determine this
                    ..MacOSHandle::empty()
                })
            },
            SDL_SYSWM_ANDROID => {
                panic!("raw-window-handle support for Android not yet implemented");
            },
            SDL_SYSWM_UIKIT => {
                panic!("raw-window-handle support for iOS not yet implemented");
            },
            x => {
                let window_system = match x {
                    SDL_SYSWM_DIRECTFB => "DirectFB",
                    SDL_SYSWM_MIR => "Mir",
                    SDL_SYSWM_WINRT => "WinRT",
                    SDL_SYSWM_VIVANTE => "Vivante",
                    _ => unreachable!(),
                };
                panic!("{} window system is not supported, please file issue with raw-window-handle: https://github.com/rust-windowing/raw-window-handle/issues/new", window_system);
            },
        }
    }
}
