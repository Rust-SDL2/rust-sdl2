#[cfg(target_os = "macos")]
extern crate objc;
#[cfg(feature = "with-raw-window-handle")]
extern crate raw_window_handle;

use std::alloc::{alloc, dealloc, Layout};

use self::raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use crate::{
    sys::{SDL_bool::SDL_FALSE, SDL_GetWindowWMInfo, SDL_SYSWM_TYPE::*, SDL_SysWMinfo},
    video::Window
};

struct InfoHandle {
    layout: Layout,
    pub ptr: *mut SDL_SysWMinfo,
}

impl InfoHandle {
    pub fn new() -> InfoHandle {
        let layout = Layout::new::<SDL_SysWMinfo>();
        let ptr = unsafe { alloc(layout) as *mut SDL_SysWMinfo };
        InfoHandle { layout, ptr }
    }
}

impl Drop for InfoHandle {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ptr as *mut u8, self.layout);
        }
    }
}

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let info_handle = InfoHandle::new();
        if unsafe { SDL_GetWindowWMInfo(self.raw(), info_handle.ptr) } == SDL_FALSE {
            panic!("Couldn't get SDL window info: {}", crate::get_error());
        }
        match unsafe { *info_handle.ptr }.subsystem {
            #[cfg(target_os = "windows")]
            SDL_SYSWM_WINDOWS => {
                use self::raw_window_handle::windows::WindowsHandle;
                RawWindowHandle::Windows(WindowsHandle {
                    hwnd: unsafe { (*info_handle.ptr).info.win }.window as *mut libc::c_void,
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
                    surface: unsafe { (*info_handle.ptr).info.wl }.surface as *mut libc::c_void,
                    display: unsafe { (*info_handle.ptr).info.wl }.display as *mut libc::c_void,
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
                    window: unsafe { (*info_handle.ptr).info.x11 }.window as *mut c_void,
                    display: unsafe { (*info_handle.ptr).info.x11 }.display as *mut c_void,
                    ..XlibHandle::empty()
                })
            },
            #[cfg(target_os = "macos")]
            SDL_SYSWM_COCOA => {
                use self::raw_window_handle::macos::MacOSHandle;
                use self::objc::{msg_send, runtime::Object, sel, sel_impl};
                let ns_window = unsafe { (*info_handle.ptr).info.cocoa }.window as *mut libc::c_void;
                let ns_view = unsafe { msg_send![ns_window as *mut Object, contentView] };
                RawWindowHandle::MacOS(MacOSHandle {
                    ns_window,
                    ns_view,
                    ..MacOSHandle::empty()
                })
            },
            SDL_SYSWM_ANDROID | SDL_SYSWM_UIKIT => {
                let window_system = match unsafe { (*info_handle.ptr).subsystem } {
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
