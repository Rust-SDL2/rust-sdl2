extern crate raw_window_handle;

use self::raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle,
    RawWindowHandle, WindowHandle,
};
use crate::{sys::SDL_Window, video::Window};

impl HasWindowHandle for Window {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        use self::SDL_SYSWM_TYPE::*;

        // Check if running on web before continuing,
        // since SDL_GetWindowWMInfo will fail on emscripten
        if cfg!(target_os = "emscripten") {
            use self::raw_window_handle::WebWindowHandle;
            let handle = WebWindowHandle::new(1);
            let handle = RawWindowHandle::Web(handle);
            let handle = unsafe { WindowHandle::borrow_raw(handle) };
            return Ok(handle);
        }

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
                use self::raw_window_handle::Win32WindowHandle;

                let hwnd = unsafe { wm_info.info.win }.window as *mut isize;
                let hwnd = core::num::NonZeroIsize::new(hwnd as isize);
                let hwnd = match hwnd {
                    Some(hwnd) => hwnd,
                    None => {
                        return Err(HandleError::Unavailable);
                    }
                };
                let mut handle = Win32WindowHandle::new(hwnd);
                let hinstance = unsafe { wm_info.info.win }.hinstance as *mut libc::c_void;
                let hinstance = core::num::NonZeroIsize::new(hinstance as isize);
                handle.hinstance = hinstance;

                let handle = RawWindowHandle::Win32(handle);
                let handle = unsafe { WindowHandle::borrow_raw(handle) };
                Ok(handle)
            }
            #[cfg(target_os = "windows")]
            SDL_SYSWM_WINRT => {
                use self::raw_window_handle::WinRtWindowHandle;

                let core_window = unsafe { wm_info.info.winrt }.core_window;
                let core_window = core::ptr::NonNull::<libc::c_void>::new(core_window);
                let core_window = match core_window {
                    Some(cw) => cw,
                    None => {
                        return Err(HandleError::Unavailable);
                    }
                };
                let handle = WinRtWindowHandle::new(core_window);
                let handle = RawWindowHandle::WinRt(handle);
                let handle = unsafe { WindowHandle::borrow_raw(handle) };

                Ok(handle)
            }
            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
            ))]
            SDL_SYSWM_WAYLAND => {
                use self::raw_window_handle::WaylandWindowHandle;

                let surf = unsafe { wm_info.info.wl }.surface as *mut libc::c_void;
                let surf = core::ptr::NonNull::<libc::c_void>::new(surf);

                match surf {
                    Some(surf) => {
                        let handle = WaylandWindowHandle::new(surf);
                        let handle = RawWindowHandle::Wayland(handle);
                        let handle = unsafe { WindowHandle::borrow_raw(handle) };
                        Ok(handle)
                    }
                    None => Err(HandleError::Unavailable),
                }
            }
            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
            ))]
            SDL_SYSWM_X11 => {
                use self::raw_window_handle::XlibWindowHandle;

                let window = unsafe { wm_info.info.x11 }.window;
                let xlib_handle = XlibWindowHandle::new(window);
                let raw_handle = RawWindowHandle::Xlib(xlib_handle);
                let handle = unsafe { WindowHandle::borrow_raw(raw_handle) };

                Ok(handle)
            }
            #[cfg(target_os = "macos")]
            SDL_SYSWM_COCOA => {
                use self::raw_window_handle::AppKitWindowHandle;

                let ns_view = core::ptr::NonNull::<libc::c_void>::new(self.context().metal_view);
                let ns_view = match ns_view {
                    Some(nv) => nv,
                    None => {
                        panic!("metal_view not initialized, please call WindowBuilder::metal_view() when building the window");
                    }
                };

                let handle = AppKitWindowHandle::new(ns_view);
                let handle = RawWindowHandle::AppKit(handle);
                let handle = unsafe { WindowHandle::borrow_raw(handle) };

                Ok(handle)
            }
            #[cfg(any(target_os = "ios"))]
            SDL_SYSWM_UIKIT => {
                use self::raw_window_handle::UiKitWindowHandle;

                let ui_window = unsafe { wm_info.info.uikit }.window as *mut libc::c_void;
                let ui_window = core::ptr::NonNull::<libc::c_void>::new(ui_window);
                let ui_window = match ui_window {
                    Some(uiw) => uiw,
                    None => {
                        return Err(HandleError::Unavailable);
                    }
                };

                // https://developer.apple.com/documentation/uikit/uiwindow
                // If this doesn't work then the actual fix is a lot more involved
                // Someone with an IOS device please test
                let handle = UiKitWindowHandle::new(ui_window);
                let handle = RawWindowHandle::UiKit(handle);
                let handle = unsafe { WindowHandle::borrow_raw(handle) };
                Ok(handle)
            }
            #[cfg(any(target_os = "android"))]
            SDL_SYSWM_ANDROID => {
                use self::raw_window_handle::AndroidNdkWindowHandle;

                let a_native_window = unsafe { wm_info.info.android }.window as *mut libc::c_void;
                let a_native_window = core::ptr::NonNull::<libc::c_void>::new(a_native_window);
                let a_native_window = match a_native_window {
                    Some(anw) => anw,
                    None => {
                        return Err(HandleError::Unavailable);
                    }
                };
                let handle = AndroidNdkWindowHandle::new(a_native_window);
                let handle = RawWindowHandle::AndroidNdk(handle);
                let handle = unsafe { WindowHandle::borrow_raw(handle) };
                Ok(handle)
            }
            x => {
                let window_system = match x {
                    SDL_SYSWM_DIRECTFB => "DirectFB",
                    SDL_SYSWM_MIR => "Mir",
                    SDL_SYSWM_VIVANTE => "Vivante",
                    _ => unreachable!(),
                };
                panic!("{} window system is not supported, please file issue with raw-window-handle: https://github.com/rust-windowing/raw-window-handle/issues/new", window_system);
            }
        }
    }
}

