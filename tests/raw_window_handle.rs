#[cfg(feature = "raw-window-handle")]
mod raw_window_handle_test {
    extern crate raw_window_handle;
    extern crate sdl2;

    use self::raw_window_handle::{
        HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle,
    };
    use self::sdl2::video::Window;

    #[cfg(target_os = "windows")]
    #[test]
    fn get_windows_handle() {
        let window = new_hidden_window();
        match window.window_handle() {
            Ok(window_handle) => match window_handle.as_raw() {
                RawWindowHandle::Win32(_) => {
                    println!("Successfully received Win32 window handle")
                }
                RawWindowHandle::WinRt(_) => {
                    println!("Successfully received WinRt window handle")
                }
                raw_handle => {
                    assert!(
                        false,
                        "Wrong window handle type for Windows: {:?}",
                        raw_handle
                    )
                }
            },
            Err(e) => {
                assert!(false, "Failed to recieve window handle on Windows: {:?}", e)
            }
        }
        match window.display_handle() {
            Ok(display_handle) => match display_handle.as_raw() {
                RawDisplayHandle::Windows(_) => {
                    println!("Successfully received Windows display handle")
                }
                raw_handle => {
                    assert!(
                        false,
                        "Wrong display handle type for Windows: {:?}",
                        raw_handle
                    )
                }
            },
            Err(e) => {
                assert!(
                    false,
                    "Failed to recieve display handle on Windows: {:?}",
                    e
                )
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
    #[test]
    fn get_linux_handle() {
        let window = new_hidden_window();
        match window.window_handle() {
            Ok(handle) => match handle.as_raw() {
                RawWindowHandle::Xlib(_) => {
                    println!("Successfully received X11 window handle")
                }
                RawWindowHandle::Wayland(_) => {
                    println!("Successfully received Wayland window handle")
                }
                raw_handle => {
                    assert!(
                        false,
                        "Wrong window handle type for Linux: {:?}",
                        raw_handle
                    )
                }
            },
            Err(e) => {
                assert!(false, "Failed to recieve window handle on Linux: {:?}", e)
            }
        }
        match window.display_handle() {
            Ok(handle) => match handle.as_raw() {
                RawDisplayHandle::Xlib(_) => {
                    println!("Successfully recieved X11 display handle")
                }
                RawDisplayHandle::Wayland(_) => {
                    println!("Successfully recieved Wayland display handle")
                }
                raw_handle => {
                    assert!(
                        false,
                        "Wrong display handle type for Linux: {:?}",
                        raw_handle
                    )
                }
            },
            Err(e) => {
                assert!(false, "Failed to recieve display handle on Linux: {:?}", e)
            }
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn get_macos_handle() {
        let window = new_hidden_window();
        match window.window_handle() {
            Ok(handle) => match handle.as_raw() {
                RawWindowHandle::AppKit(_) => {
                    println!("Successfully recieved AppKit window handle")
                }
                raw_handle => {
                    assert!(
                        false,
                        "Wrong window handle type for macOS: {:?}",
                        raw_handle
                    )
                }
            },
            Err(e) => {
                assert!(false, "Failed to recieve window handle on macOS: {:?}", e)
            }
        };
        match window.display_handle() {
            Ok(handle) => match handle.as_raw() {
                RawDisplayHandle::AppKit(_) => {
                    println!("Successfully recieved AppKit display handle")
                }
                raw_handle => {
                    assert!(
                        false,
                        "Wrong display handle type for macOS: {:?}",
                        raw_handle
                    )
                }
            },
            Err(e) => {
                assert!(false, "Failed to recieve display handle on macOS: {:?}", e)
            }
        }
    }

    pub fn new_hidden_window() -> Window {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();
        video_subsystem
            .window("Hello, World!", 800, 600)
            .hidden()
            .metal_view()
            .build()
            .unwrap()
    }
}
