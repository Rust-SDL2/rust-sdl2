#[cfg(feature = "raw-window-handle")]
mod raw_window_handle_test {
    extern crate raw_window_handle;
    extern crate sdl2;

    use self::raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
    use self::sdl2::video::Window;

    #[cfg(target_os = "windows")]
    #[test]
    fn get_windows_handle() {
        let window = new_hidden_window();
        match window.raw_window_handle() {
            RawWindowHandle::Windows(windows_handle) => {
                assert_ne!(windows_handle.hwnd, 0 as *mut libc::c_void);
                println!("Successfully received Windows RawWindowHandle!");
            },
            x => assert!(false, "Received wrong RawWindowHandle type for Windows: {:?}", x),
        }
    }

    #[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
    ))]
    #[test]
    fn get_linux_handle() {
        let window = new_hidden_window();
        match window.raw_window_handle() {
            RawWindowHandle::Xlib(x11_handle) => {
                assert_ne!(x11_handle.window, 0, "Window for X11 should not be 0");
                assert_ne!(x11_handle.display, 0 as *mut libc::c_void, "Display for X11 should not be null");
                println!("Successfully received linux X11 RawWindowHandle!");
            },
            RawWindowHandle::Wayland(wayland_handle) => {
                assert_ne!(wayland_handle.surface, 0 as *mut libc::c_void, "Surface for Wayland should not be null");
                assert_ne!(wayland_handle.display, 0 as *mut libc::c_void, "Display for Wayland should not be null");
                println!("Successfully received linux Wayland RawWindowHandle!");
            },
            x => assert!(false, "Received wrong RawWindowHandle type for linux: {:?}", x),
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn get_macos_handle() {
        let window = new_hidden_window();
        match window.raw_window_handle() {
            RawWindowHandle::MacOS(macos_handle) => {
                assert_ne!(macos_handle.ns_window, 0 as *mut libc::c_void, "ns_window should not be null");
                assert_eq!(macos_handle.ns_view, 0 as *mut libc::c_void, "nw_view should be null");
                println!("Successfully received macOS RawWindowHandle!");
            },
            x => assert!(false, "Received wrong RawWindowHandle type for macOS: {:?}", x),
        };
    }

    pub fn new_hidden_window() -> Window {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();
        video_subsystem
            .window("Hello, World!", 800, 600)
            .hidden()
            .build()
            .unwrap()
    }
}