impl HasDisplayHandle for Window {
    #[doc(alias = "SDL_GetVersion")]
    fn display_handle(&self) -> Result<DisplayHandle, HandleError> {
        use self::SDL_SYSWM_TYPE::*;

        // Check if running on web before continuing,
        // since SDL_GetWindowWMInfo will fail on emscripten
        if cfg!(target_os = "emscripten") {
            use self::raw_window_handle::WebDisplayHandle;
            let handle = WebDisplayHandle::new();
            let handle = RawDisplayHandle::Web(handle);
            let handle = unsafe { DisplayHandle::borrow_raw(handle) };
            return Ok(handle);
        }

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
            SDL_SYSWM_WINDOWS | SDL_SYSWM_WINRT => {
                use self::raw_window_handle::WindowsDisplayHandle;

                let handle = WindowsDisplayHandle::new();
                let handle = RawDisplayHandle::Windows(handle);
                let handle = unsafe { DisplayHandle::borrow_raw(handle) };

                Ok(handle)
            }
            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
            ))]
            SDL_SYSWM_WAYLAND => {
                use self::raw_window_handle::WaylandDisplayHandle;

                let display = unsafe { wm_info.info.wl }.display as *mut libc::c_void;
                let display = core::ptr::NonNull::<libc::c_void>::new(display);
                match display {
                    Some(display) => {
                        let mut handle = WaylandDisplayHandle::new(display);
                        let handle = RawDisplayHandle::Wayland(handle);
                        let handle = unsafe { DisplayHandle::borrow_raw(handle) };
                        Ok(handle)
                    }
                    None => Err(HandleError::Unavailable),
                }
            }
            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
            ))]
            SDL_SYSWM_X11 => {
                use self::raw_window_handle::XlibDisplayHandle;

                let display = unsafe { wm_info.info.x11 }.display as *mut libc::c_void;
                let display = core::ptr::NonNull::<libc::c_void>::new(display);
                let window = unsafe { wm_info.info.x11 }.window as i32;
                let handle = XlibDisplayHandle::new(display, window);
                let handle = RawDisplayHandle::Xlib(handle);
                let handle = unsafe { DisplayHandle::borrow_raw(handle) };

                Ok(handle)
            }
            #[cfg(target_os = "macos")]
            SDL_SYSWM_COCOA => {
                use self::raw_window_handle::AppKitDisplayHandle;
                let handle = AppKitDisplayHandle::new();
                let handle = RawDisplayHandle::AppKit(handle);
                let handle = unsafe { DisplayHandle::borrow_raw(handle) };
                Ok(handle)
            }
            #[cfg(any(target_os = "ios"))]
            SDL_SYSWM_UIKIT => {
                use self::raw_window_handle::UiKitDisplayHandle;

                let handle = UiKitDisplayHandle::new();
                let handle = RawDisplayHandle::UiKit(handle);
                let handle = unsafe { DisplayHandle::borrow_raw(handle) };

                Ok(handle)
            }
            #[cfg(any(target_os = "android"))]
            SDL_SYSWM_ANDROID => {
                use self::raw_window_handle::AndroidDisplayHandle;

                let handle = AndroidDisplayHandle::new();
                let handle = RawDisplayHandle::Android(handle);
                let handle = unsafe { DisplayHandle::borrow_raw(handle) };

                Ok(handle)
            }
            x => {
                let window_system = match x {
                    SDL_SYSWM_DIRECTFB => "DirectFB",
                    SDL_SYSWM_MIR => "Mir",
                    SDL_SYSWM_VIVANTE => "Vivante",
                    _ => unreachable!(),
                };
                panic!("{} window system is not supported, please file issue with raw-window-handle: https://github.com/rust-windowing/raw-window-handle/issues/new", window_system);
            }
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

    #[cfg(target_os = "ios")]
    pub info: ios::IOSSysWMinfo,

    #[cfg(target_os = "android")]
    pub info: android::AndroidSysWMinfo,
}

