extern crate raw_window_handle;

use self::raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use crate::{sys::SDL_Window, video::Window};

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use self::SDL_SYSWM_TYPE::*;

        let mut wm_info: SDL_SysWMinfo = unsafe { std::mem::zeroed() };

        // Make certain to retrieve version before querying `SDL_GetWindowWMInfo`
        // as that gives an error on certain systems
        unsafe {
            sys::SDL_GetVersion(&mut wm_info.version);
            if SDL_GetWindowWMInfo(self.raw(), &mut wm_info) == SDL_bool::SDL_FALSE {
                panic!("Couldn't get SDL window info: {}", crate::get_error());
            }
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
            SDL_SYSWM_ANDROID | SDL_SYSWM_UIKIT => {
                let window_system = match wm_info.subsystem {
                    SDL_SYSWM_ANDROID => "Android",
                    SDL_SYSWM_UIKIT => "iOS",
                    _ => unreachable!(),
                };
                panic!("raw-window-handle support for {} not yet implemented", window_system);
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

extern "C" {
    fn SDL_GetWindowWMInfo(window: *mut SDL_Window, info: *mut SDL_SysWMinfo) -> SDL_bool;
}


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types, dead_code)]
pub enum SDL_bool {
    SDL_FALSE = 0,
    SDL_TRUE = 1,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types, dead_code)]
pub enum SDL_SYSWM_TYPE {
    SDL_SYSWM_UNKNOWN = 0,
    SDL_SYSWM_WINDOWS = 1,
    SDL_SYSWM_X11 = 2,
    SDL_SYSWM_DIRECTFB = 3,
    SDL_SYSWM_COCOA = 4,
    SDL_SYSWM_UIKIT = 5,
    SDL_SYSWM_WAYLAND = 6,
    SDL_SYSWM_MIR = 7,
    SDL_SYSWM_WINRT = 8,
    SDL_SYSWM_ANDROID = 9,
    SDL_SYSWM_VIVANTE = 10,
    SDL_SYSWM_OS2 = 11,
}

impl Default for SDL_SYSWM_TYPE {
    fn default() -> Self {
        SDL_SYSWM_TYPE::SDL_SYSWM_UNKNOWN
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct SDL_SysWMinfo {
    pub version: sys::SDL_version,
    pub subsystem: SDL_SYSWM_TYPE,

    #[cfg(target_os = "windows")]
    pub info: windows::WindowsSysWMinfo,

    #[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
    ))]
    pub info: linux::LinuxSysWMinfo,

    #[cfg(target_os = "macos")]
    pub info: macos::MacOSSysWMinfo,
}

#[cfg(target_os = "windows")]
pub mod windows {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union WindowsSysWMinfo {
        pub win: Handles,
        pub dummy: [u8; 64usize],
        _bindgen_union_align: [u64; 8usize],
    }

    impl Default for WindowsSysWMinfo {
        fn default() -> Self {
            WindowsSysWMinfo {
                win: Handles::default(),
            }
        }
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Handles {
        pub window: *mut HWND,
        pub hdc: *mut HDC,
        pub hinstance: *mut HINSTANCE,
    }

    impl Default for Handles {
        fn default() -> Self {
            Handles {
                window: 0 as *mut HWND,
                hdc: 0 as *mut HDC,
                hinstance: 0 as *mut HINSTANCE,
            }
        }
    }

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct HWND {
        pub unused: libc::c_int,
    }

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct HDC {
        pub unused: libc::c_int,
    }

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct HINSTANCE {
        pub unused: libc::c_int,
    }
}

#[cfg(any(
target_os = "linux",
target_os = "dragonfly",
target_os = "freebsd",
target_os = "netbsd",
target_os = "openbsd",
))]
pub mod linux {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union LinuxSysWMinfo {
        pub x11: X11Info,
        pub wl: WaylandInfo,
        pub dummy: [u8; 64usize],
        _bindgen_union_align: [u32; 16usize],
    }

    impl Default for LinuxSysWMinfo {
        fn default() -> Self {
            LinuxSysWMinfo {
                wl: WaylandInfo::default(),
            }
        }
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct X11Info {
        pub display: *mut Display,
        pub window: Window,
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct WaylandInfo {
        pub display: *mut WLDisplay,
        pub surface: *mut WLSurface,
        pub shell_surface: *mut WLShellSurface,
    }

    impl Default for WaylandInfo {
        fn default() -> Self {
            WaylandInfo {
                display: 0 as *mut WLDisplay,
                surface: 0 as *mut WLSurface,
                shell_surface: 0 as *mut WLShellSurface,
            }
        }
    }

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct WLDisplay {
        pub _address: u8,
    }

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct WLSurface {
        pub _address: u8,
    }

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct WLShellSurface {
        pub _address: u8,
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Display {
        _unused: [u8; 0],
    }

    pub type Window = libc::c_ulong;
}

#[cfg(target_os = "macos")]
pub mod macos {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union MacOSSysWMinfo {
        pub cocoa: CocoaInfo,
        pub dummy: [u8; 64usize],
        _bindgen_union_align: [u64; 8usize],
    }

    impl Default for MacOSSysWMinfo {
        fn default() -> Self {
            MacOSSysWMinfo {
                cocoa: CocoaInfo::default(),
            }
        }
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct CocoaInfo {
        pub window: *mut NSWindow,
    }

    impl Default for CocoaInfo {
        fn default() -> Self {
            CocoaInfo {
                window: 0 as *mut NSWindow,
            }
        }
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct NSWindow {
        _unused: [u8; 0],
    }
}