#[cfg(target_os = "windows")]
pub mod windows {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union WindowsSysWMinfo {
        pub win: Win32Handles,
        pub winrt: WinRtHandles,
        pub dummy: [u8; 64usize],
        _bindgen_union_align: [u64; 8usize],
    }

    impl Default for WindowsSysWMinfo {
        fn default() -> Self {
            WindowsSysWMinfo {
                win: Win32Handles::default(),
            }
        }
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Win32Handles {
        pub window: *mut HWND,
        pub hdc: *mut HDC,
        pub hinstance: *mut HINSTANCE,
    }

    impl Default for Win32Handles {
        fn default() -> Self {
            Win32Handles {
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

    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct WinRtHandles {
        pub core_window: *mut core::ffi::c_void,
    }

    impl Default for WinRtHandles {
        fn default() -> Self {
            WinRtHandles {
                core_window: core::ptr::null_mut(),
            }
        }
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

#[cfg(target_os = "ios")]
pub mod ios {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union IOSSysWMinfo {
        pub uikit: UiKitInfo,
        pub dummy: [u8; 64usize],
        _bindgen_union_align: [u64; 8usize],
    }

    impl Default for IOSSysWMinfo {
        fn default() -> Self {
            IOSSysWMinfo {
                uikit: UiKitInfo::default(),
            }
        }
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct UiKitInfo {
        pub window: *mut UIWindow,
    }

    impl Default for UiKitInfo {
        fn default() -> Self {
            UiKitInfo {
                window: 0 as *mut UIWindow,
            }
        }
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct UIWindow {
        _unused: [u8; 0],
    }
}

#[cfg(target_os = "android")]
pub mod android {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union AndroidSysWMinfo {
        pub android: AndroidInfo,
        pub dummy: [u8; 64usize],
        _bindgen_union_align: [u64; 8usize],
    }

    impl Default for AndroidSysWMinfo {
        fn default() -> Self {
            AndroidSysWMinfo {
                android: AndroidInfo::default(),
            }
        }
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct AndroidInfo {
        pub window: *mut ANativeWindow,
    }

    impl Default for AndroidInfo {
        fn default() -> Self {
            AndroidInfo {
                window: 0 as *mut ANativeWindow,
            }
        }
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ANativeWindow {
        _unused: [u8; 0],
    }
}
